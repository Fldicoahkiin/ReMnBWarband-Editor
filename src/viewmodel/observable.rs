// 响应式数据绑定系统

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use anyhow::Result;
use tokio::sync::broadcast;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub type ObserverId = usize;
pub type ObserverCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

// 批量更新通知
#[derive(Clone)]
struct BatchUpdate<T> {
    value: T,
    timestamp: Instant,
}

// 响应式Observable值
pub struct Observable<T> {
    value: Arc<RwLock<T>>,
    observers: Arc<RwLock<HashMap<ObserverId, Box<dyn Fn(&T) + Send + Sync>>>>,
    next_id: Arc<RwLock<ObserverId>>,
    batch_sender: Option<broadcast::Sender<BatchUpdate<T>>>,
    debounce_duration: Duration,
}

impl<T> std::fmt::Debug for Observable<T> 
where 
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Observable")
            .field("value", &self.value)
            .field("observers_count", &self.observers.read().unwrap().len())
            .finish()
    }
}

impl<T> Clone for Observable<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            observers: Arc::clone(&self.observers),
            next_id: Arc::clone(&self.next_id),
            batch_sender: self.batch_sender.clone(),
            debounce_duration: self.debounce_duration,
        }
    }
}

impl<T> Observable<T> 
where 
    T: Clone + Send + Sync + 'static,
{
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial_value)),
            observers: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
            batch_sender: None,
            debounce_duration: Duration::from_millis(16), // 60fps
        }
    }
    
    pub fn with_debounce(initial_value: T, debounce_ms: u64) -> Self {
        let effective_debounce = if debounce_ms < 8 { 8 } else { debounce_ms };
        let (sender, _) = broadcast::channel(100); // 减少缓冲区大小
        let observable = Self {
            value: Arc::new(RwLock::new(initial_value)),
            observers: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
            batch_sender: Some(sender.clone()),
            debounce_duration: Duration::from_millis(effective_debounce),
        };
        
        // 启动批量处理任务
        observable.start_batch_processor(sender);
        observable
    }
    
    fn start_batch_processor(&self, sender: broadcast::Sender<BatchUpdate<T>>) {
        let observers = Arc::clone(&self.observers);
        let mut receiver = sender.subscribe();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(32)); // 30fps 减少频率
            
            loop {
                tokio::select! {
                    update = receiver.recv() => {
                        match update {
                            Ok(latest) => {
                                // 立即处理更新，减少延迟
                                let observers = observers.read().await;
                                for callback in observers.values() {
                                    callback(&latest.value);
                                }
                            }
                            Err(_) => break, // 通道关闭
                        }
                    }
                    _ = interval.tick() => {
                        // 定期清理，但不处理更新
                    }
                }
            }
        });
    }

    // 获取当前值
    pub fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }
    
    // 获取当前值的引用
    pub fn with_value<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.value.read().unwrap();
        f(&*guard)
    }

    // 设置新值并通知观察者
    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.write().unwrap();
            *value = new_value.clone();
        }
        
        if let Some(sender) = &self.batch_sender {
            let _ = sender.send(BatchUpdate {
                value: new_value,
                timestamp: Instant::now(),
            });
        } else {
            self.notify_observers_sync();
        }
    }

    // 更新值（通过闭包）
    pub fn update<F>(&self, updater: F) 
    where 
        F: FnOnce(&mut T),
    {
        let new_value = {
            let mut value = self.value.write().unwrap();
            updater(&mut *value);
            value.clone()
        };
        
        if let Some(sender) = &self.batch_sender {
            let _ = sender.send(BatchUpdate {
                value: new_value,
                timestamp: Instant::now(),
            });
        } else {
            self.notify_observers_sync();
        }
    }

    // 订阅变化
    pub fn subscribe<F>(&self, callback: F) -> ObserverId 
    where 
        F: Fn(&T) + Send + Sync + 'static,
    {
        let id = {
            let mut next_id = self.next_id.write().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let mut observers = self.observers.write().unwrap();
        observers.insert(id, Box::new(callback));

        id
    }

    // 取消订阅
    pub fn unsubscribe(&self, id: ObserverId) {
        let mut observers = self.observers.write().unwrap();
        observers.remove(&id);
    }

    // 同步通知所有观察者
    fn notify_observers_sync(&self) {
        let value = self.get();
        let observers = self.observers.read().unwrap();
        for callback in observers.values() {
            callback(&value);
        }
    }
    
    // 异步批量通知
    pub async fn notify_observers_async(&self) {
        let value = self.get();
        let observers = self.observers.read().unwrap();
        
        // 直接调用回调，避免unsafe代码
        for callback in observers.values() {
            callback(&value);
        }
    }
}


// 计算属性 - 基于其他Observable值计算的只读值
pub struct Computed<T> {
    value: Arc<RwLock<Option<T>>>,
    #[allow(dead_code)]
    dependencies: Vec<ObserverId>,
    compute_fn: Arc<dyn Fn() -> T + Send + Sync>,
}

impl<T> Computed<T> 
where 
    T: Clone + Send + Sync + 'static,
{
    pub fn new<F>(compute_fn: F) -> Self 
    where 
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            value: Arc::new(RwLock::new(None)),
            dependencies: Vec::new(),
            compute_fn: Arc::new(compute_fn),
        }
    }

    pub fn get(&self) -> T {
        {
            let value = self.value.read().unwrap();
            if let Some(cached) = value.as_ref() {
                return cached.clone();
            }
        }
        
        let mut value = self.value.write().unwrap();
        if value.is_none() {
            *value = Some((self.compute_fn)());
        }
        value.as_ref().unwrap().clone()
    }

    pub fn invalidate(&self) {
        let mut value = self.value.write().unwrap();
        *value = None;
    }
}

// 命令 - 封装用户操作
#[derive(Clone)]
pub struct Command {
    execute_fn: Arc<dyn Fn() -> Result<()> + Send + Sync>,
    can_execute_fn: Arc<dyn Fn() -> bool + Send + Sync>,
}

impl Command {
    pub fn new<F, C>(execute_fn: F, can_execute_fn: C) -> Self 
    where 
        F: Fn() -> Result<()> + Send + Sync + 'static,
        C: Fn() -> bool + Send + Sync + 'static,
    {
        Self {
            execute_fn: Arc::new(execute_fn),
            can_execute_fn: Arc::new(can_execute_fn),
        }
    }

    pub fn execute(&self) -> Result<()> {
        if self.can_execute() {
            (self.execute_fn)()
        } else {
            Err(anyhow::anyhow!("命令无法执行"))
        }
    }

    pub fn can_execute(&self) -> bool {
        (self.can_execute_fn)()
    }

    pub fn is_executing(&self) -> bool {
        // TODO：可以通过Observable<bool>来跟踪执行状态
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_observable_basic() {
        let obs = Observable::new(42);
        assert_eq!(obs.get(), 42);

        obs.set(100);
        assert_eq!(obs.get(), 100);
    }

    #[test]
    fn test_observable_subscription() {
        let obs = Observable::new(0);
        let counter = Arc::new(AtomicUsize::new(0));
        
        let counter_clone = Arc::clone(&counter);
        let _id = obs.subscribe(move |value| {
            counter_clone.store(*value, Ordering::SeqCst);
        });

        obs.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), 42);
    }
}

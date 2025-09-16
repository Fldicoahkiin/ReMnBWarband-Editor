// 响应式数据绑定系统

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use anyhow::Result;

pub type ObserverId = usize;
pub type ObserverCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

// 响应式Observable值
pub struct Observable<T> {
    value: Arc<Mutex<T>>,
    observers: Arc<Mutex<HashMap<ObserverId, Box<dyn Fn(&T) + Send + Sync>>>>,
    next_id: Arc<Mutex<ObserverId>>,
}

impl<T> std::fmt::Debug for Observable<T> 
where 
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Observable")
            .field("value", &self.value)
            .field("observers_count", &self.observers.lock().unwrap().len())
            .finish()
    }
}

impl<T> Clone for Observable<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            observers: Arc::clone(&self.observers),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

impl<T> Observable<T> 
where 
    T: Clone + Send + Sync + 'static,
{
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial_value)),
            observers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    // 获取当前值
    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    // 设置新值并通知观察者
    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.lock().unwrap();
            *value = new_value;
        }
        self.notify_observers();
    }

    // 更新值（通过闭包）
    pub fn update<F>(&self, updater: F) 
    where 
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.lock().unwrap();
            updater(&mut *value);
        }
        self.notify_observers();
    }

    // 订阅变化
    pub fn subscribe<F>(&self, callback: F) -> ObserverId 
    where 
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;

        let mut observers = self.observers.lock().unwrap();
        observers.insert(id, Box::new(callback));

        id
    }

    // 取消订阅
    pub fn unsubscribe(&self, id: ObserverId) {
        let mut observers = self.observers.lock().unwrap();
        observers.remove(&id);
    }

    // 通知所有观察者
    fn notify_observers(&self) {
        let value = self.get();
        let observers = self.observers.lock().unwrap();
        for callback in observers.values() {
            callback(&value);
        }
    }
}


// 计算属性 - 基于其他Observable值计算的只读值
pub struct Computed<T> {
    value: Arc<Mutex<Option<T>>>,
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
            value: Arc::new(Mutex::new(None)),
            dependencies: Vec::new(),
            compute_fn: Arc::new(compute_fn),
        }
    }

    pub fn get(&self) -> T {
        let mut value = self.value.lock().unwrap();
        if value.is_none() {
            *value = Some((self.compute_fn)());
        }
        value.as_ref().unwrap().clone()
    }

    pub fn invalidate(&self) {
        let mut value = self.value.lock().unwrap();
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

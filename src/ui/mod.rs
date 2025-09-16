// UI模块 - 使用Slint框架和MVVM架构

use slint::ComponentHandle;
use std::sync::Arc;
use anyhow::Result;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::viewmodel::app_viewmodel::AppViewModel;

slint::include_modules!();

// UI状态批量更新管理器
struct UiBatchUpdater {
    pending_updates: Arc<RwLock<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
    last_update: Arc<RwLock<Instant>>,
}

impl UiBatchUpdater {
    fn new() -> Self {
        Self {
            pending_updates: Arc::new(RwLock::new(HashMap::new())),
            last_update: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    async fn schedule_update<F>(&self, key: String, update_fn: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        {
            let mut updates = self.pending_updates.write().await;
            updates.insert(key, Box::new(update_fn));
        }
        
        // 防抖处理
        {
            let mut last_update = self.last_update.write().await;
            *last_update = Instant::now();
        }
        
        // 延迟执行批量更新
        let pending_updates = Arc::clone(&self.pending_updates);
        let last_update = Arc::clone(&self.last_update);
        let debounce_duration = self.debounce_duration;
        
        tokio::spawn(async move {
            tokio::time::sleep(debounce_duration).await;
            
            // 检查是否有更新的更新请求
            let current_time = Instant::now();
            let last_time = *last_update.read().await;
            
            if current_time.duration_since(last_time) >= debounce_duration {
                // 执行所有待处理的更新
                let updates = {
                    let mut pending = pending_updates.write().await;
                    let current_updates: HashMap<String, Box<dyn Fn() + Send + Sync>> = pending.drain().collect();
                    current_updates
                };
                
                for (_, update_fn) in updates {
                    update_fn();
                }
            }
        });
    }
    
    async fn start(&self, main_window: &MainWindow) {
        // ...
    }
}

// 启动UI界面
pub fn start_ui(app_viewmodel: Arc<AppViewModel>) -> Result<()> {
    let main_window = MainWindow::new()?;
    
    // 设置UI绑定和回调
    setup_ui_bindings(&main_window, &app_viewmodel)?;
    setup_app_callbacks(&main_window, &app_viewmodel)?;
    setup_state_subscriptions(&main_window, &app_viewmodel)?;
    
    // 初始化游戏检测
    if let Some(game_path) = app_viewmodel.detect_game_path() {
        println!("自动检测到游戏路径: {}", game_path);
    }
    
    // 显示窗口并运行事件循环
    main_window.show()?;
    main_window.run()?;
    
    Ok(())
}

// 设置UI绑定和回调
fn setup_ui_bindings(main_window: &MainWindow, app_viewmodel: &Arc<AppViewModel>) -> Result<()> {
    // 获取桥接器
    let app_bridge = main_window.global::<AppBridge>();
    let ui_state = main_window.global::<UiState>();
    
    // 设置初始游戏路径
    let game_path = app_viewmodel.game_path.get();
    if !game_path.is_empty() {
        app_bridge.set_game_path(game_path.into());
    }
    
    // 设置初始游戏检测状态
    app_bridge.set_game_detected(app_viewmodel.game_detected.get());
    
    // 设置初始加载状态
    app_bridge.set_is_loading(app_viewmodel.is_loading.get());
    
    // 设置初始状态消息
    app_bridge.set_status_message(app_viewmodel.status_message.get().into());
    
    // 设置初始错误消息
    if let Some(error) = &app_viewmodel.error_message.get() {
        app_bridge.set_error_message(error.clone().into());
    } else {
        app_bridge.set_error_message("".into());
    }
    
    // 设置初始页面
    let current_page = app_viewmodel.current_page.get();
    app_bridge.set_current_module(current_page.clone().into());
    let page_index = match current_page.as_str() {
        "startup" => 0,
        "editor" => 1,
        _ => 0,
    };
    ui_state.set_current_page(page_index);
    
    Ok(())
}

// 设置应用程序回调
fn setup_app_callbacks(main_window: &MainWindow, app_viewmodel: &Arc<AppViewModel>) -> Result<()> {
    let app_vm = Arc::clone(app_viewmodel);
    
    // 获取桥接器
    let app_bridge = main_window.global::<AppBridge>();
    
    // 绑定检测游戏回调
    app_bridge.on_detect_game({
        let view_model = app_vm.clone();
        move || {
            if let Err(e) = view_model.detect_game_command.execute() {
                eprintln!("检测游戏失败: {}", e);
            }
        }
    });

    // 绑定重新检测游戏回调
    app_bridge.on_redetect_game({
        let app_vm = Arc::clone(&app_viewmodel);
        move || {
            let _ = app_vm.redetect_game_command.execute();
        }
    });

    // 设置模块选择回调
    app_bridge.on_select_module({
        let app_vm = Arc::clone(&app_viewmodel);
        let window_weak = main_window.as_weak();
        move |index| {
            let modules = app_vm.modules.get();
            if let Some(module) = modules.get(index as usize) {
                app_vm.selected_module.set(module.name.clone());
                // 同时更新UI中的当前模块显示
                if let Some(window) = window_weak.upgrade() {
                    window.global::<AppBridge>().set_current_module(module.name.clone().into());
                }
            }
        }
    });

    // 浏览游戏路径回调
    main_window.global::<AppBridge>().on_browse_game_path({
        let app_vm = Arc::clone(&app_vm);
        move || {
            if let Some(path) = app_vm.browse_game_path() {
                println!("选择的游戏路径: {}", path);
            }
        }
    });
    
    // 进入编辑器回调
    main_window.global::<AppBridge>().on_enter_editor({
        let app_vm = Arc::clone(&app_vm);
        move || {
            app_vm.enter_editor();
        }
    });
    
    // 加载游戏数据回调
    main_window.global::<AppBridge>().on_load_from_game({
        let app_vm = Arc::clone(&app_vm);
        move || {
            app_vm.load_from_game();
        }
    });
    
    // 保存到游戏回调
    main_window.global::<AppBridge>().on_save_to_game({
        let app_vm = Arc::clone(&app_vm);
        move || {
            app_vm.save_to_game();
        }
    });
    
    // 物品选择回调
    main_window.global::<AppBridge>().on_select_item({
        let app_vm = Arc::clone(&app_vm);
        move |item_id| {
            app_vm.select_item(item_id.to_string());
        }
    });
    
    // 物品保存回调
    main_window.global::<AppBridge>().on_save_item({
        let app_vm = Arc::clone(&app_vm);
        move |id, name, item_type, price, weight, damage, armor| {
            if let Err(e) = app_vm.save_item(
                id.to_string(),
                name.to_string(),
                item_type.to_string(),
                price,
                weight,
                damage,
                armor
            ) {
                eprintln!("保存物品失败: {}", e);
            }
        }
    });
    
    // 窗口控制回调
    main_window.global::<WindowControlBridge>().on_minimize({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap_or_default();
            }
        }
    });
    
    main_window.global::<WindowControlBridge>().on_close({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap_or_default();
            }
        }
    });
    main_window.global::<WindowControlBridge>().on_drag_window({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(_window) = window_weak.upgrade() {
                // 在macOS上，这个功能由系统自动处理
            }
        }
    });
    
    Ok(())
}

// 设置窗口控制回调
#[allow(dead_code)]
fn setup_window_callbacks(_main_window: &MainWindow) -> Result<()> {
    // TODO: 恢复Slint窗口控制桥接器回调
    // 暂时注释掉，等待桥接器类型生成问题解决
    /*
    let window_weak = main_window.as_weak();
    
    main_window.global::<WindowControlBridge>().on_minimize_window({
        let window_weak = window_weak.clone();
        move || {
            if let Some(window) = window_weak.upgrade() {
                let _ = window.window().set_minimized(true);
            }
        }
    });
    
    main_window.global::<WindowControlBridge>().on_close_window({
        let window_weak = window_weak.clone();
        move || {
            if let Some(window) = window_weak.upgrade() {
                window.window().hide().unwrap();
            }
        }
    });
    
    main_window.global::<WindowControlBridge>().on_drag_window({
        let window_weak = window_weak.clone();
        move || {
            if let Some(window) = window_weak.upgrade() {
                // 在macOS上，这个功能由系统自动处理
            }
        }
    });
    
    main_window.global::<WindowControlBridge>().on_toggle_maximize({
        let window_weak = window_weak.clone();
        move || {
            if let Some(window) = window_weak.upgrade() {
                let is_maximized = window.window().is_maximized();
                let _ = window.window().set_maximized(!is_maximized);
            }
        }
    });
    */
    
    Ok(())
}

// 设置状态订阅
fn setup_state_subscriptions(main_window: &MainWindow, app_viewmodel: &Arc<AppViewModel>) -> Result<()> {
    let window_weak = main_window.as_weak();
    let app_vm = Arc::clone(app_viewmodel);
    // 创建批量更新管理器
    let batch_updater = Arc::new(UiBatchUpdater::new(Duration::from_millis(50)));
    batch_updater.start(main_window.as_weak());
    
    // 订阅游戏路径变化
    app_vm.game_path.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |path| {
            let window_weak = window_weak.clone();
            let path = path.clone();
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "game_path".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_game_path(path.clone().into());
                        }
                    }
                ).await;
            });
        }
    });
    
    // 订阅游戏检测状态变化
    app_vm.game_detected.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |detected| {
            let window_weak = window_weak.clone();
            let detected = *detected;
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "game_detected".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_game_detected(detected);
                        }
                    }
                ).await;
            });
        }
    });
    
    // 订阅加载状态变化
    app_vm.is_loading.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |loading| {
            let window_weak = window_weak.clone();
            let loading = *loading;
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "is_loading".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_is_loading(loading);
                        }
                    }
                ).await;
            });
        }
    });
    
    // 订阅状态消息变化
    app_vm.status_message.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |message| {
            let window_weak = window_weak.clone();
            let message = message.clone();
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "status_message".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_status_message(message.clone().into());
                        }
                    }
                ).await;
            });
        }
    });
    
    // 订阅错误消息变化
    app_vm.error_message.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |error| {
            let window_weak = window_weak.clone();
            let error_text = error.as_ref().unwrap_or(&String::new()).clone();
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "error_message".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_error_message(error_text.clone().into());
                        }
                    }
                ).await;
            });
        }
    });
    
    // 订阅物品列表变化
    app_viewmodel.items.subscribe({
        let window_weak = main_window.as_weak();
        let batch_updater = Arc::clone(&batch_updater);
        move |items| {
            let window_weak = window_weak.clone();
            let items = items.clone();
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "items_list".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            // 只在必要时重建UI列表
                            let ui_items: Vec<slint::StandardListViewItem> = items
                                .iter()
                                .map(|item| slint::StandardListViewItem::from(slint::SharedString::from(item.id.clone())))
                                .collect();
                            window.global::<AppBridge>().set_items(slint::ModelRc::new(slint::VecModel::from(ui_items)));
                        }
                    }
                ).await;
            });
        }
    });

    // 订阅模块列表变化
    app_viewmodel.modules.subscribe({
        let window_weak = main_window.as_weak();
        move |modules| {
            if let Some(window) = window_weak.upgrade() {
                let ui_modules: Vec<slint::StandardListViewItem> = modules
                    .iter()
                    .map(|module| slint::StandardListViewItem::from(slint::SharedString::from(module.name.clone())))
                    .collect();
                window.global::<AppBridge>().set_modules(slint::ModelRc::new(slint::VecModel::from(ui_modules)));
            }
        }
    });
    
    // 订阅选中物品ID变化
    app_vm.selected_item_id.subscribe({
        let window_weak = window_weak.clone();
        move |item_id| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_selected_item_id(item_id.clone().into());
            }
        }
    });
    
    // 订阅选中物品详情变化
    app_vm.selected_item.subscribe({
        let window_weak = window_weak.clone();
        move |item_option| {
            if let Some(window) = window_weak.upgrade() {
                if let Some(item) = item_option {
                    window.global::<AppBridge>().set_selected_item_name(item.name.clone().into());
                    window.global::<AppBridge>().set_selected_item_type(item.item_type.clone().into());
                    window.global::<AppBridge>().set_selected_item_price(item.price);
                    window.global::<AppBridge>().set_selected_item_weight(item.weight);
                    window.global::<AppBridge>().set_selected_item_damage(item.damage);
                    window.global::<AppBridge>().set_selected_item_armor(item.armor);
                } else {
                    // 清空选中项数据
                    window.global::<AppBridge>().set_selected_item_name("".into());
                    window.global::<AppBridge>().set_selected_item_type("".into());
                    window.global::<AppBridge>().set_selected_item_price(0);
                    window.global::<AppBridge>().set_selected_item_weight(0.0);
                    window.global::<AppBridge>().set_selected_item_damage(0);
                    window.global::<AppBridge>().set_selected_item_armor(0);
                }
            }
        }
    });
    
    // 订阅当前页面变化
    app_vm.current_page.subscribe({
        let window_weak = window_weak.clone();
        let batch_updater = Arc::clone(&batch_updater);
        move |page| {
            let window_weak = window_weak.clone();
            let page = page.clone();
            let batch_updater = Arc::clone(&batch_updater);
            
            tokio::spawn(async move {
                batch_updater.schedule_update(
                    "current_page".to_string(),
                    move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.global::<AppBridge>().set_current_module(page.clone().into());
                            let page_index = match page.as_str() {
                                "startup" => 0,
                                "editor" => 1,
                                _ => 0,
                            };
                            window.global::<UiState>().set_current_page(page_index);
                        }
                    }
                ).await;
            });
        }
    });
    
    Ok(())
}

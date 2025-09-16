// UI模块 - 使用Slint框架和MVVM架构

use slint::ComponentHandle;
use std::sync::Arc;
use anyhow::Result;

use crate::viewmodel::app_viewmodel::AppViewModel;

slint::include_modules!();

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
    app_bridge.set_current_page(current_page.clone().into());
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
    
    // 检测游戏回调
    main_window.global::<AppBridge>().on_detect_game({
        let app_vm = Arc::clone(&app_vm);
        move || {
            if let Some(game_path) = app_vm.detect_game_path() {
                println!("检测到游戏路径: {}", game_path);
            } else {
                println!("未检测到游戏路径");
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
    
    // 订阅游戏路径变化
    app_vm.game_path.subscribe({
        let window_weak = window_weak.clone();
        move |path| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_game_path(path.clone().into());
            }
        }
    });
    
    // 订阅游戏检测状态变化
    app_vm.game_detected.subscribe({
        let window_weak = window_weak.clone();
        move |detected| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_game_detected(*detected);
            }
        }
    });
    
    // 订阅加载状态变化
    app_vm.is_loading.subscribe({
        let window_weak = window_weak.clone();
        move |loading| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_is_loading(*loading);
            }
        }
    });
    
    // 订阅状态消息变化
    app_vm.status_message.subscribe({
        let window_weak = window_weak.clone();
        move |message| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_status_message(message.clone().into());
            }
        }
    });
    
    // 订阅错误消息变化
    app_vm.error_message.subscribe({
        let window_weak = window_weak.clone();
        move |error| {
            if let Some(window) = window_weak.upgrade() {
                let error_text = error.as_ref().cloned().unwrap_or_default();
                window.global::<AppBridge>().set_error_message(error_text.into());
            }
        }
    });
    
    // 订阅当前页面变化
    app_vm.current_page.subscribe({
        let window_weak = window_weak.clone();
        move |page| {
            if let Some(window) = window_weak.upgrade() {
                window.global::<AppBridge>().set_current_page(page.clone().into());
                let page_index = match page.as_str() {
                    "startup" => 0,
                    "editor" => 1,
                    _ => 0,
                };
                window.global::<UiState>().set_current_page(page_index);
            }
        }
    });
    
    Ok(())
}

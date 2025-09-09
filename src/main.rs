use std::rc::Rc;
use std::cell::RefCell;
use slint::SharedString;

mod core;
mod models;
mod utils;

use crate::core::app_manager::AppManager;
use crate::core::menu_manager::MenuManager;

slint::include_modules!();

fn update_game_paths_in_ui(ui: &MainWindow, app_manager: &Rc<RefCell<AppManager>>) {
    let am_borrow = app_manager.borrow();
    let detector = am_borrow.game_detector();
    let detector_borrow = detector.borrow();
    let installations = detector_borrow.get_installations();
    
    if !installations.is_empty() {
        let first_installation = &installations[0];
        let first_game_path = first_installation.path.display().to_string();
        
        // 自动填充检测到的游戏路径到输入框
        ui.set_selected_game(first_game_path.clone().into());
        
        // 自动加载第一个游戏的模组列表
        let module_names: Vec<slint::SharedString> = first_installation.modules.iter()
            .map(|module| module.name.clone().into())
            .collect();
        let model = std::rc::Rc::new(slint::VecModel::from(module_names));
        ui.set_available_modules(slint::ModelRc::from(model));
        
        // 计算总剧本数量
        let total_modules: usize = installations.iter().map(|inst| inst.modules.len()).sum();
        ui.set_detection_in_progress(false);
        
        // 显示通知弹窗
        ui.set_notification_message(format!("检测到游戏安装，共 {} 个剧本", total_modules).into());
        ui.set_show_notification(true);
        
        // 3秒后自动隐藏通知
        let ui_weak = ui.as_weak();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_show_notification(false);
            }
        });
        
        tracing::info!("自动选择游戏路径: {}", first_game_path);
        tracing::info!("自动加载 {} 个模组", first_installation.modules.len());
    } else {
        ui.set_detection_status("未检测到游戏，请手动选择目录".into());
        ui.set_detection_in_progress(false);
    }
    
    tracing::info!("更新游戏路径列表，共{}个路径", installations.len());
}

fn main() -> Result<(), slint::PlatformError> {
    tracing_subscriber::fmt::init();
    
    let ui = MainWindow::new()?;
    let app_manager = Rc::new(RefCell::new(AppManager::new()));
    
    {
        let mgr = app_manager.borrow_mut();
        mgr.initialize_python().expect("Python初始化失败");
    }
    
    ui.set_detection_status("正在检测游戏...".into());
    ui.set_detection_in_progress(true);
    
    {
        let mut mgr = app_manager.borrow_mut();
        let detector = mgr.game_detector_mut();
        let mut detector_borrow = detector.borrow_mut();
        
        match detector_borrow.detect_installations() {
            Ok(_games) => {
                drop(detector_borrow);
                drop(mgr);
                update_game_paths_in_ui(&ui, &app_manager);
            }
            Err(e) => {
                tracing::error!("启动时游戏检测失败: {}", e);
                ui.set_detection_status("游戏检测失败，请手动选择目录".into());
                ui.set_detection_in_progress(false);
            }
        }
    }
    
    // 设置欢迎界面回调
    let ui_handle = ui.as_weak();
    let am = app_manager.clone();
    ui.on_detect_games(move || {
        tracing::info!("手动检测游戏");
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_detection_status("正在检测游戏...".into());
            ui.set_detection_in_progress(true);
        }
        
        let mut am_borrow = am.borrow_mut();
        let detector = am_borrow.game_detector_mut();
        let mut detector_borrow = detector.borrow_mut();
        
        match detector_borrow.detect_installations() {
            Ok(games) => {
                tracing::info!("检测到 {} 个游戏", games.len());
                
                // 更新UI中的游戏路径列表
                if let Some(ui) = ui_handle.upgrade() {
                    drop(detector_borrow);
                    drop(am_borrow); // 释放借用
                    update_game_paths_in_ui(&ui, &am);
                }
                
                let game_names: Vec<slint::SharedString> = games.iter().map(|g| {
                    match &g.game_type {
                        crate::core::game_detector::GameType::NativeWarband => "Native Warband".into(),
                        crate::core::game_detector::GameType::Napoleonic => "Napoleonic Wars".into(),
                        crate::core::game_detector::GameType::Viking => "Viking Conquest".into(),
                        crate::core::game_detector::GameType::CustomMod(name) => name.clone().into(),
                    }
                }).collect();
                std::rc::Rc::new(slint::VecModel::from(game_names)).into()
            }
            Err(e) => {
                tracing::error!("游戏检测失败: {}", e);
                std::rc::Rc::new(slint::VecModel::from(Vec::<slint::SharedString>::new())).into()
            }
        }
    });
    
    let _ui_handle = ui.as_weak();
    let am = app_manager.clone();
    ui.on_get_modules_for_game(move |game_path| {
        let am_borrow = am.borrow();
        let detector = am_borrow.game_detector();
        let detector_borrow = detector.borrow();
        
        let installations = detector_borrow.get_installations();
        if let Some(installation) = installations.iter().find(|inst| inst.path.display().to_string() == game_path.as_str()) {
            let module_names: Vec<SharedString> = installation.modules.iter()
                .map(|module| module.name.clone().into())
                .collect();
            std::rc::Rc::new(slint::VecModel::from(module_names)).into()
        } else {
            std::rc::Rc::new(slint::VecModel::from(Vec::<SharedString>::new())).into()
        }
    });
    
    // 手动选择游戏目录
    let ui_handle = ui.as_weak();
    let am = app_manager.clone();
    ui.on_select_game_folder(move || {
        tracing::info!("手动选择游戏文件夹");
        
        // 直接打开文件选择器
        let folder = rfd::FileDialog::new()
            .set_title("选择骑马与砍杀战团游戏文件夹")
            .set_directory(
                dirs::home_dir()
                    .map(|home| home.join("Library/Application Support/Steam/steamapps/common"))
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            )
            .pick_folder();
            
        let final_path = if let Some(path) = folder {
            tracing::info!("用户选择了路径: {:?}", path);
            path
        } else {
            tracing::info!("用户取消了文件选择");
            return "".into();
        };
        
        // 验证选择的路径是否有效
        let modules_path = final_path.join("Modules");
        if !modules_path.exists() {
            tracing::error!("选择的路径无效：缺少Modules目录");
            return "".into();
        }
        
        let path_string = final_path.display().to_string();
        
        // 将选择的路径添加到检测器中
        {
            let mut mgr = am.borrow_mut();
            let detector = mgr.game_detector_mut();
            let mut detector_borrow = detector.borrow_mut();
            
            // 清空现有检测结果
            detector_borrow.clear_detections();
            
            if let Err(e) = detector_borrow.add_manual_path(final_path) {
                tracing::error!("添加手动选择路径失败: {}", e);
                return "".into();
            }
        }
        
        if let Some(ui) = ui_handle.upgrade() {
            update_game_paths_in_ui(&ui, &am);
        }
        
        path_string.into()
    });
    
    let ui_handle = ui.as_weak();
    let am = app_manager.clone();
    ui.on_continue_to_editor(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_show_welcome(false);
            
            let selected_game = ui.get_selected_game();
            let selected_module = ui.get_selected_module();
            
            // 设置当前游戏和模组
            {
                let selected_module_str = selected_module.as_str().to_string();
                let installation_clone;
                let module_clone;
                
                {
                    let am_borrow = am.borrow();
                    let detector = am_borrow.game_detector();
                    let detector_borrow = detector.borrow();
                    let installations = detector_borrow.get_installations();
                    
                    if let Some(installation) = installations.iter().find(|inst| 
                        inst.path.display().to_string() == selected_game.as_str()) {
                        
                        if let Some(module) = installation.modules.iter().find(|m| 
                            m.name == selected_module.as_str()) {
                            
                            installation_clone = installation.clone();
                            module_clone = module.clone();
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                }
                
                // 现在可以安全地设置游戏和模组
                if let Err(e) = am.borrow().set_current_game(installation_clone) {
                    tracing::error!("设置游戏失败: {}", e);
                }
                
                if let Err(e) = am.borrow().set_current_module(module_clone) {
                    tracing::error!("设置模组失败: {}", e);
                }
                
                ui.set_current_module(selected_module);
                tracing::info!("已设置游戏: {} 模组: {}", selected_game, selected_module_str);
            }
            
            // 初始化空的物品列表
            let empty_items: Vec<slint::SharedString> = vec![];
            let model = std::rc::Rc::new(slint::VecModel::from(empty_items));
            ui.set_item_list(slint::ModelRc::from(model));
        }
    });
    
    let ui_handle = ui.as_weak();
    let am = app_manager.clone();
    ui.on_load_from_game(move || {
        tracing::info!("开始从游戏加载数据");
        match am.borrow().load_from_game() {
            Ok(_) => {
                let am_borrow = am.borrow();
                let item_manager = am_borrow.item_manager();
                let item_manager_borrow = item_manager.borrow();
                let items = item_manager_borrow.get_items();
                let item_names: Vec<slint::SharedString> = items.iter()
                    .map(|item| item.name.clone().into())
                    .collect();
                tracing::info!("准备显示 {} 个物品到UI", item_names.len());
                
                // 更新UI中的物品列表
                if let Some(ui) = ui_handle.upgrade() {
                    let model = std::rc::Rc::new(slint::VecModel::from(item_names.clone()));
                    ui.set_item_list(slint::ModelRc::from(model));
                    tracing::info!("已更新UI物品列表");
                }
                
                std::rc::Rc::new(slint::VecModel::from(item_names)).into()
            }
            Err(e) => {
                tracing::error!("从游戏加载数据失败: {}", e);
                std::rc::Rc::new(slint::VecModel::from(Vec::<slint::SharedString>::new())).into()
            }
        }
    });
    
    // 选择物品
    let _am = app_manager.clone();
    ui.on_select_item(move |item_name| {
    });
    
    
    let menu_manager = match MenuManager::new() {
        Ok(manager) => Some(manager),
        Err(_) => None,
    };

    if let Some(menu) = &menu_manager {
        menu.init_for_nsapp().unwrap_or_default();
        
    }

    ui.run()
}

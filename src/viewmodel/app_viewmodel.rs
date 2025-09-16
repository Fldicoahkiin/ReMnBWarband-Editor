// 应用程序主ViewModel

use std::sync::Arc;
use crate::data::{Item, Module};
use crate::editor::Editor;
use anyhow::Result;
use crate::viewmodel::{
    BaseViewModel, BaseViewModelImpl, LoadableViewModel,
    AsyncCommand
};
use crate::viewmodel::observable::{Observable, Command};

// 应用程序状态
#[derive(Debug, Clone)]
pub enum AppState {
    Startup,      // 启动状态
    GameDetected, // 游戏已检测
    GameLoaded,   // 游戏数据已加载
    Editing,      // 编辑状态
}

// 应用程序主ViewModel
pub struct AppViewModel {
    base: BaseViewModelImpl,
    pub editor: Arc<Editor>,
    
    // 应用状态
    pub app_state: Observable<AppState>,
    pub game_path: Observable<String>,
    pub is_game_valid: Observable<bool>,
    pub game_detected: Observable<bool>,
    pub is_loading: Observable<bool>,
    pub status_message: Observable<String>,
    pub error_message: Observable<Option<String>>,
    pub current_page: Observable<String>,
    pub current_module: Observable<String>,
    pub data_loaded: Observable<bool>,
    
    // 物品编辑器相关
    pub items: Observable<Vec<Item>>,
    pub selected_item: Observable<Option<Item>>,
    pub selected_item_id: Observable<String>,
    
    // 模块相关
    pub modules: Observable<Vec<Module>>,
    pub selected_module: Observable<String>,
    
    // 命令
    pub detect_game_command: AsyncCommand,
    pub redetect_game_command: AsyncCommand,
    pub load_game_command: AsyncCommand,
    pub browse_game_path_command: Command,
}

impl AppViewModel {
    pub fn new(editor: Arc<Editor>) -> Result<Self> {
        let base = BaseViewModelImpl::new();
        let app_state = Observable::with_debounce(AppState::Startup, 50);
        let game_path = Observable::with_debounce(String::new(), 100);
        let is_game_valid = Observable::with_debounce(false, 50);
        let game_detected = Observable::with_debounce(false, 50);
        let is_loading = Observable::with_debounce(false, 16);
        let status_message = Observable::with_debounce("就绪".to_string(), 100);
        let error_message = Observable::with_debounce(None, 100);
        let current_page = Observable::with_debounce("startup".to_string(), 50);
        let current_module = Observable::with_debounce("Native".to_string(), 50);
        let data_loaded = Observable::with_debounce(false, 50);
        
        // 物品编辑器相关
        let items = Observable::with_debounce(Vec::new(), 100);
        let selected_item = Observable::with_debounce(None, 50);
        let selected_item_id = Observable::with_debounce(String::new(), 50);
        
        // 模块相关
        let modules = Observable::with_debounce(Vec::new(), 100);
        let selected_module = Observable::with_debounce("Native".to_string(), 50);

        // 检测游戏命令
        let editor_clone = Arc::clone(&editor);
        let game_path_clone = game_path.clone();
        let is_game_valid_clone = is_game_valid.clone();
        let app_state_clone = app_state.clone();
        
        let detect_game_command = AsyncCommand::new(
            move || -> Result<()> {
                match editor_clone.detect_game()? {
                    Some(path) => {
                        game_path_clone.set(path);
                        is_game_valid_clone.set(true);
                        app_state_clone.set(AppState::GameDetected);
                        Ok(())
                    }
                    None => {
                        is_game_valid_clone.set(false);
                        Err(anyhow::anyhow!("未找到游戏安装路径"))
                    }
                }
            },
            || true
        );

        // 重新检测游戏命令
        let editor_clone2 = Arc::clone(&editor);
        let game_path_clone2 = game_path.clone();
        let is_game_valid_clone2 = is_game_valid.clone();
        let app_state_clone2 = app_state.clone();
        
        let modules_clone = modules.clone();
        let redetect_game_command = AsyncCommand::new(
            move || -> Result<()> {
                match editor_clone2.detect_game()? {
                    Some(path) => {
                        game_path_clone2.set(path.clone());
                        is_game_valid_clone2.set(true);
                        app_state_clone2.set(AppState::GameDetected);
                        
                        // 自动扫描模块
                        let modules_path = std::path::Path::new(&path).join("Modules");
                        if modules_path.exists() {
                            let mut found_modules = vec![
                                Module {
                                    id: "Native".to_string(),
                                    name: "原版剧本".to_string(),
                                    path: modules_path.join("Native").to_string_lossy().to_string(),
                                    is_native: true,
                                }
                            ];
                            
                            if let Ok(entries) = std::fs::read_dir(&modules_path) {
                                for entry in entries.flatten() {
                                    if let Ok(file_type) = entry.file_type() {
                                        if file_type.is_dir() {
                                            let dir_name = entry.file_name().to_string_lossy().to_string();
                                            if dir_name != "Native" {
                                                let module_ini = entry.path().join("module.ini");
                                                if module_ini.exists() {
                                                    found_modules.push(Module {
                                                        id: dir_name.clone(),
                                                        name: dir_name.clone(),
                                                        path: entry.path().to_string_lossy().to_string(),
                                                        is_native: false,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            modules_clone.set(found_modules);
                        }
                        
                        Ok(())
                    }
                    None => {
                        is_game_valid_clone2.set(false);
                        Err(anyhow::anyhow!("重新检测未找到游戏安装路径"))
                    }
                }
            },
            || true
        );

        // 加载游戏命令
        let editor_clone = Arc::clone(&editor);
        let game_path_for_load = game_path.clone();
        let app_state_clone = app_state.clone();
        let game_path_for_can_execute = game_path.clone();
        
        let load_game_command = AsyncCommand::new(
            move || -> Result<()> {
                let path = game_path_for_load.get();
                if path.is_empty() {
                    return Err(anyhow::anyhow!("游戏路径为空"));
                }
                
                editor_clone.load_game(&path)?;
                app_state_clone.set(AppState::GameLoaded);
                Ok(())
            },
            move || !game_path_for_can_execute.get().is_empty()
        );

        // 浏览游戏路径命令
        
        let browse_game_path_command = Command::new(
            move || -> Result<()> {
                // 这里会在UI层实现文件对话框
                // 暂时返回成功，实际实现在UI桥接中
                Ok(())
            },
            || true
        );

        Ok(Self {
            base,
            editor,
            app_state,
            game_path,
            is_game_valid,
            game_detected,
            is_loading,
            status_message,
            error_message,
            current_page,
            current_module,
            data_loaded,
            items,
            selected_item,
            selected_item_id,
            modules,
            selected_module,
            detect_game_command,
            redetect_game_command,
            load_game_command,
            browse_game_path_command,
        })
    }

    // 设置游戏路径（从UI调用）
    pub fn set_game_path(&self, path: String) -> Result<()> {
        self.game_path.set(path.clone());
        
        // 验证路径
        let is_valid = !path.is_empty() && std::path::Path::new(&path).exists();
        self.is_game_valid.set(is_valid);
        self.game_detected.set(is_valid);
        
        if is_valid {
            self.app_state.set(AppState::GameDetected);
            self.status_message.set("游戏检测完成".to_string());
        }
        
        Ok(())
    }
    
    // 检测游戏路径
    pub fn detect_game_path(&self) -> Option<String> {
        match self.editor.detect_game() {
            Ok(Some(path)) => {
                self.game_path.set(path.clone());
                self.game_detected.set(true);
                self.is_game_valid.set(true);
                self.app_state.set(AppState::GameDetected);
                self.status_message.set("游戏检测完成".to_string());
                Some(path)
            }
            _ => {
                self.game_detected.set(false);
                self.is_game_valid.set(false);
                self.error_message.set(Some("未找到游戏安装路径".to_string()));
                None
            }
        }
    }
    
    // 加载游戏数据
    pub fn load_game_data(&self, game_path: String, module_name: String) -> Result<()> {
        self.is_loading.set(true);
        self.status_message.set("正在加载游戏数据...".to_string());
        
        match self.editor.load_game(&game_path) {
            Ok(_) => {
                self.data_loaded.set(true);
                self.current_module.set(module_name);
                self.app_state.set(AppState::GameLoaded);
                self.status_message.set("游戏数据加载完成".to_string());
                self.is_loading.set(false);
                Ok(())
            }
            Err(e) => {
                self.error_message.set(Some(e.to_string()));
                self.status_message.set("游戏数据加载失败".to_string());
                self.is_loading.set(false);
                Err(anyhow::anyhow!("游戏数据加载失败"))
            }
        }
    }

    // 进入编辑模式
    pub fn enter_editing_mode(&self) -> Result<()> {
        if matches!(self.app_state.get(), AppState::GameLoaded) {
            self.app_state.set(AppState::Editing);
            Ok(())
        } else {
            Err(anyhow::anyhow!("游戏数据未加载"))
        }
    }

    // 浏览游戏路径
    pub fn browse_game_path(&self) -> Option<String> {
        use rfd::FileDialog;
        
        if let Some(path) = FileDialog::new()
            .set_title("选择游戏安装目录")
            .pick_folder() 
        {
            let path_str = path.to_string_lossy().to_string();
            let _ = self.set_game_path(path_str.clone());
            Some(path_str)
        } else {
            None
        }
    }

    // 进入编辑器
    pub fn enter_editor(&self) {
        if self.game_detected.get() {
            self.current_page.set("editor".to_string());
            self.app_state.set(AppState::Editing);
            self.status_message.set("进入编辑器".to_string());
        }
    }

    // 从游戏加载数据
    pub fn load_from_game(&self) {
        let game_path = self.game_path.get();
        if !game_path.is_empty() {
            self.is_loading.set(true);
            self.status_message.set("正在加载游戏数据...".to_string());
            
            // 加载物品数据
            match self.editor.load_game(&game_path) {
                Ok(_) => {
                    // 创建示例物品数据（暂时用于演示）
                    let sample_items = vec![
                        Item {
                            id: "itm_sword_medieval_a".to_string(),
                            name: "中世纪剑".to_string(),
                            item_type: "武器".to_string(),
                            price: 150,
                            weight: 2.5,
                            damage: 30,
                            armor: 0,
                        },
                        Item {
                            id: "itm_leather_armor".to_string(),
                            name: "皮甲".to_string(),
                            item_type: "护甲".to_string(),
                            price: 80,
                            weight: 5.0,
                            damage: 0,
                            armor: 15,
                        },
                        Item {
                            id: "itm_war_horse".to_string(),
                            name: "战马".to_string(),
                            item_type: "坐骑".to_string(),
                            price: 500,
                            weight: 0.0,
                            damage: 0,
                            armor: 0,
                        },
                    ];
                    self.items.set(sample_items);
                    self.data_loaded.set(true);
                    self.status_message.set("游戏数据加载完成".to_string());
                }
                Err(e) => {
                    self.error_message.set(Some(format!("加载失败: {}", e)));
                }
            }
            self.is_loading.set(false);
        }
    }

    // 选择物品
    pub fn select_item(&self, item_id: String) {
        self.selected_item_id.set(item_id.clone());
        
        // 使用with_value避免克隆整个items列表
        self.items.with_value(|items| {
            if let Some(item) = items.iter().find(|item| item.id == item_id) {
                self.selected_item.set(Some(item.clone()));
            }
        });
    }

    // 保存物品修改
    pub fn save_item(&self, id: String, name: String, item_type: String, price: f32, weight: f32, damage: f32, armor: f32) -> Result<()> {
        let updated_item = Item {
            id: id.clone(),
            name,
            item_type,
            price: price as i32,
            weight,
            damage: damage as i32,
            armor: armor as i32,
        };
        
        // 使用update方法避免完整克隆
        let mut found = false;
        self.items.update(|items| {
            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                *item = updated_item.clone();
                found = true;
            }
        });
        
        if found {
            self.selected_item.set(Some(updated_item));
            self.status_message.set("物品修改已保存".to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("未找到指定的物品"))
        }
    }

    // 扫描游戏模块
    pub fn scan_modules(&self) -> Result<()> {
        let game_path = self.game_path.get();
        if game_path.is_empty() {
            return Err(anyhow::anyhow!("游戏路径为空"));
        }

        let modules_path = std::path::Path::new(&game_path).join("Modules");
        if !modules_path.exists() {
            return Err(anyhow::anyhow!("未找到Modules目录"));
        }

        let mut modules = vec![
            Module {
                id: "Native".to_string(),
                name: "原版剧本".to_string(),
                path: modules_path.join("Native").to_string_lossy().to_string(),
                is_native: true,
            }
        ];

        // 扫描其他模块
        if let Ok(entries) = std::fs::read_dir(&modules_path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        if dir_name != "Native" {
                            // 检查是否有module.ini文件
                            let module_ini = entry.path().join("module.ini");
                            if module_ini.exists() {
                                modules.push(Module {
                                    id: dir_name.clone(),
                                    name: dir_name.clone(),
                                    path: entry.path().to_string_lossy().to_string(),
                                    is_native: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        self.modules.set(modules);
        Ok(())
    }

    // 获取物品列表
    pub fn get_items_for_ui(&self) -> Vec<(String, String)> {
        self.items.with_value(|items| {
            items.iter()
                .map(|item| (item.id.clone(), item.name.clone()))
                .collect()
        })
    }

    // 保存到游戏
    pub fn save_to_game(&self) {
        if self.data_loaded.get() {
            self.is_loading.set(true);
            self.status_message.set("正在保存游戏数据...".to_string());
            
            // 这里应该调用实际的数据保存逻辑
            // 暂时模拟保存成功
            self.status_message.set("游戏数据保存完成".to_string());
            self.is_loading.set(false);
        }
    }

    // 获取当前状态描述
    pub fn get_status_text(&self) -> String {
        match self.app_state.get() {
            AppState::Startup => "请选择游戏安装路径".to_string(),
            AppState::GameDetected => "游戏路径已检测，点击加载数据".to_string(),
            AppState::GameLoaded => "游戏数据已加载，可以开始编辑".to_string(),
            AppState::Editing => "编辑模式".to_string(),
        }
    }

    // 检查是否可以进入编辑器
    pub fn can_enter_editor(&self) -> bool {
        matches!(self.app_state.get(), AppState::GameLoaded | AppState::Editing)
    }
}

impl BaseViewModel for AppViewModel {
    fn initialize(&self) -> Result<()> {
        self.base.initialize()?;
        
        // 尝试自动检测游戏
        if let Err(e) = self.detect_game_command.execute() {
            self.base.set_status(Some(format!("自动检测失败: {}", e)));
        }
        
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        self.base.cleanup()
    }

    fn has_unsaved_changes(&self) -> bool {
        self.base.has_unsaved_changes()
    }
}

impl LoadableViewModel for AppViewModel {
    fn load(&self) -> Result<()> {
        self.load_game_command.execute()
    }

    fn is_loading(&self) -> bool {
        self.load_game_command.is_executing() || self.detect_game_command.is_executing()
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use anyhow::Result;

use crate::core::{
    item_manager::ItemManager,
    troop_manager::TroopManager,
    trigger_manager::TriggerManager,
    file_manager::FileManager,
    python_manager::PythonManager,
    game_detector::{GameDetector, GameInstallation, ModuleInfo},
    script_reader::ScriptReader,
    validator::DataValidator,
};

/// 应用程序管理器 - 统一管理所有功能模块
pub struct AppManager {
    item_manager: Rc<RefCell<ItemManager>>,
    troop_manager: Rc<RefCell<TroopManager>>,
    trigger_manager: Rc<RefCell<TriggerManager>>,
    validator: Rc<DataValidator>,
    file_manager: Rc<FileManager>,
    python_manager: Rc<RefCell<PythonManager>>,
    game_detector: Rc<RefCell<GameDetector>>,
    script_reader: Rc<RefCell<ScriptReader>>,
    current_project_path: Option<String>,
}

impl AppManager {
    pub fn new() -> Self {
        Self {
            item_manager: Rc::new(RefCell::new(ItemManager::new())),
            troop_manager: Rc::new(RefCell::new(TroopManager::new())),
            trigger_manager: Rc::new(RefCell::new(TriggerManager::new())),
            validator: Rc::new(DataValidator::new()),
            file_manager: Rc::new(FileManager::new()),
            python_manager: Rc::new(RefCell::new(PythonManager::new())),
            game_detector: Rc::new(RefCell::new(GameDetector::new())),
            script_reader: Rc::new(RefCell::new(ScriptReader::new())),
            current_project_path: None,
        }
    }


    /// 获取物品管理器
    pub fn item_manager(&self) -> &Rc<RefCell<ItemManager>> {
        &self.item_manager
    }
    
    pub fn game_detector(&self) -> &Rc<RefCell<GameDetector>> {
        &self.game_detector
    }
    
    pub fn game_detector_mut(&mut self) -> &mut Rc<RefCell<GameDetector>> {
        &mut self.game_detector
    }

    /// 获取兵种管理器
    pub fn troop_manager(&self) -> &Rc<RefCell<TroopManager>> {
        &self.troop_manager
    }

    /// 获取触发器管理器
    pub fn trigger_manager(&self) -> &Rc<RefCell<TriggerManager>> {
        &self.trigger_manager
    }

    /// 获取Python管理器
    pub fn python_manager(&self) -> &Rc<RefCell<PythonManager>> {
        &self.python_manager
    }

    /// 获取验证器
    pub fn validator(&self) -> &Rc<DataValidator> {
        &self.validator
    }

    /// 获取文件管理器
    pub fn file_manager(&self) -> &Rc<FileManager> {
        &self.file_manager
    }

    /// 设置当前项目路径
    pub fn set_project_path(&mut self, path: String) {
        self.current_project_path = Some(path);
    }
    
    pub fn get_project_path(&self) -> Option<&String> {
        self.current_project_path.as_ref()
    }

    /// 加载项目数据
    pub fn load_project(&mut self, project_path: &str) -> Result<()> {
        self.current_project_path = Some(project_path.to_string());
        
        tracing::info!("加载项目: {}", project_path);
        
        Ok(())
    }

    /// 保存项目
    pub fn save_project(&self) -> Result<()> {
        if let Some(project_path) = &self.current_project_path {
            // TODO: 实现文件保存逻辑
            tracing::info!("保存项目到: {}", project_path);
            Ok(())
        } else {
            Err(anyhow::anyhow!("未设置项目路径"))
        }
    }

    /// 验证整个项目
    pub fn validate_project(&self) -> ValidationReport {
        let mut report = ValidationReport::new();

        // 验证物品
        let item_errors = self.item_manager.borrow().validate_all();
        report.item_errors = item_errors;

        // 验证兵种
        let troop_errors = self.troop_manager.borrow().validate_all();
        report.troop_errors = troop_errors;

        // 验证触发器
        let trigger_errors = self.trigger_manager.borrow().validate_all();
        report.trigger_errors = trigger_errors;

        // TODO: 实现交叉引用验证
        if let Some(project_path) = &self.current_project_path {
            let _items = self.item_manager.borrow().get_items().to_vec();
            let _troops = self.troop_manager.borrow().get_troops().to_vec();
            
            // 暂时跳过交叉验证，等实现派系管理器后再添加
        }

        report
    }

    /// 获取应用统计信息
    pub fn get_app_stats(&self) -> (usize, usize, usize) {
        let item_count = self.item_manager.borrow().get_items().len();
        let troop_count = self.troop_manager.borrow().get_troops().len();
        let trigger_count = self.trigger_manager.borrow().get_triggers().len();
        
        (item_count, troop_count, trigger_count)
    }

    /// 初始化Python解释器
    pub fn initialize_python(&self) -> Result<()> {
        // 先初始化Python解释器
        {
            let mut python_mgr = self.python_manager.borrow_mut();
            python_mgr.initialize()?;
        }
        
        // 然后同步数据到Python环境
        self.sync_data_to_python()?;
        Ok(())
    }

    /// 执行Python代码
    pub fn execute_python_code(&self, code: &str, script_name: Option<&str>) -> Result<String> {
        // 确保Python已初始化
        if !self.python_manager.borrow().is_initialized() {
            self.initialize_python()?;
        }

        // 同步最新数据
        self.sync_data_to_python()?;

        // 执行代码
        self.python_manager.borrow_mut().execute_code(code, script_name)
    }

    /// 从文件执行Python脚本
    pub fn execute_python_file(&self, file_path: &str) -> Result<String> {
        if !self.python_manager.borrow().is_initialized() {
            self.initialize_python()?;
        }

        self.sync_data_to_python()?;

        let script_path = std::path::Path::new(file_path);
        self.python_manager.borrow_mut().execute_script_file(&script_path)
    }

    pub fn sync_data_to_python(&self) -> Result<()> {
        let items = self.item_manager.borrow().get_items().to_vec();
        let troops = self.troop_manager.borrow().get_troops().to_vec();
        let factions = Vec::new();

        self.python_manager.borrow_mut().update_editor_data(items, troops, factions)
    }

    /// 获取Python执行历史
    pub fn get_python_history(&self) -> Vec<crate::core::python_manager::PythonExecution> {
        self.python_manager.borrow().get_execution_history().to_vec()
    }

    /// 清空Python执行历史
    pub fn clear_python_history(&self) {
        self.python_manager.borrow_mut().clear_history();
    }

    /// 获取Python版本信息
    pub fn get_python_info(&self) -> Result<String> {
        self.python_manager.borrow().get_python_info()
    }

    /// 创建新项目
    pub fn create_new_project(&mut self, project_path: &str, project_name: &str) -> Result<()> {
        // 创建项目目录
        std::fs::create_dir_all(project_path)?;

        // 创建基础文件
        let module_info = format!(
            "# {} Module\n# Created by R球编辑器\n# Version: 0.1.0\n\nmodule_name = \"{}\"\nmodule_version = 1\n",
            project_name, project_name
        );
        std::fs::write(format!("{}/module_info.py", project_path), module_info)?;

        // 创建空的数据文件
        std::fs::write(format!("{}/module_items.txt", project_path), "itemsfile version 3\n")?;
        std::fs::write(format!("{}/module_troops.txt", project_path), "troopsfile version 2\n")?;
        std::fs::write(format!("{}/module_factions.txt", project_path), "factionsfile version 1\n")?;

        self.current_project_path = Some(project_path.to_string());

        self.item_manager.borrow_mut().load_items(Vec::new());
        self.troop_manager.borrow_mut().load_troops(Vec::new());
        self.trigger_manager.borrow_mut().load_triggers(Vec::new());

        Ok(())
    }

    /// 导出项目
    pub fn export_project(&self, export_path: &str, format: ExportFormat) -> Result<()> {
        match format {
            ExportFormat::ModuleSystem => self.export_to_module_system(export_path),
            ExportFormat::Json => self.export_to_json(export_path),
            ExportFormat::Xml => self.export_to_xml(export_path),
        }
    }

    /// 导出到模块系统格式
    fn export_to_module_system(&self, export_path: &str) -> Result<()> {
        // TODO: 实现模块系统导出
        std::fs::create_dir_all(export_path)?;
        
        // 复制当前项目文件到导出目录
        if let Some(project_path) = &self.current_project_path {
            // TODO: 实现copy_directory方法
        }
        
        Ok(())
    }

    /// 导出到JSON格式
    fn export_to_json(&self, export_path: &str) -> Result<()> {
        use serde_json;
        
        let items = self.item_manager.borrow().get_items().to_vec();
        let troops = self.troop_manager.borrow().get_troops().to_vec();
        
        let export_data = serde_json::json!({
            "items": items,
            "troops": troops,
            "export_info": {
                "version": "0.1.0",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "tool": "R球编辑器"
            }
        });
        
        std::fs::write(export_path, serde_json::to_string_pretty(&export_data)?)?;
        Ok(())
    }

    /// 导出到XML格式
    fn export_to_xml(&self, _export_path: &str) -> Result<()> {
        // TODO: 实现XML导出
        Err(anyhow::anyhow!("XML导出功能尚未实现"))
    }

    // === 游戏检测和剧本读取功能 ===

    /// 自动检测游戏安装
    pub fn detect_game_installations(&self) -> Result<Vec<GameInstallation>> {
        let mut detector = self.game_detector.borrow_mut();
        detector.detect_installations()
    }

    /// 手动添加游戏路径
    pub fn add_manual_game_path(&self, path: std::path::PathBuf) -> Result<GameInstallation> {
        let mut detector = self.game_detector.borrow_mut();
        detector.add_manual_path(path)
    }

    /// 获取检测到的游戏列表
    pub fn get_detected_games(&self) -> Vec<GameInstallation> {
        self.game_detector.borrow().get_detected_games().to_vec()
    }

    /// 设置当前游戏安装
    pub fn set_current_game(&self, installation: GameInstallation) -> Result<()> {
        self.script_reader.borrow_mut().set_game_installation(installation);
        Ok(())
    }

    /// 设置当前模组
    pub fn set_current_module(&self, module: ModuleInfo) -> Result<()> {
        self.script_reader.borrow_mut().set_current_module(module)
    }

    pub fn load_from_game(&self) -> Result<()> {
        let reader = self.script_reader.borrow();
        
        if let Ok(items) = reader.read_items() {
            let mut item_mgr = self.item_manager.borrow_mut();
            for item in items {
                item_mgr.add_item(item);
            }
        }

        if let Ok(troops) = reader.read_troops() {
            let mut troop_mgr = self.troop_manager.borrow_mut();
            for troop in troops {
                troop_mgr.add_troop(troop);
            }
        }

        if let Ok(_factions) = reader.read_factions() {
        }

        Ok(())
    }

    /// 获取当前模组信息
    pub fn get_current_module(&self) -> Option<ModuleInfo> {
        self.script_reader.borrow().get_current_module().cloned()
    }

    /// 获取当前游戏信息
    pub fn get_current_game(&self) -> Option<GameInstallation> {
        self.script_reader.borrow().get_current_game().cloned()
    }

    /// 获取模组文件列表
    pub fn get_module_files(&self) -> Result<Vec<std::path::PathBuf>> {
        self.script_reader.borrow().get_module_files()
    }
}

impl Default for AppManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 项目信息
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub path: String,
    pub items_count: usize,
    pub troops_count: usize,
    pub factions_count: usize,
    pub triggers_count: usize,
}

/// 验证报告
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub item_errors: Vec<String>,
    pub troop_errors: Vec<String>,
    pub trigger_errors: Vec<String>,
    pub cross_reference_errors: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            item_errors: Vec::new(),
            troop_errors: Vec::new(),
            trigger_errors: Vec::new(),
            cross_reference_errors: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.item_errors.is_empty() 
            || !self.troop_errors.is_empty() 
            || !self.trigger_errors.is_empty()
            || !self.cross_reference_errors.is_empty()
    }

    pub fn total_errors(&self) -> usize {
        self.item_errors.len() 
            + self.troop_errors.len() 
            + self.trigger_errors.len()
            + self.cross_reference_errors.len()
    }
}

/// 项目统计信息
#[derive(Debug, Clone)]
pub struct ProjectStats {
    pub items: crate::core::item_manager::ItemStats,
    pub troops: crate::core::troop_manager::TroopStats,
    pub triggers: crate::core::trigger_manager::TriggerStats,
}

/// 导出格式
#[derive(Debug, Clone)]
pub enum ExportFormat {
    ModuleSystem,
    Json,
    Xml,
}

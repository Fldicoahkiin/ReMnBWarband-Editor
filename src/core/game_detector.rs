use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error};
use crate::core::parser::ModuleParser;

/// 游戏检测器 - 自动检测骑马与砍杀战团安装位置
#[derive(Debug, Clone)]
pub struct GameDetector {
    /// 可能的游戏安装路径
    possible_paths: Vec<PathBuf>,
    /// 检测到的游戏信息
    detected_games: Vec<GameInstallation>,
}

/// 游戏安装信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInstallation {
    /// 游戏路径
    pub path: PathBuf,
    /// 游戏版本
    pub version: String,
    /// 游戏类型（原版/DLC/MOD）
    pub game_type: GameType,
    /// 模组列表
    pub modules: Vec<ModuleInfo>,
}

/// 游戏类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameType {
    /// 原版战团
    NativeWarband,
    /// 拿破仑战争
    Napoleonic,
    /// 维京征服
    Viking,
    /// 自定义MOD
    CustomMod(String),
}

/// 模组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// 模组ID
    pub id: String,
    /// 模组名称
    pub name: String,
    /// 模组路径
    pub path: PathBuf,
    /// 是否为Steam创意工坊模组
    pub is_workshop: bool,
    /// Steam Workshop ID（如果适用）
    pub workshop_id: Option<String>,
    /// 模组版本
    pub version: String,
    /// 是否为官方模组
    pub is_official: bool,
}

impl GameDetector {
    /// 创建新的游戏检测器
    pub fn new() -> Self {
        let mut detector = Self {
            possible_paths: Vec::new(),
            detected_games: Vec::new(),
        };
        detector.possible_paths = Self::initialize_paths();
        detector
    }

    /// 初始化可能的游戏路径
    fn initialize_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            // Windows常见安装路径
            paths.extend([
                PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\MountBlade Warband"),
                PathBuf::from("C:\\Program Files\\Steam\\steamapps\\common\\MountBlade Warband"),
                PathBuf::from("D:\\Steam\\steamapps\\common\\MountBlade Warband"),
                PathBuf::from("C:\\Program Files (x86)\\Mount&Blade Warband"),
                PathBuf::from("C:\\Program Files\\Mount&Blade Warband"),
            ]);
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS Steam路径
            if let Some(home) = dirs::home_dir() {
                paths.extend([
                    home.join("Library/Application Support/Steam/steamapps/common/MountBlade Warband"),
                    home.join("Applications/Mount&Blade Warband.app"),
                    // 添加可能的其他Steam库位置
                    PathBuf::from("/Applications/Steam.app/Contents/MacOS/steamapps/common/MountBlade Warband"),
                ]);
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux常见安装路径 - 暂时留空，后续补充
            if let Some(home) = dirs::home_dir() {
                paths.extend([
                    home.join(".steam/steam/steamapps/common/MountBlade Warband"),
                    home.join(".local/share/Steam/steamapps/common/MountBlade Warband"),
                ]);
            }
        }
        
        paths
    }

    /// 自动检测游戏安装
    pub fn detect_installations(&mut self) -> Result<Vec<GameInstallation>> {
        self.detected_games.clear();

        for path in &self.possible_paths.clone() {
            if let Ok(installation) = self.check_installation(path) {
                info!("检测到游戏安装: {:?}", installation.path);
                self.detected_games.push(installation);
            }
        }

        // 如果没有找到，尝试通过注册表或其他方式查找
        if self.detected_games.is_empty() {
            self.search_additional_locations()?;
        }

        Ok(self.detected_games.clone())
    }

    /// 检查指定路径是否为有效的游戏安装
    fn check_installation(&self, path: &Path) -> Result<GameInstallation> {
        if !path.exists() {
            return Err(anyhow!("路径不存在: {:?}", path));
        }

        // 检查关键文件 - 更灵活的检测
        let mut exe_path = None;
        
        #[cfg(target_os = "windows")]
        {
            let possible_exe_names = vec!["mb_warband.exe", "Mount&Blade Warband.exe"];
            for exe_name in possible_exe_names {
                let test_path = path.join(exe_name);
                if test_path.exists() {
                    exe_path = Some(test_path);
                    break;
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // 对于macOS，首先检查是否有.app包
            let app_names = vec!["Mount and Blade.app", "mb_warband.app"];
            for app_name in &app_names {
                let app_path = path.join(app_name);
                if app_path.exists() {
                    // 检查app包内的可执行文件
                    let macos_dir = app_path.join("Contents/MacOS");
                    if macos_dir.exists() {
                        let possible_exe_names = vec!["Mount and Blade", "mb_warband", "Mount&Blade Warband"];
                        for exe_name in &possible_exe_names {
                            let test_path = macos_dir.join(exe_name);
                            if test_path.exists() {
                                exe_path = Some(test_path);
                                break;
                            }
                        }
                        if exe_path.is_some() {
                            break;
                        }
                    }
                }
            }
            
            // 如果没找到app包，检查直接的可执行文件
            if exe_path.is_none() {
                let possible_exe_names = vec!["mb_warband", "Mount&Blade Warband", "mb_warband_osx"];
                for exe_name in possible_exe_names {
                    let test_path = path.join(exe_name);
                    if test_path.exists() {
                        exe_path = Some(test_path);
                        break;
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            let possible_exe_names = vec!["mb_warband", "mb_warband_linux"];
            for exe_name in possible_exe_names {
                let test_path = path.join(exe_name);
                if test_path.exists() {
                    exe_path = Some(test_path);
                    break;
                }
            }
        }

        let exe_path = exe_path.ok_or_else(|| anyhow!("未找到游戏可执行文件"))?;

        // 检查Modules目录 - 更灵活的路径检测
        let mut modules_dir = path.join("Modules");
        if !modules_dir.exists() {
            // 对于macOS app包，检查Contents/Resources/Modules
            #[cfg(target_os = "macos")]
            {
                let resources_modules = path.join("Contents/Resources/Modules");
                if resources_modules.exists() {
                    modules_dir = resources_modules;
                } else {
                    return Err(anyhow!("未找到Modules目录"));
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                return Err(anyhow!("未找到Modules目录: {:?}", modules_dir));
            }
        }

        // 获取游戏版本
        let version = self.get_game_version(&exe_path)?;
        
        // 检测游戏类型
        let game_type = self.detect_game_type(&modules_dir)?;

        // 扫描模组
        let mut installation = GameInstallation {
            path: path.to_path_buf(),
            version,
            game_type,
            modules: Vec::new(),
        };
        installation.modules = self.scan_modules(&modules_dir);

        Ok(installation)
    }

    /// 获取游戏版本
    fn get_game_version(&self, exe_path: &Path) -> Result<String> {
        // 简单的版本检测，实际可以读取PE文件版本信息
        if exe_path.exists() {
            Ok("1.174".to_string()) // 默认版本
        } else {
            Err(anyhow!("无法获取游戏版本"))
        }
    }

    /// 检测游戏类型
    fn detect_game_type(&self, modules_dir: &Path) -> Result<GameType> {
        // 检查是否有拿破仑战争模组
        if modules_dir.join("Napoleonic_Wars").exists() {
            return Ok(GameType::Napoleonic);
        }
        
        // 检查是否有维京征服模组
        if modules_dir.join("Viking_Conquest").exists() {
            return Ok(GameType::Viking);
        }
        
        // 检查是否有Native模组（原版）
        if modules_dir.join("Native").exists() {
            return Ok(GameType::NativeWarband);
        }

        Ok(GameType::NativeWarband)
    }

    /// 扫描模组目录
    fn scan_modules(&self, modules_path: &Path) -> Vec<ModuleInfo> {
        let mut modules = Vec::new();
        
        if !modules_path.exists() {
            return modules;
        }
        
        // 首先扫描本地模组
        if let Ok(entries) = fs::read_dir(modules_path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let module_path = entry.path();
                    let module_name = module_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    
                    // 检查是否有module.ini文件
                    let module_ini = module_path.join("module.ini");
                    if module_ini.exists() {
                        modules.push(ModuleInfo {
                            id: module_name.clone(),
                            name: module_name.clone(),
                            path: module_path,
                            is_workshop: false,
                            workshop_id: None,
                            version: "Unknown".to_string(),
                            is_official: module_name == "Native",
                        });
                    }
                }
            }
        }
        
        // 然后扫描Steam创意工坊模组
        let workshop_xml = modules_path.join("steam_workshop_items.xml");
        if workshop_xml.exists() {
            if let Ok(workshop_modules) = self.parse_workshop_items(&workshop_xml) {
                modules.extend(workshop_modules);
            }
        }
        
        modules
    }
    
    /// 解析Steam创意工坊模组列表
    fn parse_workshop_items(&self, xml_path: &Path) -> Result<Vec<ModuleInfo>> {
        let content = fs::read_to_string(xml_path)?;
        let mut modules = Vec::new();
        
        // 简单的XML解析 - 查找module标签
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("<module ") && line.contains("name=") && line.contains("path=") {
                if let (Some(name), Some(path)) = (
                    self.extract_xml_attribute(line, "name"),
                    self.extract_xml_attribute(line, "path")
                ) {
                    let module_path = PathBuf::from(&path);
                    if module_path.exists() && module_path.join("module.ini").exists() {
                        // 从路径中提取Workshop ID
                        let workshop_id = module_path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string());
                        
                        modules.push(ModuleInfo {
                            id: format!("workshop_{}", workshop_id.as_deref().unwrap_or("unknown")),
                            name,
                            path: module_path,
                            is_workshop: true,
                            workshop_id,
                            version: "Workshop".to_string(),
                            is_official: false,
                        });
                    }
                }
            }
        }
        
        Ok(modules)
    }
    
    /// 从XML属性中提取值
    fn extract_xml_attribute(&self, line: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = line.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = line[start..].find('"') {
                return Some(line[start..start + end].to_string());
            }
        }
        None
    }

    /// 解析单个模组信息
    fn parse_module(&self, module_path: &Path) -> Result<ModuleInfo> {
        let module_name = module_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();

        // 检查module.ini文件
        let module_ini = module_path.join("module.ini");
        let (display_name, version) = if module_ini.exists() {
            self.parse_module_ini(&module_ini)?
        } else {
            (module_name.clone(), "1.0".to_string())
        };

        // 判断是否为官方模组
        let _is_official = matches!(module_name.as_str(), 
            "Native" | "Napoleonic_Wars" | "Viking_Conquest" | "Sandbox"
        );

        Ok(ModuleInfo {
            id: module_name.clone(),
            name: display_name,
            path: module_path.to_path_buf(),
            is_workshop: false,
            workshop_id: None,
            version: version,
            is_official: module_name == "Native",
        })
    }

    /// 解析module.ini文件
    fn parse_module_ini(&self, ini_path: &Path) -> Result<(String, String)> {
        let content = fs::read_to_string(ini_path)?;
        let mut name = "Unknown".to_string();
        let mut version = "1.0".to_string();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("module_name") {
                if let Some(value) = line.split('=').nth(1) {
                    name = value.trim().to_string();
                }
            } else if line.starts_with("module_version") {
                if let Some(value) = line.split('=').nth(1) {
                    version = value.trim().to_string();
                }
            }
        }

        Ok((name, version))
    }

    /// 搜索额外位置
    fn search_additional_locations(&mut self) -> Result<()> {
        // 在Windows上可以查询注册表
        #[cfg(target_os = "windows")]
        {
            if let Ok(steam_path) = self.get_steam_path_from_registry() {
                let warband_path = steam_path.join("steamapps/common/MountBlade Warband");
                if let Ok(installation) = self.check_installation(&warband_path) {
                    self.detected_games.push(installation);
                }
            }
        }

        Ok(())
    }

    /// 从注册表获取Steam路径（Windows）
    #[cfg(target_os = "windows")]
    fn get_steam_path_from_registry(&self) -> Result<PathBuf> {
        // 这里需要使用winreg crate来读取注册表
        // 暂时返回默认路径
        Ok(PathBuf::from("C:\\Program Files (x86)\\Steam"))
    }

    /// 手动添加游戏路径
    pub fn add_manual_path(&mut self, path: PathBuf) -> Result<GameInstallation> {
        let installation = self.check_installation(&path)?;
        self.detected_games.push(installation.clone());
        Ok(installation)
    }

    /// 获取检测到的游戏安装列表
    pub fn get_installations(&self) -> &Vec<GameInstallation> {
        &self.detected_games
    }

    /// 获取检测到的游戏列表
    pub fn get_detected_games(&self) -> &[GameInstallation] {
        &self.detected_games
    }

    /// 清空检测结果
    pub fn clear_detections(&mut self) {
        self.detected_games.clear();
    }

    /// 选择默认游戏安装
    pub fn get_default_installation(&self) -> Option<&GameInstallation> {
        // 优先选择原版战团
        self.detected_games.iter()
            .find(|game| matches!(game.game_type, GameType::NativeWarband))
            .or_else(|| self.detected_games.first())
    }
}

impl Default for GameDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// 剧本读取器 - 读取游戏模组数据
#[derive(Debug)]
pub struct ScriptReader {
    /// 当前游戏安装
    game_installation: Option<GameInstallation>,
    /// 当前选中的模组
    current_module: Option<ModuleInfo>,
}

impl ScriptReader {
    /// 创建新的剧本读取器
    pub fn new() -> Self {
        Self {
            game_installation: None,
            current_module: None,
        }
    }

    /// 设置游戏安装
    pub fn set_game_installation(&mut self, installation: GameInstallation) {
        self.game_installation = Some(installation);
    }

    /// 设置当前模组
    pub fn set_current_module(&mut self, module: ModuleInfo) -> Result<()> {
        if let Some(game) = &self.game_installation {
            // 验证模组是否属于当前游戏
            if module.path.starts_with(&game.path) {
                self.current_module = Some(module);
                Ok(())
            } else {
                Err(anyhow!("模组不属于当前游戏安装"))
            }
        } else {
            Err(anyhow!("未设置游戏安装"))
        }
    }

    /// 读取物品数据
    pub fn read_items(&self) -> Result<Vec<crate::models::Item>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未选择模组"))?;

        let items_file = module.path.join("module_items.txt");
        if !items_file.exists() {
            return Ok(Vec::new());
        }

        info!("读取物品文件: {:?}", items_file);
        crate::core::parser::ModuleParser::parse_items_file(&items_file)
    }

    /// 读取兵种数据
    pub fn read_troops(&self) -> Result<Vec<crate::models::Troop>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未选择模组"))?;

        let troops_file = module.path.join("module_troops.txt");
        if !troops_file.exists() {
            return Ok(Vec::new());
        }

        info!("读取兵种文件: {:?}", troops_file);
        crate::core::parser::ModuleParser::parse_troops_file(&troops_file)
    }

    /// 读取派系数据
    pub fn read_factions(&self) -> Result<Vec<crate::models::Faction>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未选择模组"))?;

        let factions_file = module.path.join("module_factions.txt");
        if !factions_file.exists() {
            return Ok(Vec::new());
        }

        info!("读取派系文件: {:?}", factions_file);
        crate::core::parser::ModuleParser::parse_factions_file(&factions_file)
    }

    /// 获取模组文件列表
    pub fn get_module_files(&self) -> Result<Vec<PathBuf>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未选择模组"))?;

        let mut files = Vec::new();
        
        // 扫描模组目录中的所有.txt文件
        if let Ok(entries) = fs::read_dir(&module.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    /// 获取当前模组信息
    pub fn get_current_module(&self) -> Option<&ModuleInfo> {
        self.current_module.as_ref()
    }

    /// 获取当前游戏安装信息
    pub fn get_current_game(&self) -> Option<&GameInstallation> {
        self.game_installation.as_ref()
    }
}

impl Default for ScriptReader {
    fn default() -> Self {
        Self::new()
    }
}

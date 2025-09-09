use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use crate::models::{Item, Troop, Faction};
use crate::core::game_detector::{GameInstallation, ModuleInfo};
use crate::core::parser::{ItemParser, TroopParser, FactionParser, DataParser};
use crate::utils::encoding::detect_and_decode;

/// 脚本文件读取器
/// 负责从游戏模组目录读取各种数据文件
pub struct ScriptReader {
    current_game: Option<GameInstallation>,
    current_module: Option<ModuleInfo>,
    item_parser: ItemParser,
    troop_parser: TroopParser,
    faction_parser: FactionParser,
}

impl ScriptReader {
    pub fn new() -> Self {
        Self {
            current_game: None,
            current_module: None,
            item_parser: ItemParser::new(),
            troop_parser: TroopParser::new(),
            faction_parser: FactionParser::new(),
        }
    }

    /// 设置当前游戏安装
    pub fn set_game_installation(&mut self, installation: GameInstallation) {
        self.current_game = Some(installation);
    }

    /// 设置当前模组
    pub fn set_current_module(&mut self, module: ModuleInfo) -> Result<()> {
        // 验证模组路径是否存在
        if !module.path.exists() {
            return Err(anyhow!("模组路径不存在: {}", module.path.display()));
        }
        
        self.current_module = Some(module);
        Ok(())
    }

    /// 获取当前游戏安装
    pub fn get_current_game(&self) -> Option<&GameInstallation> {
        self.current_game.as_ref()
    }

    /// 获取当前模组
    pub fn get_current_module(&self) -> Option<&ModuleInfo> {
        self.current_module.as_ref()
    }

    /// 获取模组文件列表
    pub fn get_module_files(&self) -> Result<Vec<PathBuf>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未设置当前模组"))?;

        let mut files = Vec::new();
        
        // 扫描模组目录中的所有.txt文件
        if let Ok(entries) = fs::read_dir(&module.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                    files.push(path);
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// 读取物品数据
    pub fn read_items(&self) -> Result<Vec<Item>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未设置当前模组"))?;

        let items_file = module.path.join("item_kinds1.txt");
        if !items_file.exists() {
            return Err(anyhow!("物品文件不存在: {}", items_file.display()));
        }

        tracing::info!("读取物品文件: {}", items_file.display());
        
        // 读取文件内容并解析
        let content = detect_and_decode(&items_file)?;
        self.parse_items_file(&content)
    }

    /// 读取兵种数据
    pub fn read_troops(&self) -> Result<Vec<Troop>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未设置当前模组"))?;

        let troops_file = module.path.join("troops.txt");
        if !troops_file.exists() {
            return Err(anyhow!("兵种文件不存在: {}", troops_file.display()));
        }

        tracing::info!("读取兵种文件: {}", troops_file.display());
        
        let content = detect_and_decode(&troops_file)?;
        self.parse_troops_file(&content)
    }

    /// 读取派系数据
    pub fn read_factions(&self) -> Result<Vec<Faction>> {
        let module = self.current_module.as_ref()
            .ok_or_else(|| anyhow!("未设置当前模组"))?;

        let factions_file = module.path.join("factions.txt");
        if !factions_file.exists() {
            return Err(anyhow!("派系文件不存在: {}", factions_file.display()));
        }

        tracing::info!("读取派系文件: {}", factions_file.display());
        
        let content = detect_and_decode(&factions_file)?;
        self.parse_factions_file(&content)
    }

    /// 解析物品文件内容
    fn parse_items_file(&self, content: &str) -> Result<Vec<Item>> {
        let lines: Vec<&str> = content.lines().collect();
        
        // 检查文件头
        if lines.is_empty() || !lines[0].starts_with("itemsfile version") {
            return Err(anyhow!("无效的物品文件格式"));
        }

        // 获取物品数量
        if lines.len() < 2 {
            return Err(anyhow!("物品文件格式错误：缺少物品数量"));
        }

        let item_count: usize = lines[1].trim().parse()
            .map_err(|_| anyhow!("无法解析物品数量: {}", lines[1]))?;

        tracing::info!("物品文件包含 {} 个物品", item_count);

        let mut items = Vec::new();
        let mut line_index = 2;

        // 解析每个物品
        for i in 0..item_count {
            if line_index >= lines.len() {
                tracing::warn!("物品 {} 数据不完整", i);
                break;
            }

            // 跳过空行
            while line_index < lines.len() && lines[line_index].trim().is_empty() {
                line_index += 1;
            }

            if line_index >= lines.len() {
                break;
            }

            // 解析物品行
            let item_line = lines[line_index];
            if let Ok(item) = self.parse_single_item(item_line) {
                items.push(item);
            } else {
                tracing::warn!("解析物品失败: {}", item_line);
            }

            line_index += 1;
        }

        tracing::info!("成功解析 {} 个物品", items.len());
        Ok(items)
    }

    /// 解析单个物品行
    fn parse_single_item(&self, line: &str) -> Result<Item> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 3 {
            return Err(anyhow!("物品行格式错误"));
        }

        let id = parts[0].to_string();
        let name = parts[1].replace('_', " ");
        
        let mut item = Item::new(id, name);
        
        // 解析其他属性（简化版本）
        if parts.len() > 10 {
            // 尝试解析价格
            if let Ok(price) = parts[10].parse::<u32>() {
                item.price = price;
            }
        }

        Ok(item)
    }

    /// 解析兵种文件内容
    fn parse_troops_file(&self, content: &str) -> Result<Vec<Troop>> {
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.is_empty() || !lines[0].starts_with("troopsfile version") {
            return Err(anyhow!("无效的兵种文件格式"));
        }

        if lines.len() < 2 {
            return Err(anyhow!("兵种文件格式错误：缺少兵种数量"));
        }

        let troop_count: usize = lines[1].trim().parse()
            .map_err(|_| anyhow!("无法解析兵种数量: {}", lines[1]))?;

        tracing::info!("兵种文件包含 {} 个兵种", troop_count);

        let mut troops = Vec::new();
        let mut line_index = 2;

        for i in 0..troop_count {
            if line_index >= lines.len() {
                tracing::warn!("兵种 {} 数据不完整", i);
                break;
            }

            while line_index < lines.len() && lines[line_index].trim().is_empty() {
                line_index += 1;
            }

            if line_index >= lines.len() {
                break;
            }

            let troop_line = lines[line_index];
            if let Ok(troop) = self.parse_single_troop(troop_line) {
                troops.push(troop);
            } else {
                tracing::warn!("解析兵种失败: {}", troop_line);
            }

            line_index += 1;
        }

        tracing::info!("成功解析 {} 个兵种", troops.len());
        Ok(troops)
    }

    /// 解析单个兵种行
    fn parse_single_troop(&self, line: &str) -> Result<Troop> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 3 {
            return Err(anyhow!("兵种行格式错误"));
        }

        let id = parts[0].to_string();
        let name = parts[1].replace('_', " ");
        
        let troop = Troop::new(id, name);
        
        Ok(troop)
    }

    /// 解析派系文件内容
    fn parse_factions_file(&self, content: &str) -> Result<Vec<Faction>> {
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.is_empty() || !lines[0].starts_with("factionsfile version") {
            return Err(anyhow!("无效的派系文件格式"));
        }

        if lines.len() < 2 {
            return Err(anyhow!("派系文件格式错误：缺少派系数量"));
        }

        let faction_count: usize = lines[1].trim().parse()
            .map_err(|_| anyhow!("无法解析派系数量: {}", lines[1]))?;

        tracing::info!("派系文件包含 {} 个派系", faction_count);

        let mut factions = Vec::new();
        let mut line_index = 2;

        for i in 0..faction_count {
            if line_index >= lines.len() {
                tracing::warn!("派系 {} 数据不完整", i);
                break;
            }

            while line_index < lines.len() && lines[line_index].trim().is_empty() {
                line_index += 1;
            }

            if line_index >= lines.len() {
                break;
            }

            let faction_line = lines[line_index];
            if let Ok(faction) = self.parse_single_faction(faction_line) {
                factions.push(faction);
            } else {
                tracing::warn!("解析派系失败: {}", faction_line);
            }

            line_index += 1;
        }

        tracing::info!("成功解析 {} 个派系", factions.len());
        Ok(factions)
    }

    /// 解析单个派系行
    fn parse_single_faction(&self, line: &str) -> Result<Faction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 2 {
            return Err(anyhow!("派系行格式错误"));
        }

        let id = parts[0].to_string();
        let name = parts[1].replace('_', " ");
        
        let faction = Faction::new(id, name);
        
        Ok(faction)
    }
}

impl Default for ScriptReader {
    fn default() -> Self {
        Self::new()
    }
}

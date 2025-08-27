use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use crate::models::{Item, Troop, Faction};
use crate::utils::encoding::detect_and_decode;

/// Module System文件解析器
/// 负责解析骑马与砍杀的各种数据文件格式
pub struct ModuleParser;

impl ModuleParser {
    /// 解析物品文件 (module_items.txt)
    pub fn parse_items_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<Item>> {
        let content = detect_and_decode(&file_path)?;
        Self::parse_items_content(&content)
    }

    /// 解析物品文件内容
    pub fn parse_items_content(content: &str) -> Result<Vec<Item>> {
        let mut items = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // 检查文件头
        if lines.is_empty() || !lines[0].starts_with("itemsfile version") {
            return Err(anyhow!("无效的物品文件格式"));
        }

        let mut i = 1;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }

            // 解析物品条目
            if let Ok(item) = Self::parse_item_entry(line) {
                items.push(item);
            }
            
            i += 1;
        }

        Ok(items)
    }

    /// 解析单个物品条目
    fn parse_item_entry(line: &str) -> Result<Item> {
        // Module System物品格式：
        // ["item_id", "Item Name", [("mesh", "item_mesh"), ("material", "item_material")], itp_flags, item_capabilities, price, weight, abundance, head_armor, body_armor, leg_armor, difficulty, hit_points, speed_rating, weapon_length, swing_damage, thrust_damage, missile_speed, weapon_flags]
        
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(anyhow!("无效的物品条目格式"));
        }

        // 移除首尾的方括号
        let content = &line[1..line.len()-1];
        
        // 简单的解析实现 - 实际需要更复杂的解析器来处理嵌套结构
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 3 {
            return Err(anyhow!("物品条目字段不足"));
        }

        // 解析基本字段
        let id = parts[0].trim_matches('"').to_string();
        let name = parts[1].trim_matches('"').to_string();
        
        // 创建默认物品，实际应该解析所有字段
        let mut item = Item::new(id, name);
        
        // 尝试解析价格和重量（如果有足够的字段）
        if parts.len() > 5 {
            if let Ok(price) = parts[5].parse::<u32>() {
                item.price = price;
            }
        }
        
        if parts.len() > 6 {
            if let Ok(weight) = parts[6].parse::<f32>() {
                item.weight = weight;
            }
        }

        Ok(item)
    }

    /// 解析兵种文件 (module_troops.txt)
    pub fn parse_troops_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<Troop>> {
        let content = detect_and_decode(&file_path)?;
        Self::parse_troops_content(&content)
    }

    /// 解析兵种文件内容
    pub fn parse_troops_content(content: &str) -> Result<Vec<Troop>> {
        let mut troops = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // 检查文件头
        if lines.is_empty() || !lines[0].starts_with("troopsfile version") {
            return Err(anyhow!("无效的兵种文件格式"));
        }

        let mut i = 1;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }

            // 解析兵种条目
            if let Ok(troop) = Self::parse_troop_entry(line) {
                troops.push(troop);
            }
            
            i += 1;
        }

        Ok(troops)
    }

    /// 解析单个兵种条目
    fn parse_troop_entry(line: &str) -> Result<Troop> {
        // Module System兵种格式：
        // ["troop_id", "Troop Name", "Troop Name Plural", tf_flags, no_scene, reserved, faction, upgrade_troop, items, str, agi, int, cha, level, wp, skills]
        
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(anyhow!("无效的兵种条目格式"));
        }

        let content = &line[1..line.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 4 {
            return Err(anyhow!("兵种条目字段不足"));
        }

        let id = parts[0].trim_matches('"').to_string();
        let name = parts[1].trim_matches('"').to_string();
        
        let mut troop = Troop::new(id, name);
        
        // 解析等级（如果有足够的字段）
        if parts.len() > 12 {
            if let Ok(level) = parts[12].parse::<u32>() {
                troop.level = level;
            }
        }

        Ok(troop)
    }

    /// 解析派系文件 (module_factions.txt)
    pub fn parse_factions_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<Faction>> {
        let content = detect_and_decode(&file_path)?;
        Self::parse_factions_content(&content)
    }

    /// 解析派系文件内容
    pub fn parse_factions_content(content: &str) -> Result<Vec<Faction>> {
        let mut factions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // 检查文件头
        if lines.is_empty() || !lines[0].starts_with("factionsfile version") {
            return Err(anyhow!("无效的派系文件格式"));
        }

        let mut i = 1;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }

            // 解析派系条目
            if let Ok(faction) = Self::parse_faction_entry(line) {
                factions.push(faction);
            }
            
            i += 1;
        }

        Ok(factions)
    }

    /// 解析单个派系条目
    fn parse_faction_entry(line: &str) -> Result<Faction> {
        // Module System派系格式：
        // ["faction_id", "Faction Name", 0, max_lords, faction_color, relations, ranks, notes]
        
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(anyhow!("无效的派系条目格式"));
        }

        let content = &line[1..line.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 2 {
            return Err(anyhow!("派系条目字段不足"));
        }

        let id = parts[0].trim_matches('"').to_string();
        let name = parts[1].trim_matches('"').to_string();
        
        let faction = Faction::new(id, name);

        Ok(faction)
    }

    /// 验证文件格式
    pub fn validate_file_format<P: AsRef<Path>>(file_path: P, expected_type: &str) -> Result<bool> {
        let content = fs::read_to_string(&file_path)?;
        let first_line = content.lines().next().unwrap_or("");
        
        Ok(first_line.starts_with(&format!("{}file version", expected_type)))
    }

    /// 获取文件版本
    pub fn get_file_version<P: AsRef<Path>>(file_path: P) -> Result<u32> {
        let content = fs::read_to_string(&file_path)?;
        let first_line = content.lines().next().unwrap_or("");
        
        // 解析版本号，格式如 "itemsfile version 3"
        if let Some(version_part) = first_line.split_whitespace().last() {
            version_part.parse::<u32>()
                .map_err(|_| anyhow!("无法解析文件版本"))
        } else {
            Err(anyhow!("文件格式无效"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item_entry() {
        let line = r#"["itm_sword", "Sword", [("mesh", "sword_mesh")], itp_weapon, itc_cutting, 100, 1.5, 10, 0, 0, 0, 0, 100, 95, 100, 30, 25, 0, 0]"#;
        
        let result = ModuleParser::parse_item_entry(line);
        assert!(result.is_ok());
        
        let item = result.unwrap();
        assert_eq!(item.id, "itm_sword");
        assert_eq!(item.name, "Sword");
    }

    #[test]
    fn test_parse_troop_entry() {
        let line = r#"["trp_player", "Player", "Players", tf_hero, no_scene, reserved, fac_player_faction, 0, [], 10, 10, 10, 10, 1, [], []]"#;
        
        let result = ModuleParser::parse_troop_entry(line);
        assert!(result.is_ok());
        
        let troop = result.unwrap();
        assert_eq!(troop.id, "trp_player");
        assert_eq!(troop.name, "Player");
    }

    #[test]
    fn test_parse_faction_entry() {
        let line = r#"["fac_player_faction", "Player Faction", 0, 0.9, [255, 255, 255], [], [], "notes"]"#;
        
        let result = ModuleParser::parse_faction_entry(line);
        assert!(result.is_ok());
        
        let faction = result.unwrap();
        assert_eq!(faction.id, "fac_player_faction");
        assert_eq!(faction.name, "Player Faction");
    }
}

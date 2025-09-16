// 游戏数据解析器

use anyhow::Result;
use std::path::Path;
use super::models::*;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }
    
    // 解析游戏数据
    pub fn parse_game_data<P: AsRef<Path>>(&self, game_path: P) -> Result<GameData> {
        let game_path = game_path.as_ref();
        
        let items = self.parse_items(game_path.join("Modules/Native/item_kinds1.txt"))?;
        let troops = self.parse_troops(game_path.join("Modules/Native/troops.txt"))?;
        let factions = self.parse_factions(game_path.join("Modules/Native/factions.txt"))?;
        
        Ok(GameData {
            items,
            troops,
            factions,
        })
    }
    
    // 解析物品文件
    fn parse_items<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Item>> {
        let content = std::fs::read_to_string(path)?;
        let mut items = Vec::new();
        
        for line in content.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(item) = self.parse_item_line(line) {
                items.push(item);
            }
        }
        
        Ok(items)
    }
    
    // 解析单个物品行
    fn parse_item_line(&self, line: &str) -> Option<Item> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }
        
        Some(Item {
            id: parts[0].to_string(),
            name: parts[1].replace('_', " "),
            item_type: "Other".to_string(),
            price: parts[2].parse().unwrap_or(0),
            weight: parts[3].parse().unwrap_or(0.0),
            damage: parts[4].parse().unwrap_or(0),
            armor: parts[5].parse().unwrap_or(0),
        })
    }
    
    // 解析兵种文件
    fn parse_troops<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Troop>> {
        let content = std::fs::read_to_string(path)?;
        let mut troops = Vec::new();
        
        for line in content.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(troop) = self.parse_troop_line(line) {
                troops.push(troop);
            }
        }
        
        Ok(troops)
    }
    
    // 解析单个兵种行
    fn parse_troop_line(&self, line: &str) -> Option<Troop> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 8 {
            return None;
        }
        
        Some(Troop {
            id: parts[0].to_string(),
            name: parts[1].replace('_', " "),
            level: parts[2].parse().unwrap_or(1),
            faction: parts[3].to_string(),
            troop_class: parts.get(8).unwrap_or(&"Infantry").to_string(),
            strength: parts[4].parse().unwrap_or(10),
            agility: parts[5].parse().unwrap_or(10),
            intelligence: parts[6].parse().unwrap_or(10),
            charisma: parts[7].parse().unwrap_or(10),
        })
    }
    
    // 解析派系文件
    fn parse_factions<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Faction>> {
        let content = std::fs::read_to_string(path)?;
        let mut factions = Vec::new();
        
        for line in content.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(faction) = self.parse_faction_line(line) {
                factions.push(faction);
            }
        }
        
        Ok(factions)
    }
    
    // 解析单个派系行
    fn parse_faction_line(&self, line: &str) -> Option<Faction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        
        Some(Faction {
            id: parts[0].to_string(),
            name: parts[1].replace('_', " "),
            color: parts[2].to_string(),
            culture: parts.get(3).unwrap_or(&"Default").to_string(),
        })
    }
}

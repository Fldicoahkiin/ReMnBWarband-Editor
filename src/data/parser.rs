// 游戏数据解析器

use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::io::{BufRead, BufReader};
use std::fs::File;
use super::models::*;

// 解析缓存
#[derive(Default)]
struct ParseCache {
    items: HashMap<String, Vec<Item>>,
    troops: HashMap<String, Vec<Troop>>,
    factions: HashMap<String, Vec<Faction>>,
    file_timestamps: HashMap<String, std::time::SystemTime>,
}

pub struct Parser {
    cache: Arc<RwLock<ParseCache>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(ParseCache::default())),
        }
    }
    
    // 检查文件是否需要重新解析
    fn needs_reparse(&self, path: &Path) -> bool {
        let cache = self.cache.read().unwrap();
        let path_str = path.to_string_lossy().to_string();
        
        match (cache.file_timestamps.get(&path_str), path.metadata()) {
            (Some(cached_time), Ok(metadata)) => {
                if let Ok(modified_time) = metadata.modified() {
                    modified_time > *cached_time
                } else {
                    true
                }
            }
            _ => true,
        }
    }
    
    // 更新缓存时间戳
    fn update_timestamp(&self, path: &Path) {
        let mut cache = self.cache.write().unwrap();
        let path_str = path.to_string_lossy().to_string();
        
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified_time) = metadata.modified() {
                cache.file_timestamps.insert(path_str, modified_time);
            }
        }
    }
    
    // 解析游戏数据
    pub fn parse_game_data<P: AsRef<Path>>(&self, game_path: P) -> Result<GameData> {
        let game_path = game_path.as_ref();
        
        let _items = self.parse_items(game_path.join("Modules/Native/item_kinds1.txt"))?;
        let _troops = self.parse_troops(game_path.join("Modules/Native/troops.txt"))?;
        let _factions = self.parse_factions(game_path.join("Modules/Native/factions.txt"))?;
        
        Ok(GameData {
            items: _items,
            troops: _troops,
            factions: _factions,
            modules: Vec::new(),
        })
    }
    
    // 解析物品文件
    fn parse_items<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Item>> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        // 检查缓存
        if !self.needs_reparse(path) {
            let cache = self.cache.read().unwrap();
            if let Some(cached_items) = cache.items.get(&path_str) {
                return Ok(cached_items.clone());
            }
        }
        
        // 流式解析文件
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut items = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(item) = self.parse_item_line(&line) {
                items.push(item);
            }
        }
        
        // 更新缓存
        {
            let mut cache = self.cache.write().unwrap();
            cache.items.insert(path_str, items.clone());
        }
        self.update_timestamp(path);
        
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
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        // 检查缓存
        if !self.needs_reparse(path) {
            let cache = self.cache.read().unwrap();
            if let Some(cached_troops) = cache.troops.get(&path_str) {
                return Ok(cached_troops.clone());
            }
        }
        
        // 流式解析文件
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut troops = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(troop) = self.parse_troop_line(&line) {
                troops.push(troop);
            }
        }
        
        // 更新缓存
        {
            let mut cache = self.cache.write().unwrap();
            cache.troops.insert(path_str, troops.clone());
        }
        self.update_timestamp(path);
        
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
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        // 检查缓存
        if !self.needs_reparse(path) {
            let cache = self.cache.read().unwrap();
            if let Some(cached_factions) = cache.factions.get(&path_str) {
                return Ok(cached_factions.clone());
            }
        }
        
        // 流式解析文件
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut factions = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(faction) = self.parse_faction_line(&line) {
                factions.push(faction);
            }
        }
        
        // 更新缓存
        {
            let mut cache = self.cache.write().unwrap();
            cache.factions.insert(path_str, factions.clone());
        }
        self.update_timestamp(path);
        
        Ok(factions)
    }
    
    // 清空缓存
    pub fn clear_cache(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.items.clear();
        cache.troops.clear();
        cache.factions.clear();
        cache.file_timestamps.clear();
    }
    
    // 获取缓存统计信息
    pub fn cache_stats(&self) -> (usize, usize, usize) {
        let cache = self.cache.read().unwrap();
        (cache.items.len(), cache.troops.len(), cache.factions.len())
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

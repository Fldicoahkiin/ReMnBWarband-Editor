// 编辑器核心功能模块

use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::data::{GameManager, Item, Troop, Faction};

#[derive(Clone)]
pub struct Editor {
    game_manager: Arc<Mutex<GameManager>>,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            game_manager: Arc::new(Mutex::new(GameManager::new())),
        })
    }
    
    // 检测游戏
    pub fn detect_game(&self) -> Result<Option<String>> {
        match GameManager::detect_game()? {
            Some(game) => Ok(Some(game.path.to_string_lossy().to_string())),
            None => Ok(None),
        }
    }
    
    // 加载游戏数据
    pub fn load_game(&self, path: &str) -> Result<()> {
        let mut manager = self.game_manager.lock().unwrap();
        manager.load_game(path)
    }
    
    // 获取物品列表
    pub fn get_items(&self) -> Vec<Item> {
        let manager = self.game_manager.lock().unwrap();
        manager.get_data()
            .map(|data| data.items.clone())
            .unwrap_or_default()
    }
    
    // 获取兵种列表
    pub fn get_troops(&self) -> Vec<Troop> {
        let manager = self.game_manager.lock().unwrap();
        manager.get_data()
            .map(|data| data.troops.clone())
            .unwrap_or_default()
    }
    
    // 获取派系列表
    pub fn get_factions(&self) -> Vec<Faction> {
        let manager = self.game_manager.lock().unwrap();
        manager.get_data()
            .map(|data| data.factions.clone())
            .unwrap_or_default()
    }
    
    // 保存数据
    pub fn save_data(&self) -> Result<()> {
        let manager = self.game_manager.lock().unwrap();
        manager.save_data()
    }
    
    // 获取JSON格式的数据
    pub fn get_items_json(&self) -> Result<String> {
        let items = self.get_items();
        serde_json::to_string(&items).map_err(|e| anyhow::anyhow!("序列化失败: {}", e))
    }
    
    pub fn get_troops_json(&self) -> Result<String> {
        let troops = self.get_troops();
        serde_json::to_string(&troops).map_err(|e| anyhow::anyhow!("序列化失败: {}", e))
    }
    
    pub fn get_factions_json(&self) -> Result<String> {
        let factions = self.get_factions();
        serde_json::to_string(&factions).map_err(|e| anyhow::anyhow!("序列化失败: {}", e))
    }
}

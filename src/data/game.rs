// 游戏检测和管理

use anyhow::Result;
use std::path::{Path, PathBuf};
use super::models::GameData;
use super::parser::Parser;

#[derive(Debug, Clone)]
pub struct GameInstance {
    pub path: PathBuf,
    pub version: String,
}

pub struct GameManager {
    parser: Parser,
    current_game: Option<GameInstance>,
    current_data: Option<GameData>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            current_game: None,
            current_data: None,
        }
    }
    
    // 检测游戏安装
    pub fn detect_game() -> Result<Option<GameInstance>> {
        // macOS Steam路径
        let steam_path = dirs::home_dir()
            .map(|home| home.join("Library/Application Support/Steam/steamapps/common/MountBlade Warband"));
        
        if let Some(path) = steam_path {
            if path.exists() {
                return Ok(Some(GameInstance {
                    path,
                    version: "1.174".to_string(),
                }));
            }
        }
        
        Ok(None)
    }
    
    // 加载游戏数据
    pub fn load_game<P: AsRef<Path>>(&mut self, game_path: P) -> Result<()> {
        let path = game_path.as_ref().to_path_buf();
        
        // 验证游戏路径
        if !path.exists() {
            return Err(anyhow::anyhow!("游戏路径不存在: {}", path.display()));
        }
        
        // 解析游戏数据
        let data = self.parser.parse_game_data(&path)?;
        
        self.current_game = Some(GameInstance {
            path,
            version: "1.174".to_string(),
        });
        self.current_data = Some(data);
        
        tracing::info!("游戏数据加载成功");
        Ok(())
    }
    
    // 获取当前游戏数据
    pub fn get_data(&self) -> Option<&GameData> {
        self.current_data.as_ref()
    }
    
    // 获取当前游戏实例
    pub fn get_game(&self) -> Option<&GameInstance> {
        self.current_game.as_ref()
    }
    
    // 保存游戏数据
    pub fn save_data(&self) -> Result<()> {
        if let (Some(game), Some(_data)) = (&self.current_game, &self.current_data) {
            tracing::info!("保存游戏数据到: {}", game.path.display());
            // 这里将来实现实际的保存逻辑
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有加载的游戏数据"))
        }
    }
}

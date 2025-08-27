use serde::{Deserialize, Serialize};
use crate::models::trigger::Trigger;

/// 派系颜色
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FactionColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 派系关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRelation {
    pub faction_id: String,
    pub relation: f32,
}

/// 派系数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// 派系ID
    pub id: String,
    /// 派系名称
    pub name: String,
    /// 派系标记
    pub flags: u64,
    /// 派系颜色
    pub color: FactionColor,
    /// 派系关系
    pub relations: Vec<FactionRelation>,
    /// 派系排名
    pub ranking: f32,
    /// 触发器
    pub triggers: Vec<Trigger>,
    /// 派系领主
    pub lords: Vec<String>,
    /// 派系城镇
    pub towns: Vec<String>,
    /// 派系村庄
    pub villages: Vec<String>,
}

impl Default for Faction {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            flags: 0,
            color: FactionColor::default(),
            relations: Vec::new(),
            ranking: 0.0,
            triggers: Vec::new(),
            lords: Vec::new(),
            towns: Vec::new(),
            villages: Vec::new(),
        }
    }
}

impl Faction {
    /// 创建新派系
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
    
    /// 添加派系关系
    pub fn add_relation(&mut self, faction_id: String, relation: f32) {
        self.relations.push(FactionRelation { faction_id, relation });
    }
    
    /// 获取与指定派系的关系
    pub fn get_relation(&self, faction_id: &str) -> Option<f32> {
        self.relations.iter()
            .find(|r| r.faction_id == faction_id)
            .map(|r| r.relation)
    }
    
    /// 验证派系数据完整性
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("派系ID不能为空".to_string());
        }
        
        if self.name.is_empty() {
            return Err("派系名称不能为空".to_string());
        }
        
        // 验证关系值范围
        for relation in &self.relations {
            if relation.relation < -100.0 || relation.relation > 100.0 {
                return Err("派系关系值必须在-100到100之间".to_string());
            }
        }
        
        Ok(())
    }
}

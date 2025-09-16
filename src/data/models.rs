// 游戏数据模型

use serde::{Deserialize, Serialize};

// 物品数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: String, // 改为String类型以简化处理
    pub price: i32,
    pub weight: f32,
    pub damage: i32,
    pub armor: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Horse,
    Other,
}

// 兵种数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Troop {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub faction: String,
    pub troop_class: String, // 添加兵种类型字段
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub charisma: i32,
}

// 派系数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub color: String,
    pub culture: String, // 添加文化字段
}

// 游戏数据集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub items: Vec<Item>,
    pub troops: Vec<Troop>,
    pub factions: Vec<Faction>,
}

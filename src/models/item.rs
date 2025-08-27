use serde::{Deserialize, Serialize};

/// 物品类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    OneHandedWeapon = 0x2,
    TwoHandedWeapon = 0x3,
    Polearm = 0x4,
    Archery = 0x5,
    Crossbow = 0x6,
    Throwing = 0x7,
    Shield = 0x8,
    HeadArmor = 0xC,
    BodyArmor = 0xD,
    LegArmor = 0xE,
    HandArmor = 0xF,
    Horse = 0x10,
    Food = 0x11,
    Book = 0x12,
    Other = 0x13,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Other
    }
}

/// 物品标记位
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemFlags {
    pub can_penetrate_shield: bool,
    pub can_knock_down: bool,
    pub two_handed: bool,
    pub thrust_weapon: bool,
    pub swing_weapon: bool,
    pub unbalanced: bool,
    pub crush_through: bool,
    pub bonus_against_shield: bool,
}

// 使用trigger模块中的定义
pub use crate::models::trigger::Trigger;

/// 物品数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// 物品ID（数据库名称）
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 网格名称（3D模型）
    pub mesh_name: String,
    /// 物品类型
    pub item_type: ItemType,
    /// 价格
    pub price: u32,
    /// 重量
    pub weight: f32,
    /// 物品标记
    pub flags: ItemFlags,
    
    // 武器属性
    /// 伤害
    pub damage: u32,
    /// 速度
    pub speed: u32,
    /// 触及距离
    pub reach: u32,
    /// 精准度
    pub accuracy: u32,
    /// 护甲值
    pub armor: u32,
    /// 腿部护甲
    pub leg_armor: u32,
    /// 难度
    pub difficulty: u32,
    /// 命中点数
    pub hit_points: u32,
    
    // 材质和贴图
    /// 材质名称
    pub material: String,
    /// 贴图名称
    pub texture: String,
    
    // 触发器
    /// 触发器列表
    pub triggers: Vec<Trigger>,
    
    // 其他属性
    /// 物品能力
    pub capabilities: u64,
    /// 物品修饰符
    pub modifiers: Vec<(String, i32)>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            mesh_name: String::new(),
            item_type: ItemType::default(),
            price: 0,
            weight: 0.0,
            flags: ItemFlags::default(),
            damage: 0,
            speed: 0,
            reach: 0,
            accuracy: 0,
            armor: 0,
            leg_armor: 0,
            difficulty: 0,
            hit_points: 0,
            material: String::new(),
            texture: String::new(),
            triggers: Vec::new(),
            capabilities: 0,
            modifiers: Vec::new(),
        }
    }
}

impl Item {
    /// 创建新物品
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
    
    /// 检查物品是否为武器
    pub fn is_weapon(&self) -> bool {
        matches!(self.item_type, 
            ItemType::OneHandedWeapon | 
            ItemType::TwoHandedWeapon | 
            ItemType::Polearm | 
            ItemType::Archery | 
            ItemType::Crossbow | 
            ItemType::Throwing
        )
    }
    
    /// 检查物品是否为护甲
    pub fn is_armor(&self) -> bool {
        matches!(self.item_type,
            ItemType::HeadArmor |
            ItemType::BodyArmor |
            ItemType::LegArmor |
            ItemType::HandArmor
        )
    }
    
    /// 获取物品类型的中文名称
    pub fn get_type_name_cn(&self) -> &'static str {
        match self.item_type {
            ItemType::OneHandedWeapon => "单手武器",
            ItemType::TwoHandedWeapon => "双手武器",
            ItemType::Polearm => "长柄武器",
            ItemType::Archery => "弓箭",
            ItemType::Crossbow => "弩",
            ItemType::Throwing => "投掷武器",
            ItemType::Shield => "盾牌",
            ItemType::HeadArmor => "头部护甲",
            ItemType::BodyArmor => "身体护甲",
            ItemType::LegArmor => "腿部护甲",
            ItemType::HandArmor => "手部护甲",
            ItemType::Horse => "马匹",
            ItemType::Food => "食物",
            ItemType::Book => "书籍",
            ItemType::Other => "其他",
        }
    }
    
    /// 验证物品数据完整性
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("物品ID不能为空".to_string());
        }
        
        if self.name.is_empty() {
            return Err("物品名称不能为空".to_string());
        }
        
        if self.price > 999999 {
            return Err("物品价格不能超过999999".to_string());
        }
        
        if self.weight < 0.0 || self.weight > 100.0 {
            return Err("物品重量必须在0-100之间".to_string());
        }
        
        Ok(())
    }
}

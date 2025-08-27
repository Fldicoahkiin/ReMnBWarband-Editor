use serde::{Deserialize, Serialize};
use crate::models::trigger::Trigger;

/// 兵种类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TroopType {
    Player = 0,
    Regular = 1,
    Mounted = 2,
    Ranged = 3,
    Hero = 4,
}

impl Default for TroopType {
    fn default() -> Self {
        TroopType::Regular
    }
}

/// 兵种标记
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopFlags {
    pub hero: bool,
    pub female: bool,
    pub guarantee_boots: bool,
    pub guarantee_armor: bool,
    pub guarantee_helmet: bool,
    pub guarantee_horse: bool,
    pub guarantee_ranged: bool,
    pub no_capture_alive: bool,
}

/// 兵种属性
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopAttributes {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub charisma: u32,
}

/// 兵种技能
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopSkills {
    pub ironflesh: u32,
    pub power_strike: u32,
    pub power_throw: u32,
    pub power_draw: u32,
    pub weapon_master: u32,
    pub shield: u32,
    pub athletics: u32,
    pub riding: u32,
    pub horse_archery: u32,
    pub looting: u32,
    pub trainer: u32,
    pub tracking: u32,
    pub tactics: u32,
    pub path_finding: u32,
    pub spotting: u32,
    pub inventory_management: u32,
    pub wound_treatment: u32,
    pub surgery: u32,
    pub first_aid: u32,
    pub engineer: u32,
    pub persuasion: u32,
    pub prisoner_management: u32,
    pub leadership: u32,
    pub trade: u32,
}

/// 武器熟练度
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WeaponProficiency {
    pub one_handed: u32,
    pub two_handed: u32,
    pub polearm: u32,
    pub archery: u32,
    pub crossbow: u32,
    pub throwing: u32,
}

/// 装备项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub item_id: String,
    pub modifier: i32,
}

/// 兵种数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Troop {
    /// 兵种ID
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 复数名称
    pub plural_name: String,
    /// 兵种类型
    pub troop_type: TroopType,
    /// 兵种标记
    pub flags: TroopFlags,
    /// 等级
    pub level: u32,
    /// 升级兵种ID
    pub upgrade_troop: Option<String>,
    /// 升级所需经验
    pub upgrade_exp: u32,
    /// 属性
    pub attributes: TroopAttributes,
    /// 技能
    pub skills: TroopSkills,
    /// 武器熟练度
    pub proficiency: WeaponProficiency,
    /// 装备列表
    pub equipment: Vec<Equipment>,
    /// 面部代码
    pub face_key_1: u64,
    pub face_key_2: u64,
    /// 声音
    pub voice: String,
    /// 触发器
    pub triggers: Vec<Trigger>,
    /// 派系
    pub faction: Option<String>,
    /// 薪水
    pub wage: u32,
}

impl Default for Troop {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            plural_name: String::new(),
            troop_type: TroopType::default(),
            flags: TroopFlags::default(),
            level: 1,
            upgrade_troop: None,
            upgrade_exp: 0,
            attributes: TroopAttributes::default(),
            skills: TroopSkills::default(),
            proficiency: WeaponProficiency::default(),
            equipment: Vec::new(),
            face_key_1: 0,
            face_key_2: 0,
            voice: String::new(),
            triggers: Vec::new(),
            faction: None,
            wage: 0,
        }
    }
}

impl Troop {
    /// 创建新兵种
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
    
    /// 获取兵种类型的中文名称
    pub fn get_type_name_cn(&self) -> &'static str {
        match self.troop_type {
            TroopType::Player => "玩家",
            TroopType::Regular => "普通兵种",
            TroopType::Mounted => "骑兵",
            TroopType::Ranged => "远程兵种",
            TroopType::Hero => "英雄",
        }
    }
    
    /// 计算总属性点数
    pub fn total_attribute_points(&self) -> u32 {
        self.attributes.strength + 
        self.attributes.agility + 
        self.attributes.intelligence + 
        self.attributes.charisma
    }
    
    /// 计算总技能点数
    pub fn total_skill_points(&self) -> u32 {
        self.skills.ironflesh + self.skills.power_strike + self.skills.power_throw +
        self.skills.power_draw + self.skills.weapon_master + self.skills.shield +
        self.skills.athletics + self.skills.riding + self.skills.horse_archery +
        self.skills.looting + self.skills.trainer + self.skills.tracking +
        self.skills.tactics + self.skills.path_finding + self.skills.spotting +
        self.skills.inventory_management + self.skills.wound_treatment + 
        self.skills.surgery + self.skills.first_aid + self.skills.engineer +
        self.skills.persuasion + self.skills.prisoner_management + 
        self.skills.leadership + self.skills.trade
    }
    
    /// 验证兵种数据完整性
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("兵种ID不能为空".to_string());
        }
        
        if self.name.is_empty() {
            return Err("兵种名称不能为空".to_string());
        }
        
        if self.level == 0 || self.level > 63 {
            return Err("兵种等级必须在1-63之间".to_string());
        }
        
        // 验证属性值范围
        if self.attributes.strength > 63 || self.attributes.agility > 63 ||
           self.attributes.intelligence > 63 || self.attributes.charisma > 63 {
            return Err("属性值不能超过63".to_string());
        }
        
        Ok(())
    }
}

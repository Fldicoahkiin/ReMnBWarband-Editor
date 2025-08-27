/// 骑砍游戏常量定义

/// 物品类型常量
pub mod item_types {
    pub const ITP_TYPE_ONE_HANDED_WEAPON: i64 = 0x2;
    pub const ITP_TYPE_TWO_HANDED_WEAPON: i64 = 0x3;
    pub const ITP_TYPE_POLEARM: i64 = 0x4;
    pub const ITP_TYPE_ARCHERY: i64 = 0x5;
    pub const ITP_TYPE_CROSSBOW: i64 = 0x6;
    pub const ITP_TYPE_THROWING: i64 = 0x7;
    pub const ITP_TYPE_SHIELD: i64 = 0x8;
    pub const ITP_TYPE_HEAD_ARMOR: i64 = 0xC;
    pub const ITP_TYPE_BODY_ARMOR: i64 = 0xD;
    pub const ITP_TYPE_LEG_ARMOR: i64 = 0xE;
    pub const ITP_TYPE_HAND_ARMOR: i64 = 0xF;
    pub const ITP_TYPE_HORSE: i64 = 0x10;
    pub const ITP_TYPE_FOOD: i64 = 0x11;
    pub const ITP_TYPE_BOOK: i64 = 0x12;
    pub const ITP_TYPE_OTHER: i64 = 0x13;
}

/// 物品标记常量
pub mod item_flags {
    pub const ITF_CAN_PENETRATE_SHIELD: i64 = 0x1;
    pub const ITF_CAN_KNOCK_DOWN: i64 = 0x2;
    pub const ITF_TWO_HANDED: i64 = 0x4;
    pub const ITF_THRUST_WEAPON: i64 = 0x8;
    pub const ITF_SWING_WEAPON: i64 = 0x10;
    pub const ITF_UNBALANCED: i64 = 0x20;
    pub const ITF_CRUSH_THROUGH: i64 = 0x40;
    pub const ITF_BONUS_AGAINST_SHIELD: i64 = 0x80;
}

/// 兵种标记常量
pub mod troop_flags {
    pub const TF_HERO: i64 = 0x1;
    pub const TF_FEMALE: i64 = 0x2;
    pub const TF_GUARANTEE_BOOTS: i64 = 0x4;
    pub const TF_GUARANTEE_ARMOR: i64 = 0x8;
    pub const TF_GUARANTEE_HELMET: i64 = 0x10;
    pub const TF_GUARANTEE_HORSE: i64 = 0x20;
    pub const TF_GUARANTEE_RANGED: i64 = 0x40;
    pub const TF_NO_CAPTURE_ALIVE: i64 = 0x80;
}

/// 操作码常量
pub mod operation_codes {
    // 条件操作码
    pub const EQ: i32 = 1;
    pub const GT: i32 = 2;
    pub const GE: i32 = 3;
    pub const LT: i32 = 4;
    pub const LE: i32 = 5;
    pub const IS_BETWEEN: i32 = 6;
    
    // 控制流操作码
    pub const TRY_BEGIN: i32 = 10;
    pub const TRY_END: i32 = 11;
    pub const ELSE_TRY: i32 = 12;
    pub const TRY_FOR_RANGE: i32 = 13;
    pub const TRY_FOR_PARTIES: i32 = 14;
    
    // 赋值操作码
    pub const ASSIGN: i32 = 100;
    pub const STORE_ADD: i32 = 101;
    pub const STORE_SUB: i32 = 102;
    pub const STORE_MUL: i32 = 103;
    pub const STORE_DIV: i32 = 104;
    
    // 游戏逻辑操作码
    pub const DISPLAY_MESSAGE: i32 = 200;
    pub const JUMP_TO_MENU: i32 = 201;
    pub const CHANGE_SCREEN_RETURN: i32 = 202;
}

/// 技能ID常量
pub mod skills {
    pub const SKL_IRONFLESH: i32 = 0;
    pub const SKL_POWER_STRIKE: i32 = 1;
    pub const SKL_POWER_THROW: i32 = 2;
    pub const SKL_POWER_DRAW: i32 = 3;
    pub const SKL_WEAPON_MASTER: i32 = 4;
    pub const SKL_SHIELD: i32 = 5;
    pub const SKL_ATHLETICS: i32 = 6;
    pub const SKL_RIDING: i32 = 7;
    pub const SKL_HORSE_ARCHERY: i32 = 8;
    pub const SKL_LOOTING: i32 = 9;
    pub const SKL_TRAINER: i32 = 10;
    pub const SKL_TRACKING: i32 = 11;
    pub const SKL_TACTICS: i32 = 12;
    pub const SKL_PATH_FINDING: i32 = 13;
    pub const SKL_SPOTTING: i32 = 14;
    pub const SKL_INVENTORY_MANAGEMENT: i32 = 15;
    pub const SKL_WOUND_TREATMENT: i32 = 16;
    pub const SKL_SURGERY: i32 = 17;
    pub const SKL_FIRST_AID: i32 = 18;
    pub const SKL_ENGINEER: i32 = 19;
    pub const SKL_PERSUASION: i32 = 20;
    pub const SKL_PRISONER_MANAGEMENT: i32 = 21;
    pub const SKL_LEADERSHIP: i32 = 22;
    pub const SKL_TRADE: i32 = 23;
}

/// 属性ID常量
pub mod attributes {
    pub const ATT_STRENGTH: i32 = 0;
    pub const ATT_AGILITY: i32 = 1;
    pub const ATT_INTELLIGENCE: i32 = 2;
    pub const ATT_CHARISMA: i32 = 3;
}

/// 武器熟练度常量
pub mod weapon_proficiencies {
    pub const WPT_ONE_HANDED_WEAPON: i32 = 0;
    pub const WPT_TWO_HANDED_WEAPON: i32 = 1;
    pub const WPT_POLEARM: i32 = 2;
    pub const WPT_ARCHERY: i32 = 3;
    pub const WPT_CROSSBOW: i32 = 4;
    pub const WPT_THROWING: i32 = 5;
}

/// 游戏限制常量
pub mod limits {
    pub const MAX_ATTRIBUTE_VALUE: u32 = 63;
    pub const MAX_SKILL_VALUE: u32 = 15;
    pub const MAX_WEAPON_PROFICIENCY: u32 = 1000;
    pub const MAX_TROOP_LEVEL: u32 = 63;
    pub const MAX_ITEM_PRICE: u32 = 999999;
    pub const MAX_ITEM_WEIGHT: f32 = 100.0;
    pub const MAX_WEAPON_DAMAGE: u32 = 999;
    pub const MAX_WEAPON_SPEED: u32 = 200;
    pub const MAX_WEAPON_REACH: u32 = 300;
}

/// 默认值常量
pub mod defaults {
    pub const DEFAULT_TROOP_LEVEL: u32 = 1;
    pub const DEFAULT_ITEM_PRICE: u32 = 100;
    pub const DEFAULT_ITEM_WEIGHT: f32 = 1.0;
    pub const DEFAULT_WEAPON_DAMAGE: u32 = 30;
    pub const DEFAULT_WEAPON_SPEED: u32 = 95;
    pub const DEFAULT_WEAPON_REACH: u32 = 100;
}

/// 文件名常量
pub mod file_names {
    pub const ITEMS_FILE: &str = "item_kinds1.txt";
    pub const TROOPS_FILE: &str = "troops.txt";
    pub const FACTIONS_FILE: &str = "factions.txt";
    pub const SCENES_FILE: &str = "scenes.txt";
    pub const PARTIES_FILE: &str = "parties.txt";
    pub const PARTY_TEMPLATES_FILE: &str = "party_templates.txt";
    pub const MAP_ICONS_FILE: &str = "map_icons.txt";
    pub const SOUNDS_FILE: &str = "sounds.txt";
    pub const MUSIC_FILE: &str = "music.txt";
    pub const MESHES_FILE: &str = "meshes.txt";
    pub const MATERIALS_FILE: &str = "materials.txt";
    pub const TEXTURES_FILE: &str = "textures.txt";
}

/// 操作码名称映射
pub fn get_operation_name(opcode: i32) -> &'static str {
    match opcode {
        operation_codes::EQ => "eq",
        operation_codes::GT => "gt",
        operation_codes::GE => "ge",
        operation_codes::LT => "lt",
        operation_codes::LE => "le",
        operation_codes::IS_BETWEEN => "is_between",
        operation_codes::TRY_BEGIN => "try_begin",
        operation_codes::TRY_END => "try_end",
        operation_codes::ELSE_TRY => "else_try",
        operation_codes::ASSIGN => "assign",
        operation_codes::STORE_ADD => "store_add",
        operation_codes::STORE_SUB => "store_sub",
        operation_codes::STORE_MUL => "store_mul",
        operation_codes::STORE_DIV => "store_div",
        operation_codes::DISPLAY_MESSAGE => "display_message",
        operation_codes::JUMP_TO_MENU => "jump_to_menu",
        operation_codes::CHANGE_SCREEN_RETURN => "change_screen_return",
        _ => "unknown_operation",
    }
}

/// 技能名称映射
pub fn get_skill_name(skill_id: i32) -> &'static str {
    match skill_id {
        skills::SKL_IRONFLESH => "铁骨",
        skills::SKL_POWER_STRIKE => "强击",
        skills::SKL_POWER_THROW => "强掷",
        skills::SKL_POWER_DRAW => "强弓",
        skills::SKL_WEAPON_MASTER => "武器掌握",
        skills::SKL_SHIELD => "盾牌",
        skills::SKL_ATHLETICS => "跑动",
        skills::SKL_RIDING => "骑术",
        skills::SKL_HORSE_ARCHERY => "骑射",
        skills::SKL_LOOTING => "掠夺",
        skills::SKL_TRAINER => "训练",
        skills::SKL_TRACKING => "追踪",
        skills::SKL_TACTICS => "战术",
        skills::SKL_PATH_FINDING => "向导",
        skills::SKL_SPOTTING => "侦察",
        skills::SKL_INVENTORY_MANAGEMENT => "物品管理",
        skills::SKL_WOUND_TREATMENT => "治疗",
        skills::SKL_SURGERY => "手术",
        skills::SKL_FIRST_AID => "急救",
        skills::SKL_ENGINEER => "工程学",
        skills::SKL_PERSUASION => "说服",
        skills::SKL_PRISONER_MANAGEMENT => "俘虏管理",
        skills::SKL_LEADERSHIP => "统御",
        skills::SKL_TRADE => "交易",
        _ => "未知技能",
    }
}

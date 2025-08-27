use super::{DataParser, ParseError, ParseUtils};
use crate::models::{Item, ItemType, ItemFlags};
use anyhow::Result;
use regex::Regex;

pub struct ItemParser {
    item_type_regex: Regex,
}

impl ItemParser {
    pub fn new() -> Self {
        Self {
            item_type_regex: Regex::new(r"itp_type_(\w+)").unwrap(),
        }
    }
    
    /// 解析物品类型标记
    fn parse_item_type(&self, flags: i64) -> ItemType {
        match flags & 0xFF {
            0x2 => ItemType::OneHandedWeapon,
            0x3 => ItemType::TwoHandedWeapon,
            0x4 => ItemType::Polearm,
            0x5 => ItemType::Archery,
            0x6 => ItemType::Crossbow,
            0x7 => ItemType::Throwing,
            0x8 => ItemType::Shield,
            0xC => ItemType::HeadArmor,
            0xD => ItemType::BodyArmor,
            0xE => ItemType::LegArmor,
            0xF => ItemType::HandArmor,
            0x10 => ItemType::Horse,
            0x11 => ItemType::Food,
            0x12 => ItemType::Book,
            _ => ItemType::Other,
        }
    }
    
    /// 解析物品标记
    fn parse_item_flags(&self, flags: i64) -> ItemFlags {
        ItemFlags {
            can_penetrate_shield: (flags & 0x1) != 0,
            can_knock_down: (flags & 0x2) != 0,
            two_handed: (flags & 0x4) != 0,
            thrust_weapon: (flags & 0x8) != 0,
            swing_weapon: (flags & 0x10) != 0,
            unbalanced: (flags & 0x20) != 0,
            crush_through: (flags & 0x40) != 0,
            bonus_against_shield: (flags & 0x80) != 0,
        }
    }
    
    /// 序列化物品类型为数值
    fn serialize_item_type(&self, item_type: &ItemType) -> i64 {
        match item_type {
            ItemType::OneHandedWeapon => 0x2,
            ItemType::TwoHandedWeapon => 0x3,
            ItemType::Polearm => 0x4,
            ItemType::Archery => 0x5,
            ItemType::Crossbow => 0x6,
            ItemType::Throwing => 0x7,
            ItemType::Shield => 0x8,
            ItemType::HeadArmor => 0xC,
            ItemType::BodyArmor => 0xD,
            ItemType::LegArmor => 0xE,
            ItemType::HandArmor => 0xF,
            ItemType::Horse => 0x10,
            ItemType::Food => 0x11,
            ItemType::Book => 0x12,
            ItemType::Other => 0x13,
        }
    }
    
    /// 序列化物品标记为数值
    fn serialize_item_flags(&self, flags: &ItemFlags) -> i64 {
        let mut result = 0i64;
        if flags.can_penetrate_shield { result |= 0x1; }
        if flags.can_knock_down { result |= 0x2; }
        if flags.two_handed { result |= 0x4; }
        if flags.thrust_weapon { result |= 0x8; }
        if flags.swing_weapon { result |= 0x10; }
        if flags.unbalanced { result |= 0x20; }
        if flags.crush_through { result |= 0x40; }
        if flags.bonus_against_shield { result |= 0x80; }
        result
    }
}

impl DataParser<Item> for ItemParser {
    fn parse_text(&self, data: &str) -> Result<Vec<Item>, ParseError> {
        let mut items = Vec::new();
        let lines = ParseUtils::split_lines(data);
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            
            // 跳过非物品定义行
            if !line.starts_with('[') || !line.ends_with(']') {
                i += 1;
                continue;
            }
            
            // 解析物品ID
            let item_id = ParseUtils::parse_string(&line[1..line.len()-1]);
            let mut item = Item::new(item_id, String::new());
            
            i += 1;
            
            // 解析物品属性
            while i < lines.len() && !lines[i].starts_with('[') {
                let line = lines[i];
                let parts = ParseUtils::split_params(line);
                
                if parts.is_empty() {
                    i += 1;
                    continue;
                }
                
                match parts[0] {
                    "name" => {
                        if parts.len() > 1 {
                            item.name = ParseUtils::parse_string(&parts[1..].join(" "));
                        }
                    },
                    "mesh" => {
                        if parts.len() > 1 {
                            item.mesh_name = ParseUtils::parse_string(parts[1]);
                        }
                    },
                    "price" => {
                        if parts.len() > 1 {
                            item.price = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "weight" => {
                        if parts.len() > 1 {
                            item.weight = ParseUtils::parse_float(parts[1])? as f32;
                        }
                    },
                    "damage" => {
                        if parts.len() > 1 {
                            item.damage = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "speed" => {
                        if parts.len() > 1 {
                            item.speed = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "reach" => {
                        if parts.len() > 1 {
                            item.reach = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "flags" => {
                        if parts.len() > 1 {
                            let flags_value = ParseUtils::parse_int(parts[1])?;
                            item.item_type = self.parse_item_type(flags_value);
                            item.flags = self.parse_item_flags(flags_value);
                        }
                    },
                    _ => {
                        // 忽略未知属性
                    }
                }
                
                i += 1;
            }
            
            // 验证并添加物品
            if let Err(e) = item.validate() {
                tracing::warn!("物品 {} 验证失败: {}", item.id, e);
            } else {
                items.push(item);
            }
        }
        
        Ok(items)
    }
    
    fn serialize_text(&self, items: &[Item]) -> Result<String, ParseError> {
        let mut result = String::new();
        
        for item in items {
            result.push_str(&format!("[{}]\n", item.id));
            result.push_str(&format!("name {}\n", item.name));
            result.push_str(&format!("mesh {}\n", item.mesh_name));
            result.push_str(&format!("price {}\n", item.price));
            result.push_str(&format!("weight {:.2}\n", item.weight));
            
            if item.is_weapon() {
                result.push_str(&format!("damage {}\n", item.damage));
                result.push_str(&format!("speed {}\n", item.speed));
                result.push_str(&format!("reach {}\n", item.reach));
            }
            
            // 序列化标记
            let type_flags = self.serialize_item_type(&item.item_type);
            let item_flags = self.serialize_item_flags(&item.flags);
            let combined_flags = type_flags | item_flags;
            result.push_str(&format!("flags 0x{:X}\n", combined_flags));
            
            result.push('\n');
        }
        
        Ok(result)
    }
    
    fn validate(&self, item: &Item) -> Result<(), ParseError> {
        item.validate()
            .map_err(|e| ParseError::ValidationError(e))
    }
}

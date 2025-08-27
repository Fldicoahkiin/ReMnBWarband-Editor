use super::{DataParser, ParseError, ParseUtils};
use crate::models::troop::{Troop, TroopType, TroopAttributes, TroopFlags, WeaponProficiency, Equipment};
use anyhow::Result;

pub struct TroopParser;

impl TroopParser {
    pub fn new() -> Self {
        Self
    }
    
    /// 解析兵种类型
    fn parse_troop_type(&self, type_value: i64) -> TroopType {
        match type_value {
            0 => TroopType::Player,
            1 => TroopType::Regular,
            2 => TroopType::Mounted,
            3 => TroopType::Ranged,
            4 => TroopType::Hero,
            _ => TroopType::Regular,
        }
    }
    
    /// 解析兵种标记
    fn parse_troop_flags(&self, flags: i64) -> TroopFlags {
        TroopFlags {
            hero: (flags & 0x1) != 0,
            female: (flags & 0x2) != 0,
            guarantee_boots: (flags & 0x4) != 0,
            guarantee_armor: (flags & 0x8) != 0,
            guarantee_helmet: (flags & 0x10) != 0,
            guarantee_horse: (flags & 0x20) != 0,
            guarantee_ranged: (flags & 0x40) != 0,
            no_capture_alive: (flags & 0x80) != 0,
        }
    }
    
    /// 序列化兵种类型
    fn serialize_troop_type(&self, troop_type: &TroopType) -> i64 {
        match troop_type {
            TroopType::Player => 0,
            TroopType::Regular => 1,
            TroopType::Mounted => 2,
            TroopType::Ranged => 3,
            TroopType::Hero => 4,
        }
    }
    
    /// 序列化兵种标记
    fn serialize_troop_flags(&self, flags: &TroopFlags) -> i64 {
        let mut result = 0i64;
        if flags.hero { result |= 0x1; }
        if flags.female { result |= 0x2; }
        if flags.guarantee_boots { result |= 0x4; }
        if flags.guarantee_armor { result |= 0x8; }
        if flags.guarantee_helmet { result |= 0x10; }
        if flags.guarantee_horse { result |= 0x20; }
        if flags.guarantee_ranged { result |= 0x40; }
        if flags.no_capture_alive { result |= 0x80; }
        result
    }
}

impl DataParser<Troop> for TroopParser {
    fn parse_text(&self, data: &str) -> Result<Vec<Troop>, ParseError> {
        let mut troops = Vec::new();
        let lines = ParseUtils::split_lines(data);
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            
            // 跳过非兵种定义行
            if !line.starts_with('[') || !line.ends_with(']') {
                i += 1;
                continue;
            }
            
            // 解析兵种ID
            let troop_id = ParseUtils::parse_string(&line[1..line.len()-1]);
            let mut troop = Troop::new(troop_id, String::new());
            
            i += 1;
            
            // 解析兵种属性
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
                            troop.name = ParseUtils::parse_string(&parts[1..].join(" "));
                        }
                    },
                    "plural_name" => {
                        if parts.len() > 1 {
                            troop.plural_name = ParseUtils::parse_string(&parts[1..].join(" "));
                        }
                    },
                    "type" => {
                        if parts.len() > 1 {
                            let type_value = ParseUtils::parse_int(parts[1])?;
                            troop.troop_type = self.parse_troop_type(type_value);
                        }
                    },
                    "level" => {
                        if parts.len() > 1 {
                            troop.level = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "upgrade_troop" => {
                        if parts.len() > 1 {
                            troop.upgrade_troop = Some(ParseUtils::parse_string(parts[1]));
                        }
                    },
                    "upgrade_exp" => {
                        if parts.len() > 1 {
                            troop.upgrade_exp = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    "flags" => {
                        if parts.len() > 1 {
                            let flags_value = ParseUtils::parse_int(parts[1])?;
                            troop.flags = self.parse_troop_flags(flags_value);
                        }
                    },
                    "attributes" => {
                        if parts.len() >= 5 {
                            troop.attributes = TroopAttributes {
                                strength: ParseUtils::parse_int(parts[1])? as u32,
                                agility: ParseUtils::parse_int(parts[2])? as u32,
                                intelligence: ParseUtils::parse_int(parts[3])? as u32,
                                charisma: ParseUtils::parse_int(parts[4])? as u32,
                            };
                        }
                    },
                    "skills" => {
                        // 解析技能（简化版本，实际需要更复杂的解析）
                        if parts.len() > 1 {
                            // 这里可以扩展解析所有技能
                            troop.skills.ironflesh = if parts.len() > 1 { ParseUtils::parse_int(parts[1])? as u32 } else { 0 };
                            troop.skills.power_strike = if parts.len() > 2 { ParseUtils::parse_int(parts[2])? as u32 } else { 0 };
                            // ... 其他技能
                        }
                    },
                    "proficiency" => {
                        if parts.len() >= 7 {
                            troop.proficiency = WeaponProficiency {
                                one_handed: ParseUtils::parse_int(parts[1])? as u32,
                                two_handed: ParseUtils::parse_int(parts[2])? as u32,
                                polearm: ParseUtils::parse_int(parts[3])? as u32,
                                archery: ParseUtils::parse_int(parts[4])? as u32,
                                crossbow: ParseUtils::parse_int(parts[5])? as u32,
                                throwing: ParseUtils::parse_int(parts[6])? as u32,
                            };
                        }
                    },
                    "equipment" => {
                        if parts.len() > 1 {
                            let equipment = Equipment {
                                item_id: ParseUtils::parse_string(parts[1]),
                                modifier: if parts.len() > 2 { ParseUtils::parse_int(parts[2])? as i32 } else { 0 },
                            };
                            troop.equipment.push(equipment);
                        }
                    },
                    "wage" => {
                        if parts.len() > 1 {
                            troop.wage = ParseUtils::parse_int(parts[1])? as u32;
                        }
                    },
                    _ => {
                        // 忽略未知属性
                    }
                }
                
                i += 1;
            }
            
            // 验证并添加兵种
            if let Err(e) = troop.validate() {
                tracing::warn!("兵种 {} 验证失败: {}", troop.id, e);
            } else {
                troops.push(troop);
            }
        }
        
        Ok(troops)
    }
    
    fn serialize_text(&self, troops: &[Troop]) -> Result<String, ParseError> {
        let mut result = String::new();
        
        for troop in troops {
            result.push_str(&format!("[{}]\n", troop.id));
            result.push_str(&format!("name {}\n", troop.name));
            result.push_str(&format!("plural_name {}\n", troop.plural_name));
            result.push_str(&format!("type {}\n", self.serialize_troop_type(&troop.troop_type)));
            result.push_str(&format!("level {}\n", troop.level));
            
            if let Some(ref upgrade) = troop.upgrade_troop {
                result.push_str(&format!("upgrade_troop {}\n", upgrade));
                result.push_str(&format!("upgrade_exp {}\n", troop.upgrade_exp));
            }
            
            let flags = self.serialize_troop_flags(&troop.flags);
            result.push_str(&format!("flags 0x{:X}\n", flags));
            
            result.push_str(&format!("attributes {} {} {} {}\n",
                troop.attributes.strength,
                troop.attributes.agility,
                troop.attributes.intelligence,
                troop.attributes.charisma
            ));
            
            result.push_str(&format!("proficiency {} {} {} {} {} {}\n",
                troop.proficiency.one_handed,
                troop.proficiency.two_handed,
                troop.proficiency.polearm,
                troop.proficiency.archery,
                troop.proficiency.crossbow,
                troop.proficiency.throwing
            ));
            
            for equipment in &troop.equipment {
                result.push_str(&format!("equipment {} {}\n", equipment.item_id, equipment.modifier));
            }
            
            result.push_str(&format!("wage {}\n", troop.wage));
            result.push('\n');
        }
        
        Ok(result)
    }
    
    fn validate(&self, troop: &Troop) -> Result<(), ParseError> {
        troop.validate()
            .map_err(|e| ParseError::ValidationError(e))
    }
}

use super::{DataParser, ParseError, ParseUtils};
use crate::models::{Faction, FactionColor, FactionRelation};
use anyhow::Result;

pub struct FactionParser;

impl FactionParser {
    pub fn new() -> Self {
        Self
    }
}

impl DataParser<Faction> for FactionParser {
    fn parse_text(&self, data: &str) -> Result<Vec<Faction>, ParseError> {
        let mut factions = Vec::new();
        let lines = ParseUtils::split_lines(data);
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            
            // 跳过非派系定义行
            if !line.starts_with('[') || !line.ends_with(']') {
                i += 1;
                continue;
            }
            
            // 解析派系ID
            let faction_id = ParseUtils::parse_string(&line[1..line.len()-1]);
            let mut faction = Faction::new(faction_id, String::new());
            
            i += 1;
            
            // 解析派系属性
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
                            faction.name = ParseUtils::parse_string(&parts[1..].join(" "));
                        }
                    },
                    "flags" => {
                        if parts.len() > 1 {
                            faction.flags = ParseUtils::parse_int(parts[1])? as u64;
                        }
                    },
                    "color" => {
                        if parts.len() >= 4 {
                            faction.color = FactionColor {
                                r: ParseUtils::parse_int(parts[1])? as u8,
                                g: ParseUtils::parse_int(parts[2])? as u8,
                                b: ParseUtils::parse_int(parts[3])? as u8,
                            };
                        }
                    },
                    "relation" => {
                        if parts.len() >= 3 {
                            let relation = FactionRelation {
                                faction_id: ParseUtils::parse_string(parts[1]),
                                relation: ParseUtils::parse_float(parts[2])? as f32,
                            };
                            faction.relations.push(relation);
                        }
                    },
                    "ranking" => {
                        if parts.len() > 1 {
                            faction.ranking = ParseUtils::parse_float(parts[1])? as f32;
                        }
                    },
                    _ => {
                        // 忽略未知属性
                    }
                }
                
                i += 1;
            }
            
            // 验证并添加派系
            if let Err(e) = faction.validate() {
                tracing::warn!("派系 {} 验证失败: {}", faction.id, e);
            } else {
                factions.push(faction);
            }
        }
        
        Ok(factions)
    }
    
    fn serialize_text(&self, factions: &[Faction]) -> Result<String, ParseError> {
        let mut result = String::new();
        
        for faction in factions {
            result.push_str(&format!("[{}]\n", faction.id));
            result.push_str(&format!("name {}\n", faction.name));
            result.push_str(&format!("flags 0x{:X}\n", faction.flags));
            result.push_str(&format!("color {} {} {}\n", 
                faction.color.r, faction.color.g, faction.color.b));
            
            for relation in &faction.relations {
                result.push_str(&format!("relation {} {:.2}\n", 
                    relation.faction_id, relation.relation));
            }
            
            result.push_str(&format!("ranking {:.2}\n", faction.ranking));
            result.push('\n');
        }
        
        Ok(result)
    }
    
    fn validate(&self, faction: &Faction) -> Result<(), ParseError> {
        faction.validate()
            .map_err(|e| ParseError::ValidationError(e))
    }
}

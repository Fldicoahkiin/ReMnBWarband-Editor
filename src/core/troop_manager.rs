use std::collections::HashMap;
use anyhow::Result;
use crate::models::troop::{Troop, Equipment};

/// 兵种管理器 - 负责兵种数据的内存管理和操作
#[derive(Debug, Clone)]
pub struct TroopManager {
    troops: Vec<Troop>,
    selected_index: Option<usize>,
    search_filter: String,
}

impl TroopManager {
    pub fn new() -> Self {
        Self {
            troops: Vec::new(),
            selected_index: None,
            search_filter: String::new(),
        }
    }

    /// 加载兵种列表
    pub fn load_troops(&mut self, troops: Vec<Troop>) {
        self.troops = troops;
        self.selected_index = None;
    }

    /// 添加单个兵种
    pub fn add_troop(&mut self, troop: Troop) {
        self.troops.push(troop);
    }

    /// 获取所有兵种
    pub fn get_troops(&self) -> &[Troop] {
        &self.troops
    }

    /// 获取过滤后的兵种列表
    pub fn get_filtered_troops(&self) -> Vec<(usize, &Troop)> {
        if self.search_filter.is_empty() {
            self.troops.iter().enumerate().collect()
        } else {
            self.troops
                .iter()
                .enumerate()
                .filter(|(_, troop)| {
                    troop.name.to_lowercase().contains(&self.search_filter.to_lowercase())
                        || troop.id.to_lowercase().contains(&self.search_filter.to_lowercase())
                })
                .collect()
        }
    }

    /// 设置搜索过滤器
    pub fn set_search_filter(&mut self, filter: String) {
        self.search_filter = filter;
    }

    /// 选择兵种
    pub fn select_troop(&mut self, index: usize) -> Option<&Troop> {
        if index < self.troops.len() {
            self.selected_index = Some(index);
            Some(&self.troops[index])
        } else {
            None
        }
    }

    /// 获取当前选中的兵种
    pub fn get_selected_troop(&self) -> Option<&Troop> {
        self.selected_index.and_then(|index| self.troops.get(index))
    }

    /// 获取当前选中的兵种（可变引用）
    pub fn get_selected_troop_mut(&mut self) -> Option<&mut Troop> {
        self.selected_index.and_then(|index| self.troops.get_mut(index))
    }

    /// 创建新兵种
    pub fn create_new_troop(&mut self) -> usize {
        let new_troop = Troop::new(
            format!("new_troop_{}", self.troops.len()),
            "新兵种".to_string()
        );

        self.troops.push(new_troop);
        let new_index = self.troops.len() - 1;
        self.selected_index = Some(new_index);
        new_index
    }

    /// 删除兵种
    pub fn delete_troop(&mut self, index: usize) -> Result<()> {
        if index < self.troops.len() {
            self.troops.remove(index);
            
            // 调整选中索引
            if let Some(selected) = self.selected_index {
                if selected == index {
                    self.selected_index = None;
                } else if selected > index {
                    self.selected_index = Some(selected - 1);
                }
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("兵种索引超出范围"))
        }
    }

    /// 复制兵种
    pub fn duplicate_troop(&mut self, index: usize) -> Result<usize> {
        if let Some(troop) = self.troops.get(index).cloned() {
            let mut new_troop = troop;
            new_troop.id = format!("{}_copy", new_troop.id);
            new_troop.name = format!("{} (副本)", new_troop.name);
            new_troop.plural_name = format!("{} (副本)", new_troop.plural_name);
            
            self.troops.push(new_troop);
            let new_index = self.troops.len() - 1;
            self.selected_index = Some(new_index);
            Ok(new_index)
        } else {
            Err(anyhow::anyhow!("兵种索引超出范围"))
        }
    }

    /// 更新选中兵种的基础信息
    pub fn update_selected_troop_basic(&mut self, 
        id: String, 
        name: String, 
        plural_name: String,
        level: u32, 
        _class: i32
    ) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            troop.id = id;
            troop.name = name;
            troop.plural_name = plural_name;
            troop.level = level;
            // TODO: Troop结构没有class字段，需要根据实际需求调整
            // troop.class = class;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 更新选中兵种的属性
    pub fn update_selected_troop_attributes(&mut self, 
        strength: u32, 
        agility: u32, 
        intelligence: u32, 
        charisma: u32
    ) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            troop.attributes.strength = strength;
            troop.attributes.agility = agility;
            troop.attributes.intelligence = intelligence;
            troop.attributes.charisma = charisma;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 更新选中兵种的技能
    pub fn update_selected_troop_skills(&mut self, skills: HashMap<String, u32>) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            // 更新技能值
            troop.skills.ironflesh = skills.get("ironflesh").copied().unwrap_or(0);
            troop.skills.power_strike = skills.get("power_strike").copied().unwrap_or(0);
            troop.skills.power_throw = skills.get("power_throw").copied().unwrap_or(0);
            troop.skills.power_draw = skills.get("power_draw").copied().unwrap_or(0);
            troop.skills.weapon_master = skills.get("weapon_master").copied().unwrap_or(0);
            troop.skills.shield = skills.get("shield").copied().unwrap_or(0);
            troop.skills.athletics = skills.get("athletics").copied().unwrap_or(0);
            troop.skills.riding = skills.get("riding").copied().unwrap_or(0);
            troop.skills.horse_archery = skills.get("horse_archery").copied().unwrap_or(0);
            troop.skills.looting = skills.get("looting").copied().unwrap_or(0);
            troop.skills.trainer = skills.get("trainer").copied().unwrap_or(0);
            troop.skills.tracking = skills.get("tracking").copied().unwrap_or(0);
            troop.skills.tactics = skills.get("tactics").copied().unwrap_or(0);
            troop.skills.path_finding = skills.get("path_finding").copied().unwrap_or(0);
            troop.skills.spotting = skills.get("spotting").copied().unwrap_or(0);
            troop.skills.inventory_management = skills.get("inventory_management").copied().unwrap_or(0);
            troop.skills.wound_treatment = skills.get("wound_treatment").copied().unwrap_or(0);
            troop.skills.surgery = skills.get("surgery").copied().unwrap_or(0);
            troop.skills.first_aid = skills.get("first_aid").copied().unwrap_or(0);
            troop.skills.engineer = skills.get("engineer").copied().unwrap_or(0);
            troop.skills.persuasion = skills.get("persuasion").copied().unwrap_or(0);
            troop.skills.prisoner_management = skills.get("prisoner_management").copied().unwrap_or(0);
            troop.skills.leadership = skills.get("leadership").copied().unwrap_or(0);
            troop.skills.trade = skills.get("trade").copied().unwrap_or(0);
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 更新选中兵种的武器熟练度
    pub fn update_selected_troop_weapon_proficiencies(&mut self, 
        one_handed: u32,
        two_handed: u32,
        polearm: u32,
        archery: u32,
        crossbow: u32,
        throwing: u32
    ) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            troop.proficiency.one_handed = one_handed;
            troop.proficiency.two_handed = two_handed;
            troop.proficiency.polearm = polearm;
            troop.proficiency.archery = archery;
            troop.proficiency.crossbow = crossbow;
            troop.proficiency.throwing = throwing;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 添加装备到选中兵种
    pub fn add_equipment_to_selected(&mut self, item_id: String, chance: u32) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            troop.equipment.push(Equipment { item_id, modifier: chance as i32 });
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 从选中兵种移除装备
    pub fn remove_equipment_from_selected(&mut self, index: usize) -> Result<()> {
        if let Some(troop) = self.get_selected_troop_mut() {
            if index < troop.equipment.len() {
                troop.equipment.remove(index);
                Ok(())
            } else {
                Err(anyhow::anyhow!("装备索引超出范围"))
            }
        } else {
            Err(anyhow::anyhow!("没有选中的兵种"))
        }
    }

    /// 获取兵种统计信息
    pub fn get_stats(&self) -> TroopStats {
        let total_troops = self.troops.len();
        let max_level = self.troops.iter().map(|t| t.level).max().unwrap_or(0);
        let avg_level = if total_troops > 0 {
            self.troops.iter().map(|t| t.level).sum::<u32>() as f32 / total_troops as f32
        } else {
            0.0
        };
        let filtered_count = self.get_filtered_troops().len();

        TroopStats {
            total_troops,
            max_level,
            avg_level,
            filtered_count,
        }
    }

    /// 验证所有兵种数据
    pub fn validate_all(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for (index, troop) in self.troops.iter().enumerate() {
            if troop.id.is_empty() {
                errors.push(format!("兵种 {} 缺少ID", index));
            }
            
            if troop.name.is_empty() {
                errors.push(format!("兵种 {} 缺少名称", index));
            }
            
            if troop.level == 0 {
                errors.push(format!("兵种 {} 等级不能为0", index));
            }
            
            if troop.level > 63 {
                errors.push(format!("兵种 {} 等级不能超过63", index));
            }
        }
        
        errors
    }
}

impl Default for TroopManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 兵种统计信息
#[derive(Debug, Clone)]
pub struct TroopStats {
    pub total_troops: usize,
    pub max_level: u32,
    pub avg_level: f32,
    pub filtered_count: usize,
}

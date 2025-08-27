// Validator module for data validation
use std::collections::{HashMap, HashSet};
use crate::models::{Item, Troop, Faction, Trigger, Operation};

/// 验证错误类型
#[derive(Debug, Clone)]
pub enum ValidationError {
    MissingReference(String),
    InvalidValue(String),
    DuplicateId(String),
    CircularReference(String),
    DataInconsistency(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingReference(msg) => write!(f, "缺少引用: {}", msg),
            ValidationError::InvalidValue(msg) => write!(f, "无效值: {}", msg),
            ValidationError::DuplicateId(msg) => write!(f, "重复ID: {}", msg),
            ValidationError::CircularReference(msg) => write!(f, "循环引用: {}", msg),
            ValidationError::DataInconsistency(msg) => write!(f, "数据不一致: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// 验证结果
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

/// 数据验证器
pub struct DataValidator {
    item_ids: HashSet<String>,
    troop_ids: HashSet<String>,
    faction_ids: HashSet<String>,
}

impl DataValidator {
    pub fn new() -> Self {
        Self {
            item_ids: HashSet::new(),
            troop_ids: HashSet::new(),
            faction_ids: HashSet::new(),
        }
    }
    
    /// 验证完整的游戏数据
    pub fn validate_game_data(
        &mut self,
        items: &[Item],
        troops: &[Troop],
        factions: &[Faction],
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // 收集所有ID
        self.collect_ids(items, troops, factions);
        
        // 验证物品
        let item_result = self.validate_items(items);
        result.merge(item_result);
        
        // 验证兵种
        let troop_result = self.validate_troops(troops);
        result.merge(troop_result);
        
        // 验证派系
        let faction_result = self.validate_factions(factions);
        result.merge(faction_result);
        
        // 验证交叉引用
        let cross_ref_result = self.validate_cross_references(items, troops, factions);
        result.merge(cross_ref_result);
        
        result
    }
    
    /// 收集所有ID
    fn collect_ids(&mut self, items: &[Item], troops: &[Troop], factions: &[Faction]) {
        self.item_ids.clear();
        self.troop_ids.clear();
        self.faction_ids.clear();
        
        for item in items {
            self.item_ids.insert(item.id.clone());
        }
        
        for troop in troops {
            self.troop_ids.insert(troop.id.clone());
        }
        
        for faction in factions {
            self.faction_ids.insert(faction.id.clone());
        }
    }
    
    /// 验证物品数据
    pub fn validate_items(&self, items: &[Item]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let mut seen_ids = HashSet::new();
        
        for item in items {
            // 检查重复ID
            if seen_ids.contains(&item.id) {
                result.add_error(ValidationError::DuplicateId(
                    format!("物品ID重复: {}", item.id)
                ));
            } else {
                seen_ids.insert(item.id.clone());
            }
            
            // 验证基本属性
            if let Err(e) = item.validate() {
                result.add_error(ValidationError::InvalidValue(
                    format!("物品 {} 验证失败: {}", item.id, e)
                ));
            }
            
            // 验证武器属性
            if item.is_weapon() {
                if item.damage == 0 {
                    result.add_warning(format!("武器 {} 的伤害为0", item.id));
                }
                if item.speed == 0 {
                    result.add_warning(format!("武器 {} 的速度为0", item.id));
                }
            }
            
            // 验证护甲属性
            if item.is_armor() {
                if item.armor == 0 {
                    result.add_warning(format!("护甲 {} 的防护值为0", item.id));
                }
            }
            
            // 验证价格合理性
            if item.price > 100000 {
                result.add_warning(format!("物品 {} 的价格过高: {}", item.id, item.price));
            }
            
            // 验证重量合理性
            if item.weight > 50.0 {
                result.add_warning(format!("物品 {} 的重量过重: {}", item.id, item.weight));
            }
        }
        
        result
    }
    
    /// 验证兵种数据
    pub fn validate_troops(&self, troops: &[Troop]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let mut seen_ids = HashSet::new();
        
        for troop in troops {
            // 检查重复ID
            if seen_ids.contains(&troop.id) {
                result.add_error(ValidationError::DuplicateId(
                    format!("兵种ID重复: {}", troop.id)
                ));
            } else {
                seen_ids.insert(troop.id.clone());
            }
            
            // 验证基本属性
            if let Err(e) = troop.validate() {
                result.add_error(ValidationError::InvalidValue(
                    format!("兵种 {} 验证失败: {}", troop.id, e)
                ));
            }
            
            // 验证升级兵种引用
            if let Some(ref upgrade_troop) = troop.upgrade_troop {
                if !self.troop_ids.contains(upgrade_troop) {
                    result.add_error(ValidationError::MissingReference(
                        format!("兵种 {} 的升级兵种 {} 不存在", troop.id, upgrade_troop)
                    ));
                }
            }
            
            // 验证装备引用
            for equipment in &troop.equipment {
                if !self.item_ids.contains(&equipment.item_id) {
                    result.add_error(ValidationError::MissingReference(
                        format!("兵种 {} 的装备 {} 不存在", troop.id, equipment.item_id)
                    ));
                }
            }
            
            // 验证属性平衡性
            let total_attributes = troop.total_attribute_points();
            if total_attributes > 200 {
                result.add_warning(format!("兵种 {} 的总属性点过高: {}", troop.id, total_attributes));
            }
            
            // 验证技能合理性
            let total_skills = troop.total_skill_points();
            if total_skills > 500 {
                result.add_warning(format!("兵种 {} 的总技能点过高: {}", troop.id, total_skills));
            }
        }
        
        result
    }
    
    /// 验证派系数据
    pub fn validate_factions(&self, factions: &[Faction]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let mut seen_ids = HashSet::new();
        
        for faction in factions {
            // 检查重复ID
            if seen_ids.contains(&faction.id) {
                result.add_error(ValidationError::DuplicateId(
                    format!("派系ID重复: {}", faction.id)
                ));
            } else {
                seen_ids.insert(faction.id.clone());
            }
            
            // 验证基本属性
            if let Err(e) = faction.validate() {
                result.add_error(ValidationError::InvalidValue(
                    format!("派系 {} 验证失败: {}", faction.id, e)
                ));
            }
            
            // 验证派系关系引用
            for relation in &faction.relations {
                if !self.faction_ids.contains(&relation.faction_id) {
                    result.add_error(ValidationError::MissingReference(
                        format!("派系 {} 的关系目标 {} 不存在", faction.id, relation.faction_id)
                    ));
                }
            }
        }
        
        result
    }
    
    /// 验证交叉引用
    pub fn validate_cross_references(
        &self,
        _items: &[Item],
        troops: &[Troop],
        factions: &[Faction],
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // 检查循环升级引用
        for troop in troops {
            if let Some(ref _upgrade_troop) = troop.upgrade_troop {
                if self.has_circular_upgrade(troop, troops, &mut HashSet::new()) {
                    result.add_error(ValidationError::CircularReference(
                        format!("兵种 {} 存在循环升级引用", troop.id)
                    ));
                }
            }
        }
        
        // 验证派系关系的对称性
        let faction_map: HashMap<String, &Faction> = factions.iter()
            .map(|f| (f.id.clone(), f))
            .collect();
        
        for faction in factions {
            for relation in &faction.relations {
                if let Some(target_faction) = faction_map.get(&relation.faction_id) {
                    // 检查对方是否也有对应的关系
                    let reverse_relation = target_faction.get_relation(&faction.id);
                    if reverse_relation.is_none() {
                        result.add_warning(format!(
                            "派系 {} 与 {} 的关系不对称",
                            faction.id, relation.faction_id
                        ));
                    }
                }
            }
        }
        
        result
    }
    
    /// 检查循环升级引用
    fn has_circular_upgrade(
        &self,
        current_troop: &Troop,
        all_troops: &[Troop],
        visited: &mut HashSet<String>,
    ) -> bool {
        if visited.contains(&current_troop.id) {
            return true;
        }
        
        visited.insert(current_troop.id.clone());
        
        if let Some(ref upgrade_id) = current_troop.upgrade_troop {
            if let Some(upgrade_troop) = all_troops.iter().find(|t| t.id == *upgrade_id) {
                if self.has_circular_upgrade(upgrade_troop, all_troops, visited) {
                    return true;
                }
            }
        }
        
        visited.remove(&current_troop.id);
        false
    }
    
    /// 验证触发器
    pub fn validate_triggers(&self, triggers: &[Trigger]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        for (index, trigger) in triggers.iter().enumerate() {
            if let Err(e) = trigger.validate() {
                result.add_error(ValidationError::InvalidValue(
                    format!("触发器 {} 验证失败: {}", index, e)
                ));
            }
            
            // 验证操作码的合理性
            for condition in &trigger.conditions {
                self.validate_operation(condition, &mut result);
            }
            
            for consequence in &trigger.consequences {
                self.validate_operation(consequence, &mut result);
            }
        }
        
        result
    }
    
    /// 验证单个操作
    fn validate_operation(&self, operation: &Operation, result: &mut ValidationResult) {
        if let Err(e) = operation.validate_parameters() {
            result.add_error(ValidationError::InvalidValue(
                format!("操作 {} 验证失败: {}", operation.name, e)
            ));
        }
        
        // 验证参数数量（这里可以根据具体的操作码进行更详细的验证）
        if operation.parameters.is_empty() && operation.requires_parameters() {
            result.add_error(ValidationError::InvalidValue(
                format!("操作 {} 缺少必需的参数", operation.name)
            ));
        }
    }
}

use std::collections::HashMap;
use anyhow::Result;
use crate::models::trigger::{Trigger, Operation, OperationType, Parameter, ParameterType};

/// 触发器管理器 - 负责触发器数据的内存管理和操作
#[derive(Debug, Clone)]
pub struct TriggerManager {
    triggers: Vec<Trigger>,
    selected_index: Option<usize>,
    search_filter: String,
    operation_templates: HashMap<i32, OperationTemplate>,
}

/// 操作模板 - 定义操作的参数要求
#[derive(Debug, Clone)]
pub struct OperationTemplate {
    pub opcode: i32,
    pub name: String,
    pub operation_type: OperationType,
    pub description: String,
    pub parameters: Vec<ParameterTemplate>,
}

/// 参数模板
#[derive(Debug, Clone)]
pub struct ParameterTemplate {
    pub name: String,
    pub param_type: ParameterType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<i64>,
}

impl TriggerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            triggers: Vec::new(),
            selected_index: None,
            search_filter: String::new(),
            operation_templates: HashMap::new(),
        };
        
        manager.init_operation_templates();
        manager
    }

    /// 初始化操作模板
    fn init_operation_templates(&mut self) {
        // 添加常用操作模板
        self.add_operation_template(OperationTemplate {
            opcode: 1,
            name: "try_begin".to_string(),
            operation_type: OperationType::Condition,
            description: "开始条件判断块".to_string(),
            parameters: vec![],
        });

        self.add_operation_template(OperationTemplate {
            opcode: 2,
            name: "try_end".to_string(),
            operation_type: OperationType::Condition,
            description: "结束条件判断块".to_string(),
            parameters: vec![],
        });

        self.add_operation_template(OperationTemplate {
            opcode: 3,
            name: "else_try".to_string(),
            operation_type: OperationType::Condition,
            description: "否则条件判断".to_string(),
            parameters: vec![],
        });

        self.add_operation_template(OperationTemplate {
            opcode: 4,
            name: "assign".to_string(),
            operation_type: OperationType::Assignment,
            description: "赋值操作".to_string(),
            parameters: vec![
                ParameterTemplate {
                    name: "destination".to_string(),
                    param_type: ParameterType::Register,
                    description: "目标寄存器".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "value".to_string(),
                    param_type: ParameterType::Constant,
                    description: "赋值".to_string(),
                    required: true,
                    default_value: Some(0),
                },
            ],
        });

        self.add_operation_template(OperationTemplate {
            opcode: 5,
            name: "store_add".to_string(),
            operation_type: OperationType::Assignment,
            description: "加法赋值".to_string(),
            parameters: vec![
                ParameterTemplate {
                    name: "destination".to_string(),
                    param_type: ParameterType::Register,
                    description: "目标寄存器".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "operand1".to_string(),
                    param_type: ParameterType::Register,
                    description: "操作数1".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "operand2".to_string(),
                    param_type: ParameterType::Constant,
                    description: "操作数2".to_string(),
                    required: true,
                    default_value: Some(0),
                },
            ],
        });

        // 添加更多操作模板...
        self.add_common_operation_templates();
    }

    /// 添加常用操作模板
    fn add_common_operation_templates(&mut self) {
        // 条件判断操作
        self.add_operation_template(OperationTemplate {
            opcode: 30,
            name: "eq".to_string(),
            operation_type: OperationType::Condition,
            description: "等于判断".to_string(),
            parameters: vec![
                ParameterTemplate {
                    name: "value1".to_string(),
                    param_type: ParameterType::Register,
                    description: "值1".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "value2".to_string(),
                    param_type: ParameterType::Constant,
                    description: "值2".to_string(),
                    required: true,
                    default_value: Some(0),
                },
            ],
        });

        self.add_operation_template(OperationTemplate {
            opcode: 31,
            name: "ge".to_string(),
            operation_type: OperationType::Condition,
            description: "大于等于判断".to_string(),
            parameters: vec![
                ParameterTemplate {
                    name: "value1".to_string(),
                    param_type: ParameterType::Register,
                    description: "值1".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "value2".to_string(),
                    param_type: ParameterType::Constant,
                    description: "值2".to_string(),
                    required: true,
                    default_value: Some(0),
                },
            ],
        });

        // 游戏操作
        self.add_operation_template(OperationTemplate {
            opcode: 1030,
            name: "party_add_members".to_string(),
            operation_type: OperationType::Consequence,
            description: "向队伍添加成员".to_string(),
            parameters: vec![
                ParameterTemplate {
                    name: "party_id".to_string(),
                    param_type: ParameterType::Constant,
                    description: "队伍ID".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "troop_id".to_string(),
                    param_type: ParameterType::Constant,
                    description: "兵种ID".to_string(),
                    required: true,
                    default_value: None,
                },
                ParameterTemplate {
                    name: "number".to_string(),
                    param_type: ParameterType::Constant,
                    description: "数量".to_string(),
                    required: true,
                    default_value: Some(1),
                },
            ],
        });
    }

    /// 添加操作模板
    fn add_operation_template(&mut self, template: OperationTemplate) {
        self.operation_templates.insert(template.opcode, template);
    }

    /// 获取操作模板
    pub fn get_operation_template(&self, opcode: i32) -> Option<&OperationTemplate> {
        self.operation_templates.get(&opcode)
    }

    /// 获取所有操作模板
    pub fn get_all_operation_templates(&self) -> Vec<&OperationTemplate> {
        let mut templates: Vec<_> = self.operation_templates.values().collect();
        templates.sort_by_key(|t| t.opcode);
        templates
    }

    /// 按类型获取操作模板
    pub fn get_operation_templates_by_type(&self, op_type: OperationType) -> Vec<&OperationTemplate> {
        self.operation_templates
            .values()
            .filter(|t| t.operation_type == op_type)
            .collect()
    }

    /// 加载触发器列表
    pub fn load_triggers(&mut self, triggers: Vec<Trigger>) {
        self.triggers = triggers;
        self.selected_index = None;
    }

    /// 获取所有触发器
    pub fn get_triggers(&self) -> &[Trigger] {
        &self.triggers
    }

    /// 获取当前选中的触发器
    pub fn get_selected_trigger(&self) -> Option<&Trigger> {
        self.selected_index.and_then(|index| self.triggers.get(index))
    }

    /// 获取当前选中的触发器（可变引用）
    pub fn get_selected_trigger_mut(&mut self) -> Option<&mut Trigger> {
        self.selected_index.and_then(|index| self.triggers.get_mut(index))
    }

    /// 获取过滤后的触发器列表
    pub fn get_filtered_triggers(&self) -> Vec<(usize, &Trigger)> {
        if self.search_filter.is_empty() {
            self.triggers.iter().enumerate().collect()
        } else {
            // 由于Trigger没有id字段，暂时返回所有触发器
            // TODO: 需要为Trigger添加id字段来支持搜索功能
            self.triggers.iter().enumerate().collect()
        }
    }

    /// 更新选中触发器的基础信息
    pub fn update_selected_trigger_basic(&mut self, 
        _id: String,
        check_period: f64,
        delay_period: f64,
        rearm_period: f64
    ) -> Result<()> {
        if let Some(trigger) = self.get_selected_trigger_mut() {
            // TODO: 需要为Trigger添加id字段来支持重命名功能
            trigger.check_period = check_period;
            trigger.delay_period = delay_period;
            trigger.rearm_period = rearm_period;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的触发器"))
        }
    }

    /// 向选中触发器添加条件
    pub fn add_condition_to_selected(&mut self, opcode: i32, parameters: Vec<Parameter>) -> Result<()> {
        if let Some(template) = self.get_operation_template(opcode) {
            let operation = Operation {
                opcode,
                operation_type: template.operation_type.clone(),
                name: template.name.clone(),
                parameters,
                comment: None,
            };

            if let Some(trigger) = self.get_selected_trigger_mut() {
                trigger.conditions.push(operation);
                Ok(())
            } else {
                Err(anyhow::anyhow!("没有选中的触发器"))
            }
        } else {
            Err(anyhow::anyhow!("未知的操作码: {}", opcode))
        }
    }

    /// 向选中触发器添加结果
    pub fn add_consequence_to_selected(&mut self, opcode: i32, parameters: Vec<Parameter>) -> Result<()> {
        if let Some(template) = self.get_operation_template(opcode) {
            let operation = Operation {
                opcode,
                operation_type: template.operation_type.clone(),
                name: template.name.clone(),
                parameters,
                comment: None,
            };

            if let Some(trigger) = self.get_selected_trigger_mut() {
                trigger.consequences.push(operation);
                Ok(())
            } else {
                Err(anyhow::anyhow!("没有选中的触发器"))
            }
        } else {
            Err(anyhow::anyhow!("未知的操作码: {}", opcode))
        }
    }

    /// 从选中触发器移除条件
    pub fn remove_condition_from_selected(&mut self, index: usize) -> Result<()> {
        if let Some(trigger) = self.get_selected_trigger_mut() {
            if index < trigger.conditions.len() {
                trigger.conditions.remove(index);
                Ok(())
            } else {
                Err(anyhow::anyhow!("条件索引超出范围"))
            }
        } else {
            Err(anyhow::anyhow!("没有选中的触发器"))
        }
    }

    /// 从选中触发器移除结果
    pub fn remove_consequence_from_selected(&mut self, index: usize) -> Result<()> {
        if let Some(trigger) = self.get_selected_trigger_mut() {
            if index < trigger.consequences.len() {
                trigger.consequences.remove(index);
                Ok(())
            } else {
                Err(anyhow::anyhow!("结果索引超出范围"))
            }
        } else {
            Err(anyhow::anyhow!("没有选中的触发器"))
        }
    }

    /// 验证触发器
    pub fn validate_trigger(&self, trigger: &Trigger) -> Vec<String> {
        let mut errors = Vec::new();

        if trigger.conditions.is_empty() && trigger.consequences.is_empty() {
            errors.push("触发器ID不能为空".to_string());
        }

        if trigger.check_period < 0.0 {
            errors.push("检查周期不能为负数".to_string());
        }

        if trigger.delay_period < 0.0 {
            errors.push("延迟周期不能为负数".to_string());
        }

        if trigger.rearm_period < 0.0 {
            errors.push("重新激活周期不能为负数".to_string());
        }

        // 验证条件和结果
        for (i, condition) in trigger.conditions.iter().enumerate() {
            if let Err(e) = condition.validate_parameters() {
                errors.push(format!("条件 {} 验证失败: {}", i, e));
            }
        }

        for (i, consequence) in trigger.consequences.iter().enumerate() {
            if let Err(e) = consequence.validate_parameters() {
                errors.push(format!("结果 {} 验证失败: {}", i, e));
            }
        }

        errors
    }

    /// 验证所有触发器
    pub fn validate_all(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for (index, trigger) in self.triggers.iter().enumerate() {
            let trigger_errors = self.validate_trigger(trigger);
            for error in trigger_errors {
                errors.push(format!("触发器 {}: {}", index, error));
            }
        }
        
        errors
    }

    /// 选择触发器
    pub fn select_trigger(&mut self, index: usize) {
        if index < self.triggers.len() {
            self.selected_index = Some(index);
        }
    }

    /// 创建新触发器
    pub fn create_new_trigger(&mut self) -> usize {
        let new_trigger = Trigger::default();
        self.triggers.push(new_trigger);
        let new_index = self.triggers.len() - 1;
        self.selected_index = Some(new_index);
        new_index
    }

    /// 删除触发器
    pub fn delete_trigger(&mut self, index: usize) -> Result<()> {
        if index < self.triggers.len() {
            self.triggers.remove(index);
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
            Err(anyhow::anyhow!("触发器索引超出范围"))
        }
    }

    /// 获取触发器统计信息
    pub fn get_stats(&self) -> TriggerStats {
        TriggerStats {
            total_triggers: self.triggers.len(),
            total_conditions: self.triggers.iter().map(|t| t.conditions.len()).sum(),
            total_consequences: self.triggers.iter().map(|t| t.consequences.len()).sum(),
            filtered_count: self.get_filtered_triggers().len(),
        }
    }
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 触发器统计信息
#[derive(Debug, Clone)]
pub struct TriggerStats {
    pub total_triggers: usize,
    pub total_conditions: usize,
    pub total_consequences: usize,
    pub filtered_count: usize,
}

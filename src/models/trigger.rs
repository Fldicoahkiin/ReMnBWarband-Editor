use serde::{Deserialize, Serialize};

/// 操作码类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    Condition,
    Consequence,
    Assignment,
    Call,
}

/// 参数类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    Constant,
    Variable,
    String,
    Register,
    Position,
}

/// 操作参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub param_type: ParameterType,
    pub value: i64,
    pub string_value: Option<String>,
}

/// 操作码结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// 操作码ID
    pub opcode: i32,
    /// 操作类型
    pub operation_type: OperationType,
    /// 操作名称
    pub name: String,
    /// 参数列表
    pub parameters: Vec<Parameter>,
    /// 注释
    pub comment: Option<String>,
}

/// 触发器结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// 检查周期
    pub check_period: f64,
    /// 延迟周期
    pub delay_period: f64,
    /// 重新激活周期
    pub rearm_period: f64,
    /// 条件列表
    pub conditions: Vec<Operation>,
    /// 结果列表
    pub consequences: Vec<Operation>,
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            check_period: 0.0,
            delay_period: 0.0,
            rearm_period: 0.0,
            conditions: Vec::new(),
            consequences: Vec::new(),
        }
    }
}

impl Parameter {
    /// 创建常量参数
    pub fn constant(value: i64) -> Self {
        Self {
            param_type: ParameterType::Constant,
            value,
            string_value: None,
        }
    }
    
    /// 创建字符串参数
    pub fn string(value: String) -> Self {
        Self {
            param_type: ParameterType::String,
            value: 0,
            string_value: Some(value),
        }
    }
    
    /// 创建变量参数
    pub fn variable(value: i64) -> Self {
        Self {
            param_type: ParameterType::Variable,
            value,
            string_value: None,
        }
    }
}

impl Operation {
    /// 创建新操作
    pub fn new(opcode: i32, name: String, operation_type: OperationType) -> Self {
        Self {
            opcode,
            operation_type,
            name,
            parameters: Vec::new(),
            comment: None,
        }
    }
    
    /// 添加参数
    pub fn add_parameter(&mut self, param: Parameter) {
        self.parameters.push(param);
    }
    
    /// 设置注释
    pub fn set_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }
    
    /// 验证操作的参数数量
    pub fn validate_parameters(&self) -> Result<(), String> {
        // 这里可以根据操作码验证参数数量和类型
        // 暂时简单验证参数不为空
        if self.parameters.is_empty() && self.requires_parameters() {
            return Err(format!("操作 {} 需要参数", self.name));
        }
        Ok(())
    }
    
    /// 检查操作是否需要参数
    pub fn requires_parameters(&self) -> bool {
        // 大部分操作都需要参数，这里可以根据具体操作码判断
        match self.opcode {
            // try_begin, try_end 等不需要参数
            1 | 2 | 3 => false,
            _ => true,
        }
    }
}

impl Trigger {
    /// 创建新触发器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 添加条件
    pub fn add_condition(&mut self, condition: Operation) {
        self.conditions.push(condition);
    }
    
    /// 添加结果
    pub fn add_consequence(&mut self, consequence: Operation) {
        self.consequences.push(consequence);
    }
    
    /// 验证触发器
    pub fn validate(&self) -> Result<(), String> {
        if self.check_period < 0.0 {
            return Err("检查周期不能为负数".to_string());
        }
        
        if self.delay_period < 0.0 {
            return Err("延迟周期不能为负数".to_string());
        }
        
        if self.rearm_period < 0.0 {
            return Err("重新激活周期不能为负数".to_string());
        }
        
        // 验证所有操作
        for condition in &self.conditions {
            condition.validate_parameters()?;
        }
        
        for consequence in &self.consequences {
            consequence.validate_parameters()?;
        }
        
        Ok(())
    }
    
    /// 获取触发器的文本表示
    pub fn to_text(&self) -> String {
        let mut result = String::new();
        
        result.push_str(&format!("触发器 {} {} {}\n", 
            self.check_period, self.delay_period, self.rearm_period));
        
        for condition in &self.conditions {
            result.push_str(&format!("  条件: {} ", condition.name));
            for param in &condition.parameters {
                match &param.string_value {
                    Some(s) => result.push_str(&format!("\"{}\" ", s)),
                    None => result.push_str(&format!("{} ", param.value)),
                }
            }
            result.push('\n');
        }
        
        for consequence in &self.consequences {
            result.push_str(&format!("  结果: {} ", consequence.name));
            for param in &consequence.parameters {
                match &param.string_value {
                    Some(s) => result.push_str(&format!("\"{}\" ", s)),
                    None => result.push_str(&format!("{} ", param.value)),
                }
            }
            result.push('\n');
        }
        
        result
    }
}

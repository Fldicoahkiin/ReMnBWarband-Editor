use super::{DataParser, ParseError, ParseUtils};
use crate::models::{Trigger, Operation, Parameter, ParameterType, OperationType};
use anyhow::Result;

pub struct TriggerParser;

impl TriggerParser {
    pub fn new() -> Self {
        Self
    }
    
    /// 解析操作类型
    fn parse_operation_type(&self, opcode: i32) -> OperationType {
        match opcode {
            1..=100 => OperationType::Condition,
            101..=500 => OperationType::Consequence,
            501..=600 => OperationType::Assignment,
            _ => OperationType::Call,
        }
    }
    
    /// 解析参数类型
    fn parse_parameter(&self, param_str: &str) -> Result<Parameter, ParseError> {
        let param_str = param_str.trim();
        
        if param_str.starts_with('"') && param_str.ends_with('"') {
            // 字符串参数
            Ok(Parameter {
                param_type: ParameterType::String,
                value: 0,
                string_value: Some(ParseUtils::parse_string(param_str)),
            })
        } else if param_str.starts_with(':') {
            // 变量参数
            let var_name = &param_str[1..];
            Ok(Parameter {
                param_type: ParameterType::Variable,
                value: ParseUtils::parse_int(var_name)?,
                string_value: None,
            })
        } else if param_str.starts_with('$') {
            // 寄存器参数
            let reg_name = &param_str[1..];
            Ok(Parameter {
                param_type: ParameterType::Register,
                value: ParseUtils::parse_int(reg_name)?,
                string_value: None,
            })
        } else if param_str.starts_with("pos") {
            // 位置参数
            Ok(Parameter {
                param_type: ParameterType::Position,
                value: ParseUtils::parse_int(&param_str[3..])?,
                string_value: None,
            })
        } else {
            // 常量参数
            Ok(Parameter {
                param_type: ParameterType::Constant,
                value: ParseUtils::parse_int(param_str)?,
                string_value: None,
            })
        }
    }
}

impl DataParser<Trigger> for TriggerParser {
    fn parse_text(&self, data: &str) -> Result<Vec<Trigger>, ParseError> {
        let mut triggers = Vec::new();
        let lines = ParseUtils::split_lines(data);
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            
            // 查找触发器开始标记
            if !line.starts_with("trigger") {
                i += 1;
                continue;
            }
            
            let parts = ParseUtils::split_params(line);
            if parts.len() < 4 {
                i += 1;
                continue;
            }
            
            // 解析触发器参数
            let mut trigger = Trigger {
                check_period: ParseUtils::parse_float(parts[1])?,
                delay_period: ParseUtils::parse_float(parts[2])?,
                rearm_period: ParseUtils::parse_float(parts[3])?,
                conditions: Vec::new(),
                consequences: Vec::new(),
            };
            
            i += 1;
            let mut in_conditions = true;
            
            // 解析触发器内容
            while i < lines.len() {
                let line = lines[i];
                
                // 检查是否到达下一个触发器或文件结束
                if line.starts_with("trigger") {
                    break;
                }
                
                // 检查是否从条件切换到结果
                if line.contains("try_begin") {
                    in_conditions = false;
                    i += 1;
                    continue;
                }
                
                let parts = ParseUtils::split_params(line);
                if parts.is_empty() {
                    i += 1;
                    continue;
                }
                
                // 解析操作
                if let Ok(opcode) = ParseUtils::parse_int(parts[0]) {
                    let operation_type = self.parse_operation_type(opcode as i32);
                    let mut operation = Operation::new(
                        opcode as i32,
                        parts[0].to_string(),
                        operation_type.clone(),
                    );
                    
                    // 解析参数
                    for param_str in &parts[1..] {
                        if let Ok(param) = self.parse_parameter(param_str) {
                            operation.add_parameter(param);
                        }
                    }
                    
                    // 添加到相应的列表
                    if in_conditions {
                        trigger.add_condition(operation);
                    } else {
                        trigger.add_consequence(operation);
                    }
                }
                
                i += 1;
            }
            
            // 验证并添加触发器
            if let Err(e) = trigger.validate() {
                tracing::warn!("触发器验证失败: {}", e);
            } else {
                triggers.push(trigger);
            }
        }
        
        Ok(triggers)
    }
    
    fn serialize_text(&self, triggers: &[Trigger]) -> Result<String, ParseError> {
        let mut result = String::new();
        
        for trigger in triggers {
            result.push_str(&format!("trigger {} {} {}\n",
                trigger.check_period,
                trigger.delay_period,
                trigger.rearm_period
            ));
            
            // 序列化条件
            for condition in &trigger.conditions {
                result.push_str(&format!("{}", condition.opcode));
                for param in &condition.parameters {
                    match &param.string_value {
                        Some(s) => result.push_str(&format!(" \"{}\"", s)),
                        None => {
                            match param.param_type {
                                ParameterType::Variable => result.push_str(&format!(" :{}", param.value)),
                                ParameterType::Register => result.push_str(&format!(" ${}", param.value)),
                                ParameterType::Position => result.push_str(&format!(" pos{}", param.value)),
                                _ => result.push_str(&format!(" {}", param.value)),
                            }
                        }
                    }
                }
                result.push('\n');
            }
            
            result.push_str("try_begin\n");
            
            // 序列化结果
            for consequence in &trigger.consequences {
                result.push_str(&format!("{}", consequence.opcode));
                for param in &consequence.parameters {
                    match &param.string_value {
                        Some(s) => result.push_str(&format!(" \"{}\"", s)),
                        None => {
                            match param.param_type {
                                ParameterType::Variable => result.push_str(&format!(" :{}", param.value)),
                                ParameterType::Register => result.push_str(&format!(" ${}", param.value)),
                                ParameterType::Position => result.push_str(&format!(" pos{}", param.value)),
                                _ => result.push_str(&format!(" {}", param.value)),
                            }
                        }
                    }
                }
                result.push('\n');
            }
            
            result.push_str("try_end\n\n");
        }
        
        Ok(result)
    }
    
    fn validate(&self, trigger: &Trigger) -> Result<(), ParseError> {
        trigger.validate()
            .map_err(|e| ParseError::ValidationError(e))
    }
}

pub mod item_parser;
pub mod troop_parser;
pub mod faction_parser;
pub mod trigger_parser;
pub mod module_parser;

pub use item_parser::ItemParser;
pub use troop_parser::TroopParser;
pub use faction_parser::FactionParser;
// pub use trigger_parser::TriggerParser;
pub use module_parser::ModuleParser;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("解析错误: {0}")]
    ParseError(String),
    #[error("格式错误: {0}")]
    FormatError(String),
    #[error("数据验证错误: {0}")]
    ValidationError(String),
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

/// 通用解析器特征
pub trait DataParser<T> {
    fn parse_text(&self, data: &str) -> Result<Vec<T>, ParseError>;
    fn serialize_text(&self, items: &[T]) -> Result<String, ParseError>;
    fn validate(&self, item: &T) -> Result<(), ParseError>;
}

/// 解析工具函数
pub struct ParseUtils;

impl ParseUtils {
    /// 解析整数，支持十六进制
    pub fn parse_int(s: &str) -> Result<i64, ParseError> {
        let s = s.trim();
        if s.starts_with("0x") || s.starts_with("0X") {
            i64::from_str_radix(&s[2..], 16)
                .map_err(|e| ParseError::ParseError(format!("无法解析十六进制数字 '{}': {}", s, e)))
        } else {
            s.parse::<i64>()
                .map_err(|e| ParseError::ParseError(format!("无法解析数字 '{}': {}", s, e)))
        }
    }
    
    /// 解析浮点数
    pub fn parse_float(s: &str) -> Result<f64, ParseError> {
        s.trim().parse::<f64>()
            .map_err(|e| ParseError::ParseError(format!("无法解析浮点数 '{}': {}", s, e)))
    }
    
    /// 解析字符串，去除引号
    pub fn parse_string(s: &str) -> String {
        let s = s.trim();
        if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
            s[1..s.len()-1].to_string()
        } else {
            s.to_string()
        }
    }
    
    /// 分割行并过滤空行和注释
    pub fn split_lines(data: &str) -> Vec<&str> {
        data.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect()
    }
    
    /// 分割参数
    pub fn split_params(line: &str) -> Vec<&str> {
        line.split_whitespace().collect()
    }
}

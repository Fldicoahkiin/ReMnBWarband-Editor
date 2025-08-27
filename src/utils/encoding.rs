use anyhow::{Result, anyhow};
use encoding_rs::{Encoding, UTF_8, GBK, WINDOWS_1252};
// use std::borrow::Cow;

/// 编码检测和转换工具
pub struct EncodingUtils;

impl EncodingUtils {
    /// 检测文本编码
    pub fn detect_encoding(bytes: &[u8]) -> &'static Encoding {
        // 检查BOM
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return UTF_8;
        }
        
        // 尝试UTF-8解码
        if std::str::from_utf8(bytes).is_ok() {
            return UTF_8;
        }
        
        // 检查是否包含中文字符（简单启发式）
        if Self::contains_chinese_chars(bytes) {
            return GBK;
        }
        
        // 默认使用Windows-1252
        WINDOWS_1252
    }
    
    /// 检查是否包含中文字符
    fn contains_chinese_chars(bytes: &[u8]) -> bool {
        // 简单检测：查找GBK编码的中文字符范围
        let mut i = 0;
        while i < bytes.len() {
            if i + 1 < bytes.len() {
                let b1 = bytes[i];
                let b2 = bytes[i + 1];
                
                // GBK中文字符范围
                if (b1 >= 0xA1 && b1 <= 0xFE) && (b2 >= 0xA1 && b2 <= 0xFE) {
                    return true;
                }
                
                // 跳过双字节字符
                if b1 >= 0x80 {
                    i += 2;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        false
    }
    
    /// 将字节转换为UTF-8字符串
    pub fn decode_to_string(bytes: &[u8]) -> Result<String> {
        let encoding = Self::detect_encoding(bytes);
        let (cow, _encoding_used, had_errors) = encoding.decode(bytes);
        
        if had_errors {
            return Err(anyhow!("解码过程中发现错误"));
        }
        
        Ok(cow.into_owned())
    }
    
    /// 将UTF-8字符串编码为指定编码的字节
    pub fn encode_string(text: &str, encoding: &'static Encoding) -> Result<Vec<u8>> {
        let (cow, _encoding_used, had_errors) = encoding.encode(text);
        
        if had_errors {
            return Err(anyhow!("编码过程中发现错误"));
        }
        
        Ok(cow.into_owned())
    }
    
    /// 转换文本编码
    pub fn convert_encoding(
        bytes: &[u8],
        from_encoding: &'static Encoding,
        to_encoding: &'static Encoding,
    ) -> Result<Vec<u8>> {
        // 先解码为UTF-8字符串
        let (cow, _encoding_used, had_errors) = from_encoding.decode(bytes);
        if had_errors {
            return Err(anyhow!("源编码解码失败"));
        }
        
        // 再编码为目标编码
        let (result, _encoding_used, had_errors) = to_encoding.encode(&cow);
        if had_errors {
            return Err(anyhow!("目标编码编码失败"));
        }
        
        Ok(result.into_owned())
    }
    
    /// 规范化文本（移除BOM，统一换行符）
    pub fn normalize_text(text: &str) -> String {
        text
            .trim_start_matches('\u{FEFF}') // 移除BOM
            .replace("\r\n", "\n")          // 统一换行符
            .replace('\r', "\n")
    }
    
    /// 为文件添加UTF-8 BOM
    pub fn add_utf8_bom(bytes: &[u8]) -> Vec<u8> {
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            bytes.to_vec()
        } else {
            let mut result = vec![0xEF, 0xBB, 0xBF];
            result.extend_from_slice(bytes);
            result
        }
    }
    
    /// 移除UTF-8 BOM
    pub fn remove_utf8_bom(bytes: &[u8]) -> &[u8] {
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            &bytes[3..]
        } else {
            bytes
        }
    }
}

/// 检测文件编码并解码为字符串
pub fn detect_and_decode<P: AsRef<std::path::Path>>(file_path: P) -> Result<String> {
    let bytes = std::fs::read(file_path)?;
    EncodingUtils::decode_to_string(&bytes)
}

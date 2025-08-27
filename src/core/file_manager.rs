use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// 文件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Items,
    Troops,
    Factions,
    Scenes,
    Parties,
    Triggers,
    Other(String),
}

/// 文件信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub encoding: String,
}

/// 备份信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub description: String,
}

/// 文件管理器
pub struct FileManager {
    project_root: Option<PathBuf>,
    backups: Vec<BackupInfo>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            project_root: None,
            backups: Vec::new(),
        }
    }
    
    /// 设置项目根目录
    pub fn set_project_root<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(anyhow!("项目路径不存在: {:?}", path));
        }
        if !path.is_dir() {
            return Err(anyhow!("项目路径不是目录: {:?}", path));
        }
        self.project_root = Some(path);
        Ok(())
    }
    
    /// 获取项目根目录
    pub fn get_project_root(&self) -> Option<&PathBuf> {
        self.project_root.as_ref()
    }
    
    /// 扫描项目文件
    pub fn scan_project_files(&self) -> Result<Vec<FileInfo>> {
        let root = self.project_root.as_ref()
            .ok_or_else(|| anyhow!("未设置项目根目录"))?;
        
        let mut files = Vec::new();
        self.scan_directory_recursive(root, &mut files)?;
        Ok(files)
    }
    
    /// 递归扫描目录
    fn scan_directory_recursive(&self, dir: &Path, files: &mut Vec<FileInfo>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.scan_directory_recursive(&path, files)?;
            } else if path.is_file() {
                if let Some(file_info) = self.analyze_file(&path)? {
                    files.push(file_info);
                }
            }
        }
        Ok(())
    }
    
    /// 分析文件类型和信息
    fn analyze_file(&self, path: &Path) -> Result<Option<FileInfo>> {
        let metadata = fs::metadata(path)?;
        let modified = DateTime::from(metadata.modified()?);
        
        let file_type = self.detect_file_type(path)?;
        
        // 只处理相关的游戏文件
        match file_type {
            FileType::Other(_) => {
                // 跳过不相关的文件
                if !self.is_game_related_file(path) {
                    return Ok(None);
                }
            },
            _ => {}
        }
        
        let encoding = self.detect_encoding(path)?;
        
        Ok(Some(FileInfo {
            path: path.to_path_buf(),
            file_type,
            size: metadata.len(),
            modified,
            encoding,
        }))
    }
    
    /// 检测文件类型
    fn detect_file_type(&self, path: &Path) -> Result<FileType> {
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let file_type = match filename.to_lowercase().as_str() {
            "item_kinds1.txt" | "items.txt" => FileType::Items,
            "troops.txt" => FileType::Troops,
            "factions.txt" => FileType::Factions,
            "scenes.txt" => FileType::Scenes,
            "parties.txt" => FileType::Parties,
            name if name.contains("trigger") => FileType::Triggers,
            _ => FileType::Other(filename.to_string()),
        };
        
        Ok(file_type)
    }
    
    /// 检查是否为游戏相关文件
    fn is_game_related_file(&self, path: &Path) -> bool {
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        // 检查文件扩展名
        let has_game_extension = match extension.to_lowercase().as_str() {
            "txt" | "py" | "brf" | "smd" | "dds" | "wav" | "ogg" => true,
            _ => false,
        };
        
        // 检查特定文件名模式
        let has_game_pattern = filename.contains("module_") ||
            filename.contains("header_") ||
            filename.contains("constants");
            
        has_game_extension || has_game_pattern
    }
    
    /// 检测文件编码
    fn detect_encoding(&self, path: &Path) -> Result<String> {
        let bytes = fs::read(path)?;
        
        // 简单的编码检测
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            Ok("UTF-8 BOM".to_string())
        } else if bytes.iter().all(|&b| b < 128) {
            Ok("ASCII".to_string())
        } else {
            // 尝试检测是否为有效的UTF-8
            match std::str::from_utf8(&bytes) {
                Ok(_) => Ok("UTF-8".to_string()),
                Err(_) => Ok("Unknown".to_string()),
            }
        }
    }
    
    /// 创建文件备份
    pub fn create_backup<P: AsRef<Path>>(&mut self, file_path: P, description: String) -> Result<PathBuf> {
        let file_path = file_path.as_ref();
        if !file_path.exists() {
            return Err(anyhow!("文件不存在: {:?}", file_path));
        }
        
        let backup_dir = self.get_backup_directory()?;
        fs::create_dir_all(&backup_dir)?;
        
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = file_path.file_name()
            .ok_or_else(|| anyhow!("无法获取文件名"))?;
        let backup_filename = format!("{}_{}.bak", 
            filename.to_string_lossy(), timestamp);
        let backup_path = backup_dir.join(backup_filename);
        
        fs::copy(file_path, &backup_path)?;
        
        let backup_info = BackupInfo {
            original_path: file_path.to_path_buf(),
            backup_path: backup_path.clone(),
            created_at: Utc::now(),
            description,
        };
        
        self.backups.push(backup_info);
        
        tracing::info!("创建备份: {:?} -> {:?}", file_path, backup_path);
        Ok(backup_path)
    }
    
    /// 获取备份目录
    fn get_backup_directory(&self) -> Result<PathBuf> {
        let root = self.project_root.as_ref()
            .ok_or_else(|| anyhow!("未设置项目根目录"))?;
        Ok(root.join(".r-ball-backups"))
    }
    
    /// 恢复文件从备份
    pub fn restore_from_backup(&self, backup_path: &Path) -> Result<()> {
        let backup_info = self.backups.iter()
            .find(|b| b.backup_path == backup_path)
            .ok_or_else(|| anyhow!("找不到备份信息"))?;
        
        if !backup_path.exists() {
            return Err(anyhow!("备份文件不存在: {:?}", backup_path));
        }
        
        fs::copy(backup_path, &backup_info.original_path)?;
        
        tracing::info!("从备份恢复文件: {:?} -> {:?}", 
            backup_path, backup_info.original_path);
        Ok(())
    }
    
    /// 获取所有备份信息
    pub fn get_backups(&self) -> &[BackupInfo] {
        &self.backups
    }
    
    /// 清理旧备份
    pub fn cleanup_old_backups(&mut self, keep_days: u64) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(keep_days as i64);
        let mut removed_count = 0;
        
        self.backups.retain(|backup| {
            if backup.created_at < cutoff_date {
                if let Err(e) = fs::remove_file(&backup.backup_path) {
                    tracing::warn!("删除旧备份失败: {:?}, 错误: {}", backup.backup_path, e);
                } else {
                    removed_count += 1;
                    tracing::info!("删除旧备份: {:?}", backup.backup_path);
                }
                false
            } else {
                true
            }
        });
        
        Ok(removed_count)
    }
    
    /// 验证文件完整性
    pub fn validate_file(&self, file_info: &FileInfo) -> Result<bool> {
        if !file_info.path.exists() {
            return Ok(false);
        }
        
        let metadata = fs::metadata(&file_info.path)?;
        let current_size = metadata.len();
        let current_modified = DateTime::from(metadata.modified()?);
        
        Ok(current_size == file_info.size && 
           (current_modified - file_info.modified).num_seconds().abs() < 2)
    }
    
    /// 获取文件的相对路径
    pub fn get_relative_path(&self, file_path: &Path) -> Result<PathBuf> {
        let root = self.project_root.as_ref()
            .ok_or_else(|| anyhow!("未设置项目根目录"))?;
        
        file_path.strip_prefix(root)
            .map(|p| p.to_path_buf())
            .map_err(|e| anyhow!("无法获取相对路径: {}", e))
    }
}

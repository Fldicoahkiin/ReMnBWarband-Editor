use anyhow::{Result, anyhow};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use std::collections::HashMap;
use tracing::{info, error};

use crate::models::{Item, Troop, Faction};

/// Python脚本管理器
/// 负责管理Python解释器，执行脚本，提供数据接口
#[derive(Debug)]
pub struct PythonManager {
    /// Python解释器是否已初始化
    initialized: bool,
    /// 已加载的Python模块
    loaded_modules: HashMap<String, Py<PyModule>>,
    /// 脚本执行历史
    execution_history: Vec<PythonExecution>,
}

/// Python脚本执行记录
#[derive(Debug, Clone)]
pub struct PythonExecution {
    pub script_name: String,
    pub code: String,
    pub result: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Python脚本接口
/// 提供给Python脚本访问编辑器数据的接口
#[pyclass]
pub struct EditorInterface {
    pub items: Vec<Item>,
    pub troops: Vec<Troop>,
    pub factions: Vec<Faction>,
}

#[pymethods]
impl EditorInterface {
    #[new]
    fn new() -> Self {
        Self {
            items: Vec::new(),
            troops: Vec::new(),
            factions: Vec::new(),
        }
    }

    /// 获取所有物品
    fn get_items(&self) -> Vec<String> {
        self.items.iter().map(|item| format!("{}: {}", item.id, item.name)).collect()
    }

    /// 获取所有兵种
    fn get_troops(&self) -> Vec<String> {
        self.troops.iter().map(|troop| format!("{}: {}", troop.id, troop.name)).collect()
    }

    /// 获取所有派系
    fn get_factions(&self) -> Vec<String> {
        self.factions.iter().map(|faction| format!("{}: {}", faction.id, faction.name)).collect()
    }

    /// 根据ID查找物品
    fn find_item(&self, id: &str) -> Option<String> {
        self.items.iter()
            .find(|item| item.id == id)
            .map(|item| serde_json::to_string(item).unwrap_or_default())
    }

    /// 根据ID查找兵种
    fn find_troop(&self, id: &str) -> Option<String> {
        self.troops.iter()
            .find(|troop| troop.id == id)
            .map(|troop| serde_json::to_string(troop).unwrap_or_default())
    }

    /// 根据ID查找派系
    fn find_faction(&self, id: &str) -> Option<String> {
        self.factions.iter()
            .find(|faction| faction.id == id)
            .map(|faction| serde_json::to_string(faction).unwrap_or_default())
    }

    /// 获取物品模板
    pub fn get_item_template(&self, name: &str, _item_type: i32) -> String {
        let new_item = Item::new(
            format!("item_{}", self.items.len()),
            name.to_string(),
        );
        serde_json::to_string(&new_item).unwrap_or_default()
    }

    /// 日志输出
    fn log(&self, message: &str) {
        info!("Python脚本: {}", message);
    }
}

impl PythonManager {
    /// 创建新的Python管理器
    pub fn new() -> Self {
        Self {
            initialized: false,
            loaded_modules: HashMap::new(),
            execution_history: Vec::new(),
        }
    }

    /// 初始化Python解释器
    pub fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        Python::with_gil(|py| -> Result<()> {
            // 注册编辑器接口类
            let editor_module = PyModule::new(py, "editor_interface")?;
            editor_module.add_class::<EditorInterface>()?;
            
            // 添加到sys.modules
            let sys = py.import("sys")?;
            let modules = sys.getattr("modules")?;
            let modules_dict: &PyDict = modules.downcast().map_err(|e| anyhow::anyhow!("Failed to downcast: {}", e))?;
            modules_dict.set_item("editor_interface", editor_module)?;

            info!("Python解释器初始化成功");
            Ok(())
        })?;

        self.initialized = true;
        Ok(())
    }

    /// 执行Python代码
    pub fn execute_code(&mut self, code: &str, script_name: Option<&str>) -> Result<String> {
        if !self.initialized {
            self.initialize()?;
        }

        let script_name = script_name.unwrap_or("inline");
        let result = Python::with_gil(|py| {
            // 创建执行环境
            let globals = PyDict::new(py);
            let locals = PyDict::new(py);

            // 添加编辑器接口
            let editor_interface = Py::new(py, EditorInterface::new())?;
            locals.set_item("editor", editor_interface)?;

            // 执行代码
            match py.run(code, Some(globals), Some(locals)) {
                Ok(_) => {
                    // 获取返回值（如果有）
                    if let Ok(Some(result)) = locals.get_item("__result__") {
                        Ok(result.to_string())
                    } else {
                        Ok("执行成功".to_string())
                    }
                }
                Err(e) => {
                    let error_msg = format!("Python执行错误: {}", e);
                    error!("{}", error_msg);
                    Err(anyhow!(error_msg))
                }
            }
        });

        // 记录执行历史
        let execution = PythonExecution {
            script_name: script_name.to_string(),
            code: code.to_string(),
            result: result.as_ref().map(|s| s.clone()).unwrap_or_else(|e| e.to_string()),
            success: result.is_ok(),
            timestamp: chrono::Utc::now(),
        };
        self.execution_history.push(execution);

        result
    }

    /// 从文件执行Python脚本
    pub fn execute_script_file(&mut self, file_path: &std::path::Path) -> Result<String> {
        let path: &std::path::Path = file_path.as_ref();
        let code = std::fs::read_to_string(path)?;
        let script_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        info!("执行Python脚本文件: {}", path.display());
        self.execute_code(&code, Some(script_name))
    }

    /// 加载Python模块
    pub fn load_module(&mut self, module_name: &str, file_path: &str) -> Result<()> {
        if !self.initialized {
            self.initialize()?;
        }

        Python::with_gil(|py| {
            let code = std::fs::read_to_string(file_path)?;
            let module = PyModule::from_code(py, &code, file_path, module_name)?;
            self.loaded_modules.insert(module_name.to_string(), module.into());
            info!("加载Python模块: {} 从 {}", module_name, file_path);
            Ok(())
        })
    }

    /// 调用已加载模块的函数
    pub fn call_module_function(&self, module_name: &str, function_name: &str, args: Vec<String>) -> Result<String> {
        if !self.initialized {
            return Err(anyhow!("Python解释器未初始化"));
        }

        let module = self.loaded_modules.get(module_name)
            .ok_or_else(|| anyhow!("模块 {} 未加载", module_name))?;

        Python::with_gil(|py| {
            let module = module.as_ref(py);
            let function = module.getattr(function_name)?;
            
            // 转换参数
            let py_args: Vec<&PyString> = args.iter()
                .map(|arg| PyString::new(py, arg))
                .collect();
            
            let result = function.call1((py_args,))?;
            Ok(result.to_string())
        })
    }

    /// 获取执行历史
    pub fn get_execution_history(&self) -> &[PythonExecution] {
        &self.execution_history
    }

    /// 清空执行历史
    pub fn clear_history(&mut self) {
        self.execution_history.clear();
    }

    /// 获取已加载的模块列表
    pub fn get_loaded_modules(&self) -> Vec<String> {
        self.loaded_modules.keys().cloned().collect()
    }

    /// 卸载模块
    pub fn unload_module(&mut self, module_name: &str) -> Result<()> {
        if self.loaded_modules.remove(module_name).is_some() {
            info!("卸载Python模块: {}", module_name);
            Ok(())
        } else {
            Err(anyhow!("模块 {} 未加载", module_name))
        }
    }

    /// 更新编辑器接口数据
    pub fn update_editor_data(&mut self, items: Vec<Item>, troops: Vec<Troop>, factions: Vec<Faction>) -> Result<()> {
        if !self.initialized {
            return Ok(());
        }

        Python::with_gil(|py| {
            // 更新全局编辑器接口
            let mut editor_interface = EditorInterface::new();
            editor_interface.items = items;
            editor_interface.troops = troops;
            editor_interface.factions = factions;

            // 将更新后的接口设置到Python环境中
            let globals = py.import("__main__")?.dict();
            let editor_py = Py::new(py, editor_interface)?;
            globals.set_item("editor", editor_py)?;
            
            Ok(())
        })
    }

    /// 验证Python代码语法
    pub fn validate_syntax(&self, code: &str) -> Result<()> {
        Python::with_gil(|py| -> Result<()> {
            use pyo3::types::PyModule;
            PyModule::from_code(py, code, "syntax_check.py", "syntax_check")?;
            Ok(())
        })
    }

    /// 获取Python版本信息
    pub fn get_python_info(&self) -> Result<String> {
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let version: String = sys.getattr("version")?.extract()?;
            Ok(std::format!("Python {}", version))
        })
    }

    /// 检查Python是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for PythonManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Python脚本模板
pub struct PythonScriptTemplates;

impl PythonScriptTemplates {
    /// 获取物品批量处理模板
    pub fn item_batch_template() -> &'static str {
        r#"
# 物品批量处理脚本模板
# 可以批量修改物品属性

def process_items():
    """批量处理物品"""
    items = editor.get_items()
    editor.log(f"找到 {len(items)} 个物品")
    
    # 示例：批量修改物品价格
    for item_info in items:
        editor.log(f"处理物品: {item_info}")
    
    editor.log("物品批量处理完成")

# 执行处理
process_items()
__result__ = "物品批量处理完成"
"#
    }

    /// 获取兵种分析模板
    pub fn troop_analysis_template() -> &'static str {
        r#"
# 兵种数据分析脚本模板
# 分析兵种数据并生成报告

def analyze_troops():
    """分析兵种数据"""
    troops = editor.get_troops()
    editor.log(f"分析 {len(troops)} 个兵种")
    
    # 示例分析逻辑
    analysis_result = {
        'total_troops': len(troops),
        'analysis_time': str(datetime.now())
    }
    
    editor.log("兵种分析完成")
    return analysis_result

import datetime
result = analyze_troops()
__result__ = f"分析结果: {result}"
"#
    }

    /// 获取数据导出模板
    pub fn data_export_template() -> &'static str {
        r#"
# 数据导出脚本模板
# 将编辑器数据导出为各种格式

def export_data():
    """导出数据"""
    items = editor.get_items()
    troops = editor.get_troops()
    factions = editor.get_factions()
    
    export_data = {
        'items': items,
        'troops': troops,
        'factions': factions,
        'export_time': str(datetime.now())
    }
    
    editor.log("数据导出完成")
    return export_data

import datetime
result = export_data()
__result__ = f"导出了 {len(result['items'])} 个物品, {len(result['troops'])} 个兵种, {len(result['factions'])} 个派系"
"#
    }
}

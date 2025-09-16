// ReMnBWarband Editor 核心库

pub mod data;
pub mod editor;
pub mod ui;
pub mod viewmodel;

use anyhow::Result;
use std::sync::Arc;
use viewmodel::{AppViewModel, BaseViewModel};

// 应用程序主结构
pub struct App {
    #[allow(dead_code)]
    editor: Arc<editor::Editor>,
    app_viewmodel: Arc<AppViewModel>,
}

impl App {
    // 创建新的应用程序实例
    pub fn new() -> Result<Self> {
        let editor = Arc::new(editor::Editor::new()?);
        let app_viewmodel = Arc::new(AppViewModel::new(Arc::clone(&editor))?);
        
        Ok(Self {
            editor,
            app_viewmodel,
        })
    }
    
    // 启动UI界面
    pub fn run(&self) -> Result<()> {
        // 初始化ViewModel
        self.app_viewmodel.initialize()?;
        
        // 启动UI并传入ViewModel
        ui::start_ui(Arc::clone(&self.app_viewmodel))
    }
}

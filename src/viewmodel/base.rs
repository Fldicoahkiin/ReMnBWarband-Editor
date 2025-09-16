// ViewModel基础架构

use anyhow::Result;
use super::observable::{Observable, Command};

// 异步命令
pub type AsyncCommand = Command;

// ViewModel基础trait
pub trait BaseViewModel: Send + Sync {
    // 初始化ViewModel
    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    // 清理资源
    fn cleanup(&self) -> Result<()> {
        Ok(())
    }

    // 验证数据
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    // 获取是否有未保存的更改
    fn has_unsaved_changes(&self) -> bool {
        false
    }
}

// 可编辑ViewModel的trait
pub trait EditableViewModel: BaseViewModel {
    // 保存更改
    fn save(&self) -> Result<()>;

    // 取消更改
    fn cancel(&self) -> Result<()>;

    // 重置到初始状态
    fn reset(&self) -> Result<()>;
}

// 可加载ViewModel的trait
pub trait LoadableViewModel: BaseViewModel {
    // 加载数据
    fn load(&self) -> Result<()>;

    // 刷新数据
    fn refresh(&self) -> Result<()> {
        self.load()
    }

    // 获取是否正在加载
    fn is_loading(&self) -> bool;
}

// 可搜索ViewModel的trait
pub trait SearchableViewModel: BaseViewModel {
    // 搜索
    fn search(&self, query: &str) -> Result<()>;

    // 清除搜索
    fn clear_search(&self) -> Result<()>;

    // 获取搜索查询
    fn get_search_query(&self) -> String;
}

// 可选择ViewModel的trait
pub trait SelectableViewModel<T>: BaseViewModel {
    // 选择项目
    fn select_item(&self, item: &T) -> Result<()>;

    // 取消选择
    fn clear_selection(&self) -> Result<()>;

    // 获取选中的项目
    fn get_selected_item(&self) -> Option<T>;

    // 获取是否有选中项目
    fn has_selection(&self) -> bool {
        self.get_selected_item().is_some()
    }
}

// 通用的ViewModel状态
#[derive(Debug, Clone)]
pub struct ViewModelState {
    pub is_loading: bool,
    pub is_dirty: bool,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
}

impl Default for ViewModelState {
    fn default() -> Self {
        Self {
            is_loading: false,
            is_dirty: false,
            error_message: None,
            status_message: None,
        }
    }
}

// 通用ViewModel基础实现
#[derive(Clone)]
pub struct BaseViewModelImpl {
    pub state: Observable<ViewModelState>,
}

impl BaseViewModelImpl {
    pub fn new() -> Self {
        Self {
            state: Observable::new(ViewModelState::default()),
        }
    }

    // 设置加载状态
    pub fn set_loading(&self, loading: bool) {
        self.state.update(|state| {
            state.is_loading = loading;
        });
    }

    // 设置错误消息
    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|state| {
            state.error_message = error;
        });
    }

    // 设置状态消息
    pub fn set_status(&self, message: Option<String>) {
        self.state.update(|state| {
            state.status_message = message;
        });
    }

    // 设置脏状态
    pub fn set_dirty(&self, dirty: bool) {
        self.state.update(|state| {
            state.is_dirty = dirty;
        });
    }

    // 清除所有消息
    pub fn clear_messages(&self) {
        self.state.update(|state| {
            state.error_message = None;
            state.status_message = None;
        });
    }
}

impl BaseViewModel for BaseViewModelImpl {
    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        Ok(())
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    fn has_unsaved_changes(&self) -> bool {
        self.state.get().is_dirty
    }
}


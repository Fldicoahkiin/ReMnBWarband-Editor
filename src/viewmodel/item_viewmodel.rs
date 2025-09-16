// 物品编辑器ViewModel

use std::sync::Arc;
use anyhow::Result;
use crate::editor::Editor;
use crate::data::Item;
use super::{BaseViewModel, BaseViewModelImpl, AsyncCommand, EditableViewModel, LoadableViewModel, SearchableViewModel, SelectableViewModel, observable::{Observable, Command}};

// 物品编辑器ViewModel
pub struct ItemViewModel {
    base: BaseViewModelImpl,
    #[allow(dead_code)]
    editor: Arc<Editor>,
    
    // 数据
    pub items: Observable<Vec<Item>>,
    pub selected_item: Observable<Option<Item>>,
    pub filtered_items: Observable<Vec<Item>>,
    
    // 搜索和过滤
    pub search_query: Observable<String>,
    pub item_type_filter: Observable<Option<String>>,
    
    // 编辑状态
    pub is_editing: Observable<bool>,
    pub edit_item: Observable<Option<Item>>,
    
    // 命令
    pub load_items_command: AsyncCommand,
    pub save_item_command: Command,
    pub add_item_command: Command,
    pub delete_item_command: Command,
    pub search_command: Command,
}

impl ItemViewModel {
    pub fn new(editor: Arc<Editor>) -> Result<Self> {
        let base = BaseViewModelImpl::new();
        let items = Observable::new(Vec::new());
        let selected_item: Observable<Option<Item>> = Observable::new(None);
        let filtered_items = Observable::new(Vec::new());
        let search_query = Observable::new(String::new());
        let item_type_filter = Observable::new(None);
        let is_editing = Observable::new(false);
        let edit_item: Observable<Option<Item>> = Observable::new(None);

        // 加载物品命令
        let editor_clone = Arc::clone(&editor);
        let items_clone = items.clone();
        let filtered_items_clone = filtered_items.clone();
        let base_clone = base.clone();
        let load_items_command = AsyncCommand::new(
            move || {
                let loaded_items = editor_clone.get_items();
                let items_len = loaded_items.len();
                items_clone.set(loaded_items.clone());
                filtered_items_clone.set(loaded_items);
                base_clone.set_status(Some(format!("已加载 {} 个物品", items_len)));
                Ok(())
            },
            || true
        );

        // 保存物品命令
        let _editor_clone = Arc::clone(&editor);
        let edit_item_clone = edit_item.clone();
        let is_editing_clone = is_editing.clone();
        let base_clone = base.clone();
        
        let save_item_command = AsyncCommand::new(
            move || -> Result<()> {
                if let Some(_item) = edit_item_clone.get() {
                    base_clone.set_loading(true);
                    
                    // 这里应该调用editor的保存方法
                    // 暂时模拟保存成功
                    
                    is_editing_clone.set(false);
                    edit_item_clone.set(None);
                    base_clone.set_dirty(false);
                    base_clone.set_loading(false);
                    base_clone.set_status(Some("物品保存成功".to_string()));
                    
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有要保存的物品"))
                }
            },
            {
                let edit_item_clone = edit_item.clone();
                let is_editing_clone = is_editing.clone();
                move || edit_item_clone.get().is_some() && is_editing_clone.get()
            }
        );

        // 开始编辑命令
        let selected_item_clone = selected_item.clone();
        let edit_item_clone = edit_item.clone();
        let is_editing_clone = is_editing.clone();
        let selected_item_clone2 = selected_item.clone();
        let is_editing_clone2 = is_editing.clone();
        let _start_edit_command = Command::new(
            move || {
                if let Some(item) = selected_item_clone.get() {
                    edit_item_clone.set(Some(item));
                    is_editing_clone.set(true);
                }
                Ok(())
            },
            move || selected_item_clone2.get().is_some() && !is_editing_clone2.get()
        );

        // 取消编辑命令
        let edit_item_clone = edit_item.clone();
        let is_editing_clone = is_editing.clone();
        let is_editing_clone2 = is_editing.clone();
        let _cancel_edit_command = Command::new(
            move || {
                edit_item_clone.set(None);
                is_editing_clone.set(false);
                Ok(())
            },
            move || is_editing_clone2.get()
        );

        // 添加物品命令
        let is_editing_clone = is_editing.clone();
        let edit_item_clone = edit_item.clone();
        
        let add_item_command = Command::new(
            move || -> Result<()> {
                let new_item = Item::default();
                edit_item_clone.set(Some(new_item));
                is_editing_clone.set(true);
                Ok(())
            },
            {
                let is_editing_clone = is_editing.clone();
                move || !is_editing_clone.get()
            }
        );

        // 删除物品命令
        let selected_item_clone = selected_item.clone();
        let items_clone = items.clone();
        let filtered_items_clone = filtered_items.clone();
        
        let delete_item_command = Command::new(
            move || -> Result<()> {
                if let Some(item) = selected_item_clone.get() {
                    // 从列表中移除物品
                    items_clone.update(|items| {
                        items.retain(|i| i.id != item.id);
                    });
                    filtered_items_clone.update(|items| {
                        items.retain(|i| i.id != item.id);
                    });
                    selected_item_clone.set(None);
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有选中的物品"))
                }
            },
            {
                let selected_item_clone = selected_item.clone();
                move || selected_item_clone.get().is_some()
            }
        );

        // 搜索命令
        let search_query_clone = search_query.clone();
        let items_clone = items.clone();
        let filtered_items_clone = filtered_items.clone();
        let item_type_filter_clone = item_type_filter.clone();
        
        let search_command = Command::new(
            move || -> Result<()> {
                let query = search_query_clone.get().to_lowercase();
                let type_filter = item_type_filter_clone.get();
                let all_items = items_clone.get();
                
                let filtered: Vec<Item> = all_items.into_iter()
                    .filter(|item| {
                        // 名称搜索
                        let name_match = query.is_empty() || 
                            item.name.to_lowercase().contains(&query);
                        
                        // 类型过滤
                        let type_match = type_filter.is_none() || 
                            type_filter.as_ref() == Some(&item.item_type);
                        
                        name_match && type_match
                    })
                    .collect();
                
                filtered_items_clone.set(filtered);
                Ok(())
            },
            || true
        );

        // 设置搜索查询变化时自动搜索
        let search_command_clone = search_command.clone();
        search_query.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        // 设置类型过滤变化时自动搜索
        let search_command_clone = search_command.clone();
        item_type_filter.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        Ok(Self {
            base,
            editor,
            items,
            selected_item,
            filtered_items,
            search_query,
            item_type_filter,
            is_editing,
            edit_item,
            load_items_command,
            save_item_command,
            add_item_command,
            delete_item_command,
            search_command,
        })
    }

    // 开始编辑物品
    pub fn start_edit(&self, item: &Item) -> Result<()> {
        if self.is_editing.get() {
            return Err(anyhow::anyhow!("已经在编辑模式中"));
        }
        
        self.edit_item.set(Some(item.clone()));
        self.is_editing.set(true);
        self.base.set_dirty(false);
        Ok(())
    }

    // 更新编辑中的物品
    pub fn update_edit_item<F>(&self, updater: F) -> Result<()> 
    where 
        F: FnOnce(&mut Item),
    {
        if !self.is_editing.get() {
            return Err(anyhow::anyhow!("不在编辑模式中"));
        }
        
        self.edit_item.update(|item_opt| {
            if let Some(item) = item_opt.as_mut() {
                updater(item);
            }
        });
        
        self.base.set_dirty(true);
        Ok(())
    }

    // 获取物品类型列表（用于过滤）
    pub fn get_item_types(&self) -> Vec<String> {
        let items = self.items.get();
        let mut types: Vec<String> = items.iter()
            .map(|item| item.item_type.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        types.sort();
        types
    }
}

impl BaseViewModel for ItemViewModel {
    fn initialize(&self) -> Result<()> {
        self.base.initialize()?;
        self.load_items_command.execute()
    }

    fn cleanup(&self) -> Result<()> {
        self.base.cleanup()
    }

    fn has_unsaved_changes(&self) -> bool {
        self.base.has_unsaved_changes()
    }
}

impl EditableViewModel for ItemViewModel {
    fn save(&self) -> Result<()> {
        self.save_item_command.execute()
    }

    fn cancel(&self) -> Result<()> {
        self.is_editing.set(false);
        self.edit_item.set(None);
        self.base.set_dirty(false);
        Ok(())
    }

    fn reset(&self) -> Result<()> {
        self.cancel()?;
        self.selected_item.set(None);
        self.search_query.set(String::new());
        self.item_type_filter.set(None);
        Ok(())
    }
}

impl LoadableViewModel for ItemViewModel {
    fn load(&self) -> Result<()> {
        self.load_items_command.execute()
    }

    fn is_loading(&self) -> bool {
        self.load_items_command.is_executing() || self.save_item_command.is_executing()
    }
}

impl SearchableViewModel for ItemViewModel {
    fn search(&self, query: &str) -> Result<()> {
        self.search_query.set(query.to_string());
        Ok(())
    }

    fn clear_search(&self) -> Result<()> {
        self.search_query.set(String::new());
        self.item_type_filter.set(None);
        Ok(())
    }

    fn get_search_query(&self) -> String {
        self.search_query.get()
    }
}

impl SelectableViewModel<Item> for ItemViewModel {
    fn select_item(&self, item: &Item) -> Result<()> {
        self.selected_item.set(Some(item.clone()));
        Ok(())
    }

    fn clear_selection(&self) -> Result<()> {
        self.selected_item.set(None);
        Ok(())
    }

    fn get_selected_item(&self) -> Option<Item> {
        self.selected_item.get()
    }
}

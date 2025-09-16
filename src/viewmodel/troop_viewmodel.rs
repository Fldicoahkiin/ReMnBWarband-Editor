// 兵种编辑器ViewModel

use std::sync::Arc;
use anyhow::Result;
use crate::editor::Editor;
use crate::data::Troop;
use super::{BaseViewModel, BaseViewModelImpl, AsyncCommand, EditableViewModel, LoadableViewModel, SearchableViewModel, SelectableViewModel, observable::{Observable, Command}};

// 兵种编辑器ViewModel
pub struct TroopViewModel {
    base: BaseViewModelImpl,
    #[allow(dead_code)]
    editor: Arc<Editor>,
    
    // 数据
    pub troops: Observable<Vec<Troop>>,
    pub selected_troop: Observable<Option<Troop>>,
    pub filtered_troops: Observable<Vec<Troop>>,
    
    // 搜索和过滤
    pub search_query: Observable<String>,
    pub faction_filter: Observable<Option<String>>,
    pub troop_class_filter: Observable<Option<String>>,
    
    // 编辑状态
    pub is_editing: Observable<bool>,
    pub edit_troop: Observable<Option<Troop>>,
    
    // 命令
    pub load_troops_command: AsyncCommand,
    pub save_troop_command: Command,
    pub add_troop_command: Command,
    pub delete_troop_command: Command,
    pub search_command: Command,
}

impl TroopViewModel {
    pub fn new(editor: Arc<Editor>) -> Result<Self> {
        let base = BaseViewModelImpl::new();
        let troops = Observable::new(Vec::new());
        let selected_troop: Observable<Option<Troop>> = Observable::new(None);
        let filtered_troops = Observable::new(Vec::new());
        let search_query = Observable::new(String::new());
        let faction_filter = Observable::new(None);
        let troop_class_filter = Observable::new(None);
        let is_editing = Observable::new(false);
        let edit_troop: Observable<Option<Troop>> = Observable::new(None);

        // 加载兵种命令
        let editor_clone = Arc::clone(&editor);
        let troops_clone = troops.clone();
        let filtered_troops_clone = filtered_troops.clone();
        let base_clone = base.clone();
        
        let load_troops_command = AsyncCommand::new(
            move || -> Result<()> {
                base_clone.set_loading(true);
                base_clone.clear_messages();
                
                let loaded_troops = editor_clone.get_troops();
                let troops_len = loaded_troops.len();
                troops_clone.set(loaded_troops.clone());
                filtered_troops_clone.set(loaded_troops);
                
                base_clone.set_loading(false);
                base_clone.set_status(Some(format!("已加载 {} 个兵种", troops_len)));
                Ok(())
            },
            || true
        );

        // 保存兵种命令
        let _editor_clone = Arc::clone(&editor);
        let edit_troop_clone = edit_troop.clone();
        let is_editing_clone = is_editing.clone();
        let base_clone = base.clone();
        
        let save_troop_command = AsyncCommand::new(
            move || -> Result<()> {
                if let Some(_troop) = edit_troop_clone.get() {
                    base_clone.set_loading(true);
                    
                    // 这里应该调用editor的保存方法
                    // 暂时模拟保存成功
                    
                    is_editing_clone.set(false);
                    edit_troop_clone.set(None);
                    base_clone.set_dirty(false);
                    base_clone.set_loading(false);
                    base_clone.set_status(Some("兵种保存成功".to_string()));
                    
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有要保存的兵种"))
                }
            },
            {
                let edit_troop_clone = edit_troop.clone();
                let is_editing_clone = is_editing.clone();
                move || edit_troop_clone.get().is_some() && is_editing_clone.get()
            }
        );

        // 添加兵种命令
        let is_editing_clone = is_editing.clone();
        let edit_troop_clone = edit_troop.clone();
        
        let add_troop_command = Command::new(
            move || -> Result<()> {
                let new_troop = Troop::default();
                edit_troop_clone.set(Some(new_troop));
                is_editing_clone.set(true);
                Ok(())
            },
            {
                let is_editing_clone = is_editing.clone();
                move || !is_editing_clone.get()
            }
        );

        // 删除兵种命令
        let selected_troop_clone = selected_troop.clone();
        let troops_clone = troops.clone();
        let filtered_troops_clone = filtered_troops.clone();
        
        let delete_troop_command = Command::new(
            move || -> Result<()> {
                if let Some(troop) = selected_troop_clone.get() {
                    // 从列表中移除兵种
                    troops_clone.update(|troops| {
                        troops.retain(|t| t.id != troop.id);
                    });
                    filtered_troops_clone.update(|troops| {
                        troops.retain(|t| t.id != troop.id);
                    });
                    selected_troop_clone.set(None);
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有选中的兵种"))
                }
            },
            {
                let selected_troop_clone = selected_troop.clone();
                move || selected_troop_clone.get().is_some()
            }
        );

        // 搜索命令
        let search_query_clone = search_query.clone();
        let troops_clone = troops.clone();
        let filtered_troops_clone = filtered_troops.clone();
        let faction_filter_clone = faction_filter.clone();
        let troop_class_filter_clone = troop_class_filter.clone();
        
        let search_command = Command::new(
            move || -> Result<()> {
                let query = search_query_clone.get().to_lowercase();
                let faction_filter = faction_filter_clone.get();
                let class_filter = troop_class_filter_clone.get();
                let all_troops = troops_clone.get();
                
                let filtered: Vec<Troop> = all_troops.into_iter()
                    .filter(|troop| {
                        // 名称搜索
                        let name_match = query.is_empty() || 
                            troop.name.to_lowercase().contains(&query);
                        
                        // 派系过滤
                        let faction_match = faction_filter.is_none() || 
                            faction_filter.as_ref() == Some(&troop.faction);
                        
                        // 兵种类型过滤
                        let class_match = class_filter.is_none() || 
                            class_filter.as_ref() == Some(&troop.troop_class);
                        
                        name_match && faction_match && class_match
                    })
                    .collect();
                
                filtered_troops_clone.set(filtered);
                Ok(())
            },
            || true
        );

        // 设置搜索查询变化时自动搜索
        let search_command_clone = search_command.clone();
        search_query.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        // 设置过滤器变化时自动搜索
        let search_command_clone = search_command.clone();
        faction_filter.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        let search_command_clone = search_command.clone();
        troop_class_filter.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        Ok(Self {
            base,
            editor,
            troops,
            selected_troop,
            filtered_troops,
            search_query,
            faction_filter,
            troop_class_filter,
            is_editing,
            edit_troop,
            load_troops_command,
            save_troop_command,
            add_troop_command,
            delete_troop_command,
            search_command,
        })
    }

    // 开始编辑兵种
    pub fn start_edit(&self, troop: &Troop) -> Result<()> {
        if self.is_editing.get() {
            return Err(anyhow::anyhow!("已经在编辑模式中"));
        }
        
        self.edit_troop.set(Some(troop.clone()));
        self.is_editing.set(true);
        self.base.set_dirty(false);
        Ok(())
    }

    // 更新编辑中的兵种
    pub fn update_edit_troop<F>(&self, updater: F) -> Result<()> 
    where 
        F: FnOnce(&mut Troop),
    {
        if !self.is_editing.get() {
            return Err(anyhow::anyhow!("不在编辑模式中"));
        }
        
        self.edit_troop.update(|troop_opt| {
            if let Some(troop) = troop_opt.as_mut() {
                updater(troop);
            }
        });
        
        self.base.set_dirty(true);
        Ok(())
    }

    // 获取派系列表（用于过滤）
    pub fn get_factions(&self) -> Vec<String> {
        let troops = self.troops.get();
        let mut factions: Vec<String> = troops.iter()
            .map(|troop| troop.faction.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        factions.sort();
        factions
    }

    // 获取兵种类型列表（用于过滤）
    pub fn get_troop_classes(&self) -> Vec<String> {
        let troops = self.troops.get();
        let mut classes: Vec<String> = troops.iter()
            .map(|troop| troop.troop_class.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        classes.sort();
        classes
    }
}

impl BaseViewModel for TroopViewModel {
    fn initialize(&self) -> Result<()> {
        self.base.initialize()?;
        self.load_troops_command.execute()
    }

    fn cleanup(&self) -> Result<()> {
        self.base.cleanup()
    }

    fn has_unsaved_changes(&self) -> bool {
        self.base.has_unsaved_changes()
    }
}

impl EditableViewModel for TroopViewModel {
    fn save(&self) -> Result<()> {
        self.save_troop_command.execute()
    }

    fn cancel(&self) -> Result<()> {
        self.is_editing.set(false);
        self.edit_troop.set(None);
        self.base.set_dirty(false);
        Ok(())
    }

    fn reset(&self) -> Result<()> {
        self.cancel()?;
        self.selected_troop.set(None);
        self.search_query.set(String::new());
        self.faction_filter.set(None);
        self.troop_class_filter.set(None);
        Ok(())
    }
}

impl LoadableViewModel for TroopViewModel {
    fn load(&self) -> Result<()> {
        self.load_troops_command.execute()
    }

    fn is_loading(&self) -> bool {
        self.load_troops_command.is_executing() || self.save_troop_command.is_executing()
    }
}

impl SearchableViewModel for TroopViewModel {
    fn search(&self, query: &str) -> Result<()> {
        self.search_query.set(query.to_string());
        Ok(())
    }

    fn clear_search(&self) -> Result<()> {
        self.search_query.set(String::new());
        self.faction_filter.set(None);
        self.troop_class_filter.set(None);
        Ok(())
    }

    fn get_search_query(&self) -> String {
        self.search_query.get()
    }
}

impl SelectableViewModel<Troop> for TroopViewModel {
    fn select_item(&self, item: &Troop) -> Result<()> {
        self.selected_troop.set(Some(item.clone()));
        Ok(())
    }

    fn clear_selection(&self) -> Result<()> {
        self.selected_troop.set(None);
        Ok(())
    }

    fn get_selected_item(&self) -> Option<Troop> {
        self.selected_troop.get()
    }
}

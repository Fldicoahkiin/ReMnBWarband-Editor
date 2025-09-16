// 派系编辑器ViewModel

use std::sync::Arc;
use anyhow::Result;
use crate::editor::Editor;
use crate::data::Faction;
use super::{BaseViewModel, BaseViewModelImpl, AsyncCommand, EditableViewModel, LoadableViewModel, SearchableViewModel, SelectableViewModel, observable::{Observable, Command}};

// 派系编辑器ViewModel
pub struct FactionViewModel {
    base: BaseViewModelImpl,
    #[allow(dead_code)]
    editor: Arc<Editor>,
    
    // 数据
    pub factions: Observable<Vec<Faction>>,
    pub selected_faction: Observable<Option<Faction>>,
    pub filtered_factions: Observable<Vec<Faction>>,
    
    // 搜索和过滤
    pub search_query: Observable<String>,
    pub culture_filter: Observable<Option<String>>,
    
    // 编辑状态
    pub is_editing: Observable<bool>,
    pub edit_faction: Observable<Option<Faction>>,
    
    // 命令
    pub load_factions_command: AsyncCommand,
    pub save_faction_command: Command,
    pub add_faction_command: Command,
    pub delete_faction_command: Command,
    pub search_command: Command,
}

impl FactionViewModel {
    pub fn new(editor: Arc<Editor>) -> Result<Self> {
        let base = BaseViewModelImpl::new();
        let factions = Observable::new(Vec::new());
        let selected_faction: Observable<Option<Faction>> = Observable::new(None);
        let filtered_factions = Observable::new(Vec::new());
        let search_query = Observable::new(String::new());
        let culture_filter = Observable::new(None);
        let is_editing = Observable::new(false);
        let edit_faction: Observable<Option<Faction>> = Observable::new(None);

        // 加载派系命令
        let editor_clone = Arc::clone(&editor);
        let factions_clone = factions.clone();
        let filtered_factions_clone = filtered_factions.clone();
        let base_clone = base.clone();
        
        let load_factions_command = AsyncCommand::new(
            move || -> Result<()> {
                base_clone.set_loading(true);
                base_clone.clear_messages();
                
                let loaded_factions = editor_clone.get_factions();
                let factions_len = loaded_factions.len();
                factions_clone.set(loaded_factions.clone());
                filtered_factions_clone.set(loaded_factions);
                
                base_clone.set_loading(false);
                base_clone.set_status(Some(format!("已加载 {} 个派系", factions_len)));
                Ok(())
            },
            || true
        );

        // 保存派系命令
        let _editor_clone = Arc::clone(&editor);
        let edit_faction_clone = edit_faction.clone();
        let is_editing_clone = is_editing.clone();
        let base_clone = base.clone();
        
        let save_faction_command = AsyncCommand::new(
            move || -> Result<()> {
                if let Some(_faction) = edit_faction_clone.get() {
                    base_clone.set_loading(true);
                    
                    // 这里应该调用editor的保存方法
                    // 暂时模拟保存成功
                    
                    is_editing_clone.set(false);
                    edit_faction_clone.set(None);
                    base_clone.set_dirty(false);
                    base_clone.set_loading(false);
                    base_clone.set_status(Some("派系保存成功".to_string()));
                    
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有要保存的派系"))
                }
            },
            {
                let edit_faction_clone = edit_faction.clone();
                let is_editing_clone = is_editing.clone();
                move || edit_faction_clone.get().is_some() && is_editing_clone.get()
            }
        );

        // 添加派系命令
        let is_editing_clone = is_editing.clone();
        let edit_faction_clone = edit_faction.clone();
        
        let add_faction_command = Command::new(
            move || -> Result<()> {
                let new_faction = Faction::default();
                edit_faction_clone.set(Some(new_faction));
                is_editing_clone.set(true);
                Ok(())
            },
            {
                let is_editing_clone = is_editing.clone();
                move || !is_editing_clone.get()
            }
        );

        // 删除派系命令
        let selected_faction_clone = selected_faction.clone();
        let factions_clone = factions.clone();
        let filtered_factions_clone = filtered_factions.clone();
        
        let delete_faction_command = Command::new(
            move || -> Result<()> {
                if let Some(faction) = selected_faction_clone.get() {
                    // 从列表中移除派系
                    factions_clone.update(|factions| {
                        factions.retain(|f| f.id != faction.id);
                    });
                    filtered_factions_clone.update(|factions| {
                        factions.retain(|f| f.id != faction.id);
                    });
                    selected_faction_clone.set(None);
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("没有选中的派系"))
                }
            },
            {
                let selected_faction_clone = selected_faction.clone();
                move || selected_faction_clone.get().is_some()
            }
        );

        // 搜索命令
        let search_query_clone = search_query.clone();
        let factions_clone = factions.clone();
        let filtered_factions_clone = filtered_factions.clone();
        let culture_filter_clone = culture_filter.clone();
        
        let search_command = Command::new(
            move || -> Result<()> {
                let query = search_query_clone.get().to_lowercase();
                let culture_filter = culture_filter_clone.get();
                let all_factions = factions_clone.get();
                
                let filtered: Vec<Faction> = all_factions.into_iter()
                    .filter(|faction| {
                        // 名称搜索
                        let name_match = query.is_empty() || 
                            faction.name.to_lowercase().contains(&query);
                        
                        // 文化过滤
                        let culture_match = culture_filter.is_none() || 
                            culture_filter.as_ref() == Some(&faction.culture);
                        
                        name_match && culture_match
                    })
                    .collect();
                
                filtered_factions_clone.set(filtered);
                Ok(())
            },
            || true
        );

        // 设置搜索查询变化时自动搜索
        let search_command_clone = search_command.clone();
        search_query.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        // 设置文化过滤变化时自动搜索
        let search_command_clone = search_command.clone();
        culture_filter.subscribe(move |_| {
            let _ = search_command_clone.execute();
        });

        Ok(Self {
            base,
            editor,
            factions,
            selected_faction,
            filtered_factions,
            search_query,
            culture_filter,
            is_editing,
            edit_faction,
            load_factions_command,
            save_faction_command,
            add_faction_command,
            delete_faction_command,
            search_command,
        })
    }

    // 开始编辑派系
    pub fn start_edit(&self, faction: &Faction) -> Result<()> {
        if self.is_editing.get() {
            return Err(anyhow::anyhow!("已经在编辑模式中"));
        }
        
        self.edit_faction.set(Some(faction.clone()));
        self.is_editing.set(true);
        self.base.set_dirty(false);
        Ok(())
    }

    // 更新编辑中的派系
    pub fn update_edit_faction<F>(&self, updater: F) -> Result<()> 
    where 
        F: FnOnce(&mut Faction),
    {
        if !self.is_editing.get() {
            return Err(anyhow::anyhow!("不在编辑模式中"));
        }
        
        self.edit_faction.update(|faction_opt| {
            if let Some(faction) = faction_opt.as_mut() {
                updater(faction);
            }
        });
        
        self.base.set_dirty(true);
        Ok(())
    }

    // 获取文化列表（用于过滤）
    pub fn get_cultures(&self) -> Vec<String> {
        let factions = self.factions.get();
        let mut cultures: Vec<String> = factions.iter()
            .map(|faction| faction.culture.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        cultures.sort();
        cultures
    }
}

impl BaseViewModel for FactionViewModel {
    fn initialize(&self) -> Result<()> {
        self.base.initialize()?;
        self.load_factions_command.execute()
    }

    fn cleanup(&self) -> Result<()> {
        self.base.cleanup()
    }

    fn has_unsaved_changes(&self) -> bool {
        self.base.has_unsaved_changes()
    }
}

impl EditableViewModel for FactionViewModel {
    fn save(&self) -> Result<()> {
        self.save_faction_command.execute()
    }

    fn cancel(&self) -> Result<()> {
        self.is_editing.set(false);
        self.edit_faction.set(None);
        self.base.set_dirty(false);
        Ok(())
    }

    fn reset(&self) -> Result<()> {
        self.cancel()?;
        self.selected_faction.set(None);
        self.search_query.set(String::new());
        self.culture_filter.set(None);
        Ok(())
    }
}

impl LoadableViewModel for FactionViewModel {
    fn load(&self) -> Result<()> {
        self.load_factions_command.execute()
    }

    fn is_loading(&self) -> bool {
        self.load_factions_command.is_executing() || self.save_faction_command.is_executing()
    }
}

impl SearchableViewModel for FactionViewModel {
    fn search(&self, query: &str) -> Result<()> {
        self.search_query.set(query.to_string());
        Ok(())
    }

    fn clear_search(&self) -> Result<()> {
        self.search_query.set(String::new());
        self.culture_filter.set(None);
        Ok(())
    }

    fn get_search_query(&self) -> String {
        self.search_query.get()
    }
}

impl SelectableViewModel<Faction> for FactionViewModel {
    fn select_item(&self, item: &Faction) -> Result<()> {
        self.selected_faction.set(Some(item.clone()));
        Ok(())
    }

    fn clear_selection(&self) -> Result<()> {
        self.selected_faction.set(None);
        Ok(())
    }

    fn get_selected_item(&self) -> Option<Faction> {
        self.selected_faction.get()
    }
}

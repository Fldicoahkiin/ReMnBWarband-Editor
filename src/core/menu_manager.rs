use muda::{Menu, MenuItem, Submenu, PredefinedMenuItem};
use std::collections::HashMap;

pub struct MenuManager {
    pub menu: Menu,
    menu_items: HashMap<String, MenuItem>,
}

impl MenuManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let menu = Menu::new();
        let mut menu_items = HashMap::new();

        let file_menu = Submenu::new("文件", true);
        let new_item = MenuItem::new("新建", true, None);
        let open_item = MenuItem::new("打开", true, Some(muda::accelerator::Accelerator::new(Some(muda::accelerator::Modifiers::CONTROL), muda::accelerator::Code::KeyO)));
        let save_item = MenuItem::new("保存", true, Some(muda::accelerator::Accelerator::new(Some(muda::accelerator::Modifiers::CONTROL), muda::accelerator::Code::KeyS)));
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("退出", true, Some(muda::accelerator::Accelerator::new(Some(muda::accelerator::Modifiers::CONTROL), muda::accelerator::Code::KeyQ)));

        file_menu.append(&new_item)?;
        file_menu.append(&open_item)?;
        file_menu.append(&save_item)?;
        file_menu.append(&separator)?;
        file_menu.append(&quit_item)?;

        let edit_menu = Submenu::new("编辑", true);
        let undo_item = MenuItem::new("撤销", true, Some(muda::accelerator::Accelerator::new(Some(muda::accelerator::Modifiers::CONTROL), muda::accelerator::Code::KeyZ)));
        let redo_item = MenuItem::new("重做", true, Some(muda::accelerator::Accelerator::new(Some(muda::accelerator::Modifiers::CONTROL | muda::accelerator::Modifiers::SHIFT), muda::accelerator::Code::KeyZ)));
        let copy_item = PredefinedMenuItem::copy(None);
        let paste_item = PredefinedMenuItem::paste(None);

        edit_menu.append(&undo_item)?;
        edit_menu.append(&redo_item)?;
        edit_menu.append(&PredefinedMenuItem::separator())?;
        edit_menu.append(&copy_item)?;
        edit_menu.append(&paste_item)?;

        let tools_menu = Submenu::new("工具", true);
        let python_item = MenuItem::new("Python脚本", true, None);
        let validate_item = MenuItem::new("数据验证", true, None);

        tools_menu.append(&python_item)?;
        tools_menu.append(&validate_item)?;

        let help_menu = Submenu::new("帮助", true);
        let about_item = MenuItem::new("关于", true, None);

        help_menu.append(&about_item)?;

        menu.append(&file_menu)?;
        menu.append(&edit_menu)?;
        menu.append(&tools_menu)?;
        menu.append(&help_menu)?;

        menu_items.insert("new".to_string(), new_item);
        menu_items.insert("open".to_string(), open_item);
        menu_items.insert("save".to_string(), save_item);
        menu_items.insert("quit".to_string(), quit_item);
        menu_items.insert("undo".to_string(), undo_item);
        menu_items.insert("redo".to_string(), redo_item);
        menu_items.insert("python".to_string(), python_item);
        menu_items.insert("validate".to_string(), validate_item);
        menu_items.insert("about".to_string(), about_item);

        Ok(Self {
            menu,
            menu_items,
        })
    }

    pub fn init_for_nsapp(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "macos")]
        {
            self.menu.init_for_nsapp();
        }
        
        Ok(())
    }


    pub fn get_menu_item(&self, id: &str) -> Option<&MenuItem> {
        self.menu_items.get(id)
    }

    pub fn handle_menu_event(&self, event: muda::MenuEvent) -> Option<String> {
        for (action, item) in &self.menu_items {
            if event.id == item.id() {
                return Some(action.clone());
            }
        }
        None
    }
}

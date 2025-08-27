use std::collections::HashMap;
use anyhow::Result;
use crate::models::{Item, ItemType};

/// 物品管理器 - 负责物品数据的内存管理和操作
#[derive(Debug, Clone)]
pub struct ItemManager {
    items: Vec<Item>,
    selected_index: Option<usize>,
    search_filter: String,
}

impl ItemManager {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_index: None,
            search_filter: String::new(),
        }
    }

    /// 加载物品列表
    pub fn load_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.selected_index = None;
    }

    /// 添加单个物品
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// 获取所有物品
    pub fn get_items(&self) -> &[Item] {
        &self.items
    }

    /// 获取过滤后的物品列表
    pub fn get_filtered_items(&self) -> Vec<(usize, &Item)> {
        if self.search_filter.is_empty() {
            self.items.iter().enumerate().collect()
        } else {
            self.items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.name.to_lowercase().contains(&self.search_filter.to_lowercase())
                        || item.id.to_lowercase().contains(&self.search_filter.to_lowercase())
                })
                .collect()
        }
    }

    /// 设置搜索过滤器
    pub fn set_search_filter(&mut self, filter: String) {
        self.search_filter = filter;
    }

    /// 选择物品
    pub fn select_item(&mut self, index: usize) -> Option<&Item> {
        if index < self.items.len() {
            self.selected_index = Some(index);
            Some(&self.items[index])
        } else {
            None
        }
    }

    /// 获取当前选中的物品
    pub fn get_selected_item(&self) -> Option<&Item> {
        self.selected_index.and_then(|index| self.items.get(index))
    }

    /// 获取当前选中的物品（可变引用）
    pub fn get_selected_item_mut(&mut self) -> Option<&mut Item> {
        self.selected_index.and_then(|index| self.items.get_mut(index))
    }

    /// 创建新物品
    pub fn create_new_item(&mut self) -> usize {
        let new_item = Item {
            id: format!("new_item_{}", self.items.len()),
            name: "新物品".to_string(),
            mesh_name: "default_mesh".to_string(),
            item_type: ItemType::Other,
            weight: 1.0,
            price: 100,
            flags: Default::default(),
            damage: 0,
            speed: 0,
            reach: 0,
            accuracy: 0,
            armor: 0,
            leg_armor: 0,
            difficulty: 0,
            hit_points: 100,
            material: "default_material".to_string(),
            texture: "default_texture".to_string(),
            triggers: Vec::new(),
            capabilities: 0,
            modifiers: Vec::new(),
        };

        self.items.push(new_item);
        let new_index = self.items.len() - 1;
        self.selected_index = Some(new_index);
        new_index
    }

    /// 删除物品
    pub fn delete_item(&mut self, index: usize) -> Result<()> {
        if index < self.items.len() {
            self.items.remove(index);
            
            // 调整选中索引
            if let Some(selected) = self.selected_index {
                if selected == index {
                    self.selected_index = None;
                } else if selected > index {
                    self.selected_index = Some(selected - 1);
                }
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("物品索引超出范围"))
        }
    }

    /// 复制物品
    pub fn duplicate_item(&mut self, index: usize) -> Result<usize> {
        if let Some(item) = self.items.get(index).cloned() {
            let mut new_item = item;
            new_item.id = format!("{}_copy", new_item.id);
            new_item.name = format!("{} (副本)", new_item.name);
            
            self.items.push(new_item);
            let new_index = self.items.len() - 1;
            self.selected_index = Some(new_index);
            Ok(new_index)
        } else {
            Err(anyhow::anyhow!("物品索引超出范围"))
        }
    }

    /// 更新选中物品的基础信息
    pub fn update_selected_item_basic(&mut self, 
        id: String, 
        name: String, 
        item_type: ItemType, 
        weight: f32, 
        price: u32
    ) -> Result<()> {
        if let Some(item) = self.get_selected_item_mut() {
            item.id = id;
            item.name = name;
            item.item_type = item_type;
            item.weight = weight;
            item.price = price;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的物品"))
        }
    }

    /// 更新选中物品的武器属性
    pub fn update_selected_item_weapon(&mut self, 
        damage: u32, 
        speed: u32, 
        reach: u32, 
        accuracy: u32
    ) -> Result<()> {
        if let Some(item) = self.get_selected_item_mut() {
            item.damage = damage;
            item.speed = speed;
            item.reach = reach;
            item.accuracy = accuracy;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的物品"))
        }
    }

    /// 更新选中物品的标志位
    pub fn update_selected_item_flags(&mut self, flags: HashMap<String, bool>) -> Result<()> {
        if let Some(item) = self.get_selected_item_mut() {
            // 更新标志位
            item.flags.two_handed = flags.get("two_handed").copied().unwrap_or(false);
            item.flags.can_penetrate_shield = flags.get("can_penetrate_shield").copied().unwrap_or(false);
            item.flags.can_knock_down = flags.get("can_knock_down").copied().unwrap_or(false);
            item.flags.bonus_against_shield = flags.get("bonus_against_shield").copied().unwrap_or(false);
            item.flags.unbalanced = flags.get("unbalanced").copied().unwrap_or(false);
            item.flags.thrust_weapon = flags.get("thrust_weapon").copied().unwrap_or(false);
            item.flags.swing_weapon = flags.get("swing_weapon").copied().unwrap_or(false);
            item.flags.crush_through = flags.get("crush_through").copied().unwrap_or(false);
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有选中的物品"))
        }
    }

    /// 获取物品统计信息
    pub fn get_stats(&self) -> ItemStats {
        let total_items = self.items.len();
        let weapon_count = self.items.iter().filter(|item| item.is_weapon()).count();
        let armor_count = self.items.iter().filter(|item| item.is_armor()).count();
        let filtered_count = self.get_filtered_items().len();

        ItemStats {
            total_items,
            weapon_count,
            armor_count,
            filtered_count,
        }
    }

    /// 验证所有物品数据
    pub fn validate_all(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for (index, item) in self.items.iter().enumerate() {
            if item.id.is_empty() {
                errors.push(format!("物品 {} 缺少ID", index));
            }
            
            if item.name.is_empty() {
                errors.push(format!("物品 {} 缺少名称", index));
            }
            
            if item.weight < 0.0 {
                errors.push(format!("物品 {} 重量不能为负数", index));
            }
            
            if item.price == 0 {
                errors.push(format!("物品 {} 价格不能为0", index));
            }
        }
        
        errors
    }
}

impl Default for ItemManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 物品统计信息
#[derive(Debug, Clone)]
pub struct ItemStats {
    pub total_items: usize,
    pub weapon_count: usize,
    pub armor_count: usize,
    pub filtered_count: usize,
}

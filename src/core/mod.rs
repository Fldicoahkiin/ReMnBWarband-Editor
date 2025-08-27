pub mod app_manager;
pub mod file_manager;
pub mod item_manager;
pub mod troop_manager;
pub mod trigger_manager;
pub mod game_detector;
pub mod python_manager;
pub mod validator;
pub mod parser;
pub mod menu_manager;

use anyhow::Result;
use std::path::Path;
use crate::models::{Item, Troop, Faction};
use parser::DataParser;

pub struct DataManager {
    item_parser: parser::ItemParser,
    troop_parser: parser::TroopParser,
    faction_parser: parser::FactionParser,
}

impl DataManager {
    pub fn new() -> Self {
        Self {
            item_parser: parser::ItemParser::new(),
            troop_parser: parser::TroopParser::new(),
            faction_parser: parser::FactionParser::new(),
        }
    }
    
    pub fn load_items<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Item>> {
        let data = std::fs::read_to_string(path)?;
        Ok(self.item_parser.parse_text(&data)?)
    }
    
    pub fn save_items<P: AsRef<Path>>(&self, items: &[Item], path: P) -> Result<()> {
        let data = self.item_parser.serialize_text(items)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    
    pub fn load_troops<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Troop>> {
        let data = std::fs::read_to_string(path)?;
        Ok(self.troop_parser.parse_text(&data)?)
    }
    
    pub fn save_troops<P: AsRef<Path>>(&self, troops: &[Troop], path: P) -> Result<()> {
        let data = self.troop_parser.serialize_text(troops)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    
    pub fn load_factions<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Faction>> {
        let data = std::fs::read_to_string(path)?;
        Ok(self.faction_parser.parse_text(&data)?)
    }
    
    pub fn save_factions<P: AsRef<Path>>(&self, factions: &[Faction], path: P) -> Result<()> {
        let data = self.faction_parser.serialize_text(factions)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

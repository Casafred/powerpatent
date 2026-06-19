use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleConfig {
    pub mode: ViewMode,
    pub theme_name: Option<String>,
    pub theme_description: Option<String>,
    pub patents: Vec<PatentModuleConfig>,
    pub global_extended: HashMap<String, ModuleLevel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatentModuleConfig {
    pub patent_id: String,
    pub is_key: bool,
    pub levels: HashMap<String, ModuleLevel>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ModuleLevel {
    Full,
    Lite,
    Off,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ViewMode {
    Single,
    Multi,
}

use serde::{Deserialize, Serialize};

fn default_order() -> i32 {
    1000
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct DocMeta {
    pub title: String,
    #[serde(default)]
    pub slug: String,
    pub category: String,
    pub tags: Vec<String>,
    pub date: String,
    pub description: String,
    #[serde(default)]
    pub lang: String,
    #[serde(default = "default_order")]
    pub order: i32,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct MenuNode {
    pub name: String,
    pub full_path: String,
    pub is_open: bool,
    pub is_active: bool,
    pub slug: Option<String>,
    pub children: Vec<MenuNode>,
    pub order: i32,
}

#[derive(Clone, Debug, Default)]
pub struct TocItem {
    pub level: u32,
    pub id: String,
    pub text: String,
}

use anyhow::Result;
use std::rc::Rc;

#[derive(Clone)]
pub enum EntityType {
    App,
    Command,
}

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub description: Option<String>,
    pub alias: Option<String>,
    pub command: Rc<Box<dyn Fn() -> Result<()>>>,
    pub r#type: EntityType,
}

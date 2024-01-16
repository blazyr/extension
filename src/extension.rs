use crate::REntity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub alias: Option<String>,
}

impl Entity {
    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl From<REntity> for Entity {
    fn from(value: REntity) -> Self {
        Self {
            id: value.id,
            name: value.name.map(|v| v.to_string()).into(),
            description: value.description.map(|v| v.to_string()).into(),
            alias: value.alias.map(|v| v.to_string()).into(),
        }
    }
}

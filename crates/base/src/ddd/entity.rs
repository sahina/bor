use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ddd::constants::ENTITY_ANONYMOUS;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Entity {
    id: String,
    name: String,
}

impl Entity {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Entity {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn from_name(name: impl Into<String>) -> Self {
        Entity {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            id: Uuid::new_v4().to_string(),
            name: ENTITY_ANONYMOUS.to_owned(),
        }
    }
}

impl From<String> for Entity {
    fn from(value: String) -> Self {
        Entity::from_name(value)
    }
}

#[cfg(test)]
mod test_entity {
    use crate::ddd::constants::ENTITY_ANONYMOUS;
    use crate::ddd::entity::Entity;
    use uuid::Uuid;

    #[test]
    fn default_entity() {
        let e = Entity::default();

        assert_eq!(e.name, ENTITY_ANONYMOUS);
        assert!(!e.id.is_empty());
    }

    #[test]
    fn new_entity() {
        let id = Uuid::new_v4().to_string();
        let e = Entity::new(id.clone(), "hello");

        assert_eq!(e.name, "hello");
        assert_eq!(e.id, id);
    }
}

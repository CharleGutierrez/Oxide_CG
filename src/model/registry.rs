use crate::model::schema::ModelSchema;
use std::collections::HashMap;
use std::sync::Arc;

/// A thread-safe, concurrent registry for all registered model schemas.
#[derive(Debug, Clone, Default)]
pub struct SchemaRegistry {
    schemas: Arc<HashMap<String, ModelSchema>>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(HashMap::new()),
        }
    }

    pub fn from_map(map: HashMap<String, ModelSchema>) -> Self {
        Self {
            schemas: Arc::new(map),
        }
    }

    pub fn get(&self, name: &str) -> Option<&ModelSchema> {
        self.schemas.get(&name.to_lowercase())
    }

    pub fn get_by_table(&self, table_name: &str) -> Option<&ModelSchema> {
        self.schemas
            .values()
            .find(|s| s.table_name.eq_ignore_ascii_case(table_name))
    }

    pub fn all(&self) -> Vec<&ModelSchema> {
        self.schemas.values().collect()
    }

    pub fn len(&self) -> usize {
        self.schemas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
}

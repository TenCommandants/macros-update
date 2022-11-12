use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ResourceId;
use super::ResourceOp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub variant: Option<String>,
    pub entity_type: EntityType,
    pub primary_key: String,
    pub description: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub created_timestamp: Option<DateTime<Utc>>,
    // TODO(tatiana): immutable definition?
    #[serde(with = "ts_seconds_option")]
    pub last_updated_timestamp: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

/// per node/edge type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityType {
    NodeEntity { tlabel: String },
    EdgeEntity { tlabel: String },
}

impl ResourceOp for Entity {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "Entity",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

impl Entity {
    pub fn new(
        name: &str,
        variant: Option<String>,
        entity_type: EntityType,
        primary_key: &str,
    ) -> Self {
        Entity {
            name: name.to_string(),
            variant,
            entity_type,
            primary_key: primary_key.to_string(),
            description: None,
            created_timestamp: Some(Utc::now()),
            last_updated_timestamp: None,
            tags: HashMap::new(),
            owners: Vec::new(),
        }
    }

    pub fn new_node_entity(
        name: &str,
        variant: Option<String>,
        tlabel: &str,
        primary_key: &str,
    ) -> Self {
        Entity::new(
            name,
            variant,
            EntityType::NodeEntity {
                tlabel: tlabel.to_string(),
            },
            primary_key,
        )
    }

    // TODO: The edge primary key is now set as `{src_entity_id}|{dst_entity_id}`
    pub fn new_edge_entity(
        name: &str,
        variant: Option<String>,
        tlabel: &str,
        src_entity: &Entity,
        dst_entity: &Entity,
    ) -> Self {
        Entity::new(
            name,
            variant,
            EntityType::EdgeEntity {
                tlabel: tlabel.to_string(),
            },
            &format!("{}|{}", src_entity.resource_id(), dst_entity.resource_id()),
        )
    }
}

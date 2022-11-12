use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Entity, TopologyType};

use super::ResourceId;
use super::ResourceOp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub name: String,
    pub variant: Option<String>,
    pub description: Option<String>,
    pub entity_ids: Vec<ResourceId>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

impl ResourceOp for Graph {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "Graph",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

impl Graph {
    pub fn new(name: &str, variant: Option<String>, entities: Vec<&Entity>) -> Self {
        Graph {
            name: name.to_string(),
            variant,
            description: None,
            entity_ids: entities.iter().map(|e| e.resource_id()).collect(),
            tags: HashMap::new(),
            owners: Vec::new(),
        }
    }
}

/// Pure graph topology data. Imported from source data files, or extracted from Graphs in the native graph database.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topology {
    pub name: String,
    pub transformation_id: Option<String>,
    pub topology_type: Option<TopologyType>, // None if in native graph database for now
    pub edge_entity_ids: Vec<ResourceId>,
    pub variant: Option<String>,
    pub description: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

impl ResourceOp for Topology {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "Topology",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

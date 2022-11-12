use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ResourceId;
use super::ResourceOp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeatureView {
    TableFeatureView(TableFeatureView),
    TopologyFeatureView(TopologyFeatureView),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableFeatureView {
    pub name: String,
    pub variant: Option<String>,
    pub entity_id: ResourceId,      // entity resource id
    pub field_ids: Vec<ResourceId>, // field resource id
    pub online: bool,
    pub description: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub owner: Option<String>,
}

impl ResourceOp for TableFeatureView {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "TableFeatureView",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TopologyType {
    AdjacencyList,
    AdjacencyMatrix,
    BipartiteGraphChain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopologyFeatureView {
    pub name: String,
    pub variant: Option<String>,
    pub topology_type: TopologyType,
    pub online: bool,
    pub topology_ids: Vec<ResourceId>, // topology resource id
    pub description: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

impl ResourceOp for TopologyFeatureView {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "TopologyFeatureView",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

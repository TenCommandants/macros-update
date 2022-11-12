use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::FeatureValueType;
use super::ResourceId;
use super::ResourceOp;
use crate::transformation::DataIdT;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransformationType {
    Cypher,
    CustomFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformation {
    pub name: String,
    pub variant: Option<String>,
    pub export_resources: Vec<(DataIdT, ResourceId)>,
    pub source_field_ids: Vec<ResourceId>,
    pub dest_type: FeatureValueType,
    pub transformation_type: TransformationType,
    pub body: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

impl ResourceOp for Transformation {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}",
            "Transformation",
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }
}

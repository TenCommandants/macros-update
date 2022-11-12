use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Entity;

use super::FeatureValueType;
use super::ResourceId;
use super::ResourceOp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub variant: Option<String>,
    pub value_type: FeatureValueType,
    pub entity_id: String,
    pub transformation_id: Option<String>,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub owners: Vec<String>,
}

// Now the Field resource id is set to be `Field/{EntityName}/{FieldName}/{FieldVariant}`
impl ResourceOp for Field {
    fn resource_id(&self) -> ResourceId {
        format!(
            "{}/{}/{}/{}",
            "Field",
            Self::id_to_name(&self.entity_id),
            &self.name,
            &self.variant.as_ref().unwrap_or(&"".to_string())
        )
    }

    fn id_to_name(id: &str) -> &str {
        id.rsplit_once('/').unwrap().0.rsplit_once('/').unwrap().1
    }
}

impl Field {
    pub fn new_fields(
        name_values: Vec<(&str, FeatureValueType)>,
        entity: &Entity,
        variant: Option<String>,
    ) -> Vec<Field> {
        name_values
            .iter()
            .map(|name_type| Field {
                name: name_type.0.to_string(),
                variant: variant.clone(),
                value_type: name_type.1.clone(),
                entity_id: entity.resource_id(),
                transformation_id: None,
                description: None,
                tags: HashMap::new(),
                owners: Vec::new(),
            })
            .collect()
    }
}

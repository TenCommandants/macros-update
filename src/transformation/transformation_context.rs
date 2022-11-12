use crate::{FeatureRegistry, ResourceOp, Transformation};

use super::{DataIdT, InnerTransformationData, TransformationData, TRANSFORMATION_NAME_PREFIX};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    rc::{Rc, Weak},
};

/// A TransformationContext contains all data involved in a data flow and is used to construct
/// transformation plans for data materialization.
#[derive(Serialize, Deserialize)]
pub struct TransformationContext {
    next_data_id: DataIdT,
    data_vec: Vec<Rc<dyn TransformationData>>,
    #[serde(skip_serializing, skip_deserializing)]
    transformation: Option<Transformation>,
}

// non-pub struct makes it difficult to be used in pub trait TransformationData. I will make it pub for now until I find any better solution.
/// Used by the tranformation operations on TransformationData to create and register new data.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataTransformationContext {
    pub id: DataIdT,
    #[serde(skip_serializing, skip_deserializing)]
    pub transformation_context: Weak<RefCell<TransformationContext>>,
}

impl DataTransformationContext {
    pub(super) fn register_data(&self, data: &Rc<impl TransformationData + 'static>) {
        self.transformation_context
            .upgrade()
            .unwrap()
            .as_ref()
            .borrow_mut()
            .add_data(data);
    }

    pub(super) fn new_data_context(&self) -> DataTransformationContext {
        DataTransformationContext {
            id: self
                .transformation_context
                .upgrade()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .new_data_id(),
            transformation_context: self.transformation_context.clone(),
        }
    }

    pub(super) fn get_transformation_id(&self) -> String {
        self.transformation_context
            .upgrade()
            .unwrap()
            .as_ref()
            .borrow_mut()
            .get_transformation()
            .resource_id()
    }
    pub(super) fn export_resource(&self, data_id: DataIdT, resource_id: String) {
        self.transformation_context
            .upgrade()
            .unwrap()
            .as_ref()
            .borrow_mut()
            .get_transformation()
            .export_resources
            .push((data_id, resource_id))
    }
}

impl std::fmt::Debug for TransformationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("num data {}", self.data_vec.len()))
    }
}

impl TransformationContext {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            next_data_id: 0,
            data_vec: Vec::new(),
            transformation: None,
        }))
    }

    pub fn new_data_id(&mut self) -> DataIdT {
        self.next_data_id += 1;
        self.next_data_id - 1
    }

    pub fn add_data(&mut self, data: &Rc<impl TransformationData + 'static>) {
        self.data_vec.push(data.clone());
    }

    pub fn get_transformation(&mut self) -> &mut Transformation {
        // create anonymous transformation
        self.transformation.get_or_insert(Transformation {
            name: format!("{}{}", TRANSFORMATION_NAME_PREFIX, Utc::now().timestamp()),
            variant: None,
            source_field_ids: Vec::new(),
            export_resources: Vec::new(),
            // TODO(tatiana): dest type dependes on exported data
            dest_type: crate::FeatureValueType::Boolean,
            // TODO(tatiana): decide later
            transformation_type: crate::TransformationType::Cypher,
            body: "".to_string(),
            description: None,
            tags: HashMap::new(),
            owners: Vec::new(),
        })
    }

    pub fn build_transformation(
        &mut self,
        name: Option<String>,
        variant: Option<String>,
    ) -> Result<Option<&Transformation>, Box<dyn Error>> {
        let body = serde_json::to_string(&self)?;
        if let Some(transformation) = &mut self.transformation {
            if let Some(name_str) = name {
                transformation.name = name_str;
            }
            transformation.body = body;
            transformation.variant = variant;
        };
        Ok(self.transformation.as_ref())
    }
}

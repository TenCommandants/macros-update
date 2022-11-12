mod entity;
mod feature_view;
mod field;
mod graph;
mod transformation;

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use entity::{Entity, EntityType};
pub use feature_view::{FeatureView, TableFeatureView, TopologyFeatureView, TopologyType};
pub use field::Field;
pub use graph::{Graph, Topology};
pub use transformation::{Transformation, TransformationType};

pub type ResourceId = String;

pub trait ResourceOp: Serialize + DeserializeOwned + Debug + Clone {
    fn resource_id(&self) -> ResourceId;
    fn id_to_name(id: &str) -> &str {
        id.split_once('/').unwrap().1.rsplit_once('/').unwrap().0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeatureValueType {
    String,
    Int,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
    Duration,
    Topology,
    Array(Box<FeatureValueType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphDataset {
    SingleGraphDataset {
        name: String,
        description: Option<String>,
        feature_views: Vec<FeatureView>,
    },

    MultipleGraphsDataset {
        name: String,
        description: Option<String>,
        feature_views: Vec<FeatureView>,
    },
}

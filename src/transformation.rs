#[allow(dead_code, unused)]
mod built_in_fns;
#[allow(dead_code, unused)]
mod cypher_result;
#[allow(dead_code, unused)]
mod dataframe;
#[allow(dead_code, unused)]
mod graph;
#[allow(dead_code, unused)]
mod transformation_context;

use std::{cell::RefCell, error::Error, rc::Rc};

pub use built_in_fns::{Aggregator, RandomWalkPath};
pub use cypher_result::{CypherResultDataFrame, CypherResultGraph, CypherTransformation};
pub use dataframe::{Column, DataFrame};
pub use graph::{GraphBase, GraphComputationOps, SingleGraph};
pub use transformation_context::TransformationContext;

pub use u32 as DataIdT;

pub(super) use graph::{EdgeSelectGraph, Selector, VertexSelectGraph};

use transformation_context::DataTransformationContext;

use crate::{FeatureStore, ResourceOp};

pub const TRANSFORMATION_NAME_PREFIX: &str = "TRANSFORMATION_";

/// A TransformationData instance is registered in the TransformationContext, and its
/// implementation has a DataTransformationContext.
#[typetag::serde]
pub trait TransformationData {
    // context getter to hide direct member access to enable impl trait function on multiple structs
    fn get_context(&self) -> &DataTransformationContext;
}

pub trait InnerTransformationData {
    fn get_data_id(&self) -> DataIdT;

    // here we can add more functions for all structs that impl TransformationData
    // TODO(tatiana): handle dependency among data
}

impl<T: TransformationData> InnerTransformationData for T {
    fn get_data_id(&self) -> DataIdT {
        self.get_context().id
    }
}

impl std::fmt::Debug for dyn InnerTransformationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("data id {}", self.get_data_id()))
    }
}

pub async fn finalize_transformation(
    fs: &FeatureStore,
    tc: &Rc<RefCell<TransformationContext>>,
    fields: Vec<&impl ResourceOp>,
    topos: Vec<&impl ResourceOp>,
) -> Result<(), Box<dyn Error>> {
    let transformation = tc
        .as_ref()
        .borrow_mut()
        .build_transformation(None, None)?
        .unwrap()
        .to_owned();
    fs.registry.register_resource(&transformation).await?;
    fs.registry.register_resources(&fields).await?;
    fs.registry.register_resources(&topos).await?;
    Ok(())
}

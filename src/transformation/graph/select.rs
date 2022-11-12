use crate::{
    transformation::{
        built_in_fns::{Expression, SamplingSpec},
        DataIdT, DataTransformationContext, GraphBase, TransformationData,
    },
    Field,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

/// The representation of vertex/edge selection, recursively defined as a union of direct access, filtering, and sampling results
#[derive(Serialize, Deserialize)]
pub enum Selector {
    /// Direct access to the set of vertices/edges in a graph
    DirectAccess {
        ltype: Option<String>,
    },
    /// A set of vertices/edges in a graph whose expression is evaluated to true
    Expression(Expression),
    /// A set of sampled vertices/edges in a graph
    Sampling(SamplingSpec),
    Union(Box<Selector>, Box<Selector>),
}

#[derive(Serialize, Deserialize)]
pub enum DataFrameSet {
    Homo((String, Vec<Field>)),                    // name, fields
    Hetero(HashMap<String, (String, Vec<Field>)>), // {type, {name, fields}}
}

/// Represents a filter-projection vertex relation from the original graph, such as
/// select vertices {feat1, feat3, feat2 / feat2.avg()} from graph where vertices.type = "Person"
#[derive(Serialize, Deserialize)]
pub struct VertexSelectGraph {
    pub(super) context: DataTransformationContext,
    pub(super) graph: DataIdT,     // graph data id
    pub(super) selector: Selector, // vertex selection
    pub(super) df: DataFrameSet,
}

/// Represents a filter-projection edge relation from the original graph
#[derive(Serialize, Deserialize)]
pub struct EdgeSelectGraph {
    pub(super) context: DataTransformationContext,
    pub(super) graph: DataIdT,     // graph data id
    pub(super) selector: Selector, // edge selection
    pub(super) df: DataFrameSet,
}

#[typetag::serde]
impl TransformationData for VertexSelectGraph {
    fn get_context(&self) -> &DataTransformationContext {
        &self.context
    }
}

impl GraphBase for VertexSelectGraph {
    fn vertices(&self) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn edges(&self) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn vertices_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>> {
        todo!()
    }

    fn edges_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>> {
        todo!()
    }
}

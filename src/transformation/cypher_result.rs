use crate::{Column, FeatureValueType, ResourceOp, Topology, TopologyType};

use super::{
    DataFrame, DataIdT, DataTransformationContext, EdgeSelectGraph, GraphBase,
    InnerTransformationData, Selector, TransformationData, VertexSelectGraph,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait CypherTransformation {
    fn cypher_to_graph(&self, query: &str) -> Rc<dyn GraphBase>;
    fn cypher_to_dataframe(&self, query: &str) -> Rc<DataFrame>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CypherResultGraph {
    pub(super) context: DataTransformationContext,
    pub(super) graph: DataIdT, // graph data id
    pub(super) query: String,
}

impl<T> CypherTransformation for T
where
    T: GraphBase + TransformationData,
{
    fn cypher_to_graph(&self, query: &str) -> Rc<dyn GraphBase> {
        let res = Rc::new(CypherResultGraph {
            context: self.get_context().new_data_context(),
            graph: self.get_data_id(),
            query: query.to_string(),
        });

        self.get_context().register_data(&res);
        res
    }

    #[allow(unused)]
    fn cypher_to_dataframe(&self, query: &str) -> Rc<DataFrame> {
        let new_data_context = self.get_context().new_data_context();
        let name = format!("cypher_to_dataframe{}", new_data_context.id);
        // FIXME(tatiana): hard code for now to test transformation flow
        let res = Rc::new(DataFrame {
            context: new_data_context,
            name,
            schema: vec![
                Rc::new(Column::new(self.get_data_id(), FeatureValueType::String)),
                Rc::new(Column::new(
                    self.get_data_id(),
                    FeatureValueType::Array(Box::new(FeatureValueType::Float)),
                )),
            ],
            col_names: vec!["user".to_string(), "dims".to_string()],
            col_by_names: HashMap::new(),
        });
        self.get_context().register_data(&res);
        res
    }
}

#[typetag::serde]
impl TransformationData for CypherResultGraph {
    fn get_context(&self) -> &DataTransformationContext {
        &self.context
    }
}

impl GraphBase for CypherResultGraph {
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

    fn export_topology(&self, name: &str) -> Topology {
        let res = Topology {
            name: name.to_string(),
            transformation_id: Some(self.get_context().get_transformation_id()),
            // FIXME(tatiana): hard code for now
            topology_type: Some(TopologyType::AdjacencyList),
            edge_entity_ids: Vec::new(), // may be existing edges or a new edge type
            variant: None,
            description: None,
            created_at: None,
            tags: HashMap::new(),
            owners: Vec::new(),
        };
        self.get_context()
            .export_resource(self.get_data_id(), res.resource_id());
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CypherResultDataFrame {
    pub query: String,
    context: DataTransformationContext,
}

#[typetag::serde]
impl TransformationData for CypherResultDataFrame {
    fn get_context(&self) -> &DataTransformationContext {
        &self.context
    }
}

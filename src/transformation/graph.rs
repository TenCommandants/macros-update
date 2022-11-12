#[allow(dead_code, unused)]
mod multiple_graphs;
#[allow(dead_code, unused)]
mod select;
#[allow(dead_code, unused)]
mod single_graph;

pub use select::{DataFrameSet, EdgeSelectGraph, Selector, VertexSelectGraph};
pub use single_graph::SingleGraph;

use std::{collections::HashMap, rc::Rc};

use crate::Topology;

use super::{
    Aggregator, CypherResultDataFrame, CypherResultGraph, CypherTransformation, DataFrame,
    RandomWalkPath,
};

/// A graph interface
pub trait GraphBase {
    /// Returns a vertex data frame containing all vertices in the graph
    fn vertices(&self) -> Rc<dyn GraphBase>;

    /// Returns an edge data frame containing all edges in the graph
    fn edges(&self) -> Rc<dyn GraphBase>;

    /// Returns a vertex data frame containing all vertices of the given type in the graph
    ///
    /// # Arguments
    /// * `t` - The type of vertices to return. If the type does not exist, an error is raised
    fn vertices_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>>;

    /// Returns a edge data frame containing all edges of the given type in the graph
    ///
    /// # Arguments
    /// * `t` - The type of edges to return. If the type does not exist, an error is raised
    fn edges_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>>;

    // FIXME(tatiana): a dummy implementation to avoid repeated implementation of immature function
    fn export_topology(&self, name: &str) -> Topology {
        Topology {
            name: name.to_string(),
            transformation_id: None,
            topology_type: None,
            edge_entity_ids: Vec::new(),
            variant: None,
            description: None,
            created_at: None,
            tags: HashMap::new(),
            owners: Vec::new(),
        }
    }
}

pub trait MultipleGraphsBase {
    // TODO(tatiana): fn sample_graphs
}

pub trait GraphComputationOps {
    /// Returns an induced subgraph of the graph containing only the given vertices
    ///
    /// # Arguments
    /// * `vertices` - The vertex set from which the induced subgraph is computed
    // FIXME(tatiana): interface for subgraph extraction?
    // fn subgraph(&self, vertices: ???) -> Rc<Self>;

    /// Returns a vertex data frame of vertices visited by the random walks. TODO(tatiana) need to consider the return type
    ///
    /// # Arguments
    ///
    /// * `path` - The random walk path length and type specification
    /// * `prob` - The name of the edge feature to use as the transition probability
    /// * `restart_prob` - Probability to terminate the current trace before each transition. If None, the probability is 0
    fn random_walk(
        &self,
        path: RandomWalkPath,
        prob: &str,
        restart_prob: Option<&Vec<f32>>,
    ) -> Rc<dyn GraphBase>;

    fn random_walk_edges(
        &self,
        path: RandomWalkPath,
        prob: &str,
        restart_prob: Option<&Vec<f32>>,
    ) -> Rc<dyn GraphBase>;

    /// Returns a vertex data frame with the same set of vertices but new vertex features computed from neighbor aggregation
    ///
    /// # Arguments
    ///
    /// * `edge_type` - The type of edges to traverse. If None, all edge types are traversed as if in a homogeneous graph
    /// * `aggregator` - The aggregator to use for aggregating the neighbor features
    /// * `output_col_name` - The name of the output column
    fn aggregate_neighbors(
        &self,
        edge_type: Option<String>,
        aggregator: Aggregator,
        output_col_name: String,
    ) -> Rc<dyn GraphBase>;

    /// Returns a vertex data frame with the same set of vertices but new vertex features computed from k-hop neighbor
    /// aggregation
    ///
    /// # Arguments
    ///
    /// * `k` - The number of hops to traverse and aggregate
    /// * `edge_types` - The type of edges to traverse for each hop. If the vector is empty, all edge types are traversed
    ///  as if in a homogeneous graph. If only one edge type is given, it is used for all hops. If multiple edge types are
    ///  given, the number of edge types must be equal to the number of hops
    /// * `aggregator` - The aggregator to use for aggregating the neighbor features for each hop. If only one aggregator
    ///  is given, it is used for all hops. If multiple aggregators are given, the number of aggregators must be equal to
    ///  the number of hops
    /// * `output_col_name` - The name of the output column
    fn aggregate_k_hop_neighbors(
        &self,
        k: u32,
        edge_types: Vec<String>,
        aggregator: Vec<Aggregator>,
        output_col_name: String,
    ) -> Rc<dyn GraphBase>;

    /// Samples a fixed number of neighbors for each vertex in the data frame, and returns the vertices with their sampled
    /// neighbors as a graph
    ///
    /// # Arguments
    ///
    /// * `fanout` - The number of neighbors to sample for each vertex
    /// * `edge_type` - The type of edges to traverse. If None, all edge types are traversed as if in a homogeneous graph
    /// * `replace` - Whether to sample with replacement
    fn sample_neighbors(
        &self,
        fanout: u32,
        edge_type: Option<String>,
        replace: bool,
    ) -> Rc<dyn GraphBase>;

    /// Samples a fixed number of k-hop neighbors for each vertex in the data frame, and returns the vertices with their sampled
    /// neighbors as a graph
    ///
    /// # Arguments
    ///
    /// * `k` - The number of hops to traverse and aggregate
    /// * `fanouts` - The number of neighbors to sample for vertices in each hop. If only one fanout is given, it is used for all
    /// hops. If multiple fanouts are given, the number of fanouts must be equal to the number of hops
    /// * `edge_types` - The type of edges to traverse for each hop. If None, all edge types are traversed as if in a homogeneous
    /// graph. If only one edge type is given, it is used for all hops. If multiple edge types are given, the number of edge
    /// types must be equal to the number of hops
    /// * `replace` - Whether to sample with replacement
    fn sample_k_hop_neighbors(
        &self,
        k: u32,
        fanouts: Vec<u32>,
        edge_types: Option<Vec<String>>,
        replace: bool,
    ) -> Rc<dyn GraphBase>;
}

impl<T: GraphBase> GraphComputationOps for T {
    fn random_walk(
        &self,
        path: super::RandomWalkPath,
        prob: &str,
        restart_prob: Option<&Vec<f32>>,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn random_walk_edges(
        &self,
        path: super::RandomWalkPath,
        prob: &str,
        restart_prob: Option<&Vec<f32>>,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn aggregate_neighbors(
        &self,
        edge_type: Option<String>,
        aggregator: super::Aggregator,
        output_col_name: String,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn aggregate_k_hop_neighbors(
        &self,
        k: u32,
        edge_types: Vec<String>,
        aggregator: Vec<super::Aggregator>,
        output_col_name: String,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn sample_neighbors(
        &self,
        fanout: u32,
        edge_type: Option<String>,
        replace: bool,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn sample_k_hop_neighbors(
        &self,
        k: u32,
        fanouts: Vec<u32>,
        edge_types: Option<Vec<String>>,
        replace: bool,
    ) -> Rc<dyn GraphBase> {
        todo!()
    }
}

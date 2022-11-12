use super::{DataFrameSet, EdgeSelectGraph, Selector, VertexSelectGraph};
use crate::transformation::{
    CypherResultDataFrame, CypherResultGraph, CypherTransformation, DataFrame, DataIdT,
    DataTransformationContext, GraphBase, GraphComputationOps, InnerTransformationData,
    TransformationContext, TransformationData,
};
use crate::{
    EntityType, FeatureRegistry, FeatureView, Field, Graph, GraphDataset, ResourceId, ResourceOp,
    Topology, TopologyFeatureView, TopologyType,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

/// A single (large) graph containing one or multiple types of vertices.
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleGraph {
    context: DataTransformationContext,
    vertex_fvs: HashMap<String, (String, Vec<Field>)>,
    edge_fvs: HashMap<String, (String, Vec<Field>)>,
    vertex_entities: HashMap<String, ResourceId>,
    edge_entities: HashMap<String, ResourceId>,
    topology_type: Option<TopologyType>,
}

impl SingleGraph {
    pub async fn from(
        context: &Rc<RefCell<TransformationContext>>,
        meta: &GraphDataset,
        registry: &FeatureRegistry,
    ) -> Result<Rc<SingleGraph>, Box<dyn Error>> {
        if let GraphDataset::SingleGraphDataset {
            name: _,
            description: _,
            feature_views,
        } = meta
        {
            let id = context.as_ref().borrow_mut().new_data_id();
            let mut res = SingleGraph {
                context: DataTransformationContext {
                    id,
                    transformation_context: Rc::downgrade(context),
                },
                vertex_fvs: HashMap::new(),
                edge_fvs: HashMap::new(),
                vertex_entities: HashMap::new(),
                edge_entities: HashMap::new(),
                topology_type: None,
            };
            for fv in feature_views {
                match fv {
                    FeatureView::TableFeatureView(view) => {
                        let entity = registry.get_entity(&view.entity_id).await?;
                        let mut fields = Vec::new();
                        for id in &view.field_ids {
                            let field = registry.get_field(id).await?;
                            fields.push(field);
                        }
                        match &entity.entity_type {
                            EntityType::NodeEntity { tlabel } => {
                                // now treat entity name as vertex type
                                res.vertex_fvs
                                    .insert(tlabel.clone(), (view.name.clone(), fields));
                                res.vertex_entities
                                    .insert(tlabel.clone(), view.entity_id.clone());
                            }
                            EntityType::EdgeEntity { tlabel } => {
                                // now treat entity name as edge type
                                res.edge_fvs
                                    .insert(tlabel.clone(), (view.name.clone(), fields));
                                res.edge_entities
                                    .insert(tlabel.clone(), view.entity_id.clone());
                            }
                        };
                    }
                    FeatureView::TopologyFeatureView(view) => {
                        res.topology_type = Some(view.topology_type.clone());
                    }
                }
            }
            let res = Rc::new(res);
            context.as_ref().borrow_mut().add_data(&res);
            Ok(res)
        } else {
            panic!("GraphDataset is not SingleGraphDataset");
        }
    } // pub fn from

    // TODO(tatiana): make this function part of GraphBase trait?
    fn get_edge_entity_ids(&self) -> Vec<ResourceId> {
        self.edge_entities.iter().map(|e| e.1.clone()).collect()
    }
}

impl Graph {
    pub async fn transform(
        &self,
        context: &Rc<RefCell<TransformationContext>>,
        registry: &FeatureRegistry,
    ) -> Result<Rc<SingleGraph>, Box<dyn Error>> {
        let mut vertex_fvs = HashMap::new();
        let mut edge_fvs = HashMap::new();
        let mut vertex_entities = HashMap::new();
        let mut edge_entities = HashMap::new();
        for id in &self.entity_ids {
            let entity = registry.get_entity(id).await?;
            let view_name = format!("{}_ALL_FIELDS", &entity.name);
            // list all fields of each entity
            // TODO(tatiana): now getting all versions, but we should use the default/newest version?
            let fields = registry.get_entity_fields(&entity.name).await?;
            match &entity.entity_type {
                EntityType::NodeEntity { tlabel } => {
                    vertex_fvs.insert(tlabel.clone(), (view_name, fields));
                    vertex_entities.insert(tlabel.clone(), id.clone());
                }
                EntityType::EdgeEntity { tlabel } => {
                    edge_fvs.insert(tlabel.clone(), (view_name, fields));
                    edge_entities.insert(tlabel.clone(), id.clone());
                }
            }
        }

        let id = context.as_ref().borrow_mut().new_data_id();
        let res = Rc::new(SingleGraph {
            context: DataTransformationContext {
                id,
                transformation_context: Rc::downgrade(context),
            },
            vertex_fvs,
            edge_fvs,
            vertex_entities,
            edge_entities,
            topology_type: None,
        });
        context.as_ref().borrow_mut().add_data(&res);
        Ok(res)
    } // fn transform
}

#[typetag::serde]
impl TransformationData for SingleGraph {
    fn get_context(&self) -> &DataTransformationContext {
        &self.context
    }
}

impl GraphBase for SingleGraph {
    fn vertices(&self) -> Rc<dyn GraphBase> {
        let res = Rc::new(VertexSelectGraph {
            context: self.context.new_data_context(),
            graph: self.context.id,
            selector: Selector::DirectAccess { ltype: None },
            df: if self.vertex_fvs.len() == 1 {
                DataFrameSet::Homo(self.vertex_fvs.values().next().unwrap().clone())
            } else {
                DataFrameSet::Hetero(self.vertex_fvs.clone())
            },
        });
        self.context.register_data(&res);
        res
    }

    fn edges(&self) -> Rc<dyn GraphBase> {
        todo!()
    }

    fn vertices_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>> {
        if let Some(df) = self.vertex_fvs.get(t) {
            let res = Rc::new(VertexSelectGraph {
                context: self.context.new_data_context(),
                graph: self.context.id,
                selector: Selector::DirectAccess {
                    ltype: Some(t.to_string()),
                },
                df: DataFrameSet::Homo(df.clone()),
            });
            self.context.register_data(&res);
            Some(res)
        } else {
            None
        }
    }

    fn edges_by_type(&self, t: &str) -> Option<Rc<dyn GraphBase>> {
        todo!()
    }

    fn export_topology(&self, name: &str) -> Topology {
        let res = Topology {
            name: name.to_string(),
            transformation_id: Some(self.get_context().get_transformation_id()),
            topology_type: self.topology_type.clone(),
            edge_entity_ids: self.get_edge_entity_ids(),
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

#[tokio::test]
async fn demo() -> Result<(), Box<dyn Error>> {
    use crate::{EtcdStorage, FeatureStore};
    use etcd_rs::{Client, ClientConfig};
    let fs = FeatureStore {
        project: "Feature Store Demo".to_string(),
        registry: FeatureRegistry {
            storage: EtcdStorage {
                client: Client::connect(ClientConfig::new(["http://127.0.0.1:2379".into()]))
                    .await?,
            },
        },
    };
    let context = TransformationContext::new();
    let meta = GraphDataset::SingleGraphDataset {
        name: "graph".to_string(),
        description: Some("demo graph".to_string()),
        feature_views: vec![/* TODO(tatiana) */],
    };
    let graph = SingleGraph::from(&context, &meta, &fs.registry).await?;
    let person = graph.vertices_by_type("Person");
    if person.is_none() {
        // assert_matches!(person, Some(_));
        println!("No person vertex type");
    }

    let graph = graph.as_ref().cypher_to_graph("query_1");
    // let df = graph.cypher_to_dataframe("query_2");

    // add test for serialization and deserialization.
    let serialized = serde_json::to_string(&*context).unwrap();
    println!("serialized = {}", serialized);
    let tc_deser: Rc<RefCell<TransformationContext>> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized TransformationContext: {:?}", tc_deser);
    Ok(())
}

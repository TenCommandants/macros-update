use chrono::Utc;
use etcd_rs::{Client, ClientConfig};
use gfs::*;
use indoc::indoc;
use log::info;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Testing register workflow");

    let fs = FeatureStore {
        project: "Feature Store Demo".to_string(),
        registry: FeatureRegistry {
            storage: EtcdStorage {
                client: Client::connect(ClientConfig::new(["http://127.0.0.1:2379".into()]))
                    .await?,
            },
        },
    };

    let entity_1 = Entity {
        name: "node_1".to_string(),
        variant: None,
        entity_type: EntityType::NodeEntity {
            tlabel: "Node".to_string(),
        },
        primary_key: "node_1".to_string(),
        description: None,
        created_timestamp: None,
        last_updated_timestamp: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let field_1 = Field {
        name: "feature_1".to_string(),
        variant: None,
        value_type: FeatureValueType::Float,
        entity_id: entity_1.resource_id(),
        transformation_id: Some("t_1".to_string()),
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let fv_1 = TableFeatureView {
        name: "fv_1".to_string(),
        variant: None,
        online: true,
        description: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
        tags: HashMap::new(),
        owner: None,
        entity_id: entity_1.resource_id(),
        field_ids: vec![field_1.resource_id()],
    };

    fs.registry.register_resource(&entity_1.clone()).await?;
    fs.registry.register_resource(&field_1.clone()).await?;
    fs.registry.register_resource(&fv_1.clone()).await?;

    let get_entity = fs.registry.get_entity(&"Entity/node_1/".into()).await?;
    info!("Got entity: {:?}", get_entity);

    let get_field = fs.registry.get_field(&"Field/feature_1/".into()).await?;
    info!("Got field: {:?}", get_field);

    let get_table_feature_view = fs
        .registry
        .get_table_feature_view(&"TableFeatureView/fv_1/".into())
        .await?;
    info!("Got table feature view: {:?}", get_table_feature_view);

    let field_2 = Field {
        name: "feature_2".to_string(),
        variant: None,
        value_type: FeatureValueType::Float,
        entity_id: entity_1.resource_id(),
        transformation_id: Some("t_2".to_string()),
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let transformation_1 = Transformation {
        name: "tf_1".to_string(),
        variant: None,
        description: None,
        source_field_ids: vec![field_1.resource_id(), field_2.resource_id()],
        export_resources: vec![],
        dest_type: FeatureValueType::Float,
        transformation_type: TransformationType::CustomFunction,
        body: indoc! {"(feature_1, feature_2)-> {
                        return feature_1 + feature_2;
                    }"}
        .to_string(),
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let field_3 = Field {
        name: "sum_of_feature_1_and_feature_2".to_string(),
        variant: None,
        value_type: FeatureValueType::Float,
        entity_id: entity_1.resource_id(),
        transformation_id: Some(transformation_1.resource_id()),
        description: Some("Sum of feature 1 and feature 2".to_string()),
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let demo_graph = Graph {
        name: "demo_graph".to_string(),
        variant: None,
        description: None,
        entity_ids: vec![entity_1.resource_id()],
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    fs.registry
        .register_resource(&transformation_1.clone())
        .await?;
    fs.registry.register_resource(&field_3.clone()).await?;
    fs.registry.register_resource(&demo_graph.clone()).await?;

    let get_transformation = fs
        .registry
        .get_transformation(&"Transformation/tf_1/".into())
        .await?;

    let get_field_3 = fs
        .registry
        .get_field(&"Field/sum_of_feature_1_and_feature_2/".into())
        .await?;

    let get_demo_graph = fs.registry.get_graph(&demo_graph.resource_id()).await?;

    info!("Got transformation: {:?}", get_transformation);
    info!("Got field: {:?}", get_field_3);
    info!("Got graph: {:?}", get_demo_graph);

    Ok(())
}

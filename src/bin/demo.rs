use etcd_rs::{Client, ClientConfig};
use gfs::*;
use log::info;
use std::{collections::HashMap, error::Error};

// TODO(tatiana): consider atomicity of resource registry? e.g. register all resources in one atomic function
async fn register_source_resources(fs: &FeatureStore) -> Result<Graph, Box<dyn Error>> {
    // entity name: graph database name + type label
    let review = entity!("neo4j_review", None, "Review", "id");
    let reviewer = entity!("neo4j_reviewer", None, "Reviewer", "reviewerId");
    let product = entity!("neo4j_product", None, "Product", "asin");
    let category = entity!("neo4j_category", None, "Category", "id");
    let style = entity!("neo4j_style", None, "Style", "id");
    let brand = entity!("neo4j_brand", None, "Brand", "id");

    fs.registry
        .register_resources(&vec![
            &review, &reviewer, &product, &category, &style, &brand,
        ])
        .await?;

    let review_fields = Field::new_fields(
        vec![
            ("id", FeatureValueType::String),
            ("vote", FeatureValueType::Int),
            ("overall", FeatureValueType::Float),
            ("summary", FeatureValueType::String),
            ("reviewText", FeatureValueType::String),
            ("unixReviewTime", FeatureValueType::Time),
            ("verified", FeatureValueType::Boolean),
            ("numImages", FeatureValueType::Int),
        ],
        &review,
        None,
    );

    let product_fields = Field::new_fields(
        vec![
            ("asin", FeatureValueType::String),
            ("title", FeatureValueType::String),
            // TODO(tatiana): do we support array type? if so, how?
            ("description", FeatureValueType::String),
            ("price", FeatureValueType::Float),
            ("rank", FeatureValueType::String),
        ],
        &product,
        None,
    );

    let reviewer_fields = Field::new_fields(
        vec![("reviewerId", FeatureValueType::String)],
        &reviewer,
        None,
    );

    let category_fields = Field::new_fields(
        vec![
            ("id", FeatureValueType::String),
            ("name", FeatureValueType::String),
        ],
        &category,
        None,
    );

    let style_fields = Field::new_fields(
        vec![
            ("id", FeatureValueType::String),
            ("key", FeatureValueType::String),
        ],
        &style,
        None,
    );

    let brand_fields = Field::new_fields(
        vec![
            ("id", FeatureValueType::String),
            ("name", FeatureValueType::String),
        ],
        &brand,
        None,
    );

    fs.registry
        .register_resources(
            &review_fields
                .iter()
                .chain(reviewer_fields.iter())
                .chain(product_fields.iter())
                .chain(category_fields.iter())
                .chain(brand_fields.iter())
                .chain(style_fields.iter())
                .collect(),
        )
        .await?;

    let is_written_by =
        entity!("neo4j_isWrittenBy", None, "isWrittenBy", &review, &reviewer);
    let refers_to = entity!("neo4j_refersTo", None, "refersTo", &review, &style);
    let rates = entity!("neo4j_rates", None, "rates", &review, &product);
    let belongs_to =
        entity!("neo4j_belongsTo", None, "belongsto", &product, &category);
    let has_brand = entity!("neo4j_hasBrand", None, "hasBrand", &product, &brand);
    let also_view = entity!("neo4j_alsoView", None, "alsoView", &product, &product);
    let also_buy = entity!("neo4j_alsoBuy", None, "alsoBuy", &product, &product);
    let is_similar_to =
        entity!("neo4j_isSimilarTo", None, "isSimilarTo", &product, &product);

    fs.registry
        .register_resources(&vec![
            &is_written_by,
            &refers_to,
            &rates,
            &belongs_to,
            &has_brand,
            &also_view,
            &also_buy,
            &is_similar_to,
        ])
        .await?;

    let refers_to_value = Field {
        name: "value".to_string(),
        variant: None,
        value_type: FeatureValueType::String,
        entity_id: refers_to.resource_id(),
        transformation_id: None,
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    fs.registry.register_resource(&refers_to_value).await?;

    let graph = Graph::new(
        "neo4j",
        None,
        vec![
            &review,
            &reviewer,
            &product,
            &brand,
            &style,
            &category,
            &is_written_by,
            &refers_to,
            &rates,
            &belongs_to,
            &has_brand,
            &also_view,
            &also_buy,
            &is_similar_to,
        ],
    );

    fs.registry.register_resource(&graph).await?;
    Ok(graph)
}

async fn extract_features(
    fs: &FeatureStore,
    graph: &Graph,
) -> Result<(Vec<Topology>, Vec<Field>), Box<dyn Error>> {
    // TODO(tatiana): shall we put transformation context in the feature store instance?
    let tc = TransformationContext::new();
    // calling build_transformation is optional, which gives a named transformation. otherwise an anonymous transformation is created
    // tc.as_ref().borrow_mut().build_transformation("demo_topo_fields_trans", None);
    let transform_graph = graph.transform(&tc, &fs.registry).await?;
    let user_product_user = transform_graph.cypher_to_graph(
        "MATCH (p: Product)-[:belongsTo]->(cat: Category {name: \" Books\"}) MATCH (r1: Review)-[:rates]->(p)<-[:rates]-(r2: Review) MATCH (r1: Review)-[:isWrittenBy]->(u1: Reviewer) MATCH (r2: Review)-[:isWrittenBy]->(u2: Reviewer) RETURN u1, collect(u2)",
    ).export_topology("upu");
    let rating_counts = transform_graph
        .cypher_to_dataframe(
            "MATCH (r:Review)-[:isWrittenBy]->(u:Reviewer)
            WITH u, r.overall as rate, count(r) as cnt ORDER BY rate
            WITH u, sum(cnt) as total, collect(cnt) as cnts
            UNWIND cnts as number
            WITH u, number/total as ratio, number
            RETURN u.reviewerID as user, collect(number)+collect(ratio) as dims",
        )
        .select(
            vec!["user".to_string()]
                .into_iter()
                .chain((0..10).map(|i| format!("dims.str[{}]", i)))
                .collect(),
        )
        .export();

    finalize_transformation(
        fs,
        &tc,
        rating_counts.iter().collect(),
        vec![&user_product_user],
    )
    .await?;

    Ok((vec![user_product_user], rating_counts))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Fraud Detection Demo");

    let fs = FeatureStore {
        project: "Fraud Detection Graph Feature Store Demo".to_string(),
        registry: FeatureRegistry {
            storage: EtcdStorage {
                client: Client::connect(ClientConfig::new(["http://127.0.0.1:2379".into()]))
                    .await?,
            },
        },
    };

    let graph = register_source_resources(&fs).await?;

    let (topos, fields) = extract_features(&fs, &graph).await?;

    // debug print
    println!(
        "{:#?}",
        fs.registry
            .get_transformation(topos.first().unwrap().transformation_id.as_ref().unwrap())
            .await?
    );
    println!("extracted {} topos {} fields", topos.len(), fields.len());
    Ok(())
}

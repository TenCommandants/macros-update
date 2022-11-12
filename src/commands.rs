use neo4rs::*;
use rusqlite::Connection;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;

fn load_json(path: &str) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let json: Value = serde_json::from_reader(reader)?;
            Ok(json)
        }
        Err(e) => Err(e.into()),
    }
}

fn load_cypher(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut cypher = String::new();
            reader.read_to_string(&mut cypher)?;
            Ok(cypher)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn apply() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = "feature_store.json";
    let json = load_json(path)?;
    println!("{:#?}", json);

    match json["data_sources"][0]["type"].as_str().unwrap() {
        "cypher" => {
            println!("processing cypher source");
            let cypher = load_cypher(json["data_sources"][0]["path"].as_str().unwrap())?;
            let uri = json["gdb"]["uri"].as_str().unwrap();
            let user = json["gdb"]["user"].as_str().unwrap();
            let pass = json["gdb"]["password"].as_str().unwrap();

            let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());

            let txn = graph.start_txn().await.unwrap();
            txn.run(query(&cypher)).await.unwrap();
            txn.commit().await.unwrap();
        }
        _ => {
            println!("Unknown data source type");
        }
    }

    Ok(())
}

pub async fn clean() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Clean");
    let path = "feature_store.json";
    let json = load_json(path)?;
    // println!("{:#?}", json);
    let cypher = "MATCH (n) DETACH DELETE n";
    let uri = json["gdb"]["uri"].as_str().unwrap();
    let user = json["gdb"]["user"].as_str().unwrap();
    let pass = json["gdb"]["password"].as_str().unwrap();

    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());

    let txn = graph.start_txn().await.unwrap();
    txn.run(query(cypher)).await.unwrap();
    txn.commit().await.unwrap();

    Ok(())
}

pub async fn materialize() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = "feature_store.json";
    let json = load_json(path)?;

    let uri = json["gdb"]["uri"].as_str().unwrap();
    let user = json["gdb"]["user"].as_str().unwrap();
    let pass = json["gdb"]["password"].as_str().unwrap();

    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());

    let feature_view_json = load_json(json["feature_views"][0].as_str().unwrap())?;
    let q = format!(
        "MATCH (n:{}) RETURN n",
        feature_view_json["node_type"].as_str().unwrap()
    );
    let mut result = graph.execute(query(&q)).await.unwrap();

    let conn = Connection::open(json["online_store"][0]["path"].as_str().unwrap())?;
    conn.execute(
        format!(
            "DROP TABLE IF EXISTS {}",
            feature_view_json["name"].as_str().unwrap()
        )
        .as_str(),
        [],
    )?;
    conn.execute(
        format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY, 
                title TEXT,
                released INTEGER,
                tagline TEXT
            )",
            feature_view_json["name"].as_str().unwrap()
        )
        .as_str(),
        [],
    )?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("n").unwrap();
        // println!("{:#?}", node.get::<String>("title").unwrap());
        conn.execute(
            format!(
                "INSERT INTO {} (id, title, released, tagline) VALUES ({}, ?, {}, ?)",
                feature_view_json["name"].as_str().unwrap(),
                node.id(),
                node.get::<i64>("released").unwrap_or(0),
            )
            .as_str(),
            [
                node.get::<String>("title").unwrap_or_default(),
                node.get::<String>("tagline").unwrap_or_default(),
            ],
        )?;
    }

    Ok(())
}

#[tokio::test]
async fn test_neo4rs() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let uri = "localhost:7687";
    let user = "neo4j";
    let password = "neo4j";

    let graph = Arc::new(Graph::new(uri, user, password).await.unwrap());
    let mut result = graph
        .execute(query("EXPLAIN MATCH (n) RETURN n;"))
        .await
        .unwrap();

    while let Ok(Some(row)) = result.next().await {
        println!("{:#?}", row);
        println!("==================");
    }

    Ok(())
}

#[tokio::test]
async fn test_bolt_rs() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Acknowledgement to https://docs.rs/bolt-client/latest/bolt_client/

    use bolt_client::*;
    use bolt_proto::{message::*, version::V4_3, Message};
    use std::env;
    use tokio::io::BufStream;
    use tokio_util::compat::*;

    env::set_var("BOLT_TEST_ADDR", "localhost:7687");
    env::set_var("BOLT_TEST_USERNAME", "neo4j");
    env::set_var("BOLT_TEST_PASSWORD", "neo4j");

    // Let's say you have a type that implements AsyncRead + AsyncWrite. Here's one
    // provided by the `tokio-stream` feature of this library.
    let stream = Stream::connect(
        env::var("BOLT_TEST_ADDR")?,
        env::var("BOLT_TEST_DOMAIN").ok(),
    )
    .await?;
    let stream = BufStream::new(stream).compat();

    // Create a new connection to the server and perform a handshake to establish a
    // protocol version. This example demonstrates usage of the v4.3 or v4.2 protocol.
    let mut client = Client::new(stream, &[V4_3, 0, 0, 0]).await?;

    // Send a HELLO message with authentication details to the server to initialize
    // the session.
    let response: Message = client
        .hello(Metadata::from_iter(vec![
            ("user_agent", "bolt-rs/1.0"),
            ("scheme", "basic"),
            ("principal", &env::var("BOLT_TEST_USERNAME")?),
            ("credentials", &env::var("BOLT_TEST_PASSWORD")?),
        ]))
        .await?;
    assert!(Success::try_from(response).is_ok());

    // Submit a query for execution on the server
    let response = client
        .run("EXPLAIN MATCH (n:Reviewer)<-[:isWrittenBy]-(:Review)-[:rates]->(:Product)<-[:rates]-(:Review)-[:isWrittenBy]->(m:Reviewer) RETURN n,m LIMIT 10;", None, None)
        .await?;

    println!("{:#?}", &response);

    // Successful responses will include a SUCCESS message with related metadata
    // Consuming these messages is optional and will be skipped for the rest of the example
    assert!(Success::try_from(response).is_ok());

    // Use PULL to retrieve results of the query, organized into RECORD messages
    // We get a (Vec<Record>, Message) returned from a PULL
    let pull_meta = Metadata::from_iter(vec![("n", 1)]);
    let (records, response) = client.pull(Some(pull_meta.clone())).await?;

    println!("==================");
    println!("{:#?}", &response);
    println!("{:#?}", &records);

    let response = client
        .run("MATCH (n:Reviewer)<-[:isWrittenBy]-(:Review)-[:rates]->(:Product)<-[:rates]-(:Review)-[:isWrittenBy]->(m:Reviewer) RETURN n,m LIMIT 10;", None, None)
        .await?;

    println!("{:#?}", &response);
    assert!(Success::try_from(response).is_ok());

    let (records, response) = client.pull(Some(pull_meta.clone())).await?;

    println!("==================");
    println!("{:#?}", &response);
    println!("{:#?}", &records);

    // End the connection with the server
    client.goodbye().await?;

    Ok(())
}

use etcd_rs::{Client, KeyValueOp};
use std::error::Error;

// pub struct Resource {}
pub struct ResourceLookup {}
pub trait StorageProvider {
    fn get_resource_lookup(&self, resource: &ResourceLookup) -> Result<(), Box<dyn Error>>;
}
pub struct LocalStorageProvider {}

pub struct EtcdStorage {
    pub client: Client,
}

impl EtcdStorage {
    pub async fn put(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let _resp = self.client.put((key, value)).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let resp = self.client.get(key).await?;
        let value = resp.kvs.first().unwrap().value_str();
        Ok(value.to_string())
    }

    pub async fn get_all(&self, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let resp = self.client.get_by_prefix(key).await?;
        Ok(resp.kvs.iter().map(|e| e.value_str().to_string()).collect())
    }
}

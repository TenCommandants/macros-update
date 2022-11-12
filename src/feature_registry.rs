use log::info;

use crate::*;
use std::error::Error;

pub struct FeatureRegistry {
    pub storage: EtcdStorage,
}

impl FeatureRegistry {
    pub async fn register_resource(
        &self,
        resource: &impl ResourceOp,
    ) -> Result<(), Box<dyn Error>> {
        let key = resource.resource_id();
        let value = serde_json::to_string(&resource)?;
        info!("Registering resource: {} -> {}", &key, &value);
        self.storage.put(&key, &value).await?;
        Ok(())
    }

    pub async fn register_resources(
        &self,
        resources: &Vec<&impl ResourceOp>,
    ) -> Result<(), Box<dyn Error>> {
        for &resource in resources {
            self.register_resource(resource).await?;
        }
        Ok(())
    }

    // TODO(tatiana): TBD, provide range getter interface
    pub async fn get_entities(&self) -> Result<Vec<String>, Box<dyn Error>> {
        self.storage.get_all("Entity").await
    }

    pub async fn get_entity_fields(&self, entity_name: &str) -> Result<Vec<Field>, Box<dyn Error>> {
        let values: Result<Vec<Field>, serde_json::Error> = self
            .storage
            .get_all(&format!("Field/{}", entity_name))
            .await?
            .iter()
            .map(|jstr| serde_json::from_str::<Field>(jstr))
            .collect();
        Ok(values?)
    }

    pub async fn get_entity(&self, entity_id: &ResourceId) -> Result<Entity, Box<dyn Error>> {
        let value = self.storage.get(entity_id).await?;
        let entity = serde_json::from_str::<Entity>(&value)?;
        Ok(entity)
    }

    pub async fn get_field(&self, field_id: &ResourceId) -> Result<Field, Box<dyn Error>> {
        let value = self.storage.get(field_id).await?;
        let field = serde_json::from_str::<Field>(&value)?;
        Ok(field)
    }

    pub async fn get_table_feature_view(
        &self,
        table_feature_view_id: &ResourceId,
    ) -> Result<TableFeatureView, Box<dyn Error>> {
        let value = self.storage.get(table_feature_view_id).await?;
        let table_feature_view = serde_json::from_str::<TableFeatureView>(&value)?;
        Ok(table_feature_view)
    }

    pub async fn get_transformation(
        &self,
        transformation_id: &ResourceId,
    ) -> Result<Transformation, Box<dyn Error>> {
        let value = self.storage.get(transformation_id).await?;
        let transformation = serde_json::from_str::<Transformation>(&value)?;
        Ok(transformation)
    }

    pub async fn get_graph(&self, graph_id: &ResourceId) -> Result<Graph, Box<dyn Error>> {
        let value = self.storage.get(graph_id).await?;
        let graph = serde_json::from_str::<Graph>(&value)?;
        Ok(graph)
    }
}

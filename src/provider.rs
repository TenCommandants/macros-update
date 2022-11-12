use crate::*;
use std::error::Error;

pub trait InfraProvider {
    fn update_infra(
        &self,
        views_to_delete: Vec<FeatureView>,
        views_to_keep: Vec<FeatureView>,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

pub struct GDBProvider {
    pub graph_data_handler: GraphDataHandler,
    // pub data_sources: Vec<DataSource>,
}

impl GDBProvider {
    pub fn new(credentials: GraphDatabaseCredentials) -> Result<Self, Box<dyn Error>> {
        Ok(GDBProvider {
            graph_data_handler: GraphDataHandler { credentials },
        })
    }
}

impl InfraProvider for GDBProvider {
    fn update_infra(
        &self,
        views_to_delete: Vec<FeatureView>,
        views_to_keep: Vec<FeatureView>,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

pub struct GraphDatabaseCredentials {
    pub uri: String,
    pub user: String,
    pub pass: String,
}
pub struct GraphDataHandler {
    pub credentials: GraphDatabaseCredentials,
}

pub trait DataSourceIngestion {
    fn ingest_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>>;
    fn load_offline_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>>;
    fn subscribe_online_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>>;
}

impl DataSourceIngestion for GraphDataHandler {
    fn ingest_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>> {
        match data_source.data_source_type {
            DataSourceType::OfflineDataSourceType(_) => {
                self.load_offline_data_source(data_source)?;
            }
            DataSourceType::OnlineDataSourceType(_) => {
                self.subscribe_online_data_source(data_source)?;
            }
        }
        Ok(())
    }

    fn load_offline_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>> {
        match data_source.data_source_type {
            DataSourceType::OfflineDataSourceType(OfflineDataSourceType::CsvSource) => {
                unimplemented!()
            }
            DataSourceType::OfflineDataSourceType(OfflineDataSourceType::CypherSource) => {
                unimplemented!()
            }
            DataSourceType::OfflineDataSourceType(OfflineDataSourceType::ParquetSource) => {
                unimplemented!()
            }
            _ => todo!(),
        }
    }

    fn subscribe_online_data_source(&self, data_source: &DataSource) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

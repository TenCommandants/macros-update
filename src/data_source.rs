/// Data Sources
/// https://docs.featureform.com/getting-started/overview#source
/// - Data Sources
///     - Primary (transformation: None)
///     - Transformation (transformation: SourceTransformation)

pub struct DataSource {
    pub name: String,
    pub path: String,
    pub data_source_type: DataSourceType,
    pub transformation: Option<SourceTransformation>,
}

pub enum DataSourceType {
    OfflineDataSourceType(OfflineDataSourceType),
    OnlineDataSourceType(OnlineDataSourceType),
}

pub enum OfflineDataSourceType {
    CsvSource,
    CypherSource,
    ParquetSource,
}

pub enum OnlineDataSourceType {
    KafkaSource,
    PulsarSource,
}

/// from source to graph
pub enum SourceTransformation {}

// pub trait OfflineSourceIngestion {
//     fn ingest(&self, graph_data_handler: &GraphDataHandler) -> Result<(), Box<dyn Error>>;
// }

// impl OfflineSourceIngestion for OfflineDataSourceType {
//     fn ingest(&self, graph_data_handler: &GraphDataHandler) -> Result<(), Box<dyn Error>> {
//         match self {
//             OfflineDataSourceType::CsvSource => unimplemented!(),
//             OfflineDataSourceType::CypherSource => unimplemented!(),
//             OfflineDataSourceType::ParquetSource => unimplemented!(),
//         }
//     }
// }

// pub trait OnlineSourceSubscription {
//     fn subscribe(&self, graph_data_handler: &GraphDataHandler) -> Result<(), Box<dyn Error>>;
// }

// impl OnlineSourceSubscription for OnlineDataSourceType {
//     fn subscribe(&self, graph_data_handler: &GraphDataHandler) -> Result<(), Box<dyn Error>> {
//         unimplemented!()
//     }
// }

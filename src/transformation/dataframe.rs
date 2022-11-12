use super::{
    built_in_fns::Expression, DataIdT, DataTransformationContext, InnerTransformationData,
    TransformationContext, TransformationData,
};
use crate::{FeatureValueType, Field, ResourceOp, TableFeatureView};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    /// The origin DataFrame
    origin: DataIdT,
    /// The string expression used to compute the column from the origin
    expr: Option<String>,
    /// The expression encoder for evaluation
    encoder: Option<Expression>,
    /// The column value type
    value_type: FeatureValueType,
}

impl Column {
    pub fn new(origin: DataIdT, value_type: FeatureValueType) -> Self {
        Column {
            origin,
            expr: None,
            encoder: None,
            value_type,
        }
    }

    fn to_field(&self, name: &str, transformation_id: String) -> Field {
        Field {
            name: name.to_string(),
            variant: None,
            value_type: self.value_type.clone(),
            // FIXME(tatiana): how should we determine the entity? trace back input column? or let user assign by function parameter?
            entity_id: "Entity/entity_name/".to_string(),
            transformation_id: Some(transformation_id),
            description: None,
            tags: HashMap::new(),
            owners: Vec::new(),
        }
    }
}

/// Similar to pandas DataFrame or Spark DataFrame. A DataFrame instance can be initialized from a TableFeatureView or a Field.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFrame {
    pub(super) context: DataTransformationContext,
    pub(super) name: String,
    pub(super) schema: Vec<Rc<Column>>, // from meta or transformation
    pub(super) col_names: Vec<String>,  // from meta or transformation
    pub(super) col_by_names: HashMap<String, usize>,
}

impl DataFrame {
    pub fn from(context: &Rc<RefCell<TransformationContext>>, meta: &TableFeatureView) -> Rc<Self> {
        todo!()
        // DataFrame::new(context, meta.name, meta.field_ids)
    }

    pub fn export(&self) -> Vec<Field> {
        let res: Vec<Field> = self
            .schema
            .iter()
            .enumerate()
            .map(|(i, col)| col.to_field(&self.col_names[i], self.context.get_transformation_id()))
            .collect();
        res.iter().for_each(|e| {
            self.get_context()
                .export_resource(self.get_data_id(), e.resource_id());
        });
        res
    }

    pub fn from_field(context: &Rc<RefCell<TransformationContext>>, field: &Field) -> Rc<Self> {
        DataFrame::new(context, &field.name, vec![field])
    }

    pub fn new(
        context: &Rc<RefCell<TransformationContext>>,
        name: &str,
        schema: Vec<&Field>,
    ) -> Rc<Self> {
        let mut tc = context.as_ref().borrow_mut();
        let id = tc.new_data_id();
        let res = Rc::new(Self {
            context: DataTransformationContext {
                id,
                transformation_context: Rc::downgrade(context),
            },
            name: name.to_string(),
            schema: schema
                .iter()
                .map(|f| {
                    Rc::new(Column {
                        origin: id,
                        expr: Some(f.name.clone()),
                        encoder: None,
                        value_type: f.value_type.clone(),
                    })
                })
                .collect(),
            col_names: schema.iter().map(|f| f.name.clone()).collect(),
            col_by_names: schema
                .iter()
                .enumerate()
                .map(|(idx, f)| (f.name.clone(), idx))
                .collect(),
        });
        tc.add_data(&res);
        res
    }

    pub fn select(&self, expr: Vec<String>) -> Rc<Self> {
        let mut res = Self {
            context: self.context.new_data_context(),
            name: self.name.clone(),
            schema: Vec::new(),
            col_names: Vec::new(),
            col_by_names: HashMap::new(),
        };
        res.schema = expr
            .iter()
            .map(|e| {
                let encoder = Expression::new(e); // TODO(tatiana): need to parse the expression
                let mut col = Column {
                    origin: self.context.id,
                    expr: Some(e.to_string()),
                    encoder: None,
                    value_type: encoder.get_type(),
                };
                col.encoder = Some(encoder);
                Rc::new(col)
            })
            .collect();
        res.col_names = res
            .schema
            .iter()
            .enumerate()
            .map(|(idx, col)| {
                col.encoder
                    .as_ref()
                    .unwrap()
                    .get_col_name()
                    .unwrap_or(format!("col{}", idx))
            })
            .collect();
        res.col_by_names = res
            .col_names
            .iter()
            .enumerate()
            .map(|(idx, name)| (name.clone(), idx))
            .collect();
        let res = Rc::new(res);
        self.context.register_data(&res);
        res
    }

    pub fn with_column(&self, colname: &str, col: Rc<Column>) -> Rc<Self> {
        // TODO(tatiana): need to check the origin of the column must be compatible with this
        // Dataframe (i.e. same size, same indexing)
        let mut res = Self {
            context: self.context.new_data_context(),
            name: self.name.clone(),
            schema: self.schema.clone(),
            col_names: self.col_names.clone(),
            col_by_names: self.col_by_names.clone(),
        };
        res.schema.push(col);
        res.col_names.push(colname.to_string());
        res.col_by_names
            .insert(colname.to_string(), res.schema.len() - 1);
        let res = Rc::new(res);
        self.context.register_data(&res);
        res
    }

    pub fn col(&self, colname: &str) -> Option<Rc<Column>> {
        self.col_by_names
            .get(colname)
            .map(|entry| self.schema[*entry].clone())
    }
}

#[typetag::serde]
impl TransformationData for DataFrame {
    fn get_context(&self) -> &DataTransformationContext {
        &self.context
    }
}

#[test]
fn demo_use_dataframe() {
    use crate::{Entity, EntityType, FeatureValueType, ResourceOp};

    let entity = Entity {
        name: "node_1".to_string(),
        variant: None,
        entity_type: EntityType::NodeEntity {
            tlabel: "Product".to_string(),
        },
        primary_key: "node_1".to_string(),
        description: None,
        created_timestamp: None,
        last_updated_timestamp: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let col1 = Field {
        name: "feature_1".to_string(),
        variant: None,
        value_type: FeatureValueType::Int,
        entity_id: entity.resource_id(),
        transformation_id: Some("t_1".to_string()),
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let col2 = Field {
        name: "feature_2".to_string(),
        variant: None,
        value_type: FeatureValueType::Int,
        entity_id: entity.resource_id(),
        transformation_id: Some("t_1".to_string()),
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let col3 = Field {
        name: "feature_3".to_string(),
        variant: None,
        value_type: FeatureValueType::Int,
        entity_id: entity.resource_id(),
        transformation_id: Some("t_2".to_string()),
        description: None,
        tags: HashMap::new(),
        owners: Vec::new(),
    };

    let context = TransformationContext::new();
    let df = DataFrame::new(&context, "test", vec![&col1, &col2, &col3]);
    let df2 = df.select(vec![
        "col1 + col2 * avg(col3)".to_string(),
        "col1".to_string(),
    ]);
    let df3 = df.with_column("computed_col", df2.col("col0").unwrap());

    println!("{:#?}", df3);
    println!("{:#?}", context);

    // add test for serialization and deserialization.
    let serialized = serde_json::to_string(&*context).unwrap();
    println!("serialized = {}", serialized);
    let events: Rc<RefCell<TransformationContext>> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized TransformationContext: {:?}", events);
}

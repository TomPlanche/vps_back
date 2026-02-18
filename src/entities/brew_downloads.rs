use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "brew_downloads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project: String,
    pub version: String,
    pub platform: String,
    pub count: i32,
    #[sea_orm(created_at)]
    pub created_at: chrono::NaiveDateTime,
    #[sea_orm(updated_at)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

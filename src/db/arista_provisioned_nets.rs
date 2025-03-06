//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "arista_provisioned_nets")]
pub struct Model {
    pub tenant_id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub network_id: Option<String>,
    pub segmentation_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

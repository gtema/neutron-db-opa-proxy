//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ovn_hash_ring")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub node_uuid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_name: String,
    pub hostname: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

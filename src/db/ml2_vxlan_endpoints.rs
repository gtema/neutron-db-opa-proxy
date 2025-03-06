//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ml2_vxlan_endpoints")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub ip_address: String,
    pub udp_port: i32,
    #[sea_orm(unique)]
    pub host: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

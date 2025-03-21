//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::Direction;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "securitygroupdefaultrules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub standard_attr_id: i64,
    pub remote_group_id: Option<String>,
    pub remote_address_group_id: Option<String>,
    pub direction: Direction,
    pub ethertype: Option<String>,
    pub protocol: Option<String>,
    pub port_range_min: Option<i32>,
    pub port_range_max: Option<i32>,
    pub remote_ip_prefix: Option<String>,
    pub used_in_default_sg: i8,
    pub used_in_non_default_sg: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::standardattributes::Entity",
        from = "Column::StandardAttrId",
        to = "super::standardattributes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Standardattributes,
}

impl Related<super::standardattributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Standardattributes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

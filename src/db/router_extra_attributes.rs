//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "router_extra_attributes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub router_id: String,
    pub distributed: i8,
    pub service_router: i8,
    pub ha: i8,
    pub ha_vr_id: Option<i32>,
    pub availability_zone_hints: Option<String>,
    pub enable_default_route_ecmp: i8,
    pub enable_default_route_bfd: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::routers::Entity",
        from = "Column::RouterId",
        to = "super::routers::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Routers,
}

impl Related<super::routers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

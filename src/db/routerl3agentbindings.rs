//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "routerl3agentbindings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub router_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub l3_agent_id: String,
    pub binding_index: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::agents::Entity",
        from = "Column::L3AgentId",
        to = "super::agents::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Agents,
    #[sea_orm(
        belongs_to = "super::routers::Entity",
        from = "Column::RouterId",
        to = "super::routers::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Routers,
}

impl Related<super::agents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agents.def()
    }
}

impl Related<super::routers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

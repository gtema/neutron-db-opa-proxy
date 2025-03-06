//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "qos_network_policy_bindings")]
pub struct Model {
    pub policy_id: String,
    #[sea_orm(primary_key)]
    pub network_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::networks::Entity",
        from = "Column::NetworkId",
        to = "super::networks::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Networks,
    #[sea_orm(
        belongs_to = "super::qos_policies::Entity",
        from = "Column::PolicyId",
        to = "super::qos_policies::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    QosPolicies,
}

impl Related<super::networks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Networks.def()
    }
}

impl Related<super::qos_policies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QosPolicies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

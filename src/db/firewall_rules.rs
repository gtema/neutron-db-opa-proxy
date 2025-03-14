//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::Action;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "firewall_rules")]
pub struct Model {
    pub tenant_id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub firewall_policy_id: Option<String>,
    pub shared: Option<i8>,
    pub protocol: Option<String>,
    pub ip_version: i32,
    pub source_ip_address: Option<String>,
    pub destination_ip_address: Option<String>,
    pub source_port_range_min: Option<i32>,
    pub source_port_range_max: Option<i32>,
    pub destination_port_range_min: Option<i32>,
    pub destination_port_range_max: Option<i32>,
    pub action: Option<Action>,
    pub enabled: Option<i8>,
    pub position: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::firewall_policies::Entity",
        from = "Column::FirewallPolicyId",
        to = "super::firewall_policies::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FirewallPolicies,
}

impl Related<super::firewall_policies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FirewallPolicies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

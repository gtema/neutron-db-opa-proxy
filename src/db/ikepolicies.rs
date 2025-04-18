//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::AuthAlgorithm;
use super::sea_orm_active_enums::EncryptionAlgorithm;
use super::sea_orm_active_enums::IkeVersion;
use super::sea_orm_active_enums::LifetimeUnits;
use super::sea_orm_active_enums::Pfs;
use super::sea_orm_active_enums::Phase1NegotiationMode;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ikepolicies")]
pub struct Model {
    pub tenant_id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth_algorithm: AuthAlgorithm,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub phase1_negotiation_mode: Phase1NegotiationMode,
    pub lifetime_units: LifetimeUnits,
    pub lifetime_value: i32,
    pub ike_version: IkeVersion,
    pub pfs: Pfs,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ipsec_site_connections::Entity")]
    IpsecSiteConnections,
}

impl Related<super::ipsec_site_connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpsecSiteConnections.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

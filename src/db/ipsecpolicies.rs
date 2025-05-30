//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::AuthAlgorithm;
use super::sea_orm_active_enums::EncapsulationMode;
use super::sea_orm_active_enums::EncryptionAlgorithm;
use super::sea_orm_active_enums::LifetimeUnits;
use super::sea_orm_active_enums::Pfs;
use super::sea_orm_active_enums::TransformProtocol;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ipsecpolicies")]
pub struct Model {
    pub tenant_id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub transform_protocol: TransformProtocol,
    pub auth_algorithm: AuthAlgorithm,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub encapsulation_mode: EncapsulationMode,
    pub lifetime_units: LifetimeUnits,
    pub lifetime_value: i32,
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

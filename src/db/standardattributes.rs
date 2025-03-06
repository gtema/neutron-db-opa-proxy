//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "standardattributes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub resource_type: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub description: Option<String>,
    pub revision_number: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::address_groups::Entity")]
    AddressGroups,
    #[sea_orm(has_one = "super::floatingips::Entity")]
    Floatingips,
    #[sea_orm(has_one = "super::local_ips::Entity")]
    LocalIps,
    #[sea_orm(has_one = "super::logs::Entity")]
    Logs,
    #[sea_orm(has_one = "super::ndp_proxies::Entity")]
    NdpProxies,
    #[sea_orm(has_one = "super::network_segment_ranges::Entity")]
    NetworkSegmentRanges,
    #[sea_orm(has_one = "super::networks::Entity")]
    Networks,
    #[sea_orm(has_one = "super::networksegments::Entity")]
    Networksegments,
    #[sea_orm(has_many = "super::ovn_revision_numbers::Entity")]
    OvnRevisionNumbers,
    #[sea_orm(has_one = "super::portforwardings::Entity")]
    Portforwardings,
    #[sea_orm(has_one = "super::ports::Entity")]
    Ports,
    #[sea_orm(has_many = "super::provisioningblocks::Entity")]
    Provisioningblocks,
    #[sea_orm(has_one = "super::qos_policies::Entity")]
    QosPolicies,
    #[sea_orm(has_one = "super::routers::Entity")]
    Routers,
    #[sea_orm(has_one = "super::securitygroupdefaultrules::Entity")]
    Securitygroupdefaultrules,
    #[sea_orm(has_one = "super::securitygrouprules::Entity")]
    Securitygrouprules,
    #[sea_orm(has_one = "super::securitygroups::Entity")]
    Securitygroups,
    #[sea_orm(has_one = "super::subnetpools::Entity")]
    Subnetpools,
    #[sea_orm(has_one = "super::subnets::Entity")]
    Subnets,
    #[sea_orm(has_many = "super::tags::Entity")]
    Tags,
    #[sea_orm(has_one = "super::trunks::Entity")]
    Trunks,
}

impl Related<super::address_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddressGroups.def()
    }
}

impl Related<super::floatingips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Floatingips.def()
    }
}

impl Related<super::local_ips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LocalIps.def()
    }
}

impl Related<super::logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logs.def()
    }
}

impl Related<super::ndp_proxies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NdpProxies.def()
    }
}

impl Related<super::network_segment_ranges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NetworkSegmentRanges.def()
    }
}

impl Related<super::networks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Networks.def()
    }
}

impl Related<super::networksegments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Networksegments.def()
    }
}

impl Related<super::ovn_revision_numbers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OvnRevisionNumbers.def()
    }
}

impl Related<super::portforwardings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Portforwardings.def()
    }
}

impl Related<super::ports::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ports.def()
    }
}

impl Related<super::provisioningblocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Provisioningblocks.def()
    }
}

impl Related<super::qos_policies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QosPolicies.def()
    }
}

impl Related<super::routers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routers.def()
    }
}

impl Related<super::securitygroupdefaultrules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Securitygroupdefaultrules.def()
    }
}

impl Related<super::securitygrouprules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Securitygrouprules.def()
    }
}

impl Related<super::securitygroups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Securitygroups.def()
    }
}

impl Related<super::subnetpools::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subnetpools.def()
    }
}

impl Related<super::subnets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subnets.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tags.def()
    }
}

impl Related<super::trunks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Trunks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

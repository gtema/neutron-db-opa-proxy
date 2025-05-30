//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bgp_speakers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub local_as: i32,
    pub ip_version: i32,
    pub tenant_id: Option<String>,
    pub advertise_floating_ip_host_routes: i8,
    pub advertise_tenant_networks: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bgp_speaker_dragent_bindings::Entity")]
    BgpSpeakerDragentBindings,
    #[sea_orm(has_many = "super::bgp_speaker_network_bindings::Entity")]
    BgpSpeakerNetworkBindings,
    #[sea_orm(has_many = "super::bgp_speaker_peer_bindings::Entity")]
    BgpSpeakerPeerBindings,
}

impl Related<super::bgp_speaker_dragent_bindings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BgpSpeakerDragentBindings.def()
    }
}

impl Related<super::bgp_speaker_network_bindings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BgpSpeakerNetworkBindings.def()
    }
}

impl Related<super::bgp_speaker_peer_bindings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BgpSpeakerPeerBindings.def()
    }
}

impl Related<super::bgp_peers::Entity> for Entity {
    fn to() -> RelationDef {
        super::bgp_speaker_peer_bindings::Relation::BgpPeers.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::bgp_speaker_peer_bindings::Relation::BgpSpeakers
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

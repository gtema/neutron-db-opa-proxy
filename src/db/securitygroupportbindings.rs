//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "securitygroupportbindings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub port_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub security_group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ports::Entity",
        from = "Column::PortId",
        to = "super::ports::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Ports,
    #[sea_orm(
        belongs_to = "super::securitygroups::Entity",
        from = "Column::SecurityGroupId",
        to = "super::securitygroups::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Securitygroups,
}

impl Related<super::ports::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ports.def()
    }
}

impl Related<super::securitygroups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Securitygroups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

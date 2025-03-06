//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "serviceprofiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub description: Option<String>,
    pub driver: String,
    pub enabled: i8,
    pub metainfo: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::flavorserviceprofilebindings::Entity")]
    Flavorserviceprofilebindings,
}

impl Related<super::flavorserviceprofilebindings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flavorserviceprofilebindings.def()
    }
}

impl Related<super::flavors::Entity> for Entity {
    fn to() -> RelationDef {
        super::flavorserviceprofilebindings::Relation::Flavors.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::flavorserviceprofilebindings::Relation::Serviceprofiles
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

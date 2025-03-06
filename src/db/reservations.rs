//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "reservations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: Option<String>,
    pub expiration: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::resourcedeltas::Entity")]
    Resourcedeltas,
}

impl Related<super::resourcedeltas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resourcedeltas.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

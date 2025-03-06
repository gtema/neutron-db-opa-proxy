//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "logs")]
pub struct Model {
    pub project_id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub standard_attr_id: i64,
    pub name: Option<String>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub target_id: Option<String>,
    pub event: String,
    pub enabled: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::standardattributes::Entity",
        from = "Column::StandardAttrId",
        to = "super::standardattributes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Standardattributes,
}

impl Related<super::standardattributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Standardattributes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

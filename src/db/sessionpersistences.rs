//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::Type;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sessionpersistences")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub vip_id: String,
    pub r#type: Type,
    pub cookie_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::vips::Entity",
        from = "Column::VipId",
        to = "super::vips::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Vips,
}

impl Related<super::vips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Vips.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "nexthops")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub rule_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub nexthop: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::routerrules::Entity",
        from = "Column::RuleId",
        to = "super::routerrules::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Routerrules,
}

impl Related<super::routerrules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routerrules.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

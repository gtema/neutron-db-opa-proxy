//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use super::sea_orm_active_enums::Direction;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "meteringlabelrules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub direction: Option<Direction>,
    pub remote_ip_prefix: Option<String>,
    pub metering_label_id: String,
    pub excluded: Option<i8>,
    pub source_ip_prefix: Option<String>,
    pub destination_ip_prefix: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::meteringlabels::Entity",
        from = "Column::MeteringLabelId",
        to = "super::meteringlabels::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Meteringlabels,
}

impl Related<super::meteringlabels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Meteringlabels.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

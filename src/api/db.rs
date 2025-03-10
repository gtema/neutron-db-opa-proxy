use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;
use sea_orm::DatabaseConnection;
use sea_orm::entity::*;
use sea_orm::query::*;

use crate::api::error::ApiError;
use crate::api::types::*;
use crate::db::{
    floatingips as db_floating_ip, networkrbacs as db_networkrbacs, networks as db_network,
    prelude::{
        Floatingips as DbFloatingIp, Networkrbacs as DbNetworkRbacs, Networks as DbNetwork,
        Securitygroups as DbSecurityGroups,
    },
    securitygroups as db_security_group,
};

#[async_trait]
pub trait Neutron {
    async fn get_floating_ip<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<FloatingIP>, ApiError>;

    async fn get_network<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<Network>, ApiError>;

    async fn get_security_group<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<SecurityGroup>, ApiError>;
}

#[derive(Clone, Default)]
pub(crate) struct DbWorker {}

#[async_trait]
impl Neutron for DbWorker {
    async fn get_floating_ip<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<FloatingIP>, ApiError> {
        let select = DbFloatingIp::find_by_id(id);
        let entry: Option<db_floating_ip::Model> = select.one(db).await?;
        Ok(entry.map(FloatingIP::from))
    }

    async fn get_network<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<Network>, ApiError> {
        let select = DbNetwork::find_by_id(id);
        let entry: Option<db_network::Model> = select.one(db).await?;
        match &entry {
            Some(net) => {
                let mut n = Network::from(net);
                let rbac: Option<db_networkrbacs::Model> = DbNetworkRbacs::find()
                    .filter(db_networkrbacs::Column::Action.eq("access_as_shared"))
                    .filter(db_networkrbacs::Column::ObjectId.eq(id))
                    .filter(db_networkrbacs::Column::TargetProject.eq("*"))
                    .one(db)
                    .await?;
                if rbac.is_some() {
                    n.shared = true;
                }
                Ok(Some(n))
            }
            None => Ok(None),
        }
    }

    async fn get_security_group<'a>(
        &self,
        db: &DatabaseConnection,
        id: &'a str,
    ) -> Result<Option<SecurityGroup>, ApiError> {
        let select = DbSecurityGroups::find_by_id(id);
        let entry: Option<db_security_group::Model> = select.one(db).await?;
        Ok(entry.map(SecurityGroup::from))
    }
}

#[cfg(test)]
mock! {
    #[derive(Clone)]
    pub DbWorker {}

    #[async_trait]
    impl Neutron for DbWorker {
        async fn get_floating_ip<'a>(&self, db: &DatabaseConnection, id: &'a str) -> Result<Option<FloatingIP>, ApiError>;
        async fn get_network<'a>(&self, db: &DatabaseConnection, id: &'a str) -> Result<Option<Network>, ApiError>;
        async fn get_security_group<'a>(&self, db: &DatabaseConnection, id: &'a str) -> Result<Option<SecurityGroup>, ApiError>;
    }

    impl Clone for DbWorker {
        fn clone(&self) -> Self;
    }
}

impl From<db_floating_ip::Model> for FloatingIP {
    fn from(value: db_floating_ip::Model) -> Self {
        Self {
            id: value.id.clone(),
            floating_ip_address: value.floating_ip_address.clone(),
            project_id: value.project_id.clone(),
            tenant_id: value.project_id.clone(),
            status: value.status.clone(),
        }
    }
}

impl From<&db_network::Model> for Network {
    fn from(value: &db_network::Model) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            project_id: value.project_id.clone(),
            tenant_id: value.project_id.clone(),
            status: value.status.clone(),
            shared: false,
        }
    }
}

impl From<db_security_group::Model> for SecurityGroup {
    fn from(value: db_security_group::Model) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            project_id: value.project_id.clone(),
            tenant_id: value.project_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction};

    use super::*;
    use crate::db::{floatingips, networkrbacs, networks, securitygroups};

    #[tokio::test]
    async fn test_get_fip() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![floatingips::Model {
                id: "id".into(),
                floating_ip_address: "1.2.3.4".into(),
                floating_network_id: "fake".into(),
                floating_port_id: "fake".into(),
                fixed_port_id: None,
                fixed_ip_address: None,
                router_id: None,
                last_known_router_id: None,
                status: Some("broken".into()),
                standard_attr_id: 0,
                project_id: Some("project".into()),
            }]])
            .into_connection();

        let _res = DbWorker {}.get_floating_ip(&db, "id").await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "floatingips"."project_id", "floatingips"."id", "floatingips"."floating_ip_address", "floatingips"."floating_network_id", "floatingips"."floating_port_id", "floatingips"."fixed_port_id", "floatingips"."fixed_ip_address", "floatingips"."router_id", "floatingips"."last_known_router_id", "floatingips"."status", "floatingips"."standard_attr_id" FROM "floatingips" WHERE "floatingips"."id" = $1 LIMIT $2"#,
                ["id".into(), 1u64.into()]
            ),]
        );
    }

    #[tokio::test]
    async fn test_get_network() {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![networks::Model {
                id: "id".into(),
                name: Some("name".into()),
                status: Some("broken".into()),
                admin_state_up: Some(0),
                project_id: "project".into(),
                standard_attr_id: 0,
            }]])
            .append_query_results([vec![networkrbacs::Model {
                id: "rbacid".into(),
                object_id: "network_id".into(),
                project_id: Some("project".into()),
                target_project: "*".into(),
                action: "access_as_shared".into(),
            }]])
            .into_connection();

        let _res = DbWorker {}.get_network(&db, "id").await;

        assert_eq!(
            db.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "networks"."project_id", "networks"."id", "networks"."name", "networks"."status", "networks"."admin_state_up", "networks"."standard_attr_id" FROM "networks" WHERE "networks"."id" = $1 LIMIT $2"#,
                    ["id".into(), 1u64.into()]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "networkrbacs"."id", "networkrbacs"."object_id", "networkrbacs"."project_id", "networkrbacs"."target_project", "networkrbacs"."action" FROM "networkrbacs" WHERE "networkrbacs"."action" = $1 AND "networkrbacs"."object_id" = $2 AND "networkrbacs"."target_project" = $3 LIMIT $4"#,
                    [
                        "access_as_shared".into(),
                        "id".into(),
                        "*".into(),
                        1u64.into()
                    ]
                ),
            ]
        );
    }

    #[tokio::test]
    async fn test_get_sg() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![securitygroups::Model {
                id: "id".into(),
                name: Some("default".into()),
                standard_attr_id: 0,
                stateful: 0,
                project_id: Some("project".into()),
            }]])
            .into_connection();

        let _res = DbWorker {}.get_security_group(&db, "id").await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "securitygroups"."project_id", "securitygroups"."id", "securitygroups"."name", "securitygroups"."standard_attr_id", "securitygroups"."stateful" FROM "securitygroups" WHERE "securitygroups"."id" = $1 LIMIT $2"#,
                ["id".into(), 1u64.into()]
            ),]
        );
    }
}

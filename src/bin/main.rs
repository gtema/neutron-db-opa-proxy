use axum::http::{Request, header};
use clap::Parser;
use color_eyre::eyre::{Report, Result};
use sea_orm::ConnectOptions;
use sea_orm::Database;
use std::io;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit, ServiceBuilderExt,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, info_span};
use tracing_subscriber::{filter::LevelFilter, prelude::*};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use neutron_db_opa_proxy::{Config, Service, api};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the keystone config file
    #[arg(short, long)]
    config: String,

    /// Verbosity level. Repeat to increase level.
    #[arg(short, long, global=true, action = clap::ArgAction::Count, display_order = 920)]
    pub verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    let args = Args::parse();

    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(match args.verbose {
            0 => LevelFilter::WARN,
            1 => LevelFilter::INFO,
            2 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        });

    // build the tracing registry
    tracing_subscriber::registry().with(log_layer).init();

    let cfg = Config::new(args.config.into())?;
    let db_url = cfg.database.get_connection();
    let mut opt = ConnectOptions::new(db_url.to_owned());
    if args.verbose < 2 {
        opt.sqlx_logging(false);
    }

    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");

    let shared_state = Arc::new(Service::new(cfg, conn).unwrap());

    let (router, api) = OpenApiRouter::with_openapi(api::ApiDoc::openapi())
        .merge(api::openapi_router())
        .split_for_parts();

    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .layer(
            TraceLayer::new_for_http()
                //.make_span_with(DefaultMakeSpan::new().include_headers(true))
                .make_span_with(|request: &Request<_>| {
                    info_span!(
                        "request",
                        method = ?request.method(),
                        some_other_field = tracing::field::Empty,
                        uri = ?request.uri().path(),
                        x_request_id = ?request.headers().get("x-openstack-request-id")
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        // Compress responses
        .compression()
        .sensitive_response_headers(sensitive_headers);

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .layer(middleware)
        .with_state(shared_state.clone());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    Ok(axum::serve(listener, app.into_make_service()).await?)
}

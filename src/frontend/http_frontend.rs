use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle};
use tower::Service;

use crate::{core::document_service::DocumentService, error::Error};

pub mod handlers;

#[derive(Clone)]
pub struct HttpFrontend {
    port: u16,
    pub document_service: Arc<dyn DocumentService>,
}

impl HttpFrontend {
    pub fn new(port: u16, document_service: Arc<dyn DocumentService>) -> Self {
        Self { port, document_service }
    }
}

pub async fn run(frontend: HttpFrontend, subsys: SubsystemHandle) -> Result<(), Error> {
    let port = frontend.port;

    tracing::trace!("Starting frontend...");

    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().map_err(|_| Error::BadPort(port))?;
    let routes = frontend.get_routes();

    tracing::info!("Listening on port {}", port);

    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (socket, remote_addr) = tokio::select! {
            _ = subsys.on_shutdown_requested() => {
                break;
            }

            result = listener.accept() => {
                result.unwrap()
            }
        };

        tracing::debug!("connection {} accepted", remote_addr);
        let tower_service = routes.clone();
        let name = format!("handler-{remote_addr}");
        subsys.start(SubsystemBuilder::new(name, move |h| handler(socket, remote_addr, tower_service, h)));
    }

    tracing::trace!("Frontend shutting down");
    Ok(())
}

pub async fn handler(socket: TcpStream, remote_addr: SocketAddr, tower_service: Router<()>, subsys: SubsystemHandle) -> Result<(), Error> {
    let socket = TokioIo::new(socket);
    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| tower_service.clone().call(request));
    let conn = hyper::server::conn::http1::Builder::new().serve_connection(socket, hyper_service);
    let mut conn = std::pin::pin!(conn);

    tokio::select! {
        result = conn.as_mut() => {
            if let Err(err) = result {
                tracing::warn!("Failed to serve connection{}: {:#}", remote_addr, err);
            }
        }

        _ = subsys.on_shutdown_requested() => {
            tracing::debug!("signal received, starting graceful shutdown");
        }
    }

    tracing::debug!("Connection {} closed", remote_addr);
    Ok(())
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self)).into_response()
    }
}

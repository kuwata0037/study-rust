use std::net::{Ipv4Addr, SocketAddr};

pub mod pb {
    pub mod schema {
        tonic::include_proto!("_include");
    }
}

pub mod service {
    pub mod example {
        pub mod v1 {
            use crate::pb::schema::example::v1::{
                greeter_service_server::{GreeterService, GreeterServiceServer},
                SayHelloRequest, SayHelloResponse,
            };

            #[derive(Debug)]
            pub struct GreeterServiceImpl {}

            impl GreeterServiceImpl {
                pub fn server() -> GreeterServiceServer<Self> {
                    GreeterServiceServer::new(Self {})
                }
            }

            #[tonic::async_trait]
            impl GreeterService for GreeterServiceImpl {
                #[tracing::instrument(skip(self))]
                async fn say_hello(
                    &self,
                    request: tonic::Request<SayHelloRequest>,
                ) -> std::result::Result<tonic::Response<SayHelloResponse>, tonic::Status>
                {
                    let request = request.into_inner();

                    let response = SayHelloResponse {
                        message: format!("Hello, {}", request.name),
                    };

                    Ok(tonic::Response::new(response))
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt().pretty().try_init()?;

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 50051));

    tracing::info!(%addr, "start server");
    tonic::transport::Server::builder()
        .add_service(service::example::v1::GreeterServiceImpl::server())
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received");
}

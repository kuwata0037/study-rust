use tracing_subscriber::layer::SubscriberExt;

#[derive(Debug)]
struct Parameter {
    id: u32,
    name: String,
}

#[tracing::instrument]
fn service(param: Parameter) {
    tracing::info!(?param, param.id, param.name, "call service");
    use_case(param.id + 1);
}

#[tracing::instrument]
fn use_case(id: u32) {
    tracing::warn!("call use case");
    infrastructure::repository((id + 1).to_string().as_str());
}

mod infrastructure {
    #[tracing::instrument(ret)]
    pub(super) fn repository(id: &str) {
        tracing::error!("call repository");
    }
}

fn main() {
    tracing::subscriber::with_default(tracing_subscriber::FmtSubscriber::new(), || {
        tracing::info!(format = "default", "start tracing subscriber");
        service(Parameter {
            id: 1,
            name: "default".to_string(),
        });
        println!();
    });

    tracing::subscriber::with_default(
        tracing_subscriber::FmtSubscriber::builder()
            .compact()
            .finish(),
        || {
            tracing::info!(format = "compact", "start tracing subscriber");
            service(Parameter {
                id: 10,
                name: "compact".to_string(),
            });
            println!();
        },
    );

    tracing::subscriber::with_default(
        tracing_subscriber::fmt::SubscriberBuilder::default()
            .pretty()
            .finish(),
        || {
            tracing::info!(format = "pretty", "start tracing subscriber");
            service(Parameter {
                id: 100,
                name: "pretty".to_string(),
            });
            println!();
        },
    );

    tracing::subscriber::with_default(tracing_subscriber::fmt().json().finish(), || {
        tracing::info!(format = "json", "start tracing subscriber");
        service(Parameter {
            id: 1000,
            name: "json".to_string(),
        });
        println!();
    });

    tracing::subscriber::with_default(
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
            .with(tracing_subscriber::EnvFilter::from_default_env()),
        || {
            tracing::info!(format = "flatten json", "start tracing subscriber");
            service(Parameter {
                id: 2000,
                name: "flatten json".to_string(),
            });
            println!();
        },
    );
}

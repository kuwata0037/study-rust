use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod yak_shave {
    // ref: https://github.com/tokio-rs/tracing/tree/v0.1.x?tab=readme-ov-file#in-libraries

    #[tracing::instrument]
    fn shave(yak: usize) -> Result<(), Box<dyn std::error::Error + 'static>> {
        tracing::debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");

        if yak == 3 {
            tracing::warn!("cloud not locate yak!");
            return Err(Box::new(std::io::Error::other("shaving yak failed")));
        } else {
            tracing::debug!("yak shaved successfully");
        }

        Ok(())
    }

    #[tracing::instrument]
    pub fn shave_all(yaks: usize) -> usize {
        tracing::info!("shaving yaks");

        let mut yaks_shaved = 0;
        for yak in 1..=yaks {
            let res = shave(yak);
            tracing::debug!(yak, shaved = res.is_ok());

            if let Err(ref error) = res {
                tracing::error!(yak, error, "failed to shave yak!");
            } else {
                yaks_shaved += 1;
            }
            tracing::debug!(yak, yaks_shaved);
        }

        yaks_shaved
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("OTEL_SERVICE_NAME", env!("CARGO_PKG_NAME"));

    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    let format_layer = tracing_subscriber::fmt::layer();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("failed to initialize tracer");
    let otel_layer = tracing_opentelemetry::OpenTelemetryLayer::new(tracer);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(format_layer)
        .with(otel_layer)
        .init();

    let number_of_yaks = 3;
    tracing::info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    tracing::info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed"
    );

    opentelemetry::global::shutdown_tracer_provider();
}

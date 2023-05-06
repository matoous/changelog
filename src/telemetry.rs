use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::{
    export::trace::stdout,
    trace::{self, Sampler},
    Resource,
};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource;
use tracing_subscriber::Registry;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_telemetry(debug: bool) {
    let tracer = if debug {
        stdout::new_pipeline().install_simple()
    } else {
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::ParentBased(Box::new(Sampler::AlwaysOn)))
                    .with_resource(Resource::new(vec![
                        KeyValue::new(resource::SERVICE_NAME.to_string(), "feature-togglers"),
                        KeyValue::new(resource::SERVICE_NAMESPACE.to_string(), "platform"),
                        KeyValue::new(resource::SERVICE_VERSION.to_string(), env!("VERSION")),
                        KeyValue::new("sumup.org.tribe", "platform"),
                        KeyValue::new("sumup.org.comm.slack", "#team-dev"),
                    ])),
            )
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_env()
                    .with_protocol(opentelemetry_otlp::Protocol::Grpc),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Error - Failed to create tracer.")
    };

    global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_filter(EnvFilter::from_default_env().add_directive(tracing::Level::ERROR.into()));

    Registry::default()
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into())
                // don't trace the otel exporter itself
                .add_directive("h2=error".parse().unwrap()),
        )
        .with(telemetry)
        .with(formatting_layer)
        .init()
}

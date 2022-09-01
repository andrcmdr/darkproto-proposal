use tracing_subscriber::EnvFilter;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Initialize logging
pub(crate) fn init_logging() {
    // Filters can be customized through RUST_LOG environment variable via CLI
    let mut env_filter = EnvFilter::new(
        "darkproto=info,darkproto_run_time=info",
    );

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        if !rust_log.is_empty() {
            for directive in rust_log.split(',').filter_map(|s| match s.parse() {
                Ok(directive) => Some(directive),
                Err(err) => {
                    eprintln!("Ignoring directive `{}`: {}", s, err);
                    None
                }
            }) {
                env_filter = env_filter.add_directive(directive);
            }
        }
    }

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stdout)
        .init();
}

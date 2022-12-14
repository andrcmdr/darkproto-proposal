use tracing::error;

use tracing_subscriber::EnvFilter;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Initialize logging
pub fn init_logging() {
    // Filters can be customized through RUST_LOG environment variable via CLI
    let mut env_filter = EnvFilter::new(
        "darkproto=info,darkproto_runtime=info",
    );

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        if !rust_log.is_empty() {
            for directive in rust_log.split(',').filter_map(|s| match s.parse() {
                Ok(directive) => Some(directive),
                Err(err) => {
                    error!(target: "darkproto_runtime", "`{}` is not a valid directive. Ignoring directive `{}`. Failed while parsing a directive `{}` with error: {:?}", s, s, s, err);
                    eprintln!("`{}` is not a valid directive. Ignoring directive `{}`. Failed while parsing a directive `{}` with error: {:?}", s, s, s, err);
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

pub trait ExitGracefully<T, E> {
    fn ok_or_exit(self, message: &str) -> T;
}

impl<T, E: std::fmt::Debug> ExitGracefully<T, E> for Result<T, E> {
    fn ok_or_exit(self, message: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                error!(target: "darkproto_runtime", "{:?}: {}", err, message);
                std::process::exit(1);
            }
        }
    }
}

#[macro_export]
macro_rules! create_app {
    () => {
        App::new("DarkProto Enclave Streaming")
            .about("DarkBlock's DarkProto streaming app with execution of sensitive encryption/decryption/re-encryption part inside the Intel SGX enclave (based on AWS Nitro Enclaves).")
            .setting(AppSettings::ArgRequiredElseHelp)
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::with_name("config")
                .short('c')
                .long("config")
                .help("Configuration settings")
                .takes_value(true)
                .required(true),
            )
            .subcommand(
                SubCommand::with_name("server")
                    .about("Listen on a given port number.")
                    .arg(
                        Arg::with_name("port")
                            .long("port")
                            .help("receiving port number")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("client")
                    .about("Connect to a given CID and port number.")
                    .arg(
                        Arg::with_name("cid")
                            .long("cid")
                            .help("Context Identifier (CID)")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("port")
                            .long("port")
                            .help("transmitting port number")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("enclave-mode")
                    .about("Enclave mode: listen on a given port, receive data (from host), then connect to a given CID and port and stream data (to host).")
                    .arg(
                        Arg::with_name("rx-port")
                            .long("rx-port")
                            .help("receiving port number")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("tx-port")
                            .long("tx-port")
                            .help("transmitting port number")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("cid")
                            .long("cid")
                            .help("Context Identifier (CID)")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("cipher")
                        .long("cipher")
                        .help("Set cipher/cryptosystem for encryption/re-encryption [ chacha20 | xchacha20 | aes256-ctr | aes256-ofb | aes256-cfb | aes256-cfb8 | BLS-threshold | BLS-DKG-threshold-transform ]")
                        .takes_value(true)
                        .required(false),
                    )
                    .arg(
                        Arg::with_name("re-enc")
                        .long("re-enc")
                        .help("Make re-encryption with re-encryption key from `config.toml` file")
                        .takes_value(false)
                        .required(false),
                    ),

            )
            .subcommand(
                SubCommand::with_name("host-mode")
                    .about("Host mode: connect to a given CID and port, stream data (to the enclave), and listen on a given port to receive data (from inside the enclave).")
                    .arg(
                        Arg::with_name("cid")
                            .long("cid")
                            .help("Context Identifier (CID)")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("tx-port")
                            .long("tx-port")
                            .help("transmitting port number")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("rx-port")
                            .long("rx-port")
                            .help("receiving port number")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("re-enc")
                        .long("re-enc")
                        .help("Make re-encryption with re-encryption key from `config.toml` file")
                        .takes_value(false)
                        .required(false),
                    ),
            )
        };
}

use clap::{App, AppSettings, Arg, SubCommand};

/// For logging initialization
use darkproto_enclave_streaming::cli::init_logging;

use darkproto_enclave_streaming::create_app;
use darkproto_enclave_streaming::cli_parser::{ClientArgs, ServerArgs};
use darkproto_enclave_streaming::config::AppConfig;
use darkproto_enclave_streaming::{client, server};

fn main() {
    // Initialize logging
    init_logging();

    let app = create_app!();
    let args = app.get_matches();

    let default_config_path = "./.config/config.toml".to_string();
    let config_path = args
        .get_one("config")
        .unwrap_or(&default_config_path);

    let raw_config_string = std::fs::read_to_string(config_path).expect("Missing `config.toml` file.");
    let _app_config: AppConfig = toml::from_str(raw_config_string.as_str()).expect("Failed to parse `config.toml` file.");

    match args.subcommand() {
        Some(("server", args)) => {
            let server_args = ServerArgs::new_with(args).unwrap();
            server(server_args).unwrap();
        }
        Some(("client", args)) => {
            let client_args = ClientArgs::new_with(args).unwrap();
            client(client_args).unwrap();
        }
        Some(_) | None => ()
    }
}

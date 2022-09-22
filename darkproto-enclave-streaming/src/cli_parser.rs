use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct ServerArgs {
    pub port: u32,
}

impl ServerArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ServerArgs {
            port: parse_arg("port", args)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientArgs {
    pub cid: u32,
    pub port: u32,
}

impl ClientArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ClientArgs {
            cid: parse_arg("cid", args)?,
            port: parse_arg("port", args)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RxTxArgs {
    pub rx_port: u32,
    pub tx_port: u32,
    pub cid: u32,
}

impl RxTxArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(RxTxArgs {
            rx_port: parse_arg("rx-port", args)?,
            tx_port: parse_arg("tx-port", args)?,
            cid: parse_arg("cid", args)?,
        })
    }
}

fn parse_arg(flag: &str, args: &ArgMatches) -> Result<u32, String> {
    let arg = args
        .get_one::<String>(flag)
        .ok_or(format!("Could not find --{:?} argument", flag))?;
    arg.parse()
        .map_err(|_err| format!("--{:?} is not a number", flag))
}

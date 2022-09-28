use clap::ArgMatches;

use tracing::error;

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
    pub re_enc: bool,
}

impl RxTxArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(RxTxArgs {
            rx_port: parse_arg("rx-port", args)?,
            tx_port: parse_arg("tx-port", args)?,
            cid: parse_arg("cid", args)?,
            re_enc: arg_presence("re-enc", args)?,
        })
    }
}

fn parse_arg(flag: &str, args: &ArgMatches) -> Result<u32, String> {
    let arg = args
        .get_one::<String>(flag)
        .ok_or({
            error!(target: "darkproto", "Could not find --{} argument", flag);
            format!("Could not find --{} argument", flag)
        })?;
    arg.parse()
        .map_err(|_err| {
            error!(target: "darkproto", "--{} is not containing a number", flag);
            format!("--{} is not containing a number", flag)
        })
}

fn arg_presence(flag: &str, args: &ArgMatches) -> Result<bool, String> {
    args
        .try_contains_id(flag)
        .map_err(|err| {
            error!(target: "darkproto", "--{} is not a valid argument. Failed while matching an argument with error: {:?}", flag, err);
            format!("--{} is not a valid argument. Failed while matching an argument with error: {:?}", flag, err)
        })
}

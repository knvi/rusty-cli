use super::*;
use crate::{
    sdk::SDK,
    utils::{btreemap::ToBTreeMap, choice::Choice, config::get_config, table::Table},
};
/// Get all environment variables for the current configured directory
#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    key: Option<String>,

    #[clap(short, long)]
    project_id: Option<String>,

    /// Output as JSON - JSON has the highest precedence and will override other output formats
    #[clap(long)]
    json: bool,

    /// Output as a list of key=value pairs
    #[clap(long)]
    kv: bool,
}

pub async fn command(args: Args) -> Result<()> {
    let mode = Mode::from_args(&args);

    let config = get_config()?;
    let key = config.get_key_or_default(args.key)?;
    let project_id = Choice::try_project(args.project_id, &key.fingerprint).await?;
    let mut kvpairs = SDK::get_variables_pruned(&project_id, &key.fingerprint).await?;
    kvpairs.sort_by(|a, b| a.key.cmp(&b.key));

    match mode {
        Mode::KV => {
            kvpairs.iter().for_each(|kv| println!("{}", kv));
        }
        Mode::Json => {
            let btreemap = kvpairs.to_btreemap()?;
            println!("{}", serde_json::to_string_pretty(&btreemap)?);
        }
        Mode::Table => {
            let btreemap = kvpairs.to_btreemap()?;
            Table::new("Variables".into(), btreemap).print()?;
        }
    }

    Ok(())
}

enum Mode {
    KV,
    Json,
    Table,
}

impl Mode {
    fn from_args(args: &Args) -> Self {
        if args.json {
            Self::Json
        } else if args.kv {
            Self::KV
        } else {
            Self::Table
        }
    }
}

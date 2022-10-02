use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use dsync::{GenerationConfig, TableOptions};

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = DESCRIPTION)]
struct Args {
    /// Input file
    #[structopt(
        short = "i",
        long = "input",
        help = "Required; rust file to read diesel schema information from",
        required = true
    )]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output",
        help = "Required; directory to write generated code to"
    )]
    output: PathBuf,

    #[structopt(
        long = "tsync",
        help = "Optional: adds the #[tsync] attribute to all structs; see https://github.com/Wulf/tsync"
    )]
    tsync: bool,

    #[structopt(
        short = "g",
        long = "autogenerated-columns",
        help = "Optional; List of columns which are automatically generated but are not primary keys (for example: `created_at`, `updated_at`, etc.)"
    )]
    autogenerated_columns: Option<Vec<String>>
}

fn main() {
    let args: Args = Args::from_args();

    dsync::generate_files(args.input, args.output, GenerationConfig {
        default_table_options: TableOptions {
            ignore: false,
            autogenerated_columns: args.autogenerated_columns.unwrap_or(vec![]).iter().map(|t| t.as_str()).collect::<Vec<&str>>(),
            tsync: false,
        },
        table_options: HashMap::from([])
    });
}

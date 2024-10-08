use clap::Parser;
use dsync_hasezoey::{FileChangesStatus, GenerationConfig, TableOptions};
use std::collections::HashMap;

mod clap_conf;
mod completions;

fn main() {
    let res = actual_main();

    if let Err(err) = res {
        eprintln!("Error:\n{err}");
        let backtrace = err.backtrace().to_string();

        if backtrace == "disabled backtrace" {
            eprintln!(
                "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
            );
        } else {
            eprintln!("{}", backtrace);
        }

        std::process::exit(1);
    }
}

fn actual_main() -> anyhow::Result<()> {
    let cli = clap_conf::CliDerive::parse();

    if let Some(subcommand) = cli.subcommands {
        return match subcommand {
            clap_conf::SubCommands::Completions(subcommand) => {
                completions::command_completions(&subcommand)
            }
        };
    }

    let args = cli
        .args
        .expect("cli.args should be defined if not subcommand is given");

    let cols = args.autogenerated_columns.unwrap_or_default();
    let mut default_table_options = TableOptions::default()
        .autogenerated_columns(cols.iter().map(|t| t.as_str()).collect::<Vec<&str>>());

    #[cfg(feature = "tsync")]
    if args.tsync {
        default_table_options = default_table_options.tsync();
    }

    #[cfg(feature = "async")]
    if args.use_async {
        default_table_options = default_table_options.use_async();
    }

    if args.no_serde {
        default_table_options = default_table_options.disable_serde();
    }

    if args.only_necessary_derives {
        default_table_options = default_table_options.only_necessary_derives();
    }

    if args.no_impls {
        default_table_options = default_table_options.disable_impls();
    }

    if args.create_str {
        default_table_options = default_table_options.create_str_over_string();
    }

    let changes = dsync_hasezoey::generate_files(
        args.input,
        args.output,
        GenerationConfig {
            default_table_options,
            table_options: HashMap::from([]),
            connection_type: args.connection_type,
            schema_path: args.schema_path,
            model_path: args.model_path,
            once_common_structs: args.once_common_structs,
            single_model_file: args.single_model_file,
            file_mode: args.file_mode.into(),
            read_only_prefix: args.read_only_prefix,
            once_connection: args.once_connection,
            lessen_conflicts: args.lessen_conflicts,
        },
    )?;

    let mut modified: usize = 0;

    for change in changes {
        println!("{} {}", change.status, change.file.to_string_lossy());
        if change.status != FileChangesStatus::Unchanged {
            modified += 1;
        }
    }

    println!("Modified {} files", modified);

    Ok(())
}

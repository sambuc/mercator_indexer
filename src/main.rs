#[macro_use]
extern crate measure_time;

use std::process::exit;

use mercator_db::json::storage;

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let datasets;

    if std::env::var("MERCATOR_DATASETS").is_err() {
        std::env::set_var("MERCATOR_DATASETS", "test");
    }

    match std::env::var("MERCATOR_DATASETS") {
        Ok(val) => {
            datasets = val
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        }
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_DATASETS", val);
            exit(1);
        }
    };

    for dataset in datasets {
        println!();
        warn!("Indexing dataset: {}", dataset);
        warn_time!("Indexed dataset: {}", dataset);

        // Convert to binary the JSON data:
        {
            info_time!("Converting to binary JSON data");
            storage::convert(&dataset);
        }

        // Build a Database Index:
        {
            info_time!("Building database index");
            storage::build(&dataset);
        }
    }
}

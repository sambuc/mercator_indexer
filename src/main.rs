#[macro_use]
extern crate measure_time;

use std::process::exit;

use mercator_db::json::storage;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// List of datasets to index.
    datasets: Vec<String>,
}

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let opt = Opt::from_args();

    for dataset in opt.datasets {
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

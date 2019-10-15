#[macro_use]
extern crate measure_time;

use mercator_db::json::storage;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Optional list of scale factors to be applied to generate coarser indices
    /// as well as the full-resolution index.
    #[structopt(long, short)]
    scales: Option<Vec<u32>>,

    /// Optional hint to be used when you wish to generate extra, coarser
    /// indices. This argument is ignored when `scales` is also provided.
    #[structopt(long, short)]
    max_elements: Option<Option<usize>>,

    /// List of datasets to index, with the following syntax per dataset:
    /// name[:version]: where name is the basename of the input files, and
    /// `version` a string to add to the dataset description
    datasets: Vec<String>,
}

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let opt = Opt::from_args();
    let scales = match opt.scales {
        None => None,
        Some(v) => {
            if v.is_empty() {
                None
            } else {
                let v = v.iter().map(|x| vec![*x, *x, *x]).collect();
                Some(v)
            }
        }
    };

    let max_elements = match opt.max_elements {
        None => None,
        Some(e) => match e {
            None => Some(0),
            s @ Some(_) => s,
        },
    };

    for dataset in opt.datasets {
        println!();

        let v = dataset.split(':').collect::<Vec<_>>();
        if v.len() > 2 {
            warn!("Invalid dataset definition, too many fields: '{:?}'", v);
            continue;
        }

        let title = v[0];
        let version = if v.len() == 2 { v[1] } else { "" };

        warn!("Indexing dataset: {}", title);
        warn_time!("Indexed dataset: {}", title);

        // Convert to binary the JSON data:
        {
            info_time!("Converting to binary JSON data");
            storage::convert(&title);
        }

        // Build a Database Index:
        {
            info_time!("Building database index");
            storage::build(&title, version, scales.clone(), max_elements);
        }
    }
}

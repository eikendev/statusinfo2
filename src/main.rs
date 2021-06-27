use argh::FromArgs;
use env_logger::Env;
use log::error;
use std::convert::TryFrom;

mod gadget;

#[derive(FromArgs)]
/// Prints various metrics of your system.
struct Args {
    /// how many spaces between gadgets
    #[argh(option, default = "4")]
    separator: usize,

    /// how many spaces between icon and data
    #[argh(option, default = "2")]
    space: usize,

    #[argh(positional, from_str_fn(validate_gadget))]
    /// the metric to print
    gadget: gadget::Gadget,

    #[argh(positional, from_str_fn(validate_gadget))]
    /// the metrics to print
    gadgets: Vec<gadget::Gadget>,
}

fn validate_gadget(gadget: &str) -> Result<gadget::Gadget, String> {
    gadget::Gadget::try_from(gadget)
}

fn process_gadget(gadget: gadget::Gadget, space: usize, results: &mut Vec<String>) {
    let result = gadget.run();
    results.push(result.print(space, "???"));

    if let Err(e) = &result.data {
        error!("{}", e);
    }
}

fn main() {
    human_panic::setup_panic!();
    env_logger::Builder::from_env(Env::default().filter_or("LOG_LEVEL", "info")).init();

    let args: Args = argh::from_env();

    let mut results: Vec<String> = vec![];

    process_gadget(args.gadget, args.space, &mut results);

    for gadget in args.gadgets {
        process_gadget(gadget, args.space, &mut results);
    }

    let separator = str::repeat(" ", args.separator);
    let joined = results.join(&separator);

    println!("{}", joined);
}

use argh::FromArgs;
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
    /// the metrics to print
    gadgets: Vec<gadget::Gadget>,
}

fn validate_gadget(gadget: &str) -> Result<gadget::Gadget, String> {
    gadget::Gadget::try_from(gadget)
}

fn main() {
    human_panic::setup_panic!();

    let args: Args = argh::from_env();

    let mut results: Vec<String> = vec![];

    for arg in args.gadgets {
        let result = arg.run();
        results.push(result.print(args.space, "???"));
    }

    let separator = str::repeat(" ", args.separator);
    let joined = results.join(&separator);

    println!("{}", joined);
}

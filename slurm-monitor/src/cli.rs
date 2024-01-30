use clap::{Arg, Command, ArgMatches, ArgAction};

pub fn init() -> ArgMatches {
    let args = Command::new("Slurm Monitor")
        .version("1.0.0")
        .author("Johannes Blaschke")
        .about("Monitors Slurm for certain jobs")
        .arg(
            Arg::new("settings")
            .long("settings")
            .help("Location of settings file")
            .value_name("SETTINGS")
            .required(true)
        )
        .arg(
            Arg::new("drain")
            .short('d')
            .long("drain")
            .help("Drain all jobs.")
            .num_args(0)
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    return args;
}

pub struct CLI<'a> {
    pub settings: &'a str,
    pub drain: bool
}

pub fn parse<'a>(args: &'a ArgMatches) -> CLI<'a> {
    let settings = args.get_one::<String>("settings").unwrap().as_str();
    CLI {
        settings: settings,
        drain: * args.get_one::<bool>("drain").unwrap()
    }
}

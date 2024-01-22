use clap::{Arg, Command, ArgMatches};

pub fn init() -> ArgMatches {
    let args = Command::new("Slurm Monitor")
        .version("1.0.0")
        .author("Johannes Blaschke")
        .about("Monitors Slurm for certain jobs")
        .arg(
            Arg::new("name")
            .long("name")
            .help("Name of the job to check for")
            .value_name("NAME")
            .required(true)
        )
        .get_matches();

    return args;
}

pub struct CLI<'a> {
    pub name: &'a str
}

pub fn parse<'a>(args: &'a ArgMatches) -> CLI<'a> {
    let name = args.get_one::<String>("name").unwrap().as_str();
    CLI {
        name: name
    }
}

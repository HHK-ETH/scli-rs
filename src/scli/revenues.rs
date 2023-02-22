use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    let network_arg = Arg::new("network")
        .help("Network to query. Ex: ethereum.")
        .short('n')
        .long("network");
    let days_arg = Arg::new("days")
        .help("Days in the past to query. Ex: 7 (for weekly revenue)")
        .short('d')
        .long("days")
        .default_value("1");

    let revenues = Command::new("revenues")
        .about("Get revenues for all chains by comparing fees with sushi spent.")
        .arg(network_arg)
        .arg(days_arg);

    revenues
}

fn parse_days(days: &String) -> u32 {
    let days = match days.parse::<u32>() {
        Ok(days) => days,
        Err(error) => {
            eprintln!(
                "Error while parsing days param, make sure to enter a valid number. {}",
                error
            );
            panic!()
        }
    };
    if days > 30 || days == 0 {
        eprintln!("Please enter a days number between 1 and 30.");
        panic!()
    }
    days
}

pub fn execute(params: &ArgMatches) -> () {
    let network = params.get_one::<String>("network");
    let days = parse_days(params.get_one::<String>("days").unwrap());
}

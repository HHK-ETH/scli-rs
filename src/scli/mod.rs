use clap::Command;

mod revenues;

pub fn scli() {
    let revenues = revenues::command();
    let scli = Command::new("scli-rs").subcommand(revenues);

    match scli.get_matches().subcommand() {
        Some(("revenues", params)) => {
            revenues::execute(params);
        }
        _ => println!("none"),
    }
}

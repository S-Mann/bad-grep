#[macro_use]
extern crate clap;
use clap::App;

mod app_logic;
use app_logic::{query_file};

fn main() {
    let yaml = load_yaml!("../assets/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let query = matches.value_of("query").unwrap();
    let filename = matches.value_of("file").unwrap();

    let result = query_file(query, filename);
    for val in result {
        println!("{}", val.get_result());
    }
}

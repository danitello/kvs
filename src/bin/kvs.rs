#[macro_use]
extern crate clap;
use clap::App;
use kvs::KvStore;

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let mut store = KvStore::new();
    match matches.subcommand() {
        ("get", Some(_matches)) => match store.get(_matches.value_of("key").unwrap().to_string()) {
            Some(s) => println!("{}", s),
            None => println!("Not found"),
        },
        ("set", Some(_matches)) => {
            store.set(
                _matches.value_of("key").unwrap().to_string(),
                _matches.value_of("value").unwrap().to_string(),
            );
        }
        ("rm", Some(_matches)) => panic!("unimplemented"),
        _ => eprintln!("Subcommand Not Found"),
    }
}

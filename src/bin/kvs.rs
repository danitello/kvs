#[macro_use]
extern crate clap;
use clap::App;

fn main() {
   let cli_yaml = load_yaml!("cli.yml");
   let matches = App::from_yaml(cli_yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION")).get_matches();

   match matches.subcommand(){
       ("get", Some(_matches)) => panic!("unimplemented"),
       ("set", Some(_matches)) => panic!("unimplemented"),
       ("rm", Some(_matches)) => panic!("unimplemented"),
       _ => eprintln!("Subcommand Not Found")
   }
}

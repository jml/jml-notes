use clap::{App, SubCommand};


fn main() {
    App::new("jml-notes")
        .version("0.0.1")
        .about("Create notebook posts")
        .author("Jonathan M. Lange")
        .subcommand(SubCommand::with_name("new"))
        .subcommand(SubCommand::with_name("edit"))
        .get_matches();
}

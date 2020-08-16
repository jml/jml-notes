use clap::{App, SubCommand};
use std::process;


fn new_post() {
    println!("new-post");
}


fn edit_post() {
    println!("edit-post");
}


fn main() {
    let matches = App::new("jml-notes")
        .version("0.0.1")
        .about("Create notebook posts")
        .author("Jonathan M. Lange")
        .subcommand(SubCommand::with_name("new"))
        .subcommand(SubCommand::with_name("edit"))
        .get_matches();
    match matches.subcommand_name() {
        Some("new") => new_post(),
        Some("edit") => edit_post(),
        _ => {
            eprintln!("No such subcommand");
            process::exit(1);
        }
    }
}

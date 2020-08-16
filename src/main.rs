use clap::App;


fn main() {
    App::new("jml-notes")
        .version("0.0.1")
        .about("Create notebook posts")
        .author("Jonathan M. Lange")
        .get_matches();
}

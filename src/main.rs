#[macro_use]
extern crate lazy_static;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

mod posts;
use posts::Posts;


/// Create a new blog post.
pub fn new_post(posts: &Posts) -> Result<(), Box<dyn Error>> {
    let name = posts.new_post()?;
    edit_and_commit_post(&posts, &name)?;
    Ok(())
}

pub fn edit_post(posts: &Posts) -> io::Result<()> {
    let latest_file = posts.get_latest_file()?;
    // TODO: Return errors for not finding posts.
    // TODO: See if we can avoid nested match.
    match latest_file {
        None => {
            println!("Could not find post to edit.");
            Ok(())
        }
        Some(path) => match path.file_stem().and_then(|stem| stem.to_str()) {
            None => {
                println!("Could not find post to edit.");
                Ok(())
            }
            Some(name) => edit_and_commit_post(&posts, name),
        },
    }
}

/// Edit the blog post with the given name inside the posts directory.
///
/// If it changes, ensure the change is committed.
fn edit_and_commit_post(posts: &Posts, name: &str) -> io::Result<()> {
    let post_file = posts.get_post_filename(name);
    let changed = edit_file(&post_file)?;
    if changed {
        posts.commit_post(&post_file, name)
    } else {
        Ok(())
    }
}

fn edit_file(filename: &Path) -> io::Result<bool> {
    let prev = contents(filename);
    edit(filename)?;
    let current = contents(filename);
    Ok(prev != current)
}

/// Get the contents of a file as a vector.
///
/// If the file doesn't exist, return None. Panic if we get any other kind of
/// error.
fn contents(path: &Path) -> Option<Vec<u8>> {
    match fs::read(&path) {
        Ok(bytes) => Some(bytes),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => None,
            _ => panic!("Could not read file: {}: {}", path.display(), err),
        },
    }
}

/// Edit a file in my preferred editor.
fn edit(file: &Path) -> io::Result<()> {
    process::Command::new("emacsclient")
        .arg("-c")
        .arg(file)
        .status()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("jml-notes")
        .version("0.0.1")
        .about("Create notebook posts")
        .author("Jonathan M. Lange")
        .arg(Arg::with_name("posts_dir").long("posts-dir").env("NOTEBOOK_POSTS_DIR").help("Path to directory containing notebook posts.").required(true))
        .subcommand(SubCommand::with_name("new"))
        .subcommand(SubCommand::with_name("edit"))
        .get_matches();
    let posts_dir = matches.value_of("posts_dir").unwrap();
    let posts = Posts::new(Path::new(posts_dir).to_owned());
    match matches.subcommand_name() {
        Some("new") => new_post(&posts)?,
        Some("edit") => edit_post(&posts)?,
        _ => {
            eprintln!("No such subcommand");
            process::exit(1);
        }
    }
    Ok(())
}

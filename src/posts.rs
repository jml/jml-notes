use chrono::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use tera::{Context, Tera};

const POST_FILENAME_FORMAT: &str = "%Y-%m-%d-%H:%M";
const POST_SLUG_FORMAT: &str = "%Y-%m-%d-%H-%M";

#[derive(Debug)]
pub struct Posts {
    path: PathBuf,
}

impl Posts {
    pub fn new(path: PathBuf) -> Posts {
        Posts { path }
    }

    pub fn new_post(&self) -> Result<String, Box<dyn Error>> {
        let now = Utc::now();
        let name = format!("{}", now.format(POST_FILENAME_FORMAT));
        let post_path = self.get_post_filename(&name);
        let mut post_file = fs::File::create(&post_path)?;
        let contents = render_new_post(&now)?;
        post_file.write_all(contents.as_bytes())?;
        Ok(name)
    }

    pub fn get_latest_file(&self) -> io::Result<Option<PathBuf>> {
        let entries = fs::read_dir(&self.path)?;
        let mut paths = vec![];
        for entry in entries {
            let entry = entry?;
            paths.push(entry.path())
        }
        Ok(paths.into_iter().max())
    }

    pub fn get_post_filename(&self, name: &str) -> PathBuf {
        let mut post_file = self.path.to_owned();
        post_file.push(name);
        post_file.set_extension("md");
        post_file
    }

    pub fn commit_post(&self, post_file: &Path, name: &str) -> io::Result<()> {
        process::Command::new("git")
            .current_dir(&self.path)
            .arg("add")
            .arg(post_file)
            .status()?;
        process::Command::new("git")
            .current_dir(&self.path)
            .arg("commit")
            .arg("-m")
            .arg(format!("Add new post {}", name))
            .status()?;
        Ok(())
    }
}

fn render_new_post(now: &DateTime<Utc>) -> tera::Result<String> {
    let mut context = Context::new();
    context.insert("title", "Title goes here");
    context.insert("date", &now.to_rfc3339_opts(SecondsFormat::Secs, true));
    let slug = format!("{}", now.format(POST_SLUG_FORMAT));
    context.insert("slug", &slug);
    TEMPLATES.render("post.md", &context)
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut t = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                panic!("Parsing error(s): {}", e);
            }
        };
        let post = include_str!("../templates/post.md");
        t.add_raw_template("post.md", post).unwrap();
        t
    };
}

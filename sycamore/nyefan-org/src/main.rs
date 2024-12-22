mod util;

use std::error::Error;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use sycamore::prelude::*;
use sycamore::web::events as ev;
use sycamore::web::tags::*;
use tracing::debug;

#[component]
fn App() -> View {
    let name = create_signal(String::new());

    let handle_change = move |_| {};
    body()
        .children(
            div().children((
                h1().children((
                    "Hello ",
                    if !name.with(String::is_empty) {
                        span().children(name)
                    } else {
                        span().children("World")
                    },
                    "!",
                )),
                input()
                    .placeholder("What is your name?")
                    .on(ev::input, handle_change),
            )),
        )
        .into()
}

fn render_md_files(input_directory: &str, output_directory: &str) -> Result<(), Box<dyn Error>> {
    let _ = std::fs::remove_dir_all(output_directory);
    util::walk_directory(Path::new(input_directory))?
        .iter()
        .map(DirEntry::path)
        .inspect(|path| debug!("Reading {:?}", path))
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .map(|path| {
            let output_path = PathBuf::from(output_directory)
                .join(path.with_extension("html").strip_prefix(input_directory)?);
            Ok((
                PathBuf::from(output_path),
                util::render_md_file_to_html_string(&path)?,
            ))
        })
        .try_for_each(|result: Result<(PathBuf, String), Box<dyn Error>>| {
            let (path, content) = result?;
            debug!("Writing {:?}", path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, content)?;
            Ok::<(), Box<dyn Error>>(())
        })?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    util::setup_logging();
    let s = sycamore::render_to_string(App);
    if std::fs::exists("dist")? {
        std::fs::remove_dir_all("dist")?;
    }
    std::fs::create_dir("dist")?;
    std::fs::write("dist/index.html", s)?;
    render_md_files("content/_posts", "dist/posts")?;
    Ok(())
}

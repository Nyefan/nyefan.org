#![feature(macro_metavar_expr_concat)]
mod pages;
mod util;

use crate::util::components::ContentSection;
use std::error::Error;
use std::path::PathBuf;
use tracing::debug;

fn main() -> Result<(), Box<dyn Error>> {
    util::setup_logging();
    if std::fs::exists("dist")? {
        std::fs::remove_dir_all("dist")?;
    }
    std::fs::create_dir("dist")?;

    util::render_md_file(
        "content/_pages/about.md",
        "dist/about.html",
        pages::about::template,
    )?;
    util::render_md_file(
        "content/_pages/contact.md",
        "dist/contact.html",
        pages::contact::template,
    )?;
    util::render_md_file(
        "content/_pages/404.md",
        "dist/404.html",
        pages::http_404_not_found::template,
    )?;
    util::copy_directory("content/slides", "dist/slides")?;

    let posts = util::parse_md_files_in_directory("content/_posts", pages::post::parse)?;

    {
        let previews: [ContentSection; 3] = posts
            .iter()
            .take(3)
            .map(pages::post::preview)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "invalid state")?;
        let html = sycamore::render_to_string(pages::index::template(previews));
        std::fs::write("dist/index.html", html)?;
    }

    for post in &posts {
        let output_path = PathBuf::from("dist/posts").join(
            PathBuf::from(&post.origin_path)
                .strip_prefix("content/_posts")?
                .with_extension("html"),
        );
        let html = sycamore::render_to_string(|| pages::post::template(&post));
        debug!("Writing post to {output_path:?}");
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(output_path, html)?;
    }

    {
        let output_path = PathBuf::from("dist/posts.html");
        let metadatas = posts
            .iter()
            .map(|p| &p.metadata)
            .collect::<Vec<_>>();
        let html = sycamore::render_to_string(|| pages::posts::template(metadatas));
        debug!("Writing posts to dist/posts.html");
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(output_path, html)?;
    }

    Ok(())
}

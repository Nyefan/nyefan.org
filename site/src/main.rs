#![feature(macro_metavar_expr_concat)]
mod pages;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    util::setup_logging();
    let s = sycamore::render_to_string(pages::index::template(pages::index::test_data()));
    if std::fs::exists("dist")? {
        std::fs::remove_dir_all("dist")?;
    }
    std::fs::create_dir("dist")?;
    std::fs::write("dist/index.html", s)?;
    util::render_md_files_in_directory("content/_posts", "dist/posts", pages::post::template)?;
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
    Ok(())
}

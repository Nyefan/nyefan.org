mod index;
mod post;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    util::setup_logging();
    let s = sycamore::render_to_string(index::template);
    if std::fs::exists("dist")? {
        std::fs::remove_dir_all("dist")?;
    }
    std::fs::create_dir("dist")?;
    std::fs::write("dist/index.html", s)?;
    util::render_md_files_in_directory("content/_posts", "dist/posts", post::template)?;
    Ok(())
}

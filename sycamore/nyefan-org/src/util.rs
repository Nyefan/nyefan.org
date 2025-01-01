use std::error::Error;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use sycamore::prelude::View;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;

pub(crate) fn render_md_files_in_directory(input_directory: &str, output_directory: &str, embed_html_function: fn(String) -> View) -> Result<(), Box<dyn Error>> {
    let _ = std::fs::remove_dir_all(output_directory);
    walk_directory(Path::new(input_directory))?
        .iter()
        .map(DirEntry::path)
        .inspect(|path| debug!("Reading {:?}", path))
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .map(|path| {
            let output_path = PathBuf::from(output_directory)
                .join(path.with_extension("html").strip_prefix(input_directory)?);
            let content = render_md_file_to_html_string(&path)?;
            let page = embed_html_function(content);
            Ok((
                PathBuf::from(output_path),
                sycamore::render_to_string(|| page)
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

pub(crate) fn walk_directory(directory: &Path) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        if entry.path().is_dir() {
            entries.extend(walk_directory(&entry.path())?);
        } else {
            entries.push(entry.into());
        }
    }
    Ok(entries)
}

pub(crate) fn render_md_file_to_html_string(path: &Path) -> Result<String, Box<dyn Error>> {
    let markdown = std::fs::read_to_string(path)?;
    let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());

    let mut html = String::new();
    // without this, ' renders as â€™ ~~because browsers default to Windows-1252~~
    // because the web is fundamentally broken
    //
    // the `Content-Type` field overrides this as well, so it might still break in github pages with
    // absolutely no way for developers to fix it, because obviously the content-server is more
    // likely to know the encoding of the content than the content author
    // https://html.spec.whatwg.org/multipage/parsing.html#determining-the-character-encoding
    // html.push_str("<meta charset=\"UTF-8\">\n");
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(html)
}

pub(crate) fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

use std::error::Error;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use sycamore::prelude::View;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;

mod colors;
pub mod components;
mod styles;

pub(crate) fn render_md_files_in_directory(
    input_directory: &str,
    output_directory: &str,
    embed_html_function: fn(String) -> View,
) -> Result<(), Box<dyn Error>> {
    read_md_files_in_directory(input_directory, render_md_to_html_string::<String>)?
        .iter()
        .map(|(input_path, content)| (input_path, embed_html_function(content.clone())))
        .map(|(input_path, view)| (input_path, sycamore::render_to_string(|| view)))
        .try_for_each(|(input_path, html)| {
            let output_path = PathBuf::from(output_directory).join(
                input_path
                    .strip_prefix(input_directory)?
                    .with_extension("html"),
            );
            debug!("Writing to {:?}", output_path);
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(output_path, html)?;
            Ok::<(), Box<dyn Error>>(())
        })?;
    Ok(())
}

type MdString = String;
pub(crate) fn read_md_files_in_directory<T, E>(
    input_directory: &str,
    parse: fn(MdString) -> Result<T, E>,
) -> Result<Vec<(PathBuf, T)>, E>
where
    E: From<Box<dyn Error>> + From<std::io::Error>,
{
    walk_directory(Path::new(input_directory))?
        .iter()
        .map(DirEntry::path)
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .inspect(|path| debug!("Reading {path:?}"))
        .map(|path| (path.clone(), std::fs::read_to_string(&path)))
        .map(|(path, raw)| Ok((path, parse(raw?)?)))
        .collect()
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

pub(crate) fn render_md_to_html_string<S: AsRef<str>>(
    markdown: S,
) -> Result<String, Box<dyn Error>> {
    let parser = pulldown_cmark::Parser::new_ext(markdown.as_ref(), pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(html)
}

pub(crate) fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

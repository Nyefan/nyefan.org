use crate::util::components;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use std::error::Error;
use std::path::PathBuf;
use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template(post: &Post) -> View {
    (
        components::site_head(),
        components::site_body(components::wrapper((
            components::site_header(),
            components::main((
                components::sidebar(),
                components::content(components::content_section((
                    h2().children(
                        a().style(format!("color: {}", crate::util::colors::LAVENDER_MEDIUM))
                            .href(format!(
                                "/posts/{}-{}",
                                post.metadata.date,
                                post.metadata.title.replace(" ", "-")
                            ))
                            .children(post.metadata.title.clone()),
                    ),
                    div().dangerously_set_inner_html(post.html_content.clone()),
                ))),
            )),
            components::site_footer(),
        ))),
    )
        .into()
}

#[component]
pub(crate) fn preview(post: &Post) -> components::ContentSection {
    components::content_section((
        h2().children(
            a().style(format!("color: {}", crate::util::colors::LAVENDER_MEDIUM))
                .href(format!(
                    "/posts/{}-{}",
                    post.metadata.date,
                    post.metadata.title.replace(" ", "-")
                ))
                .children(post.metadata.title.clone()),
        ),
        div().dangerously_set_inner_html(post.html_preview.clone()),
    ))
}

pub(crate) fn parse(path: PathBuf) -> Result<Post, Box<dyn Error>> {
    let raw_content = std::fs::read_to_string(&path)?;
    let parsed_matter = Matter::<YAML>::new().parse(&raw_content);
    let html_content = {
        let parser =
            pulldown_cmark::Parser::new_ext(&parsed_matter.content, pulldown_cmark::Options::all());
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    };
    let html_preview = {
        // TODO: something more intelligent than taking the first eight lines of the md file
        let preview_lines = parsed_matter
            .content
            .lines()
            .take(8)
            .map(|l| l.to_string() + "\n")
            .collect::<String>()
            + "\n...\n";
        let parser =
            pulldown_cmark::Parser::new_ext(&*preview_lines, pulldown_cmark::Options::all());
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    };
    let (date, title): (Option<String>, Option<String>) = {
        path.with_extension("")
            .file_name()
            .and_then(|p| p.to_str())
            .map(|s| s.split("-").collect::<Vec<&str>>())
            .map(|v| match v.len() {
                0 => (None, None),
                1 => (None, None),
                2 => (None, None),
                3 => (Some(v[0..3].join("-")), None),
                _ => (Some(v[0..3].join("-")), Some(v[3..].join(" "))),
            })
            .unwrap_or((None, None))
    };
    Ok(Post {
        origin_path: path.to_str().unwrap().to_string(),
        _raw_content: raw_content,
        html_content,
        html_preview,
        metadata: PostMetadata {
            _raw_front_matter: parsed_matter.matter,
            title: title.unwrap_or("".to_string()),
            _author: "Nyefan".to_string(),
            description: "".to_string(),
            _tags: vec![],
            date: date.unwrap_or("".to_string()),
        },
    })
}

pub(crate) struct Post {
    pub(crate) origin_path: String,
    _raw_content: String,
    pub(crate) html_content: String,
    html_preview: String,
    pub(crate) metadata: PostMetadata,
}

pub(crate) struct PostMetadata {
    _raw_front_matter: String,
    pub(crate) title: String,
    _author: String,
    pub(crate) description: String,
    _tags: Vec<String>,
    pub(crate) date: String,
}

pub(crate) enum _Tag {
    Gaming,
    Programming,
    Music,
    Presentation,
    Runbook,
    Other,
}

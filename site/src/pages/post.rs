use crate::util::components;
use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template(content: String) -> View {
    (
        components::site_head(),
        components::site_body(components::wrapper((
            components::site_header(),
            components::main((
                components::sidebar(),
                components::content(components::content_section(
                    div().dangerously_set_inner_html(content),
                )),
            )),
            components::site_footer(),
        ))),
    )
        .into()
}

#[expect(dead_code)]
#[component]
pub(crate) fn preview(post: Post) -> View {
    components::content_section(div().dangerously_set_inner_html(post.html_content))
}

#[expect(dead_code)]
pub(crate) struct Post {
    path: String,
    raw_content: String,
    html_content: String,
    html_preview: String,
    metadata: PostMetadata,
}

#[expect(dead_code)]
pub(crate) struct PostMetadata {
    raw_gray_matter: String,
    title: String,
    author: String,
    description: String,
    tags: Vec<String>,
    date: String,
}

#[expect(dead_code)]
pub(crate) enum Tag {
    Gaming,
    Programming,
    Music,
    Presentation,
    Runbook,
    Other,
}

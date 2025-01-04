use crate::pages::post::PostMetadata;
use crate::util::components;
use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template(post_metadatas: Vec<&PostMetadata>) -> View {
    let content = post_metadatas
        .iter()
        .map(|post_metadata| -> View {
            (
                h2().style("margin-bottom: 0px;").children(
                    a().style(format!("color: {};", crate::util::colors::LAVENDER_MEDIUM,))
                        .href(format!(
                            "/posts/{}-{}",
                            post_metadata.date,
                            post_metadata.title.replace(" ", "-")
                        ))
                        .children(post_metadata.title.clone()),
                ),
                h5().children(post_metadata.date.clone()),
                h3().style("margin-bottom: 20px;")
                    .children(post_metadata.description.clone()),
            )
                .into()
        })
        .collect::<Vec<View>>();
    (
        components::site_head(),
        components::site_body(components::wrapper((
            components::site_header(),
            components::main((
                components::sidebar(),
                components::content(components::content_section((
                    h2().children(
                        a().style(format!("color: {};", crate::util::colors::LAVENDER_MEDIUM,))
                            .href("/posts")
                            .children("Blog"),
                    ),
                    div().children(content),
                ))),
            )),
            components::site_footer(),
        ))),
    )
        .into()
}

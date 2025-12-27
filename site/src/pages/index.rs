use crate::util::components;
use sycamore::component;
use sycamore::prelude::*;

#[component]
pub(crate) fn template(content_previews: [components::ContentSection; 3]) -> impl FnOnce() -> View {
    let [a, b, c] = content_previews;
    let content_previews = (a, b, c);
    move || -> View {
        (
            components::site_head(),
            components::site_body(components::wrapper((
                components::site_header(),
                components::main((components::sidebar(), components::content(content_previews))),
                components::site_footer(),
            ))),
        )
            .into()
    }
}

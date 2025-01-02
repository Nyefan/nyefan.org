use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;
use crate::components;

#[component]
pub(crate) fn template(content: String) -> View {
    (
        components::site_head(),
        components::site_body(components::wrapper((
            components::site_header(),
            components::main((components::sidebar(), components::content(components::content_section(div().dangerously_set_inner_html(content))))),
            components::site_footer(),
        ))),
    )
        .into()
}

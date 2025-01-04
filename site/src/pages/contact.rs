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
                components::content(components::content_section((
                    h2().children(
                        a().style(format!("color: {};", crate::util::colors::LAVENDER_MEDIUM,))
                            .href("/contact")
                            .children("Contact"),
                    ),
                    div().dangerously_set_inner_html(content),
                ))),
            )),
            components::site_footer(),
        ))),
    )
        .into()
}

use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template(content: String) -> View {
    (
        meta().charset("utf-8"),
        body().children(div().dangerously_set_inner_html(content)),
    )
        .into()
}

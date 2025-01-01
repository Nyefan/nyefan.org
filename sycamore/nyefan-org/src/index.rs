use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template() -> View {
    let name = "nyefan";
    (
        meta().charset("utf-8"),
        body().children(div().children(h1().children(("Hello ", name, "!")))),
    )
        .into()
}

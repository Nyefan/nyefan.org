use crate::components;
use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

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

pub(crate) fn test_data() -> [components::ContentSection; 3] {
    [
        components::content_section((
            h2().children("Welcome to nyefan.org!"),
            p().children("This is a placeholder for additional content, like recent blog posts or categories."),
            p().children("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam.")
        )),
        components::content_section((
            h2().children("Example Code Block"),
            pre().children(
                r#"
// A simple Rust example
fn main() {
    println!("Hello, world!");
}
"#,
            ),
        )),
        components::content_section((
            h2().children("Technical Insights"),
            p().children("This section dives deep into technical topics such as software development, optimization techniques, and emerging trends in technology."),
            p().children("Stay tuned for detailed breakdowns and walkthroughs of exciting new projects and tools in the tech space.")
        ))
    ]
}

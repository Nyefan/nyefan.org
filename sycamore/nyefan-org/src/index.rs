use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub(crate) fn template() -> View {
    (
        head().children((
            meta().charset("utf-8"),
            meta()
                .name("viewport")
                .content("width=device-width, initial-scale=1.0"),
            title().title("nyefan.org"),
            style().children(
                format!(" {} {} {} {} {} {} {} {} {} {} {} ",
                    style::star(),
                    style::html_body(),
                    style::wrapper(),
                    style::header(),
                    style::nav(),
                    style::main(),
                    style::content(),
                    style::content_section(),
                    style::sidebar(),
                    style::pre(),
                    style::footer()
                )),
        )),
        body().children(
            div().class("wrapper").children((
                header().children((
                    h1().children("nyefan.org"),
                    nav().children((
                        a().href("/").children("Home"),
                        a().href("/about").children("About"),
                        a().href("/contact").children("Contact"),
                    )),
                )),
                div().class("main").children((
                    aside().class("sidebar").children((
                            h3().children("Sidebar"),
                            p().children("This is a placeholder for additional content, like recent blog posts or categories.")
                    )),
                    div().class("content").children((
                        div().class("content-section").children((
                            h2().children("Welcome to nyefan.org!"),
                            p().children("This is a placeholder for additional content, like recent blog posts or categories."),
                            p().children("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam.")
                        )),
                        div().class("content-section").children((
                            h2().children("Example Code Block"),
                            pre().children(
r#"
// A simple Rust example
fn main() {
    println!("Hello, world!");
}
"#
                            ),
                        )),
                        div().class("content-section").children((
                            h2().children("Technical Insights"),
                            p().children("This section dives deep into technical topics such as software development, optimization techniques, and emerging trends in technology."),
                            p().children("Stay tuned for detailed breakdowns and walkthroughs of exciting new projects and tools in the tech space.")
                        )),
                    ))
                )),
                footer().children(
                    p().children((
                        "© 2019-2024 nyefan.org - ",
                        a().href("https://github.com/Nyefan/nyefan.org").children("This site on GitHub")
                        ))
                )
            ))
        )
    )
        .into()
}



mod style {
    use crate::color::*;

    pub(super) fn star() -> String {
        format!(
            "* {{
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn html_body() -> String {
        format!(
            "html, body {{
                height: 100%;
                font-family: Arial, sans-serif;
                line-height: 1.6;
                background-color: {BEIGE_LIGHT};
                color: {GRAY_DARCULA};
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn wrapper() -> String {
        format!(
            ".wrapper {{
                display: flex;
                flex-direction: column;
                min-height: 100%;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn header() -> String {
        format!(
            "header {{
                background-color: {LAVENDER_DARK};
                color: {BEIGE_LIGHT};
                padding: 20px;
                text-align: center;
                margin: 20px;
            }}
            header h1 {{
                margin: 0;
                font-size: 2rem;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn nav() -> String {
        format!(
            "nav {{
                margin-top: 10px;
            }}
            nav a {{
                color: {BEIGE_LIGHT};
                text-decoration: none;
                margin: 0 15px;
            }}
            nav a:hover {{
                text-decoration: underline;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn main() -> String {
        ".main {
            flex: 1; /* Allow the main section to expand */
            display: flex;
        }"
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn content() -> String {
        ".content {
            flex: 3;
            display: flex;
            flex-direction: column;
            gap: 20px;
            margin-right: 20px;
        }"
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn content_section() -> String {
        format!(
            ".content-section {{
                padding: 20px;
                background: {WHITE_SOFT};
                border-radius: 10px;
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                flex: 1;
            }}

            .content-section h2 {{
                color: {LAVENDER_MEDIUM};
                margin-bottom: 10px;
            }}

            .content-section p {{
                margin-bottom: 10px;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn sidebar() -> String {
        format!(
            ".sidebar {{
                flex: 1;
                background-color: {MOSS};
                color: {BEIGE_LIGHT};
                padding: 20px;
                border-radius: 10px;
                margin-right: 20px;
                margin-left: 20px;

            }}

            .sidebar h3 {{
                margin-bottom: 10px;
                color: {BEIGE_LIGHT};
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn pre() -> String {
        format!(
            "pre {{
                background-color: {GRAY_DARCULA};
                color: {BEIGE_LIGHT};
                padding: 10px;
                border-radius: 6px;
                overflow-x: auto;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
    pub(super) fn footer() -> String {
        format!(
            "footer {{
                text-align: center;
                font-size: 0.9rem;
                color: {GRAY_DARCULA};
                background-color: {WHITE_SOFT};
                padding: 10px 20px;
                margin: 20px;
            }}

            footer a {{
                color: {GOLDENROD_DARK};
                text-decoration: none;
            }}

            footer a:hover {{
                text-decoration: underline;
            }}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }
}

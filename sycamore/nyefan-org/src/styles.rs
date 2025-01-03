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
            display: flex;
            align-items: center;
            justify-content: center;
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
            max-height: 100%;
            max-width: calc(100vh * 1.2);
            min-width: calc(100vh / 1.4);
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
pub(super) fn sidebar() -> String {
    format!(
        ".sidebar {{
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: 12px;
            background-color: {JADE};
            color: {BEIGE_LIGHT};
            padding: 20px;
            border-radius: 10px;
            margin-right: 20px;
            margin-left: 20px;
        }}

        .sidebar h2 {{
            margin-bottom: 0px;
            color: {BEIGE_LIGHT};
        }}

        .sidebar a {{
            color: {GOLDENROD_DARK};
            text-decoration: none;
        }}

        .sidebar a:hover {{
            text-decoration: underline;
        }}"
    )
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

        .content-section h3 {{
            color: {GOLDENROD_DARK};
        }}

        .content-section a {{
            color: {GOLDENROD_DARK};
        }}

        .content-section a:hover {{
            text-decoration: underline;
        }}

        .content-section code {{
            background-color: {BEIGE_LIGHT};
            border-radius: 6px;
        }}

        .content-section pre code {{
            background-color: {GRAY_DARCULA};
            color: {BEIGE_LIGHT};
        }}

        .content-section ul {{
            margin-left: 20px;
        }}

        .content-section ol {{
            margin-left: 20px;
        }}

        .content-section p {{
            margin-bottom: 10px;
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
pub(super) fn all() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        star(),
        html_body(),
        wrapper(),
        header(),
        nav(),
        main(),
        content(),
        content_section(),
        sidebar(),
        pre(),
        footer()
    )
}

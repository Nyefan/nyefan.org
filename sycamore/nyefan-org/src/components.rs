use crate::styles;
use sycamore::component;
use sycamore::prelude::*;
use sycamore::web::tags::*;

type SiteHead = View;

#[component]
pub(crate) fn site_head() -> SiteHead {
    head()
        .children((
            meta().charset("utf-8"),
            meta()
                .name("viewport")
                .content("width=device-width, initial-scale=1.0"),
            title().title("nyefan.org"),
            style().children(styles::all()),
        ))
        .into()
}

type SiteBody = View;
#[component]
pub(crate) fn site_body<T>(site_body: T) -> SiteBody
where
    T: Into<View>,
{
    body().children(site_body).into()
}

type Wrapper = View;
#[component]
pub(crate) fn wrapper<T>(wrapper: T) -> Wrapper
where
    T: Into<View>,
{
    div().class("wrapper").children(wrapper).into()
}

type SiteHeader = View;
#[component]
pub(crate) fn site_header() -> SiteHeader {
    header()
        .children((
            h1().children("nyefan.org"),
            nav().children((
                a().href("/").children("Home"),
                a().href("/posts").children("Blog"),
                a().href("/about").children("About"),
                a().href("/contact").children("Contact"),
            )),
        ))
        .into()
}

type Main = View;
#[component]
pub(crate) fn main<T>(main: T) -> Main
where
    T: Into<View>,
{
    div().class("main").children(main).into()
}

type Sidebar = View;
#[component]
pub(crate) fn sidebar() -> Sidebar {
    aside().class("sidebar").children((
        h2().children("Nyefan"),
        p().style("flex:1;text-align:justify;").children(
            "is a Software Engineer & Consultant with a passion for software development, devops, \
            open-source software, and hard problems.  As a former materials science researcher who \
            sold out for industry, Nyefan enjoys approaching problems from a systems perspective \
            and building scalable, reliable, appropriately scoped, and performant software."
        ),
        p().style("flex:1;text-align:justify;").children(
            "This site is part runbook, part cv, part bookmark collection, and part personal blog."
        ),
        h3().children("Find out more"),
        ul().style("flex:1;margin-left:10px").children((
            li().children(("see all of my ", a().href("/posts").children("blog posts"))),
            li().children(("subscribe to the ", a().href("/feed.xml").children("RSS feed"))),
            li().children(("check out ", a().href("https://github.com/Nyefan/nyefan.org").children("this site on github"))),
            li().children(("find me on ", a().href("https://linkedin.com/in/nyefan").children("linkedin"))),
            li().children((a().href("/contact").children("contact me"), " with interesting work")),
        )),
        div().style("flex:5;"),
        p().dangerously_set_inner_html(
            "<em>\"The only way to discover the limits of the possible is to go beyond them into \
            the impossible.\"</em> - Arthur C. Clarke"
        )
    )).into()
}

type Content = View;
#[component]
pub(crate) fn content<T>(content: T) -> Content
where
    T: Into<View>,
{
    div().class("content").children(content).into()
}

type ContentSection = View;
#[component]
pub(crate) fn content_section<T>(content_section: T) -> ContentSection
where
    T: Into<View>,
{
    div()
        .class("content-section")
        .children(content_section)
        .into()
}

type SiteFooter = View;
#[component]
pub(crate) fn site_footer() -> SiteFooter {
    footer()
        .children(
            p().children((
                "© 2019-2025 nyefan.org - ",
                a().href("https://github.com/Nyefan/nyefan.org")
                    .children("This site on GitHub"),
            )),
        )
        .into()
}

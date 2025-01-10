use sfmacro::scrape_website_page;
fn main() {
    #[scrape_website_page(url="hello")]
    struct Page {
        title: String
    }

    let user = Page{title: "".into(), page_content: "".into()};
    todo!();
}
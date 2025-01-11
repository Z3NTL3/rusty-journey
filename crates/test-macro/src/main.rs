use sfmacro::scrape_website;

#[scrape_website(url="https://httpbin.org/anything")]
struct Page {
    #[allow(unused)]
    title: String
}

#[tokio::main]
async fn main() {
    let mut page = Page::default();
    page.scrape().await.unwrap();

    println!("{:?}", page.page_content);
}
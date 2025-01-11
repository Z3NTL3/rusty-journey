use sfmacro::scrape_website;

#[scrape_website(url="https://httpbin.org/uuid")]
struct Page {
    #[allow(unused)]
    title: String
}

#[tokio::main]
async fn main() {
    let mut page = Page::default();
    println!("{:?}", page.scrape().await.unwrap());
}
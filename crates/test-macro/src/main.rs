use sfmacro::scrape_website;

#[scrape_website(url="https://httpbin.org/uuid")]
struct Page<T> {
    #[allow(unused)]
    title: T
}

#[tokio::main]
async fn main() {
    let page = Page::<String>::default();
    println!("{:?}", page.scrape().await.unwrap());
}
// Futures and the Async Syntax

use trpl::Html;

fn main() {
    
}

async fn page_title(
    url: &str
) -> Option<String> {
    let response = trpl::get(url)
        .await
        .text()
        .await;
    // let response_text = response.text().await;
    Html::parse(&response)
        .select_first("title")
        .map(|title| title.inner_html())
}

// Futures and the Async Syntax

use trpl::{Either, Html};

// `main` function is 
// not allowed to be `async`
fn main() {
    let args: Vec<String> =
        std::env::args().collect();
    
    trpl::block_on(async {
        let titel_fut_1 = page_title(&args[1]);
        let titel_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(
                titel_fut_1, 
                titel_fut_2
            ).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!(
                "The title for {url} was {title}"
            ),
            None => println!("{url} had no title")
        }
    })

}

async fn page_title(
    url: &str
) -> (&str, Option<String>) {
    let response = trpl::get(url)
        .await
        .text()
        .await;
    // let response_text = response.text().await;
    let title = Html::parse(&response)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}

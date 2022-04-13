use pagination_example::github::{Client, IssueSearchParams, GITHUB_API_BASE};
use smol::stream::StreamExt;

fn main() {
    // search_issues();
    search_issues_iter();
}

/// This just tests the basics, to make sure that the requests and the
/// deserialization work correctly.
#[allow(dead_code)]
fn search_issues() {
    smol::block_on(async {
        let client = Client::new(GITHUB_API_BASE, None).unwrap();
        let params = IssueSearchParams::new(r#""bug" is:open"#);

        println!("{:#?}", client.search_issues(&params).await);
    });
}

/// This is the real test of the paginator.
#[allow(dead_code)]
fn search_issues_iter() {
    smol::block_on(async {
        let client = Client::new(GITHUB_API_BASE, None).unwrap();
        let params = IssueSearchParams::new(r#""bug" is:open"#);

        let mut stream = client.search_issues_iter(params);
        let mut count = 0_usize;

        while let Some(result) = stream.next().await {
            count += 1;

            match &result {
                Ok(item) => println!("{}: {}", item.id, item.html_url),
                Err(error) => eprintln!("Error encountered after {} items!\n{:#?}", count, error),
            }

            assert!(result.is_ok());
        }
    });
}

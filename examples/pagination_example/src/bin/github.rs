use pagination_example::github::{Client, Error, IssueSearchParams, GITHUB_API_BASE};
use smol::stream::StreamExt;

// GitHub may not serve the best for an example like this, as we hit the
// secondary rate limit. <https://docs.github.com/en/rest/overview/resources-in-the-rest-api#secondary-rate-limits>
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
            match &result {
                Ok(item) => println!("Issue {}: {}", item.id, item.html_url),
                Err(error) => {
                    eprintln!("Error encountered after {} items!", count);

                    match error {
                        Error::Request(error) => eprintln!("{:#?}", error),
                        Error::Deserialize { error, url, bytes } => {
                            eprintln!(
                                "URL\n{}\nError:\n{}\nData:\n{}",
                                url,
                                error,
                                std::str::from_utf8(bytes.as_slice()).unwrap()
                            )
                        }
                    }
                }
            }

            assert!(result.is_ok());
            count += 1;
        }
    });
}

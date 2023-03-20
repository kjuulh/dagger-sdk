mod container;
mod directory;
mod git;

use dagger_sdk::connect;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_err_message() {
    let client = connect().await.unwrap();

    let alpine = client.container().from("fake.invalid:latest").id().await;
    assert_eq!(alpine.is_err(), true);
    let err = alpine.expect_err("Tests expect err");

    let error_msg = r#"
GQLClient Error: Look at json field for more details
Message: pull access denied, repository does not exist or may require authorization: server message: insufficient_scope: authorization failed
"#;

    assert_eq!(err.to_string().as_str(), error_msg);
}

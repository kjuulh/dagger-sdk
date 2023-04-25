use dagger_sdk::{connect, ContainerExecOptsBuilder};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_example_container() {
    let client = connect().await.unwrap();

    let alpine = client.container().from("alpine:3.16.2");

    let out = alpine
        .exec_opts(
            ContainerExecOptsBuilder::default()
                .args(vec!["cat", "/etc/alpine-release"])
                .build()
                .unwrap(),
        )
        .stdout()
        .await
        .unwrap();

    assert_eq!(out, "3.16.2\n".to_string())
}

#[tokio::test]
async fn test_container() {
    let client = connect().await.unwrap();

    let alpine = client.container().from("alpine:3.16.2");

    let contents = alpine
        .fs()
        .file("/etc/alpine-release")
        .contents()
        .await
        .unwrap();
    assert_eq!(contents, "3.16.2\n".to_string());

    let out = alpine
        .exec_opts(
            ContainerExecOptsBuilder::default()
                .args(vec!["cat", "/etc/alpine-release"])
                .build()
                .unwrap(),
        )
        .stdout()
        .await
        .unwrap();
    assert_eq!(out, "3.16.2\n".to_string());

    let id = alpine.id().await.unwrap();
    let contents = client
        .container_opts(dagger_sdk::QueryContainerOpts {
            id: Some(id),
            platform: None,
        })
        .fs()
        .file("/etc/alpine-release")
        .contents()
        .await
        .unwrap();
    assert_eq!(contents, "3.16.2\n".to_string());
}

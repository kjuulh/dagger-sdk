use dagger_sdk::{connect, ContainerExecOptsBuilder};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_directory() {
    let c = connect().await.unwrap();

    let contents = c
        .directory()
        .with_new_file("/hello.txt", "world")
        .file("/hello.txt")
        .contents()
        .await
        .unwrap();

    assert_eq!("world", contents)
}

use dagger_sdk::connect;
use pretty_assertions::assert_eq;

static TEST_FILE_PATH: &str = "./data/";
static TEST_FILE_NAME: &str = "test.txt";
static TEST_FILE_CONTENT: &str = "This is a test file.";
static ALPINE_VERSION: &str = "3.16.2";

#[tokio::test]
async fn test_file_from_host() -> eyre::Result<()> {
    setup_file().await?;

    let client = connect().await.unwrap();

    let file = client.host().directory(TEST_FILE_PATH).file(TEST_FILE_NAME);
    let file_content = file.contents().await?;
    assert_eq!(file_content, TEST_FILE_CONTENT);

    let file_retrieved_by_id = client.file(file.id().await?);
    let file_retrieved_by_id_content = file_retrieved_by_id.contents().await?;
    assert_eq!(file_retrieved_by_id_content, TEST_FILE_CONTENT);

    cleanup_file().await
}

#[tokio::test]
async fn test_file_from_container() -> eyre::Result<()> {
    let client = connect().await.unwrap();

    let alpine = client.container().from(format!("alpine:{}", ALPINE_VERSION));
    let file = alpine.file("/etc/alpine-release");

    let file_content = file.contents().await?;
    let expected_file_content = format!("{}\n", ALPINE_VERSION);
    assert_eq!(file_content, expected_file_content); 

    let file_retrieved_by_id = client.file(file.id().await?);
    let file_retrieved_by_id_contents = file_retrieved_by_id.contents().await?;
    assert_eq!(file_retrieved_by_id_contents, expected_file_content);

    Ok(())
}

async fn setup_file() -> eyre::Result<()> {
    let full_file_path = String::from(TEST_FILE_PATH) + TEST_FILE_NAME;
    tokio::fs::create_dir(TEST_FILE_PATH)
        .await
        .map_err(eyre::Error::from)?;
    tokio::fs::write(full_file_path, TEST_FILE_CONTENT)
        .await
        .map_err(eyre::Error::from)
}

async fn cleanup_file() -> eyre::Result<()> {
    let full_file_path = String::from(TEST_FILE_PATH) + TEST_FILE_NAME;
    tokio::fs::remove_file(full_file_path)
        .await
        .map_err(eyre::Error::from)?;
    tokio::fs::remove_dir(TEST_FILE_PATH)
        .await
        .map_err(eyre::Error::from)
}
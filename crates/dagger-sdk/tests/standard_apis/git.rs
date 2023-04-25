use dagger_sdk::connect;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_git() {
    let c = connect().await.unwrap();

    let tree = c.git("github.com/dagger/dagger").branch("main").tree();

    let _ = tree
        .entries()
        .await
        .unwrap()
        .iter()
        .find(|f| f.as_str() == "README.md")
        .unwrap();

    let readme_file = tree.file("README.md");

    let readme = readme_file.contents().await.unwrap();
    assert_eq!(true, readme.find("Dagger").is_some());

    let readme_id = readme_file.id().await.unwrap();
    let other_readme = c.file(readme_id).contents().await.unwrap();

    assert_eq!(readme, other_readme);
}

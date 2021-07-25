use pixiv_rs::client::PixivClient;

mod common;

static SAFE_ILLUSTRATION_IDS: &[&str] = &[
    "15909278",
    "43663273",
    "42648936",
    "7758820"
];
static R18_ILLUSTRATION_IDS: &[&str] = &[
    "1878082",
    "78654878"
];
static INVALID_ILLUSTRATION_IDS: &[&str] = &[
    "56006815",
    ""
];

#[tokio::test]
async fn it_returns_safe_illustrations() {
    common::setup();
    let client = PixivClient::new();

    for id in SAFE_ILLUSTRATION_IDS {
        let illustration = client.illustration(id).await.unwrap();
        assert_eq!(illustration.id, *id);
        assert!(illustration.title.len() > 0);
        assert!(illustration.tags.tags.len() > 0);
        assert_eq!(illustration.age_restrict, 0);
    }
}

#[tokio::test]
async fn it_returns_r18_illustrations() {
    common::setup();
    let client = PixivClient::new();
    for id in R18_ILLUSTRATION_IDS {
        let illustration = client.illustration(id).await.unwrap();
        assert_eq!(illustration.id, *id);
        assert!(illustration.title.len() > 0);
        assert!(illustration.tags.tags.len() > 0);
        assert_eq!(illustration.age_restrict, 1);
    }
}

#[tokio::test]
async fn it_returns_errors_on_invalid_illustrations() {
    common::setup();
    let client = PixivClient::new();
    for id in INVALID_ILLUSTRATION_IDS {
        let result = client.illustration(id).await;
        assert!(result.is_err())
    }
}
# Pixiv for Rust

This crate allows calls to the pixiv internal ajax api to retrieve information about posts.

## Example

```rust
use pixiv_rs::client::PixivClient;

async fn main() {
    let illustration = client.illustration("43663273").await.unwrap();
    for tag in illustration.tags.tags {
        println!("{}", tag.tag);
    }
}
```

## License

Apache-2.0
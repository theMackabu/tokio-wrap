mod sync {
    #[tokio_wrap::sync]
    pub fn sync_function() -> String {
        async {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            "Hello, async world!".to_string()
        }
        .await
    }
}

fn main() {
    let result = sync::sync_function();
    assert_eq!(result, "Hello, async world!");
}

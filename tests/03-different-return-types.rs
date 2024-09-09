#[tokio_wrap::sync]
fn sync_function_returns_unit() {
    async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    .await
}

#[tokio_wrap::sync]
fn sync_function_returns_result() -> Result<String, &'static str> {
    async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        Ok("Success".to_string())
    }
    .await
}

fn main() {
    sync_function_returns_unit();
    let result = sync_function_returns_result().unwrap();
    assert_eq!(result, "Success");
}

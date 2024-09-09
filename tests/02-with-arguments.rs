#[tokio_wrap::sync]
fn sync_function_with_args(x: i32, y: i32) -> i32 {
    async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        x + y
    }
    .await
}

fn main() {
    let result = sync_function_with_args(5, 7);
    assert_eq!(result, 12);
}

use tokio_wrap::block as b;

fn test_block_usage() {
    let result = b! {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        42
    };

    assert_eq!(result, 42);
}

fn test_block_with_let() {
    let x = 10;
    let result = b! {
        let y = x + 5;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        y * 2
    };

    assert_eq!(result, 30);
}

fn main() {
    test_block_usage();
    test_block_with_let();
}

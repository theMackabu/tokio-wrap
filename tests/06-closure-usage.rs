use tokio_wrap::closure as c;

fn test_closure_usage() {
    let value = c!(() => {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        42
    });

    assert_eq!(value(), 42);
}

fn test_closure_with_args() {
    let multiply = c!((x: i32, y: i32) => {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        x * y
    });

    assert_eq!(multiply(6, 7), 42);
}

fn main() {
    test_closure_usage();
    test_closure_with_args();
}

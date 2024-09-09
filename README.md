# tokio-wrap

## Purpose

When working with asynchronous Rust code, especially with the Tokio runtime, you sometimes need to call async functions from synchronous contexts. This macro simplifies that process by automatically wrapping your function in a Tokio runtime, allowing you to use `await` syntax in otherwise synchronous functions.

## Usage

Here's a basic example of how to use the `tokio-wrap` macro:

```rust
#[tokio_wrap::sync]
async fn async_function() -> String {
	 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
	 "Hello, async world!".to_string()
}

#[tokio_wrap::sync]
fn sync_function() -> String {
	 async_function().await
}

fn main() {
	 let result = sync_function();
	 println!("Result: {}", result);
}
```

## Features

- Automatically wraps functions in a Tokio runtime
- Supports functions with and without arguments
- Handles different return types, including `Result`

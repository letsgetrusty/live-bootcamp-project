# Sprint 1, Task 1: Test Helpers

## Overview
Set up integration testing infrastructure for the auth service, including HTTP client setup and test helper functions.

## Key Concepts

### 1. Dev Dependencies vs Dependencies
```toml
[dependencies]
# Used in production code (src/)
serde = "1.0.228"

[dev-dependencies]
# Only used in tests, benchmarks, examples
reqwest = { version = "0.12.24", features = ["json"] }
```

**Why this matters:**
- Dev dependencies are NOT included in production binaries
- Smaller, faster production builds
- Clear separation of concerns
- In our case: auth service receives HTTP requests, doesn't make them (except in tests)

### 2. Test App Pattern
```rust
pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        // Build app with random port (127.0.0.1:0)
        // Spawn app in background task
        // Create HTTP client for making requests
    }
}
```

**Benefits:**
- Each test gets isolated app instance
- Tests can run in parallel
- Simulates real HTTP requests
- Clean, reusable test setup

### 3. Background Task Spawning
```rust
#[allow(clippy::let_underscore_future)]
let _ = tokio::spawn(app.run());
```

**Why `let _`:**
- We don't need to await the server
- Server runs in background during tests
- `#[allow(clippy::let_underscore_future)]` suppresses warning

### 4. Generic Helper Methods
```rust
pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
where
    Body: serde::Serialize,
{
    self.http_client
        .post(&format!("{}/signup", &self.address))
        .json(body)  // Automatically serializes to JSON
        .send()
        .await
        .expect("Failed to execute request.")
}
```

**Key points:**
- Generic over `Body` type
- Trait bound: `serde::Serialize`
- Works with any serializable type
- Clean, type-safe API

### 5. UUID for Test Data
```rust
pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
```

**Why random data:**
- Tests don't interfere with each other
- Can run tests multiple times
- No cleanup needed
- Prevents flaky tests

## Files Modified

1. **Cargo.toml**
   - Added reqwest as dev-dependency

2. **tests/api/helpers.rs**
   - Created TestApp struct
   - Implemented helper methods for all routes
   - Added get_random_email helper

3. **tests/api/routes.rs**
   - Added integration tests for all routes
   - Each test creates TestApp instance
   - Asserts HTTP status codes

4. **src/lib.rs**
   - Added route handlers (initially returning 200 OK)

## Common Patterns

### Test Structure
```rust
#[tokio::test]
async fn test_name() {
    // Arrange
    let app = TestApp::new().await;
    let test_data = serde_json::json!({ ... });

    // Act
    let response = app.post_route(&test_data).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}
```

### Parallel Tool Calls
When operations are independent, make calls in parallel:
```rust
// Read multiple files in parallel
Read(file1), Read(file2), Read(file3)
```

## Troubleshooting

### Issue: Tests timeout
**Cause:** Server not running in background
**Solution:** Verify `tokio::spawn(app.run())` is called

### Issue: Port already in use
**Cause:** Using fixed port instead of random
**Solution:** Use `127.0.0.1:0` to get random available port

### Issue: Test data conflicts
**Cause:** Using same email across tests
**Solution:** Use `get_random_email()` for unique test data

## Best Practices

1. **Isolation**: Each test should be independent
2. **Random Data**: Use UUIDs for test data
3. **Background Tasks**: Spawn server without awaiting
4. **Generic Helpers**: Make helpers work with any serializable type
5. **Clear Naming**: Helper methods should describe what they do

## Key Takeaways

- Integration tests simulate real HTTP requests
- Dev dependencies keep production builds lean
- Test helpers reduce code duplication
- Random test data prevents conflicts
- Tokio enables async testing

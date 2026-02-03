# Sprint 1, Task 6: JSON Validation (422 Status)

## Overview
Implement automatic JSON validation using Axum's Json extractor to return 422 Unprocessable Entity for malformed requests.

## Key Concepts

### 1. Axum Json Extractor
```rust
pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    // Axum automatically validates JSON here!
    // If validation fails, returns 422
}
```

**How it works:**
- Axum intercepts the request
- Attempts to deserialize JSON to SignupRequest
- If deserialization fails → automatic 422 response
- If successful → handler executes

### 2. Serde Serialization
```rust
#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]  // JSON uses camelCase
    pub requires_2fa: bool,            // Rust uses snake_case
}
```

**Key features:**
- `Deserialize` trait enables JSON → Rust
- `#[serde(rename)]` maps between naming conventions
- Type safety at compile time
- Automatic validation

### 3. Field Renaming
```rust
#[serde(rename = "requires2FA")]
pub requires_2fa: bool,
```

**Why rename:**
- JSON often uses camelCase
- Rust convention is snake_case
- Serde handles the conversion
- Both sides use their preferred style

### 4. Test Cases for 422
```rust
let test_cases = [
    serde_json::json!({
        // Missing email
        "password": "password123",
        "requires2FA": true
    }),
    serde_json::json!({
        // Missing password
        "email": "test@example.com",
        "requires2FA": true
    }),
];
```

**What triggers 422:**
- Missing required fields
- Wrong field types
- Invalid JSON syntax
- Extra fields (if using `#[serde(deny_unknown_fields)]`)

## Dependencies

### serde
```toml
serde = { version = "1.0.228", features = ["derive"] }
```
- Serialization/deserialization framework
- `derive` feature enables `#[derive(Serialize, Deserialize)]`
- Used for JSON ↔ Rust struct conversion

### serde_json
```toml
serde_json = "1.0.145"
```
- JSON-specific implementation
- Provides `json!` macro for test data
- Handles JSON encoding/decoding

### uuid
```toml
uuid = { version = "1.18.1", features = ["v4", "serde"] }
```
- Generate random UUIDs
- `v4` enables UUID v4 (random)
- `serde` enables UUID serialization

## Implementation Steps

### 1. Add Dependencies
```toml
[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
uuid = { version = "1.18.1", features = ["v4", "serde"] }
```

### 2. Create Request Struct
```rust
#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
```

### 3. Update Route Handler
```rust
pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    // Access validated data
    let email = request.email;
    let password = request.password;
    let requires_2fa = request.requires_2fa;

    // ... handle request
}
```

### 4. Update Test Helper
```rust
pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
where
    Body: serde::Serialize,  // Any serializable type
{
    self.http_client
        .post(&format!("{}/signup", &self.address))
        .json(body)  // Automatically serializes
        .send()
        .await
        .expect("Failed to execute request.")
}
```

### 5. Add Tests
```rust
#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({ "password": "test", "requires2FA": true }),
        serde_json::json!({ "email": "test@test.com", "requires2FA": true }),
        serde_json::json!({ "email": "test@test.com", "password": "test" }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}
```

## Validation Levels

### 1. JSON Syntax (Automatic)
Axum validates JSON is well-formed:
```json
// Valid JSON
{"email": "test@test.com"}

// Invalid JSON (missing quote)
{email: "test@test.com"}  // 422
```

### 2. Schema Validation (Automatic)
Serde validates against struct:
```rust
struct SignupRequest {
    email: String,  // Required field
}

// Missing required field → 422
{"password": "test"}
```

### 3. Type Validation (Automatic)
Serde validates types:
```rust
struct SignupRequest {
    requires_2fa: bool,  // Expects boolean
}

// Wrong type → 422
{"requires_2fa": "true"}  // String, not bool
```

### 4. Business Logic (Manual - Next Tasks)
Custom validation in handler:
```rust
if email.is_empty() || !email.contains('@') {
    return Err(AuthAPIError::InvalidCredentials);  // 400
}
```

## Common Patterns

### Generic Body Parameter
```rust
pub async fn post<Body>(&self, body: &Body) -> Response
where
    Body: serde::Serialize,
{
    // Works with any serializable type
}
```

### Test Data with json! Macro
```rust
let data = serde_json::json!({
    "email": "test@example.com",
    "password": "password123",
    "requires2FA": true
});
```

### Random Test Data
```rust
let email = format!("{}@example.com", Uuid::new_v4());
```

## Troubleshooting

### Issue: Always getting 422
**Cause:** JSON field names don't match struct
**Solution:** Use `#[serde(rename)]` or fix JSON field names

### Issue: Tests fail with deserialization error
**Cause:** Test data doesn't match expected schema
**Solution:** Check all required fields are present

### Issue: Can't use camelCase in Rust
**Cause:** Rust lints prefer snake_case
**Solution:** Use `#[serde(rename)]` to keep both conventions

## Best Practices

1. **Use Extractors**: Let Axum handle validation
2. **Type Safety**: Define structs for all requests/responses
3. **Rename Fields**: Map between language conventions
4. **Test Edge Cases**: Missing fields, wrong types, etc.
5. **Clear Errors**: 422 indicates client error, not server error

## Key Takeaways

- Axum's Json extractor provides automatic validation
- Serde handles serialization/deserialization
- 422 status indicates unprocessable entity (malformed JSON)
- Field renaming allows different naming conventions
- Type safety catches errors at compile time
- Test edge cases to ensure validation works

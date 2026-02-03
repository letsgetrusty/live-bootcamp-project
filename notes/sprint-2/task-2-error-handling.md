# Sprint 2, Task 2: Error Handling (400, 409, 500)

## Overview
Implement comprehensive error handling with custom error types, validation logic, and proper HTTP status codes for different error scenarios.

## Key Concepts

### 1. Custom Error Enum
```rust
pub enum AuthAPIError {
    UserAlreadyExists,      // 409 Conflict
    InvalidCredentials,     // 400 Bad Request
    UnexpectedError,        // 500 Internal Server Error
}
```

**Benefits:**
- Type-safe error handling
- Clear error semantics
- Easy to extend
- Compiler enforces handling

### 2. IntoResponse Trait
```rust
impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
```

**How it works:**
- Map error variants to HTTP status codes
- Create JSON error response
- Axum automatically calls this when you return an error
- Consistent error format

### 3. Result Type in Handlers
```rust
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {  // Returns Result!
    // Validation
    if email.is_empty() || !email.contains('@') {
        return Err(AuthAPIError::InvalidCredentials);  // Early return
    }

    // Success case
    Ok((StatusCode::CREATED, response))
}
```

**Benefits:**
- Early returns for errors
- Question mark operator (?)
- Type safety
- Clear success/failure paths

### 4. Validation Patterns

#### Email Validation
```rust
if email.is_empty() || !email.contains('@') {
    return Err(AuthAPIError::InvalidCredentials);
}
```

#### Password Validation
```rust
if password.len() < 8 {
    return Err(AuthAPIError::InvalidCredentials);
}
```

#### Duplicate Check
```rust
match user_store.add_user(user).await {
    Ok(_) => { /* success */ },
    Err(UserStoreError::UserAlreadyExists) => {
        return Err(AuthAPIError::UserAlreadyExists)
    },
    Err(_) => return Err(AuthAPIError::UnexpectedError),
}
```

## HTTP Status Codes

| Status | Code | Meaning | Use Case |
|--------|------|---------|----------|
| OK | 200 | Success | General success |
| Created | 201 | Resource created | New user created |
| Bad Request | 400 | Invalid input | Validation failure |
| Conflict | 409 | Resource exists | Duplicate email |
| Unprocessable Entity | 422 | Malformed JSON | JSON parsing error |
| Internal Server Error | 500 | Server error | Unexpected errors |

## Implementation Steps

### 1. Create Error Enum
```rust
// src/domain/error.rs
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}
```

### 2. Create Error Response Struct
```rust
// src/lib.rs
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
```

### 3. Implement IntoResponse
```rust
impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists =>
                (StatusCode::CONFLICT, "User already exists"),
            // ... other variants
        };

        (status, Json(ErrorResponse {
            error: error_message.to_string()
        })).into_response()
    }
}
```

### 4. Add Validation to Route
```rust
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // Email validation
    if email.is_empty() || !email.contains('@') {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Password validation
    if password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    // Handle storage errors
    match user_store.add_user(user).await {
        Ok(_) => {
            Ok((StatusCode::CREATED, Json(SignupResponse {
                message: "User created successfully!".to_string(),
            })))
        }
        Err(UserStoreError::UserAlreadyExists) =>
            Err(AuthAPIError::UserAlreadyExists),
        Err(_) =>
            Err(AuthAPIError::UnexpectedError),
    }
}
```

### 5. Add Tests for Error Cases
```rust
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "",  // Empty email
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "invalid",  // No @
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "test@test.com",
            "password": "short",  // < 8 chars
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 400);

        let error_response = response.json::<ErrorResponse>().await.unwrap();
        assert_eq!(error_response.error, "Invalid credentials");
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let email = get_random_email();

    let body = serde_json::json!({
        "email": email,
        "password": "password123",
        "requires2FA": true
    });

    // First signup - should succeed
    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Second signup - should fail with 409
    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 409);

    let error_response = response.json::<ErrorResponse>().await.unwrap();
    assert_eq!(error_response.error, "User already exists");
}
```

## Error Handling Patterns

### Pattern Matching
```rust
match user_store.add_user(user).await {
    Ok(_) => { /* handle success */ },
    Err(UserStoreError::UserAlreadyExists) => { /* handle duplicate */ },
    Err(_) => { /* handle unexpected */ },
}
```

### Early Returns
```rust
if !is_valid(email) {
    return Err(AuthAPIError::InvalidCredentials);
}
// Continue with happy path
```

### Question Mark Operator
```rust
let user = user_store.get_user(email)?;  // Auto-converts error types
```

## Validation Layers

### Layer 1: JSON Schema (422)
- Handled by Axum
- Malformed JSON
- Missing required fields
- Type mismatches

### Layer 2: Business Logic (400)
- Email format validation
- Password requirements
- Custom business rules

### Layer 3: Data Integrity (409)
- Unique constraints
- Foreign key constraints
- Race conditions

### Layer 4: Unexpected (500)
- Database failures
- Network errors
- Unknown errors

## Testing Strategy

### Test Each Error Scenario
1. **400 Bad Request**
   - Empty email
   - Invalid email format
   - Short password

2. **409 Conflict**
   - Duplicate email signup

3. **422 Unprocessable Entity**
   - Missing fields
   - Wrong types

4. **201 Created**
   - Valid input (happy path)

### Assert Response Body
```rust
let error_response = response
    .json::<ErrorResponse>()
    .await
    .expect("Could not deserialize error response");

assert_eq!(error_response.error, "User already exists");
```

## Common Patterns

### Mapping Between Error Types
```rust
match storage_error {
    UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
    UserStoreError::InvalidCredentials => AuthAPIError::InvalidCredentials,
    _ => AuthAPIError::UnexpectedError,
}
```

### Custom Error Response
```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
```

## Troubleshooting

### Issue: Error not returning correct status code
**Cause:** IntoResponse not implemented correctly
**Solution:** Verify match arms map to correct StatusCode

### Issue: Can't deserialize error response in tests
**Cause:** ErrorResponse struct not public or missing derives
**Solution:** Add `pub` and `#[derive(Serialize, Deserialize)]`

### Issue: All errors return 500
**Cause:** Not propagating custom errors
**Solution:** Use `Result<impl IntoResponse, AuthAPIError>`

## Best Practices

1. **Specific Error Types**: Create domain-specific errors
2. **Clear Messages**: Error messages should be actionable
3. **Consistent Format**: Use same JSON structure for all errors
4. **Test All Paths**: Test success and all error scenarios
5. **Early Validation**: Validate input before processing
6. **Map Errors**: Convert internal errors to API errors
7. **Don't Leak Details**: Don't expose internal errors to users

## Key Takeaways

- Custom error enums provide type-safe error handling
- IntoResponse trait enables automatic HTTP response generation
- Different status codes indicate different error types
- Validation should happen in layers (JSON → Business → Data)
- Pattern matching handles different error scenarios
- Result types make error handling explicit
- Comprehensive testing ensures robust error handling

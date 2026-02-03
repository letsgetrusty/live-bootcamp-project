# Sprint 2, Task 1: User Creation (201 Status)

## Overview
Implement complete user signup functionality with data persistence, state management, and proper HTTP responses.

## Key Concepts

### 1. Domain Layer
```rust
// domain/user.rs
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}
```

**Purpose:**
- Core business entities
- Framework-independent
- Reusable across application
- Clear data model

### 2. Service Layer
```rust
// services/hashmap_user_store.rs
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
}
```

**Purpose:**
- External service implementations
- Data storage logic
- Separated from business logic
- Testable in isolation

### 3. Application State
```rust
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}
```

**Purpose:**
- Shared state across routes
- Thread-safe with Arc<RwLock<T>>
- Cloneable for Axum

### 4. Smart Pointers

#### Arc (Atomic Reference Counting)
```rust
Arc<T>  // Shared ownership, thread-safe
```
- Multiple owners of same data
- Thread-safe reference counting
- When last Arc is dropped, data is cleaned up

#### RwLock (Read-Write Lock)
```rust
RwLock<T>  // Multiple readers OR one writer
```
- Many concurrent readers
- Exclusive write access
- Prevents data races

#### Combined Pattern
```rust
Arc<RwLock<HashmapUserStore>>
```
- **Arc**: Shared ownership across threads
- **RwLock**: Safe concurrent read/write
- **Result**: Thread-safe, mutable shared state

### 5. Axum State Extractor
```rust
pub async fn signup(
    State(state): State<AppState>,  // Extract shared state
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user)?;
}
```

**How it works:**
- Axum clones AppState for each request
- Arc::clone just increments counter (cheap!)
- Access via `.read()` or `.write()`
- Async-aware locking

## Architecture Layers

```
┌─────────────────────────────────────┐
│      Presentation (Routes)          │
│  - signup route handler             │
│  - State extraction                 │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│        Application State             │
│  - AppState                          │
│  - Shared across routes             │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│         Service Layer                │
│  - HashmapUserStore                 │
│  - Data operations                  │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│         Domain Layer                 │
│  - User entity                      │
│  - Business rules                   │
└─────────────────────────────────────┘
```

## Implementation Steps

### 1. Create Domain Models
```rust
// src/domain/user.rs
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self { email, password, requires_2fa }
    }
}
```

### 2. Create User Store
```rust
// src/services/hashmap_user_store.rs
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }
}
```

### 3. Create Application State
```rust
// src/app_state.rs
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}
```

### 4. Configure Router with State
```rust
// src/lib.rs
impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route("/signup", post(routes::signup))
            .with_state(app_state);  // Attach state to router
        // ...
    }
}
```

### 5. Update Main to Create State
```rust
// src/main.rs
#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
```

### 6. Implement Signup Route
```rust
// src/routes/signup.rs
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}
```

## Testing Strategy

### Unit Tests
```rust
#[tokio::test]
async fn test_add_user() {
    let mut store = HashmapUserStore::default();
    let user = User::new("test@test.com".to_string(), "pass".to_string(), false);

    assert!(store.add_user(user.clone()).is_ok());
    assert_eq!(store.add_user(user), Err(UserStoreError::UserAlreadyExists));
}
```

### Integration Tests
```rust
#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 201);
}
```

## Common Patterns

### Accessing Shared State
```rust
// Read access (multiple concurrent readers)
let user_store = state.user_store.read().await;
let user = user_store.get_user("email")?;

// Write access (exclusive access)
let mut user_store = state.user_store.write().await;
user_store.add_user(user)?;
```

### HTTP Status Codes
```rust
StatusCode::CREATED  // 201 - Resource created
StatusCode::OK       // 200 - Success
```

### Response Tuples
```rust
(StatusCode::CREATED, Json(response))  // Status + Body
```

## Troubleshooting

### Issue: Cannot borrow as mutable
**Cause:** Using `.read()` instead of `.write()`
**Solution:** Use `.write().await` for mutations

### Issue: Tests interfere with each other
**Cause:** Sharing state across tests
**Solution:** Each test creates new TestApp (isolated state)

### Issue: Deadlock
**Cause:** Holding lock while calling async function
**Solution:** Drop lock before awaiting

## Best Practices

1. **Separation of Concerns**: Domain, Service, Application layers
2. **Type Safety**: Use strong types (User, not HashMap)
3. **Error Handling**: Return Result types
4. **Thread Safety**: Use Arc<RwLock<T>> for shared state
5. **Test Isolation**: Each test gets fresh state
6. **Async Aware**: Use Tokio's RwLock, not std::sync

## Key Takeaways

- Arc<RwLock<T>> enables thread-safe shared mutable state
- Layered architecture improves maintainability
- Axum's State extractor provides access to shared state
- 201 Created indicates successful resource creation
- Domain models are framework-independent
- Test isolation prevents flaky tests

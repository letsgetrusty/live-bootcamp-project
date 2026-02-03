# Sprint 2, Task 3: Dependency Inversion Principle

## Overview
Refactor the application to follow the Dependency Inversion Principle (SOLID) by introducing trait abstractions for the user store, enabling easy swapping of implementations.

## Key Concepts

### 1. Dependency Inversion Principle (DIP)
> "High-level modules should not depend on low-level modules. Both should depend on abstractions."

**Before (Tight Coupling):**
```rust
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;  // Concrete type
pub struct AppState {
    pub user_store: UserStoreType,
}
```
- AppState depends on HashmapUserStore (concrete implementation)
- Hard to swap implementations
- Tightly coupled

**After (Loose Coupling):**
```rust
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;  // Trait object
pub struct AppState {
    pub user_store: UserStoreType,
}
```
- AppState depends on UserStore trait (abstraction)
- Easy to swap implementations
- Loosely coupled

### 2. Trait Abstraction
```rust
#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError>;
}
```

**Benefits:**
- Defines contract for all user stores
- Multiple implementations can coexist
- Testable with mock implementations
- Clear API boundary

### 3. async-trait Crate
```rust
#[async_trait::async_trait]
pub trait UserStore {
    async fn method(&self) -> Result<T>;
}
```

**Why needed:**
- Async fn in traits don't work with trait objects (yet)
- async-trait provides a workaround
- Enables `dyn UserStore` to be used
- Macro handles the complexity

### 4. Trait Objects
```rust
// Static dispatch (compile-time)
fn process<T: UserStore>(store: T) { }

// Dynamic dispatch (runtime) - What we use
fn process(store: Box<dyn UserStore>) { }
```

**Trait Objects:**
- `dyn UserStore` - dynamic dispatch
- Size unknown at compile time
- Need pointer: `Box`, `Arc`, or `&`
- Runtime polymorphism

### 5. Send + Sync Bounds
```rust
Arc<RwLock<dyn UserStore + Send + Sync>>
```

**Why needed:**
- `Send`: Can be sent across thread boundaries
- `Sync`: Can be shared across threads
- Required for async/multi-threaded usage
- Compiler enforces thread safety

## Implementation Steps

### 1. Add async-trait Dependency
```toml
[dependencies]
async-trait = "0.1.89"
```

### 2. Create UserStore Trait
```rust
// src/domain/data_stores.rs
use super::User;

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &str, password: &str)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
```

### 3. Implement Trait for HashmapUserStore
```rust
// src/services/hashmap_user_store.rs
use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str)
        -> Result<(), UserStoreError> {
        let user = self.users.get(email)
            .ok_or(UserStoreError::UserNotFound)?;

        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}
```

### 4. Update AppState to Use Trait
```rust
// src/app_state.rs
use crate::domain::UserStore;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}
```

### 5. Update Instantiation
```rust
// src/main.rs
let user_store: Arc<RwLock<dyn UserStore + Send + Sync>> =
    Arc::new(RwLock::new(HashmapUserStore::default()));
let app_state = AppState::new(user_store);
```

### 6. Update Route to Await Async Methods
```rust
// src/routes/signup.rs
match user_store.add_user(user).await {  // .await added!
    Ok(_) => { /* success */ },
    Err(UserStoreError::UserAlreadyExists) => { /* handle error */ },
}
```

## Architecture Transformation

### Before
```
┌─────────────────────────────────────┐
│       AppState                      │
│       ↓ (depends on)                │
│   HashmapUserStore (concrete)       │
└─────────────────────────────────────┘
```

### After
```
┌─────────────────────────────────────┐
│       AppState                      │
│       ↓ (depends on)                │
│   UserStore trait (abstraction)     │
│       ↑ (implemented by)            │
│   HashmapUserStore                  │
│   PostgresUserStore (future)        │
│   MongoUserStore (future)           │
└─────────────────────────────────────┘
```

## Benefits of Dependency Inversion

### 1. Easy to Swap Implementations
```rust
// Switch to Postgres - only 1 line changes!
let user_store: Arc<RwLock<dyn UserStore + Send + Sync>> =
    Arc::new(RwLock::new(PostgresUserStore::new()));
```

### 2. Testable with Mocks
```rust
struct MockUserStore;

#[async_trait::async_trait]
impl UserStore for MockUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Mock implementation for testing
        Ok(())
    }
    // ... other methods
}
```

### 3. Multiple Implementations Coexist
```rust
struct HashmapUserStore { /* ... */ }
struct PostgresUserStore { /* ... */ }
struct RedisUserStore { /* ... */ }
struct S3UserStore { /* ... */ }

// All implement the same trait!
```

### 4. Framework Independence
```rust
// Domain trait doesn't depend on framework
pub trait UserStore {
    // No Axum, no Tokio in trait definition
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
}
```

## Testing Strategy

### Unit Tests Still Work
```rust
#[tokio::test]
async fn test_add_user() {
    let mut store = HashmapUserStore::default();
    let user = User::new("test@test.com".to_string(), "pass".to_string(), false);

    assert!(store.add_user(user.clone()).await.is_ok());
    assert_eq!(store.add_user(user).await, Err(UserStoreError::UserAlreadyExists));
}
```

### Integration Tests Use Trait
```rust
let user_store: Arc<RwLock<dyn UserStore + Send + Sync>> =
    Arc::new(RwLock::new(HashmapUserStore::default()));
```

## Common Patterns

### Trait Object Construction
```rust
// Type annotation on left
let store: Arc<RwLock<dyn UserStore + Send + Sync>> =
    Arc::new(RwLock::new(HashmapUserStore::default()));

// OR explicit cast
let store = Arc::new(RwLock::new(
    HashmapUserStore::default() as Box<dyn UserStore + Send + Sync>
));
```

### Async Trait Methods
```rust
#[async_trait::async_trait]
impl UserStore for MyStore {
    async fn method(&self) -> Result<T> {
        // Can use .await inside
        some_async_fn().await
    }
}
```

### Calling Trait Methods
```rust
let mut store = state.user_store.write().await;
let result = store.add_user(user).await;  // Must await async methods
```

## Troubleshooting

### Issue: Trait bound not satisfied
**Cause:** Missing `+ Send + Sync` bounds
**Solution:** Add bounds: `dyn UserStore + Send + Sync`

### Issue: async fn in traits not supported
**Cause:** Rust doesn't fully support async in trait objects yet
**Solution:** Use `#[async_trait::async_trait]` attribute

### Issue: Forgot to await
**Cause:** async methods return Future, must be awaited
**Solution:** Add `.await` to all trait method calls

### Issue: Size of trait object unknown
**Cause:** Trait objects have dynamic size
**Solution:** Use pointer: `Box`, `Arc`, or `&dyn Trait`

## Best Practices

1. **Depend on Abstractions**: Use traits for dependencies
2. **Trait Objects for Runtime Polymorphism**: When type unknown at compile time
3. **async-trait for Async Methods**: Required for trait objects
4. **Send + Sync Bounds**: Required for multi-threaded usage
5. **Keep Traits Focused**: Single Responsibility Principle
6. **Document Trait Contract**: Clear expectations for implementers

## SOLID Principles Comparison

### Before (Violated DIP)
- AppState → HashmapUserStore (concrete)
- High-level depends on low-level
- Hard to extend

### After (Follows DIP)
- AppState → UserStore (abstraction) ← HashmapUserStore
- Both depend on abstraction
- Easy to extend

## Future Extensions

With this architecture, adding new stores is easy:

```rust
// PostgreSQL implementation
struct PostgresUserStore {
    pool: PgPool,
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query("INSERT INTO users ...")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    // ... other methods
}

// Switch to Postgres (1 line change in main.rs!)
let user_store: Arc<RwLock<dyn UserStore + Send + Sync>> =
    Arc::new(RwLock::new(PostgresUserStore::new(pool)));
```

## Key Takeaways

- Dependency Inversion Principle promotes loose coupling
- Traits define contracts for behavior
- Trait objects enable runtime polymorphism
- async-trait enables async methods in trait objects
- Send + Sync bounds ensure thread safety
- Easy to swap implementations without changing dependent code
- Future-proof architecture for growth
- SOLID principles lead to maintainable code

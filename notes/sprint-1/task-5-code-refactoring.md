# Sprint 1, Task 5: Code Refactoring

## Overview
Refactor the codebase to improve organization by splitting routes and tests into separate modules.

## Key Concepts

### 1. Module Organization
```
src/
├── routes/
│   ├── mod.rs          # Module declarations and re-exports
│   ├── signup.rs       # Signup route handler
│   ├── login.rs        # Login route handler
│   └── ...
```

**Benefits:**
- Each route in its own file
- Easier to find and modify code
- Scales better as project grows
- Clear separation of concerns

### 2. Module Pattern
```rust
// mod.rs
mod signup;
mod login;

pub use signup::*;
pub use login::*;
```

**Key points:**
- `mod signup;` declares the module
- `pub use signup::*;` re-exports everything
- Parent module controls what's public
- Clean imports for consumers

### 3. Test Organization
```
tests/api/
├── main.rs           # Module declarations
├── helpers.rs        # Shared test utilities
├── signup.rs         # Signup tests
├── login.rs          # Login tests
└── ...
```

**Benefits:**
- Tests are organized by feature
- Easy to find relevant tests
- Clear test output (e.g., `signup::should_return_201`)
- Matches production code structure

## Refactoring Process

### Step 1: Create Route Modules
```bash
mkdir src/routes
touch src/routes/mod.rs
touch src/routes/signup.rs
# ... create other route files
```

### Step 2: Move Route Handlers
**Before (lib.rs):**
```rust
async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
```

**After (routes/signup.rs):**
```rust
use axum::{response::IntoResponse, http::StatusCode};

pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
```

### Step 3: Update Module Declarations
**routes/mod.rs:**
```rust
mod signup;
// ... other modules

pub use signup::*;
// ... other re-exports
```

**lib.rs:**
```rust
pub mod routes;

// In router setup:
.route("/signup", post(routes::signup))
```

### Step 4: Organize Tests
**Before (routes.rs):**
```rust
#[tokio::test]
async fn signup_returns_200() { ... }

#[tokio::test]
async fn login_returns_200() { ... }
```

**After (signup.rs, login.rs):**
```rust
// signup.rs
#[tokio::test]
async fn should_return_201() { ... }

// login.rs
#[tokio::test]
async fn should_return_200() { ... }
```

## Files Created

### Production Code
- `src/routes/mod.rs`
- `src/routes/signup.rs`
- `src/routes/login.rs`
- `src/routes/logout.rs`
- `src/routes/verify_2fa.rs`
- `src/routes/verify_token.rs`

### Test Code
- `tests/api/root.rs`
- `tests/api/signup.rs`
- `tests/api/login.rs`
- `tests/api/logout.rs`
- `tests/api/verify_2fa.rs`
- `tests/api/verify_token.rs`

## Common Patterns

### Re-exporting Pattern
```rust
// mod.rs
mod internal_module;

// Re-export everything
pub use internal_module::*;

// OR re-export specific items
pub use internal_module::{PublicThing, AnotherPublicThing};
```

### Nested Modules
```rust
// Flat structure
src/
├── routes/
│   ├── mod.rs
│   └── signup.rs

// Nested structure (if needed)
src/
├── routes/
│   ├── mod.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── signup.rs
│   │   └── login.rs
│   └── user/
│       └── ...
```

## Test Output Improvements

**Before:**
```
test routes::signup_returns_200 ... ok
test routes::login_returns_200 ... ok
```

**After:**
```
test signup::should_return_201 ... ok
test login::should_return_200 ... ok
```

The module name now clearly indicates which feature is being tested!

## Troubleshooting

### Issue: Module not found
**Cause:** Missing `mod` declaration in parent module
**Solution:** Add `mod module_name;` to parent's mod.rs or main file

### Issue: Items not accessible
**Cause:** Not re-exported with `pub use`
**Solution:** Add `pub use module_name::*;` to make items public

### Issue: Rust-analyzer cache issues
**Cause:** Language server hasn't picked up file changes
**Solution:** Run "Rust Analyzer: Reload Workspace" command

## Best Practices

1. **One Feature Per File**: Each file should handle one concern
2. **Clear Naming**: File names should match what's inside
3. **Consistent Structure**: Production code and tests should mirror each other
4. **Re-export Pattern**: Use mod.rs for module declarations
5. **Flat When Possible**: Don't over-nest modules

## Key Takeaways

- Module organization improves code maintainability
- `mod.rs` is the entry point for a module directory
- Re-exports control the public API
- Test organization should mirror production code
- Refactoring improves scalability without changing behavior

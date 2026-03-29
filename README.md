# ferrox-traits

Shared traits for the ferrox ecosystem. Zero dependencies on Leptos or any
runtime library — safe to use at any layer of the stack, including shared
domain/DTO crates.

The derive macros for these traits live in
[ferrox-webapp-macros](https://github.com/rbenitez22/ferrox-webapp-macros),
but are re-exported here so a single dependency and a single import gives you
both the trait and the derive macro — the same pattern serde uses.

---

## Setup

```toml
[dependencies]
ferrox-traits = { git = "https://github.com/rbenitez22/ferrox-traits" }
# or local path during co-development:
# ferrox-traits = { path = "../../ferrox-traits" }
```

---

## Traits

### `HasId`

A type that can identify itself with a `String` id. Used by `webapp-lib` for
URL construction, list updates, and delete operations.

```rust
use ferrox_traits::{HasId, HasName};

// Derive — defaults to a field named `id`
#[derive(Clone, HasId)]
pub struct ShoppingList {
    pub id:   String,
    pub name: String,
}

// Override field name
#[derive(Clone, HasId)]
#[has_id(field = "email")]
pub struct UserAccountRequest {
    pub email:        String,
    pub display_name: String,
}

// Manual implementation
impl HasId for ShoppingList {
    fn get_id(&self) -> String { self.id.clone() }
}
```

**Requirements:**
- The target field must be of type `String`.
- `HasId` must be in scope (provided by this crate).

---

### `HasName`

A type that can provide a display name. Used by `webapp-lib` for default
sorting and rendering `<datalist>` options.

```rust
use ferrox_traits::{HasId, HasName};

// Derive — defaults to a field named `name`
#[derive(Clone, HasName)]
pub struct ShoppingList {
    pub id:   String,
    pub name: String,
}

// Override field name
#[derive(Clone, HasName)]
#[has_name(field = "display_name")]
pub struct UserAccount {
    pub id:           String,
    pub display_name: String,
}
```

**Requirements:**
- The target field must be of type `String`.
- `HasName` must be in scope (provided by this crate).

---

## Combining derives

```rust
use ferrox_traits::{HasId, HasName};

#[derive(Clone, HasId, HasName)]
#[has_name(field = "display_name")]
pub struct UserAccount {
    pub id:           String,
    pub display_name: String,
    pub email:        String,
}
```

---

## Dependency chain

```
ferrox-webapp-macros   ← proc-macro only (syn, quote, proc-macro2)
        ↑
ferrox-traits          ← this crate; defines traits, re-exports derives
        ↑
   shared / domain     ← derive HasId/HasName on your types here
   webapp-lib          ← re-exports from ferrox-traits for UI consumers
```

`shared` only needs `ferrox-traits`. It does not need `webapp-lib` or Leptos.

---

## Planned traits

| Trait | Purpose |
|---|---|
| `HasId` | String identity — implemented |
| `HasName` | Display name — implemented |
| `Auditable` | Before/after snapshot for audit trails |
| `Validatable` | Field-level validation, `validate() -> Result` |


// ---------------------------------------------------------------------------
// HasId
//
// Identifies a type by a String id.
//
// Derive with #[derive(HasId)]. Defaults to a field named `id`.
// Override with #[has_id(field = "other_field")].
// ---------------------------------------------------------------------------

pub trait HasId {
    fn get_id(&self) -> String;
}

// ---------------------------------------------------------------------------
// HasName
//
// Provides a display name for a type.
//
// Derive with #[derive(HasName)]. Defaults to a field named `name`.
// Override with #[has_name(field = "other_field")].
// ---------------------------------------------------------------------------

pub trait HasName {
    fn get_name(&self) -> String;
}

// ---------------------------------------------------------------------------
// HasParentId
//
// Identifies a type that belongs to an optional parent, referenced by id.
//
// Derive with #[derive(HasParentId)]. Defaults to a field named `parent_id`.
// Override with #[has_parent_id(field = "other_field")].
// ---------------------------------------------------------------------------

pub trait HasParentId {
    fn get_parent_id(&self) -> Option<String>;
}

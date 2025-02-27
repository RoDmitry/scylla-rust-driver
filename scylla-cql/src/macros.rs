/// #[derive(FromRow)] derives FromRow for struct
/// Works only on simple structs without generics etc
pub use scylla_macros::FromRow;

/// #[derive(FromUserType)] allows to parse struct as a User Defined Type
/// Works only on simple structs without generics etc
pub use scylla_macros::FromUserType;

/// #[derive(IntoUserType)] allows to pass struct a User Defined Type Value in queries
/// Works only on simple structs without generics etc
pub use scylla_macros::IntoUserType;

/// #[derive(ValueList)] allows to pass struct as a list of values for a query
pub use scylla_macros::ValueList;

/// Derive macro for the [`SerializeCql`](crate::types::serialize::value::SerializeCql) trait
/// which serializes given Rust structure as a User Defined Type (UDT).
///
/// At the moment, only structs with named fields are supported.
///
/// Serialization will fail if there are some fields in the Rust struct that don't match
/// to any of the UDT fields.
///
/// If there are fields in UDT that are not present in Rust definition:
/// - serialization will succeed in "match_by_name" flavor (default). Missing
///   fields in the middle of UDT will be sent as NULLs, missing fields at the end will not be sent
///   at all.
/// - serialization will succed if suffix of UDT fields is missing. If there are missing fields in the
///   middle it will fail. Note that if "skip_name_checks" is enabled, and the types happen to match,
///   it is possible for serialization to succeed with unexpected result.
/// This behavior is the default to support ALTERing UDTs by adding new fields.
/// You can require exact match of fields using `force_exact_match` attribute.
///
/// In case of failure, either [`BuiltinTypeCheckError`](crate::types::serialize::value::BuiltinTypeCheckError)
/// or [`BuiltinSerializationError`](crate::types::serialize::value::BuiltinSerializationError)
/// will be returned.
///
/// # Example
///
/// A UDT defined like this:
///
/// ```notrust
/// CREATE TYPE ks.my_udt (a int, b text, c blob);
/// ```
///
/// ...can be serialized using the following struct:
///
/// ```rust
/// # use scylla_cql::macros::SerializeCql;
/// #[derive(SerializeCql)]
/// # #[scylla(crate = scylla_cql)]
/// struct MyUdt {
///     a: i32,
///     b: Option<String>,
///     // No "c" field - it is not mandatory by default for all fields to be present
/// }
/// ```
///
/// # Struct attributes
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach serialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   fields in the UDT. During serialization, the implementation will take
///   care to serialize the fields in the order which the database expects.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the fields in the UDT.
///   If the order is incorrect, type checking/serialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///
/// `#[scylla(crate = crate_name)]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`SerializeCql`](crate::types::serialize::value::SerializeCql) trait
/// using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::SerializeCql;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
/// `#[scylla(skip_name_checks)]`
///
/// _Specific only to the `enforce_order` flavor._
///
/// Skips checking Rust field names against names of the UDT fields. With this
/// annotation, the generated implementation will allow mismatch between Rust
/// struct field names and UDT field names, i.e. it's OK if i-th field has a
/// different name in Rust and in the UDT. Fields are still being type-checked.
///
/// `#[scylla(force_exact_match)]`
///
/// Forces Rust struct to have all the fields present in UDT, otherwise
/// serialization fails.
///
/// # Field attributes
///
/// `#[scylla(rename = "name_in_the_udt")]`
///
/// Serializes the field to the UDT struct field with given name instead of
/// its Rust name.
pub use scylla_macros::SerializeCql;

/// Derive macro for the [`SerializeRow`](crate::types::serialize::row::SerializeRow) trait
/// which serializes given Rust structure into bind markers for a CQL statement.
///
/// At the moment, only structs with named fields are supported.
///
/// Serialization will fail if there are some bind markers/columns in the statement
/// that don't match to any of the Rust struct fields, _or vice versa_.
///
/// In case of failure, either [`BuiltinTypeCheckError`](crate::types::serialize::row::BuiltinTypeCheckError)
/// or [`BuiltinSerializationError`](crate::types::serialize::row::BuiltinSerializationError)
/// will be returned.
///
/// # Example
///
/// A UDT defined like this:
/// Given a table and a query:
///
/// ```notrust
/// CREATE TABLE ks.my_t (a int PRIMARY KEY, b text, c blob);
/// INSERT INTO ks.my_t (a, b, c) VALUES (?, ?, ?);
/// ```
///
/// ...the values for the query can be serialized using the following struct:
///
/// ```rust
/// # use scylla_cql::macros::SerializeRow;
/// #[derive(SerializeRow)]
/// # #[scylla(crate = scylla_cql)]
/// struct MyValues {
///     a: i32,
///     b: Option<String>,
///     c: Vec<u8>,
/// }
/// ```
///
/// # Struct attributes
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach serialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   columns/bind markers. During serialization, the implementation will take
///   care to serialize the fields in the order which the database expects.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the columns/bind markers.
///   If the order is incorrect, type checking/serialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///
/// `#[scylla(crate = crate_name)]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`SerializeRow`](crate::types::serialize::row::SerializeRow) trait
/// using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::SerializeRow;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
/// `#[scylla(skip_name_checks)]
///
/// _Specific only to the `enforce_order` flavor._
///
/// Skips checking Rust field names against names of the columns / bind markers.
/// With this annotation, the generated implementation will allow mismatch
/// between Rust struct field names and the column / bind markers, i.e. it's
/// OK if i-th Rust struct field has a different name than the column / bind
/// marker. The values are still being type-checked.
///
/// # Field attributes
///
/// `#[scylla(rename = "column_or_bind_marker_name")]`
///
/// Serializes the field to the column / bind marker with given name instead of
/// its Rust name.
pub use scylla_macros::SerializeRow;

// Reexports for derive(IntoUserType)
pub use bytes::{BufMut, Bytes, BytesMut};

pub use crate::impl_from_cql_value_from_method;

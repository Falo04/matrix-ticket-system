# sqlx-sqlite

This crate is a workaround for an issue with the dependency resolver.
This crate is not intended to be used by anyone.

## The issue

`matrix-sdk` depends on `rusqlite` and `galvyn` depends on `sqlx-sqlite`.
Both depend on `libsqlite3-sys` but different versions of it.
This is not allowed because it links to a system library.
`galvyn` actually never compile `sqlx-sqlite` because it is disabled through feature flags.

## The fix

The workspace's `Cargo.toml` "patches" the actual sqlx-sqlite crate with this one.
The dependency resolver sees that this does not depend on `libsqlite3-sys` and satisfies.
This is a workaround for the issue.
# Microbloggy

A Rust-based microblogger.

## Building and running

You will need the following environment variables to build and run:

```bash
export ADMIN_USERNAME=testuser
export ADMIN_PASSWORD=testpassword
export SESSION_SECRET=session-secret-atleast-32-chars
export DATABASE_URL=sqlite:path-to-db-used-for-migration.sqlite

# Ensure testing sqlite database exists
$ cargo install sqlx-cli  --no-default-features --features sqlite
$ sqlx database create
$ sqlx migrate run

# Build and run
$ cargo run

# Run tests (note that this uses the same database as DATABASE_URL, so be careful)
$ cargo test
```

## TODO

Building:

- [x] Use Sqlx for database migrations
- [ ] Use Makefile or other solution to ensure that the database exists already
- [ ] Add additional tests for post flow
- [ ] Build and test dockerfile
- [ ] Load testing on large databases (is it HN proof)

Features:

- [ ] Customize User Profile
- [ ] Actual error message when the login credentials are wrong
- [ ] Allow editing and deleting posts
- [ ] Allow setting a short friend URL for posts for sharing reasons
- [ ] Make some colors customizable
- [ ] Allow uploading and resizing photos
- [ ] Support multiple users

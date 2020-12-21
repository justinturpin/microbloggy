# Microbloggy

A Rust-based microblogger.

## Building and running

You will need the following environment variables to build and run:

```bash
export ADMIN_USERNAME=testuser
export ADMIN_PASSWORD=testpassword
export SESSION_SECRET=session-secret-atleast-32-chars
export DATABASE_URL=sqlite:path-to-db-used-for-migration.sqlite
export UPLOADS_PATH=uploads

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
- [x] Use Makefile or other solution to ensure that the database exists already
    - Somewhat mitigated by using create_if_not_exists in the sqlite pool options
- [ ] Add additional tests for post flow
- [x] Build and test Dockerfile
    - Dockerfile uses Ubuntu for builder and runner because I was having some Glibc compatibility
      issues using rust as the builder
- [x] Load testing on large databases (is it HN proof)
    - Ish? Getting 350 requests per second with Siege on a 1 Cpu Digitalocean VM which is fast
      enough but maybe slower than I would hope for how simple it is. Is generating a CSRF token
      per page load the performance issue?

Internals
- [ ] Middleware for authentication verification instead of boilerplate session checks
- [ ] Middlware for CSRF validation

Features:

- [x] Customize User Profile
- [ ] Actual error message when the login credentials are wrong
- [x] Allow editing and deleting posts
- [ ] Allow setting a short friend URL for posts for sharing reasons
- [ ] Make some colors customizable
- [ ] Allow uploading and resizing photos
- [ ] Support multiple users
- [x] Show multiple pages of posts
- [x] Show local times and dates based on users browser (render UTC in HTML)
- [x] Scale textareas based on size
  - This is _kind of_ implemented but isn't very dynamic and is a bit hardcoded

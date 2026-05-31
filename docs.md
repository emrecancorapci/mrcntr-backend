# Documentation

## Prerequisites

- [rust](https://rust-lang.org/)
- [diesel-cli](https://crates.io/crates/diesel_cli)
- [just](https://just.systems/man/en/) (optional)
- [watchexec](https://watchexec.github.io/) (optional)

## Just Commands

### Development

Runs the project with hot-reloading. Uses `watchexec`.

```bash
just dev
```

### Migrations

Creates migration files for `TABLE_NAME`. Alias for `diesel migration generate create_{tname}`.

```bash
just gen TABLE_NAME
```

Runs all pending migrations. Alias for `diesel migration run`

```bash
just sync
```

Alias for `diesel migration`

```bash
just m MIGRATION_COMMANDS
```

### Database

Resets the database and then runs `diesel database setup`.

```bash
just db reset
```

Setups the database.

```bash
just db setup
```

## Structure

```textfile
/src
  /modules
    /module_name
      handlers.rs     # Endpoint implementations 
      models.rs       # Models and DTOs
      repository.rs   # ORM implementations (normally i won't use this but i want to try SeaORM)
    module_name.rs    # Holds imports and exports of the module
    ...
  lib.rs              # Holds all imports and exports instead of using main.rs for it
  main.rs             # Where the magic happens
  router.rs           # Just the router
```

# Backend App for [mrcn.tr](https://mrcn.tr)

## Prerequisites

- [rust](https://rust-lang.org/) (rustc, rustup, etc.)
- [diesel-cli](https://crates.io/crates/diesel_cli)
- [just](https://just.systems/man/en/) (optional)
- [watchexec](https://watchexec.github.io/) (optional)

### For Linux

- [wild](https://github.com/wild-linker/wild) (linker for linux)
- [sccache](https://github.com/mozilla/sccache) (compiler caching tool)

### For macOS

Mac users should update their bash via `brew install bash` command to use `just new-mod`.

## Just Commands

### Development

Runs the project with hot-reloading. Uses `watchexec`.

```bash
just dev
```

Creates migration files for `TABLE_NAME`. Alias for `diesel migration generate create_{TABLE_NAME}`.

```bash
just new-mig TABLE_NAME
```

Runs all pending migrations. Alias for `diesel migration run`

```bash
just sync
```

Creates a new module in `modules` folder with its handler and resolver function. Only models need to be filled.

```bash
just new-mod MODULE_NAME
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

## Project Structure

I decided to follow a monolith modular structure.

```textfile
/src
  /modules
    /module_name
      handlers.rs     # Endpoint implementations 
      models.rs       # Models and DTOs
      repository.rs   # ORM implementations (normally i won't use this but i also want to try SeaORM)
    module_name.rs    # Holds imports and exports of the module
    ...
  lib.rs              # Holds all imports and exports instead of using main.rs for it
  main.rs             # Where the magic happens
  router.rs           # Just the router
```

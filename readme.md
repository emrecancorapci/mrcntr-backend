# Backend App for [mrcn.tr](https://mrcn.tr)

## Development

### Prerequisites

- [Rust](https://rust-lang.org/) (rustc, rustup, etc.)
- [diesel-cli](https://crates.io/crates/diesel_cli)
- [just](https://just.systems/man/en/) (optional, but highly recommended)
- [watchexec](https://watchexec.github.io/) (optional)

#### For Linux

To optimize the build process, an external linker and a compiler caching tool are used. You can disable them by deleting `/.cargo/config.toml`.

- [wild](https://github.com/wild-linker/wild) (Linker for Linux)
- [sccache](https://github.com/mozilla/sccache) (Compiler caching tool)

#### For macOS

Mac users must update Bash via `brew install bash` to use the `just new-mod` script.

> 💡 *The default macOS Bash (v3) lacks modern features (like associative arrays) used in the automation scripts.*

---

### Just Commands

| Command | Description |
| :--- | :--- |
| `just dev` | Runs the project with hot-reloading (requires `watchexec`). |
| `just new-mig TABLE_NAME` | Generates a new migration file. Alias for `diesel migration generate create_{TABLE_NAME}`. |
| `just sync` | Runs all pending migrations. Alias for `diesel migration run`. |
| `just new-mod MODULE_NAME` | Scaffolds a new module directory with its handler and resolver files. |
| `just db setup` | Initializes and sets up the database from scratch. |
| `just db reset` | Resets the database and runs `diesel database setup`. |

---

## Project Structure

```text
/src
  /modules
    /module
      /sub_module     # Same structure as module
      handlers.rs     # Endpoint implementations 
      models.rs       # Models and DTOs
      repository.rs   # ORM implementations
    module.rs         # Module entry point (imports/exports)
  lib.rs              # App root layout
  main.rs             # Application entry point
  router.rs           # Routing definitions
```

## Additional Resources & Documentation

- [OpenAPI Trait](https://docs.rs/utoipa/latest/utoipa/derive.OpenApi.html)
- [ToSchema Trait (For Models)](https://docs.rs/utoipa/latest/utoipa/derive.ToSchema.html)
- [path Attribute Macro (For Handlers)](https://docs.rs/utoipa/latest/utoipa/attr.path.html)

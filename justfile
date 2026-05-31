dev:
    watchexec -e rs -r cargo run

m cmd:
    diesel migration {{ cmd }}

sync:
    diesel migration run

gen db-name:
    diesel migration generate create_{{ db-name }}

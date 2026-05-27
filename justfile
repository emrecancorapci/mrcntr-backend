dev:
    watchexec -e rs -r cargo run

m cmd:
    diesel migration {{ cmd }}

gen db-name:
    diesel migration generate {{ db-name }}

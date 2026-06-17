default:
    @just --list

dev:
    watchexec -e rs -r cargo run

m *args:
    diesel migration {{ args }}

sync:
    @just m run

db *args:
    diesel database {{ args }}

new-mig db-name:
    @just m generate create_{{ db-name }}

new-mod mod-name:
    ./scripts/create_module.sh {{ mod-name }}

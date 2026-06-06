dev:
    watchexec -e rs -r cargo run

m *args:
    diesel migration {{ args }}

sync:
    @just m run

gen db-name:
    @just m generate create_{{ db-name }}

db *args:
    diesel database {{ args }}

mod cmd mod-name="":
    @if [ "{{ cmd }}" = "new" ]; then \
        mkdir -p src/modules/{{ mod-name }}; \
        printf "mod handlers;\nmod models;\npub mod repository;\n\npub use handlers::*;\npub use models::*;\n" > src/modules/{{ mod-name }}.rs; \
        touch src/modules/{{ mod-name }}/handlers.rs; \
        touch src/modules/{{ mod-name }}/models.rs; \
        touch src/modules/{{ mod-name }}/repository.rs; \
        echo "pub mod {{ mod-name }};" >> src/modules.rs; \
    else \
        echo "Unknown command: {{ cmd }}"; \
    fi
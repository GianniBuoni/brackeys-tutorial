debug:
    git add .
    nix build .#debug

build:
    git add .
    nix build .

godot:
    godot godot/project.godot &

[working-directory: './rust']
lint:
    cargo clippy --all-targets -- -D warnings

build:
    git add .
    nix build .

editor:
    godot godot/project.godot &

[working-directory: './rust']
lint:
    cargo clippy --all-targets -- -D warnings

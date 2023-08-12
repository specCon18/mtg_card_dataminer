tailwind:
    nix run .#tailwind
dev:
    cargo watch -x 'run test_data/test.json'

prettier:
    pnpm prettier --write --ignore-unknown .

init-env:
    pnpm i

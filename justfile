tailwind:
    export PATH=${pkgs.nodejs_20}/bin:${pkgs.nodePackages_latest.pnpm}/bin:$PATH
    pnpm dlx tailwindcss -i src/styles/tailwind.css -o assets/main.css --watch

dev:
    cargo watch -x 'run test_data/test.json'

prettier:
    pnpm prettier --write --ignore-unknown .

init-env:
    pnpm i

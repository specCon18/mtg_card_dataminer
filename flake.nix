{
  description = "A TCG card trading tool written in Rust";

  inputs.nixpkgs.url = "nixpkgs/23.05";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    #Shell scripts for managing the two package maangers from one interface
    build-tailwind = pkgs.writeShellScriptBin "build-tailwind" ''
      #!/usr/bin/env zsh
      set -e
      export PATH=${pkgs.nodejs_20}/bin:${pkgs.nodePackages_latest.pnpm}/bin:$PATH
      pnpm dlx tailwindcss -i src/styles/tailwind.css -o assets/main.css --watch
    '';
    run-dev = pkgs.writeShellScriptBin "run-dev" ''
      #!/usr/bin/env zsh
      set -e
      cargo watch -x 'run test_data/test.json'
    '';
    run-prettier = pkgs.writeShellScriptBin "run-prettier" ''
      #!/usr/bin/env zsh
      set -e
      pnpm prettier --write --ignore-unknown .
    '';

  in {
    defaultPackage.${system} = with pkgs; stdenv.mkDerivation {
      name = "sk-tcg-trader";
      src = self;

      buildInputs = [
        openssl
        pkgconfig
        rustc
        cargo
        cargo-watch
        nodejs_20
        nodePackages_latest.pnpm
      ];

      shellHook = ''
        export OPENSSL_DIR=${openssl.dev}
        export OPENSSL_LIB_DIR=${openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
      '';
    };

    build-tailwind = build-tailwind;
    run-dev = run-dev;
    run-prettier = run-prettier;
  };
}

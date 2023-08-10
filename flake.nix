{
  description = "A TCG card trading tool written in Rust";

  inputs={
    nixpkgs.url = "nixpkgs/23.05";
    nixago = {
      url = "github:jmgilman/nixago";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixago-exts = {
      url = "github:nix-community/nixago-extensions";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  
  outputs = { self, nixpkgs, nixago, nixago-exts }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    build-tailwind = pkgs.writeShellScriptBin "build-tailwind" ''
      #!/usr/bin/env zsh
      set -e
      export PATH=${pkgs.nodejs_20}/bin:${pkgs.nodePackages_latest.pnpm}/bin:$PATH
      pnpm dlx tailwindcss -i src/styles/tailwind.css -o assets/main.css --watch
    '';

    # gitignore = import "${self}/.config/nix/.gitignore.nix" { inherit pkgs; };
    # npmrc = import "${self}/.config/nix/.npmrc.nix" { inherit nixago; };
    # cargoConfig = import "${self}/.config/nix/cargo.nix" { inherit nixago; };
    # env = import "${self}/.config/nix/env.nix" { inherit pkgs; };
    # package = import "${self}/.config/nix/package.nix" { inherit nixago; };


    # Combine all shell hooks
    # combinedShellHook = ''
      # ${gitignore}
      # ${npmrc.shellHook}
      # ${cargoConfig.shellHook}
      # ${env}
      # ${package.shellHook}
    # '';

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
        just
        nodejs_20
        nodePackages_latest.pnpm
      ];

      shellHook = ''
        export OPENSSL_DIR=${openssl.dev}
        export OPENSSL_LIB_DIR=${openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
      '';
        # ${combinedShellHook} # Include the combined shell hooks goes in shell hook
    };
    tailwind = build-tailwind;
  };
}

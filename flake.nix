{
  description = "A TCG card trading tool written in Rust";

  inputs.nixpkgs.url = "nixpkgs/23.05";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

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
    };
  };
}

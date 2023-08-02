{
  description = "A simple Rust project";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: {

    defaultPackage.x86_64-linux = with nixpkgs.legacyPackages.x86_64-linux; stdenv.mkDerivation {
      name = "my-rust-project";
      src = self;
      
      buildInputs = [
        openssl
        pkgconfig
        rustc
        cargo
      ];
      
      shellHook = ''
        export OPENSSL_DIR=${openssl.dev}
        export OPENSSL_LIB_DIR=${openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
      '';
    };
  };
}

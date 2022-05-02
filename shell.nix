{ pkgs ? import <nixpkgs> {
    overlays = [
(import "${(fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz")}/overlay.nix")
      (self: super: {
          rustc = super.fenix.latest.rustc;
          cargo  = super.fenix.latest.cargo;
          rust-src = super.fenix.latest.rust-src;
      }
        )
    ];
  }
}:

pkgs.mkShell {
  packages = with pkgs; [
    openssl
    pkg-config
    perl
    rustc
    cargo
    rust-analyzer
    bitcoin
    clightning
    jq
  ];

buildInputs = [
   (import ./default.nix )
   ];

  RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
  OPENSSL_DIR = "${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

shellHook =
  ''
    echo "Hello shell"
    SRC_DIR="$( cd -- "$( dirname -- "$0" )/.." &> /dev/null && pwd )"
    cp -r $out/target $SRC_DIR/target
  '';
}


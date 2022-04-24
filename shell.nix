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
buildInputs = [
  (import ./default.nix )
];
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rust-analyzer
    bitcoin
    clightning
    jq
  ];

  RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
}

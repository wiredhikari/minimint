{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [
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

}


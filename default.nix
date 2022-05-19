{ lib, fetchFromGitHub, rustPlatform }:
let
    pkgs = import <nixpkgs> {};
    sources = import ./nix/sources.nix;
    naersk = pkgs.callPackage sources.naersk {};
rustPlatform.buildRustPackage rec {
  pname = "minimint";
  version = "master";
  src = fetchFromGitHub {
    url = "https://github.com/fedimint/minimint";
    ref = "master";
  };
  copyTarget = true;
  buildInputs = [
      pkgs.openssl
      pkgs.pkg-config
      pkgs.perl
  ];
  shellHook =
  ''
    SRC_DIR="$( cd -- "$( dirname -- "''${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
    cp -r $out/target $SRC_DIR/target
  '';
}
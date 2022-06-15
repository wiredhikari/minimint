{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default = crane.lib.${system}.buildPackage {
        src = ./.;

        # Add extra inputs here or any other derivation settings
        # doCheck = true;
        buildInputs = [
      pkgs.openssl
      pkgs.pkg-config
      pkgs.perl
        ];
        # nativeBuildInputs = [];
      };
    });
}
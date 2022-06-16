{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};

        commonArgs = {
          src = ./.;

          buildInputs = with pkgs; [
      openssl
      pkg-config
      perl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "mycrate-deps";
        });

        myCrateClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "-- --deny warnings";
        });

        myCrate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        myCrateCoverage = craneLib.cargoTarpaulin (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages.default = minimint;
      });
}
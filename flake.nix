{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;

    flake-utils.lib.eachDefaultSystem (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustVersion = "1.61.0";

         packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
    
      # parentheses disambiguate each makeOverride call as a single list element
      (pkgs.rustBuilder.rustLib.makeOverride {
          name = "secp256k1";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
              pkgs.secp256k1
            ];
          };
      })
    ];

        };

        # The workspace defines a development shell with all of the dependencies
        # and environment settings necessary for a regular `cargo build`
        workspaceShell = rustPkgs.workspaceShell {
          # This adds cargo2nix to the project shell via the cargo2nix flake
          packages = [ cargo2nix.packages."${system}".cargo2nix ];
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        devShells = {
          default = workspaceShell; # nix develop
        };

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#minimint
          # nix build .#packages.x86_64-linux.minimint
          minimint = (rustPkgs.workspace.minimint {}).bin;
          # nix build
          default = packages.minimint;
        };

        # nix run github:positron-solutions/minimint
        apps = rec {
          minimint = { type = "app"; program = "${packages.default}/bin/minimint"; };
          default = minimint;
        };
      }
    );
}

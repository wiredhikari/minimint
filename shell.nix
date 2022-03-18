{ nixpkgs ? import <nixpkgs> { }}:
let
  rustOverlay = builtins.fetchTarball "https://github.com/fedimint/minimint/archive/master.tar.gz";
  pinnedPkgs = nixpkgs.fetchFromGitHub {
    owner  = "fedimint";
    repo   = "minimint";
    rev    = "";
    sha256 = "";
  };
    pkgs = import pinnedPkgs {
    overlays = [ (import rustOverlay) ];
  };
    rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
in
  with pkgs;
   buildInputs = [
      # Rust
      rust
      rust-analyzer
      ];
   RUST_BACKTRACE = 1;
}


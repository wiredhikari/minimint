let
    pkgs = import <nixpkgs> {};
    sources = import ./nix/sources.nix;
    naersk = pkgs.callPackage sources.naersk {};
in naersk.buildPackage {
  pname = "minimint";
  version = "master";
  src = builtins.fetchGit {
    url = "https://github.com/fedimint/minimint";
    ref = "master";
  };
  copyTarget = true; # copy the target directory to the build dir
  buildInputs = [
      pkgs.openssl
      pkgs.pkg-config
      pkgs.perl
  ];
}
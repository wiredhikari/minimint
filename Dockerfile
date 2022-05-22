FROM nixos/nix
ADD . .
RUN nix-channel --update
CMD nix-shell --command nix-build default.nix
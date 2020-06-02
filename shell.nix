let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    # rust
    rls
    rustc
    cargo
    cargo-watch

    # dependencies
    libgit2
    libiconv
    openssl
    pkg-config
  ];
}

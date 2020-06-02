let pkgs = import <nixpkgs> { };
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

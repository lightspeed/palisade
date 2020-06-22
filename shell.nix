let
  sources = import ./nix/sources.nix;
  mozilla = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs { overlays = [ mozilla ]; };
  darwinDeps = if pkgs.stdenv.isDarwin then
    [ pkgs.darwin.apple_sdk.frameworks.Security ]
  else
    [ ];
in pkgs.mkShell {
  buildInputs = with pkgs;
    [
      # rust
      latest.rustChannels.stable.rust
      cargo-watch

      # dependencies
      libgit2
      libiconv
      openssl
      pkg-config
    ] ++ darwinDeps;
}

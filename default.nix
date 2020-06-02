{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { }
, naersk ? import sources.naersk { } }:
with pkgs;

let
  srcNoTarget = dir:
    builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    dir;
  naersk = pkgs.callPackage sources.naersk { };
  src = srcNoTarget ./.;
  remapPathPrefix = true;
in naersk.buildPackage {
  inherit src remapPathPrefix;
  buildInputs = with pkgs; [ pkg-config openssl libgit2 ];
}

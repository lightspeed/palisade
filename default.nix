{ sources ? import ./nix/sources.nix, mozilla ? import sources.nixpkgs-mozilla
, pkgs ? import sources.nixpkgs {
  overlays = [
    mozilla
    (self: super: {
      rustc = self.latest.rustChannels.stable.rust;
      cargo = self.latest.rustChannels.nightly.cargo;
    })
  ];
}, naersk ? import sources.naersk { } }:

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
  buildInputs = with pkgs; [ pkg-config openssl libgit2 zlib ];
}

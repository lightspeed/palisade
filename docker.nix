{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix { };
  pkgs = import sources.nixpkgs { };
  palisade = import ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "lightspeedhq/palisade";
      tag = "latest";

      contents = [ pkgs.cacert pkg ];

      config = {
        Cmd = [ "/bin/palisade" "cut" ];
        Env = [ "RUST_LOG=info" ];
        WorkingDir = "/";
      };
    };

in dockerImage palisade

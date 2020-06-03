{ system ? builtins.currentSystem }:

let
  pkgs = import <nixpkgs> { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  palisade = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "lightspeedretail/palisade";
      tag = "${palisade.version}";

      contents = [ pkgs.cacert pkg ];

      config = {
        Cmd = [ "/bin/palisade" "cut" ];
        WorkingDir = "/";
      };
    };

in dockerImage palisade

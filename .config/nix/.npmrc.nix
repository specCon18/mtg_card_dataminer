{ nixago }:

let
  data = {
    modules-dir = "project_dependencies/node/";
  };
in
nixago.lib.make {
  inherit data;
  output = ".npmrc";
  format = "ini"; # Optional if it matches the file extension
  engine = nixago.engines.nix { }; # Optional as this is the default engine
}
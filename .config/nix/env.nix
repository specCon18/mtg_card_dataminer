{ pkgs }:

let
  toEnvFile = attrs: pkgs.writeText "myenvfile.env" (nixpkgs.lib.generators.toKeyValue {} attrs);
in
  toEnvFile {
    UPDATE_INTERVAL = "12";
    PORT = "8080";
  }

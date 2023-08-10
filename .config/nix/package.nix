{ nixago }:

let
  data = {
    devDependencies = {
      prettier = "^3.0.1";
      prettier-plugin-tailwindcss = "^0.4.1";
      tailwindcss = "^3.3.3";
    };
  };
in
nixago.lib.make {
  inherit data;
  output = "package.json";
  format = "json"; # Optional if it matches the file extension
  engine = nixago.engines.nix { }; # Optional as this is the default engine
}
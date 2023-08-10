{nixago}:

let
  data = {
    package = {
      name = "sk_tcg_trader";
      version = "0.1.0";
      edition = "2021";
    };
    dependencies = {
      axum = "0.6.19";
      chrono = "0.4.26";
      dotenv = "0.15.0";
      hyper = "0.14.27";
      indicatif = "0.17.5";
      lazy_static = "1.4.0";
      prometheus = "0.13.3";
      reqwest = { 
        version = "0.11.18"; 
        features = ["json"]; 
      };
      serde = {
        version = "1.0.178";
        features = ["derive"];
      };
      serde_json = "1.0.104";
      tokio = {
        version = "1.29.1";
        features = ["full"];
      };
      anyhow = "1.0.72";
      askama = "0.12.0";
      tower = "0.4.13";
      tower-http = {
        version = "0.4.3";
        features = ["fs"];
      };
      tracing = "0.1.37";
      tracing-subscriber = {
        version = "0.3.17";
        features = ["env-filter"];
      };
    };
  };
in
nixago.lib.make {
  inherit data;
  output = "Cargo.toml";
  format = "toml"; # Optional if it matches the file extension
  engine = nixago.engines.nix { }; # Optional as this is the default engine
}
{ lib, rustPlatform, stdenv, darwin }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-vKJMzanMEsndbrMb0RPVfnyBMsmw5SlkxrHzTUsSuTc=";

  env = { VERGEN_IDEMPOTENT = true; };

  meta = {
    description = "add comments to a static Jekyll site";
    homepage = "https://github.com/Samasaur1/jekyll-comments";
  };
}

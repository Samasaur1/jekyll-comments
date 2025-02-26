{ lib, rustPlatform, stdenv, darwin }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-HvYmDsE+tN9A97rkQlBvJjAPbU9dgBP9yVdU5IRGlzk=";

  env = { VERGEN_IDEMPOTENT = true; };

  meta = with lib; {
    description = "The server-side software for Remote Text";
    homepage = "https://github.com/Remote-Text/remote-text-server";
    license = with licenses; [ ];
    maintainers = with maintainers; [ ];
  };
}

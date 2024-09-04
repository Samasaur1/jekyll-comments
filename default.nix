{ lib, rustPlatform, stdenv, darwin }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  cargoHash = "sha256-uUbz1Ay4Pvg7IpxB6OJ7A3cOnjTUW5tCSrt57nQTfZY=";

  buildInputs = [ ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.IOKit
    darwin.apple_sdk.frameworks.Security
  ];

  env = { VERGEN_IDEMPOTENT = true; };

  meta = with lib; {
    description = "The server-side software for Remote Text";
    homepage = "https://github.com/Remote-Text/remote-text-server";
    license = with licenses; [ ];
    maintainers = with maintainers; [ ];
  };
}

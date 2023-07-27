{ lib, rustPlatform, stdenv, darwin }:

let cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.name;
  version =
    (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

  src = ./.;

  cargoHash = "sha256-gbqLqstx6dS6sqxZpe+FbyMy0GLwRMJL5Ld2dEZQPB4=";

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

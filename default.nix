{ lib, rustPlatform, stdenv, darwin }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-x88Gf/a6WxB3GvaMcwo33MecQuD/x0ES6k3mUY1+pDg=";

  env = { VERGEN_IDEMPOTENT = true; };

  meta = with lib; {
    description = "The server-side software for Remote Text";
    homepage = "https://github.com/Remote-Text/remote-text-server";
    license = with licenses; [ ];
    maintainers = with maintainers; [ ];
  };
}

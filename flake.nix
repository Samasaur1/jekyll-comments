{
  description = "The server-side software for Remote Text";

  outputs = { nixpkgs, ... }:
    let
      forAllSystems = gen:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed
        (system: gen nixpkgs.legacyPackages.${system});
    in {
      packages = forAllSystems (pkgs: { default = pkgs.callPackage ./. { }; });

      modules.default = import ./module.nix;

      formatter = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed
        (system: nixpkgs.legacyPackages.${system}.nixfmt);
    };
}

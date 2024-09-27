{ inputs, self, ... }:
{
  flake = {
    checks."x86_64-linux" =
      let
        pkgs = inputs.nixpkgs.legacyPackages."x86_64-linux";
      in
      {
        verify-game-starts = pkgs.callPackage ./tests/verify-game-starts.nix {
          game = self.packages."x86_64-linux".game;
        };
      };
    nixosModules =
      {
      };
  };
}

{
  inputs = {
	nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      buildInputs = with pkgs; [ wayland raylib ];
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        inherit buildInputs;
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        RUSTFLAGS = "-L${pkgs.raylib}/lib -lraylib";
      };
    };
}

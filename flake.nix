{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      buildInputs = with pkgs; [
        pkg-config
        libGL
        xorg.libXi
        xorg.libX11
        libxkbcommon
        alsa-lib
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = buildInputs;
        nativeBuildInputs = buildInputs;
        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
        '';
      };
    };
}

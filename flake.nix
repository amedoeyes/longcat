{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default =
          with pkgs;
          pkgs.mkShell.override { stdenv = llvmPackages_20.libcxxStdenv; } rec {
            buildInputs = [
              rustc
              cargo
              mold
              pkg-config

              udev
              alsa-lib-with-plugins
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              libxkbcommon
              wayland
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
      }
    );
}

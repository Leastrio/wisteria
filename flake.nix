{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... } @inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer

            fontconfig
            libxkbcommon
            wayland
            vulkan-loader
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            wayland
            vulkan-loader
          ]);
        };
      }
    );
}

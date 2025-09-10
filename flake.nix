{ 
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  nixConfig = {
    extra-substituters = [ "http://nix-cache.ant-lab.tw" ];
    extra-trusted-public-keys = [ "nix-cache.ant-lab.tw-1:zIdryBfFvXk6AyoaN8P5WWFELzDWOK7bQvIzl8nL5Y8=" ];
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };
        cargoToml = "${self}/Cargo.toml";
        cargoTomlConfig = builtins.fromTOML (builtins.readFile cargoToml);
        nativeBuildInputs = with pkgs; [ 
          # TODO Need to check
          pkg-config
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell (rec {
            buildInputs = nativeBuildInputs ++  [ 
              pkgs.rust-bin.stable."1.88.0".minimal
              pkgs.alsa-lib
              pkgs.libudev-zero
              pkgs.libxkbcommon

              # pkgs.wayland  # wayland
              pkgs.vulkan-loader pkgs.vulkan-tools pkgs.xorg.libX11 pkgs.xorg.libXcursor pkgs.xorg.libXi pkgs.xorg.libXrandr # X11
            ];
            inherit nativeBuildInputs;
            # DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1=1;
            WINIT_UNIX_BACKEND = "x11";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          });
        };
      }
    );
}

{
  description = "A very basic for development and packaging of lunarfetch";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    nil = {
      url = "github:oxalica/nil";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    nil,
    rust-overlay,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
      with pkgs; {
        formatter = pkgs.alejandra;
        devShells.default = mkShell {
          buildInputs = with pkgs; [
            pkgs.zsh
            pkgs.nil
            rust-bin.beta.latest.default
            pkgs.rust-analyzer
          ];
          shellHook = ''
            test ~/.zshrc && exec zsh
          '';
        };
        packages = rec {
          lunarfetch = naersk.lib.${system}.buildPackage {
            pname = "lunarfetch";
            root = ./.;
            nativeBuildInputs = with pkgs; [llvmPackages_16.libcxxClang mold];
          };

          default = lunarfetch;
        };

        apps = {
          lunarfetch = {
            type = "lunarfetch";
            program = self.packages.lunarfetch;
          };
          defaultApp = lunarfetch;
        };
      });
}

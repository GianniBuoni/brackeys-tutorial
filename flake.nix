{
  description = ''
    Starter Ratatui dev and build flake.
  '';
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {inherit system;};
        nativeBuildInputs = with pkgs; [
          cargo
          clippy
          just
          rustc
          rust-analyzer
          pkg-config
        ];
        buildInputs = [
        ];
        naersk' = pkgs.callPackage naersk {};
        mkApp = release: mode: {
          src = ./rust/.;
          copyLibs = true;
          inherit nativeBuildInputs buildInputs release mode;
        };
      in {
        packages = {
          default = naersk'.buildPackage (mkApp true "build");
          debug = naersk'.buildPackage (mkApp false "build");
          test = naersk'.buildPackage (mkApp false "test");
        };
        devShell = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
        };
      }
    );
}

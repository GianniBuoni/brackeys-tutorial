{
  description = ''
    Starter Rust dev and build flake.
  '';

  inputs = {
    devenv.url = "github:cachix/devenv";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {nixpkgs.follows = "nixpkgs";};
    };
  };

  outputs = {
    self,
    devenv,
    flake-utils,
    naersk,
    nixpkgs,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {inherit system;};
        nativeBuildInputs = with pkgs; [pkg-config];
        buildInputs = [];
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
        devShell = devenv.lib.mkShell {
          inherit pkgs inputs;
          modules = [./devenv.nix];
        };
      }
    );
}

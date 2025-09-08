{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        buildInputs = with pkgs; [

        ];

        nativeBuildInputs = with pkgs; [

        ];
      in
      rec {
        defaultPackage = packages.app;
        packages =
          {
            app = naersk'.buildPackage {
              src = ./.;
              nativeBuildInputs = nativeBuildInputs;
              buildInputs = buildInputs;
            };
            contianer = pkgs.dockerTools.buildImage
              {
                name = "app";
                config = {
                  entrypoint = [ "${packages.app}/bin/app" ];
                };
              };
          };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [
              nixpkgs-fmt
              cmake
              rustc
              cargo
              clippy
              rustfmt
            ] ++ buildInputs ++ nativeBuildInputs;
        };
      }
    );
}

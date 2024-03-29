let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/refs/tags/22.05.tar.gz")) {};
in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rust-analyzer
  ];
}

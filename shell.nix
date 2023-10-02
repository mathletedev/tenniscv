{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    clang
    gtk2
    libclang
    llvmPackages_latest.llvm
    (opencv.override { enableGtk2 = true; })
    pkg-config
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}

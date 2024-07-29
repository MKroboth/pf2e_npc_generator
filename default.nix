{
  pkgs ? import <nixpkgs> { },
}:
let
  libPath =
    with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
in
{
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      bacon
      clippy
      cargo
      rustc
      rust-analyzer
    ];
    RUST_LOG = "npc_generator=debug,npc_generator_core=debug,warn";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    LD_LIBRARY_PATH = libPath;
  };
}

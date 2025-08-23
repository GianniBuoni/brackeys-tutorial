{pkgs, ...}: {
  packages = with pkgs; [
    alsa-lib
    just
    udev
    wayland
  ];

  env.LD_LIBRARY_PATH = with pkgs;
    lib.makeLibraryPath [
      libxkbcommon
      vulkan-loader
    ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}

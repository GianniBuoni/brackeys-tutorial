{pkgs, ...}: {
  packages = with pkgs; [
    just
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}

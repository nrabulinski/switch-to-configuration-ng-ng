{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    pkg-config
    dbus
  ];

  SYSTEMD_DBUS_INTERFACE_DIR = "${pkgs.pkgsBuildHost.systemd}/share/dbus-1/interfaces";
}

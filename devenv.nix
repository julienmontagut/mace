{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [ git ripgrep fd just nixfmt ];

  languages.rust = { enable = true; };

  # processes.cargo-watch.exec = "cargo watch -x check -x test";

  pre-commit.hooks = {
    # clippy.enable = true;
    rustfmt.enable = true;
    nixfmt.enable = true;
  };
}

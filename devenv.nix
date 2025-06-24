{ pkgs, lib, config, inputs, ... }:

{
  dotenv.enable = true;

  languages.rust = { enable = true; };
  packages = with pkgs; [ git ripgrep fd just nixfmt ];

  pre-commit.hooks = {
    # clippy.enable = true;
    rustfmt.enable = true;
    nixfmt.enable = true;
  };

  devcontainer = {
    enable = true;
    settings = { updateContentCommand = "direnv allow"; };
  };
}

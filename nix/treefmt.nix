{
  perSystem = {
    # should be covered by pre-commit
    treefmt = {
      flakeCheck = false;

      settings.on-unmatched = "fatal";
      settings.excludes = [
        ".clang-format"
        ".clangd"
      ];

      # cpp
      programs.clang-format.enable = true;

      # cmake
      programs.cmake-format.enable = true;

      # nix
      programs.nixfmt.enable = true;

      # sh
      programs.shfmt.enable = true;
      programs.shellcheck.enable = true;
      settings.formatter.shfmt.includes = [ ".envrc" ];
      settings.formatter.shellcheck.includes = [ ".envrc" ];
    };
  };
}

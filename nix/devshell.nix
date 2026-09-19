{
  perSystem =
    {
      config,
      pkgs,
      lib,
      ...
    }:
    {
      devshells.default = {
        devshell.startup.pre-commit-hook.text = config.pre-commit.installationScript;
        devshell.packages = with pkgs; [
          cmake
          ninja
          gcc
          clang-tools
          gdb
          (lib.getDev gtest)
        ];
        env = [
          {
            name = "CMAKE_PREFIX_PATH";
            eval = "$DEVSHELL_DIR";
          }
        ];
      };
    };
}

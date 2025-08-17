# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-24.05"; # or "unstable"
  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.rust-analyzer
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rustup
    pkgs.stdenv.cc
    pkgs.file
    pkgs.rar
  ];
  # Sets environment variables in the workspace
  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      "rust-lang.rust-analyzer"
      "tamasfe.even-better-toml"
      "serayuzgur.crates"
      "vscode-icons-team.vscode-icons"
      "vadimcn.vscode-lldb"
    ];
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        # Welcome message, available in the "Welcome to IDX" panel
        welcome = ''
          # Welcome to your new Rust project in IDX!

          Your environment is pre-configured with the following tools:

          - **Rust Toolchain:** `rustc`, `cargo`, and `rustfmt`
          - **Build Tools:** A C compiler (`stdenv.cc`)
          - **VS Code Extensions:** `rust-analyzer`, `even-better-toml`, `crates`, and `vscode-lldb`

          To get started, open `src/main.rs` and start coding.
        '';
        # Commands to run when the workspace is created
        # Example:
        # demo = "echo 'Hello, world!' >/tmp/hello.txt"
        open-readme = "devenv foreground open README.md";
        open-main = "devenv foreground open src/main.rs";
      };
      # Runs when a workspace is started
      onStart = {
        # Example:
        # say-hello = "echo 'Welcome back to your workspace!'"
      };
    };
  };
}

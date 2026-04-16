flake: {pkgs, ...}: let
  # Hostplatform system
  system = pkgs.hostPlatform.system;

  # Production package
  base = flake.packages.${system}.default;
in
  pkgs.mkShell {
    inputsFrom = [base];

    packages = with pkgs; [
      nixd
      statix
      deadnix
      alejandra

      rustfmt
      clippy
      rust-analyzer
      cargo-watch
      # rustup

      rustPlatform.bindgenHook

      # pkg-config

      # Other packages here
      # openssl
      # libressl
      # ...
    ];

    strictDeps = true;

    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    # RUSTC_VERSION = overrides.toolchain.channel;

    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      # export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${pkgs.stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
    '';
  }

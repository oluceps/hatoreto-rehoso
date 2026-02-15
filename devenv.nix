{
  inputs,
  pkgs,
  ...
}:
{

  # overlays = [ inputs.rust-overlay.overlays.default ];

  # cachix.enable = true;
  # cachix.pull = [ "wrangler" ];
  dotenv.enable = true;
  languages.javascript = {
    enable = true;
    npm.install.enable = false;
    pnpm.install.enable = false;
  };

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "miri"
    ];
  };
  # cachix.enable = true;
  # cachix.pull = [ "wrangler" ];
  packages = with pkgs; [
    nodePackages_latest.pnpm
    typescript
    biome
    rustup
    wasm-pack
    protobuf
    dbus # Added for btleplug
    pkg-config # Added to help resolve system library dependencies
    # (rust-bin.nightly.latest.default.override { extensions = [ "rust-src" ]; })
    gemini-cli
    # netlify-cli
  ];
  # ++ [ inputs.wrangler.packages.${pkgs.system}.default ];
}

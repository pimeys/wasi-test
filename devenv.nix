{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = with pkgs; [
    rustup
    wasmtime
    openssl
    pkg-config
    tinygo
    gopls
    wasm-tools
    wit-bindgen
  ];

  languages.c.enable = true;
  languages.cplusplus.enable = true;
  languages.go.enable = true;

  env.COMPONENT_ADAPTER_REACTOR = "/home/pimeys/code/cargo-component-test/wasi_snapshot_preview1.reactor.wasm";
}

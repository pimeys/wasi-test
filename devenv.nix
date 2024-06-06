{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = with pkgs; [rustup wasmtime openssl pkg-config];
}

staging:
  cargo publish -p httpfile-build --dry-run
  cargo publish -p httpfile --dry-run

publish:
  cargo publish -p httpfile-build
  cargo publish -p httpfile

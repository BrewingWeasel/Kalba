# Run lints and check formatting
lint:
  cargo check
  cargo clippy -- --deny warnings
  leptosfmt -- check ./src-ui/src
  cargo fmt --check

# format code
format:
  leptosfmt ./src-ui/src
  cargo fmt
# TODO: some sort of css formatting (prettier?)

# Fix code not passing pre-commit hook (uses --allow-dirty + --allow-staged on clippy)
force-fix: && format
  cargo clippy --fix --allow-dirty --allow-staged

# Fix code not passing pre-commit hook
fix: && format
  cargo clippy --fix

# test code
test:
  cargo test

rust-check: test lint

ci-check:
  actionlint

check: 
  rust-check

pre-commit:
  #!/usr/bin/env sh
  newfiles=$(git status --porcelain | awk '{ print $2 }')
  if (echo $newfiles | grep ".rs" ); then
    just rust-check
  fi
  if (echo $newfiles | grep ".github" ); then
    just ci-check
  fi
# TODO: do this with other languages
# TODO: make this only do stuff for the specific rust directories modified

_install program command:
  #!/usr/bin/env sh
  if command -v {{program}} &> /dev/null; then
    echo "{{program}} is already installed"
  else
    {{command}}
  fi

_cargoinstall program: (_install program replace("cargo install program", "program", program) )

# Set up development related tools
setup-dev: (_cargoinstall "leptosfmt") (_cargoinstall "trunk") (_install "cargo-tauri" "cargo install tauri-cli") (_install "cargo-clippy" "rustup component add clippy")
# TODO: probably lots more that I'm forgetting

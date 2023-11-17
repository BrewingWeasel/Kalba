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
  @# TODO: some sort of css formatting (prettier?)

# Fix code not passing pre-commit hook (uses --allow-dirty + --allow-staged on clippy)
force-fix: && format
  cargo clippy --fix --allow-dirty --allow-staged

# Fix code not passing pre-commit hook
fix: && format
  cargo clippy --fix

# test code
test:
  cargo test

pre-commit: test lint

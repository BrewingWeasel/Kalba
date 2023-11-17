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

pip-command := if `command -v pip` != "" { 
  "pip" 
} else if `command -v pip3` != "" { 
  "pip3" 
} else if `command -v python` != "" {
  "python -m pip"
} else if `command -v python3` != "" {
  "python3 -m pip"
} else {
  error("unable to find pip or python")
}

install_deps_command := "apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev glibc-source libc6 python3-dev"

build: (_install "cargo-tauri" "cargo install tauri-cli")  (_cargoinstall "trunk") 
  #!/usr/bin/env sh
  echo "Adding wasm target"
  rustup target add wasm32-unknown-unknown
  echo "installing spacy"
  {{pip-command}} install spacy --upgrade

  if command -v apt-get; then
    echo "you can install all external dependencies with the following command:"
    echo "{{install_deps_command}}"
    echo "do you want to run it? (y/N)"
    read answer
    if [ "$answer" = "y"]; then 
      {{install_deps_command}}
    fi
  fi

  cargo tauri build
  echo "installed successfully"

# Run the test suite
default: fmt lint build test

@test *selectors:
    cargo nextest run {{selectors}}

# Build the binaries
@build:
    cargo build

@install:
    cargo install --path .

# Run the application
@run *args:
    cargo run --bin over -- {{args}}

@fmt:
    cargo fmt --all

@lint:
    cargo clippy

bench:
    RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph --bin BoxSlayer2d

push:
    cargo fmt --all -- --check
    #cargo doc --locked --workspace --profile ci --all-features --document-private-items --no-deps
    cargo clippy --locked --workspace --all-targets --profile ci --all-features
    #bevy_lint --locked --workspace --all-targets --profile ci --all-features
    #cargo test --locked --workspace --all-targets --profile ci --no-fail-fast
    #cargo check --config 'profile.web.inherits="dev"' --profile ci --no-default-features --features dev --target wasm32-unknown-unknown
    jj bookmark set main -r @
    jj git push

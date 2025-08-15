bench:
    RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph --bin BoxSlayer2d

push:
    jj bookmark set main -r @
    cargo fmt
    jj git push

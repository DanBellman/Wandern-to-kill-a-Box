bench:
    RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph --bin BoxSlayer2d

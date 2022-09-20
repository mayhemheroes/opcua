# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd lib/src/core/fuzz && cargo +nightly -Z sparse-registry fuzz build --fuzz-dir . && popd" && \
    bash -c "pushd lib/src/types/fuzz && cargo +nightly -Z sparse-registry fuzz build --fuzz-dir . && popd" && \
    mv lib/src/core/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_comms /fuzz_comms && \
    mv lib/src/types/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_deserialize /fuzz_deserialize && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd lib/src/core/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    bash -c "pushd lib/src/types/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv lib/src/core/fuzz/target/release/fuzz_comms /fuzz_comms_no_inst && \
    mv lib/src/types/fuzz/target/release/fuzz_deserialize /fuzz_deserialize_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /fuzz_comms /fuzz_comms_no_inst /
COPY --from=builder /fuzz_deserialize /fuzz_deserialize_no_inst /

# `script` phase: you usually build, test and generate docs in this phase

set -ex

. $(dirname $0)/utils.sh

# PROTIP Always pass `--target $TARGET` to cargo commands, this makes cargo output build artifacts
# to target/$TARGET/{debug,release} which can reduce the number of needed conditionals in the
# `before_deploy`/packaging phase
run_test_suite() {
    case $TARGET in
        # configure emulation for transparent execution of foreign binaries
        aarch64-unknown-linux-gnu)
            export TEST_CMD='qemu-aarch64 -L /usr/aarch64-linux-gnu'
            ;;
        arm*-unknown-linux-gnueabihf)
            export TEST_CMD='qemu-arm -L /usr/arm-linux-gnueabihf'
            ;;
        *)
            ;;
    esac

    if [ ! -z "$QEMU_LD_PREFIX" ]; then
        # Run tests on a single thread when using QEMU user emulation
        export RUST_TEST_THREADS=1
    fi

    cargo build --target $TARGET --verbose

    # cargo test should™ just run if we request qemu-user-static and
    # binfmt-support, but the test crashes. Since that requires sudo and that
    # makes it run on even more ancient host than otherwise, we run the tests
    # manually…
    # And then, we actually don't anyway, because TravisCI does not have
    # a working qemu in any setup.
    if [ -n "$TEST_CMD" ]; then
	cargo test --target $TARGET --no-run
	TESTS=$(find target/$TARGET/debug -maxdepth 1 -executable -type f -print)
	[ -n "$TESTS" ]
	for TEST in $TESTS; do
	    $TEST_CMD $TEST
	done
	# $TEST_CMD target/$TARGET/debug/examples/<example>
    else
	cargo test --target $TARGET
	# cargo run --target $TARGET --example <example>
    fi

    # sanity check the file type
    file target/$TARGET/debug/liblocale.rlib
}

main() {
    run_test_suite
}

main

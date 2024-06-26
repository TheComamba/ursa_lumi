#/bin/bash

# Go to git root
cd $(git rev-parse --show-toplevel)

cargo build --release

exe="./target/release/ursa_lumi"

function expect_success {
    if [ $? -ne 0 ]; then
        echo "Test:"
        echo $1
        echo "failed."
        exit 1
    fi
}

function expect_failure {
    if [ $? -eq 0 ]; then
        echo "Test:"
        echo $1
        echo "failed."
        exit 1
    fi
}

function expect_file {
    if [ ! -f $1 ]; then
        echo "Test:"
        echo $2
        echo "Expected file"
        echo $1
        echo "not found."
        exit 1
    fi
}

echo "Running integration tests."

testname="Running executable without arguments returns an error."
$exe 2>/dev/null
expect_failure "$testname"

testname="Running executable with generation parameters but no output fails."
$exe --params ./integration_tests/example_params.json 2>/dev/null
expect_failure "$testname"

testname="Running executable with generation parameters and output creates the output."
rm -rf ./integration_tests/example_output.json
$exe --params ./integration_tests/example_params.json --out ./integration_tests/example_output.json >/dev/null
expect_success "$testname"
expect_file "./integration_tests/example_output.json" "$testname"

echo "All integration tests passed."
exit 0

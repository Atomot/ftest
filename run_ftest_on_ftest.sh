#!/bin/sh

cargo build --release && cp target/release/ftest ./ftest && ./ftest && rm ./ftest

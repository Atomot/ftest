#!/bin/sh

cargo build && cp target/release/ftest ./ftest && ./ftest $@
rm -f ./ftest

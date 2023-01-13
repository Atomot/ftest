#!/bin/sh

cargo build && cp target/debug/ftest ./ftest && ./ftest $@
rm -f ./ftest

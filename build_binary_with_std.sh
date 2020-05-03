#!/bin/sh

rm tmp/*.ij

make std

mkdir -p tmp
mkdir -p tmp/out

IJSSEL_SOURCE=$1
touch tmp/_tmp.ij
cat src/std/std.ij >> tmp/_tmp.ij
cat $IJSSEL_SOURCE >> tmp/_tmp.ij

cargo run -q -- tmp/_tmp.ij

ld tmp/std.o tmp/_tmp.o /usr/lib/libc.dylib -o tmp/out/binary
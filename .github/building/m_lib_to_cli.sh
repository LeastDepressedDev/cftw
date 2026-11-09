#!/bin/bash
if [[ ! -d cli/include ]]; then
    mkdir cli/include
fi
cp libbmp/headers/bmplib.h cli/include/bmplib.h
cp libbmp/target/release/libbmp.a cli
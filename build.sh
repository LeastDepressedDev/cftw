#!/bin/bash

if [[ ! -d build ]]; then
    mkdir build
fi

./.github/building/b_lib.sh &&
./.github/building/m_lib_to_cli.sh &&
./.github/building/b_cli.sh &&

cp cli/build/cftw-cli ./build
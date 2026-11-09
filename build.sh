#!/bin/bash

if [[ ! -d build ]]; then
    mkdir build
fi

./.github/workflows/b_lib.sh &&
./.github/workflows/m_lib_to_cli.sh &&
./.github/workflows/b_cli.sh &&

cp cli/build/cftw-cli ./build
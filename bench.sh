#!/bin/bash

set -e

mkdir -pv build result

cd build

cmake .. -DCMAKE_BUILD_TYPE=Release

make

cd ../result

function run {
    echo Start bench $1
    GTK_IM_MODULE=$1 ../build/bench 2> /dev/null | tee $1.ret
}

run kime
run uim
run fcitx
run ibus

echo END

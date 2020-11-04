#!/bin/bash

cd ..
mkdir build
cd build

export CC=/usr/bin/clang

cmake -G Ninja ..

ninja

cd ../scripts

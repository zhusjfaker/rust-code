#!/bin/zsh

cd ./clib
clang -arch arm64 -c ./index.c -o ./index.o
ar -r ./libindex.a ./index.o
rm -rf ./index.o
cd -
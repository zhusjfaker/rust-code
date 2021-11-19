#!/bin/zsh

### 此为连接 本测试项目 C语言静态库的脚本

cd ./clib
clang -arch arm64 -c ./index.c -o ./index.o
ar -r ./libindex.a ./index.o
rm -rf ./index.o
cd -
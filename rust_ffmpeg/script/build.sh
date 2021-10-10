#!/bin/sh

rm -rf ../ff-output

cd ../source/FFmpeg

./configure \
--prefix=../../ff-output \
--disable-everything \
--disable-all \
--disable-ffplay \
--disable-ffprobe \
--disable-x86asm \
--disable-programs \
--disable-asm \
--disable-doc \
--disable-autodetect \
--disable-optimizations \
--disable-devices \
--disable-pthreads \
--disable-w32threads \
--disable-network \
--disable-hwaccels \
--disable-parsers \
--disable-bsfs \
--disable-debug \
--disable-stripping \
--disable-protocols \
--disable-indevs \
--disable-outdevs \
--disable-swresample \
--disable-x86asm \
--disable-ffplay \
--disable-hwaccels \
--enable-protocol=file \
--enable-protocol=rtmp \
--enable-decoder=aac \
--enable-demuxer=mov \
--enable-demuxer=flv \
--enable-demuxer=rtsp \
--enable-demuxer=mp3 \
--enable-demuxer=h264 \
--enable-demuxer=aac \
--enable-muxer=mp4 \
--enable-demuxer=mp4 \
--enable-muxer=flv \
--enable-muxer=h264 \
--enable-muxer=adts \
--enable-muxer=mp3 \
--enable-decoder=aac \
--enable-decoder=mp3 \
--enable-decoder=opus \
--enable-decoder=h264 \
--enable-decoder=hevc \
--enable-encoder=aac \
--enable-encoder=libx264 \
--enable-demuxer=ogg \
--enable-demuxer=matroska \
--enable-demuxer=mov \
--enable-demuxer=mpegts \
--enable-demuxer=flv \
--enable-demuxer=avi \
--enable-muxer=mpegts \
--enable-muxer=avi \
--enable-parser=aac \
--enable-encoder=jmpeg \
--enable-decoder=jmpeg \
--arch=aarch64 \
--target-os=darwin 

make -j 12

make install

make clean

cd -
#!/bin/sh

rm -rf ../ff-output

cd ../source/FFmpeg

./configure \
--prefix=../../ff-output \
--disable-everything \
--disable-ffplay \
--disable-ffprobe \
--disable-x86asm \
--disable-programs \
--disable-doc \
--disable-autodetect \
--disable-optimizations \
--disable-debug \
--disable-stripping \
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
--target-os=darwin 

make

make install

cd -
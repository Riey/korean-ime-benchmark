#!/bin/sh

export GTK_IM_MODULE=kime

echo kime

cargo run --release 2>/dev/null

export GTK_IM_MODULE=uim

echo uim

cargo run --release 2>/dev/null

export GTK_IM_MODULE=fcitx

echo fcitx

cargo run --release 2>/dev/null


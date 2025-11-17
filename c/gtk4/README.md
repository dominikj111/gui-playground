# Set the environment

## Debian

sudo apt install libgtk-4-dev libglib2.0-dev gcc make meson ninja-build libgirepository1.0-dev pkg-config

## Build

gcc `pkg-config --cflags gtk4` -o gtk_app main.c `pkg-config --libs gtk4`

## Run

./gtk_app

## Test environments

Raspberry Pi 4 [Debian GNU/Linux 12 (bookworm)]

Linux MX 23 [Debian GNU/Linux 12 (bookworm)] - Radeon HD 6290

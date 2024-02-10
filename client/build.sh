#!/bin/bash
function install_pre_requisites() {
    echo "Installing pre-requisites"
    sudo apt-get update
    sudo apt-get install -y curl
    echo "Installing pre-requisites done"
}

function install_rust() {
    echo "Installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "Installing rust done"
}

function install_tauri_cli() {
    echo "Installing tauri-cli"
    cargo install tauri-cli --force
    echo "Installing tauri-cli done"
}

function build() {
    TAURI_BUILD_TARGET=deb # appimage, msi, app or dmg
    echo "Building the app to $TAURI_BUILD_TARGET"
    cargo tauri build -b $TAURI_BUILD_TARGET
    echo "Building the app done"
}

# install_pre_requisites
# install_rust
# install_tauri_cli
build

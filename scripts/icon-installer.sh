#!/bin/sh

set -e

gh_repo="papirus-icon-theme"
gh_desc="Papirus icon theme"

: "${LOCAL_DESTDIR:=$HOME/.local/share/icons}"
: "${EXTRA_THEMES=Papirus-Dark Papirus-Light}"
: "${TAG:=master}"

temp_file="$(mktemp -u)"
temp_dir="$(mktemp -d)"

download() {
    echo "Getting the latest version from GitHub ..."
    wget -O "$temp_file" \
        "https://github.com/PapirusDevelopmentTeam/$gh_repo/archive/$TAG.tar.gz"
    echo "Unpacking archive ..."
    tar -xzf "$temp_file" -C "$temp_dir"
}


install() {
    # shellcheck disable=2068
    set -- $@  # split args by space

    for theme in "$@"; do
        test -d "$temp_dir/$gh_repo-$TAG/$theme" || continue
        echo "Installing '$theme' ..."
        cp -R "$temp_dir/$gh_repo-$TAG/$theme" $1
    done
}

cleanup() {
    echo "Clearing cache ..."
    rm -rf "$temp_file" "$temp_dir"
    rm -f "$HOME/.cache/icon-cache.kcache"
    echo "Done!"
}


download

error_message=$(mkdir -p $LOCAL_DESTDIR 2>&1)

install $LOCAL_DESTDIR Papirus "$EXTRA_THEMES"

trap cleanup EXIT HUP INT TERM

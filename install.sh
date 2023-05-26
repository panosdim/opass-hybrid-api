#!/bin/sh
if [ "$(id -u)" != "0" ]; then
    exec sudo bash "$0" "$@"
fi

is_service_exists() {
    x="$1"
    if systemctl status "${x}" 2>/dev/null | grep -Fq "Active:"; then
        return 0
    else
        return 1
    fi
}

INSTALL_PATH=/opt/opass-hybrid

# Build software
cargo build release
ret_code=$?
if [ $ret_code != 0 ]; then
    printf "Error: [%d] when building executable. Check that you have rustup installed." $ret_code
    exit $ret_code
else
    cp target/release/opass-hybrid-api opass
fi

# Check if needed files exist
if [ -f tolls.db3 ] && [ -f opass ] && [ -f opass.service ]; then
    # Check if we upgrade or install for first time
    if is_service_exists 'opass.service'; then
        systemctl stop opass.service
        cp opass $INSTALL_PATH
        cp tolls.db3 $INSTALL_PATH
        systemctl start opass.service
    else
        mkdir -p $INSTALL_PATH
        cp opass $INSTALL_PATH
        cp tolls.db3 $INSTALL_PATH
        cp opass.service /usr/lib/systemd/system
        systemctl start opass.service
        systemctl enable opass.service
    fi
else
    echo "Not all needed files found. Installation failed."
    exit 1
fi

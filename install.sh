#!/bin/sh

# Function to check if a service is installed
is_service_exists() {
    x="$1"
    if systemctl status "${x}" 2>/dev/null | grep -Fq "Active:"; then
        return 0
    else
        return 1
    fi
}

INSTALL_PATH=/opt/opass-hybrid
NGINX_CONF_PATH=/etc/nginx/conf.d

# Build software
cargo build --release
ret_code=$?
if [ $ret_code != 0 ]; then
    printf "Error: [%d] when building executable. Check that you have rustup installed.\n" $ret_code
    exit $ret_code
else
    cp target/release/opass-hybrid-api opass
fi

# Check if needed files exist
if [ -f tolls.db3 ] && [ -f opass ] && [ -f opass.service ] && [ -f opass.conf ]; then
    # Check if we upgrade or install for first time
    if is_service_exists 'opass.service'; then
        sudo systemctl stop opass.service
        sudo cp opass $INSTALL_PATH
        sudo cp tolls.db3 $INSTALL_PATH
        sudo systemctl start opass.service
    else
        sudo mkdir -p $INSTALL_PATH
        sudo cp opass $INSTALL_PATH
        sudo cp tolls.db3 $INSTALL_PATH
        sudo cp opass.service /usr/lib/systemd/system
        sudo systemctl start opass.service
        sudo systemctl enable opass.service
	sudo cp opass.conf $NGINX_CONF_PATH
	sudo nginx -s reload
    fi
    rm opass
else
    echo "Not all needed files found. Installation failed."
    exit 1
fi

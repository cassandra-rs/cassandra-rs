#!/bin/bash

set -e

cd "$(dirname "$0")"

sudo apt-get update

# Install dependencies of the cpp-driver even if they are already on CI so that we can run this locally
sudo apt-get install -y libuv1 libuv1-dev cmake g++ libssl-dev zlib1g-dev

# Set VERSION to one of the tags here: https://github.com/datastax/cpp-driver/tags
# This is the version of the cpp driver that will be installed and therefore tested against
VERSION=2.16.2

PACKAGE_NAME="cassandra-cpp-driver_${VERSION}-1_amd64"
FILE_PATH="packages/${PACKAGE_NAME}.deb"

# Create package if it doesnt already exist
if [ ! -f "$FILE_PATH" ]; then
    rm -rf cpp-driver # Clean just in case the script failed halfway through last time
    git clone --depth 1 --branch $VERSION https://github.com/datastax/cpp-driver
    pushd cpp-driver

    cmake -DCMAKE_INSTALL_PREFIX:PATH=/usr -DCMAKE_INSTALL_LIBDIR:PATH=/usr/lib -Wno-error .
    make

    mkdir -p $PACKAGE_NAME/DEBIAN
    make DESTDIR="$PACKAGE_NAME/" install

    cp ../cassandra-cpp-driver.control $PACKAGE_NAME/DEBIAN/control
    sed -i "s/VERSION/${VERSION}/g" $PACKAGE_NAME/DEBIAN/control
    dpkg-deb --build $PACKAGE_NAME

    mkdir -p ../packages
    cp ${PACKAGE_NAME}.deb ../$FILE_PATH

    popd
    rm -rf cpp-driver
fi

sudo dpkg -i $FILE_PATH

#!/bin/sh
# Script for packaging a binary release of URM
set -e

FILE_LIST="target/release/urm \
           target/release/genuser \
           urm.toml \
           Rocket.toml \
           README.md \
           static/ \
           templates/"

DIR_NAME="urm"
TARBALL_NAME="urm-$(git describe --always)"

if [ "$1" = "strip" ]; then
  STRIP=1
  shift
fi

echo '===== Building URM ====='
cargo build --release
cd utils/genuser
cargo build --release
cd ../..

echo '===== Copying Files ====='
mkdir -v "${DIR_NAME}"

for file in ${FILE_LIST}; do
  cp -rv "${file}" "${DIR_NAME}"
done

for extra in "$@"; do
  cp -rv "${extra}" "${DIR_NAME}"
done

if [ "${STRIP}" ]; then
  echo '===== Stripping Binaries ====='
  strip --strip-all --verbose $(find "${DIR_NAME}" -type f -executable)
fi

echo '===== Packing Binary Release ====='
tar cv "${DIR_NAME}" | xz -zecv -T 0 > "${TARBALL_NAME}.tar.xz"

echo '===== Removing Temporary Files ====='
rm -rv "${DIR_NAME}"

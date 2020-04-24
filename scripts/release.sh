#!/bin/sh
# Script for packaging a binary release of URM
set -eo pipefail

FILE_LIST=(
  "target/release/urm"
  "target/release/genuser"
  "urm.toml"
  "Rocket.toml"
  "README.md"
  "static/"
  "templates/"
)

DIR_NAME="urm"
TARBALL_NAME="urm-$(git describe --always)"

if [ "$1" = "strip" ]; then
  STRIP=1
  shift
fi

cargo build --release
pushd utils/genuser
cargo build --release
popd

mkdir "${DIR_NAME}"

cp -r ${FILE_LIST[@]} "${DIR_NAME}"

if [ "${STRIP}" ]; then
  strip --strip-all --verbose $(find "${DIR_NAME}" -type f -executable)
fi

for extra in "$@"; do
  cp -rv "${extra}" "${DIR_NAME}"
done

tar c "${DIR_NAME}" | xz -zecv -T 0 > "${TARBALL_NAME}.tar.xz"

rm -r "${DIR_NAME}"

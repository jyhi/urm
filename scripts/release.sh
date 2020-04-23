#!/bin/sh
# Script for packaging a binary release of URM
set -eo pipefail

NAME="urm-$(git describe --always)-$(date '+%Y%m%d')"

if [ "$1" = "strip" ]; then
  STRIP=1
  shift
fi

cargo build --release
pushd utils/genuser
cargo build --release
popd

mkdir "${NAME}"

cp target/release/{urm,genuser} "${NAME}"
if [ "${STRIP}" ]; then
  strip --strip-all "${NAME}/urm"
  strip --strip-all "${NAME}/genuser"
fi

cp urm.toml Rocket.toml README.md "${NAME}"
cp -r static/ templates/ "${NAME}"

for extra in "$@"; do
  cp -r "${extra}" "${NAME}"
done

tar c "${NAME}" | xz -zecv -T 0 > "${NAME}.tar.xz"

rm -r "${NAME}"

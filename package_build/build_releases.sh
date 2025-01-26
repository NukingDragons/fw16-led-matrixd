#!/bin/bash

cd $(dirname $0)/../
echo "Building the workspace for x86_64 linux"
cargo b --release
echo "Building the workspace for i686 linux"
cargo b --target=i686-unknown-linux-gnu --release
echo "Building the workspace for x86_64 windows"
cargo b --target=x86_64-pc-windows-gnu --release

cd package_build
version=$(sed -z 's/.*version = "\([^"]*\)".*/\1/g' ../Cargo.toml)
name="fw16-led-matrixd-${version}"
debname="fw16-led-matrixd_${version}"

echo "Building source code archive for release ${version}"
rm ${name}.tar.xz ${name}.tar.xz.sig
tar --exclude 'target' --exclude 'package_build' --exclude-vcs -cJf ${name}.tar.xz ../ --transform "s/^/${name}\//g" 2>&1 >/dev/null
gpg --output ${name}.tar.xz.sig --detach-sig ${name}.tar.xz
checksum=$(sha256sum ${name}.tar.xz | cut -d' ' -f1)
sigchecksum=$(sha256sum ${name}.tar.xz.sig | cut -d' ' -f1)

echo "Creating PKGBUILD for version ${version}"
cat > pacman/release/PKGBUILD << EOF
# Maintainer: Sabrina Andersen <sabrina@utd.tf>
pkgname=fw16-led-matrixd
pkgver=${version}
pkgrel=1
pkgdesc="A cross-platform daemon for controlling the Framework 16 LED Matrixes"
arch=('x86_64' 'i686')
url="https://github.com/NukingDragons/fw16-led-matrixd"
license=('MIT')
makedepends=('git' 'rust')
optdepends=('systemd: systemd service support')
conflicts=('fw16-led-matrixd-git')
backup=('etc/fw16-led-matrixd/config.toml')
validpgpkeys=('B2FA6C185A694EFB2A2A1612EDB944713B73E150')
options=('!debug')
source=("https://github.com/nukingdragons/fw16-led-matrixd/releases/download/\${pkgver}/\${pkgname}-\${pkgver}.tar.xz"{,.sig})
sha256sums=('${checksum}' '${sigchecksum}')

prepare()
{
	cd "\$pkgname-\$pkgver"

	export RUSTUP_TOOLCHAIN=nightly
	cargo fetch --locked --target "\$(rustc -vV | sed -n 's/host: //p')"
}

build()
{
	cd "\$pkgname-\$pkgver"

	export RUSTUP_TOOLCHAIN=nightly
	export CARGO_TARGET_DIR="target"
	cargo build --frozen --release --all-features
}

check()
{
	cd "\$pkgname-\$pkgver"

	export RUSTUP_TOOLCHAIN=nightly
	cargo test --frozen --all-features
}

package()
{
	cd "\$pkgname-\$pkgver"

	install -Dm755 target/release/fw16-led-matrixd "\${pkgdir}/usr/bin/fw16-led-matrixd"
	install -Dm755 target/release/ledcli "\${pkgdir}/usr/bin/ledcli"
	install -Dm644 sample-posix-config.toml "\${pkgdir}/etc/fw16-led-matrixd/config.toml"
	install -Dm644 daemon/fw16-led-matrixd.service "\${pkgdir}/usr/lib/systemd/system/fw16-led-matrixd.service"
	install -Dm644 LICENSE "\${pkgdir}/usr/share/licenses/\${pkgname}/LICENSE"
}
EOF

echo "Generating .SRCINFOs"
cd pacman/git
makepkg --printsrcinfo > .SRCINFO
cd ../..

cd pacman/release
makepkg --printsrcinfo > .SRCINFO
cd ../..

echo "Creating x86_64 deb package for version ${version}"
mkdir -p deb/DEBIAN deb/etc/fw16-led-matrixd deb/usr/bin deb/usr/lib/systemd/system
rm deb/DEBIAN/control deb/usr/bin/fw16-led-matrixd deb/usr/bin/ledcli deb/etc/fw16-led-matrixd/config.toml deb/usr/lib/systemd/system/fw16-led-matrixd.service 2>/dev/null
cat > deb/DEBIAN/control << EOF
Package: fw16-led-matrixd
Version: ${version}
Architecture: amd64
Maintainer: Sabrina Andersen <sabrina@utd.tf>
Description: A cross-platform daemon for controlling the Framework 16 LED Matrixes
Homepage: https://github.com/NukingDragons/fw16-led-matrixd
Section: main
Priority: optional
EOF
cp ../target/release/fw16-led-matrixd deb/usr/bin/
cp ../target/release/ledcli deb/usr/bin/
cp ../sample-posix-config.toml deb/etc/fw16-led-matrixd/config.toml
cp ../daemon/fw16-led-matrixd.service deb/usr/lib/systemd/system/
dpkg-deb --build --root-owner-group deb
mv deb.deb ${debname}-1_amd64.deb

echo "Creating i386 deb package for version ${version}"
rm deb/DEBIAN/control deb/usr/bin/fw16-led-matrixd deb/usr/bin/ledcli 2>/dev/null
cat > deb/DEBIAN/control << EOF
Package: fw16-led-matrixd
Version: ${version}
Architecture: i386
Maintainer: Sabrina Andersen <sabrina@utd.tf>
Description: A cross-platform daemon for controlling the Framework 16 LED Matrixes
Homepage: https://github.com/NukingDragons/fw16-led-matrixd
Section: main
Priority: optional
EOF
cp ../target/i686-unknown-linux-gnu/release/fw16-led-matrixd deb/usr/bin/
cp ../target/i686-unknown-linux-gnu/release/ledcli deb/usr/bin/
dpkg-deb --build --root-owner-group deb
mv deb.deb ${debname}-1_i386.deb

echo "Creating apt repository"
cd nukingdragons.github.io
git pull
cp ../*.deb docs/repos/fw16-led-matrixd/apt/pool/
cd docs/repos/fw16-led-matrixd/apt
dpkg-scanpackages --multiversion --arch amd64 pool/ > dists/stable/main/binary-amd64/Packages
cat dists/stable/main/binary-amd64/Packages | gzip -9 > dists/stable/main/binary-amd64/Packages.gz
dpkg-scanpackages --multiversion --arch i386 pool/ > dists/stable/main/binary-i386/Packages
cat dists/stable/main/binary-i386/Packages | gzip -9 > dists/stable/main/binary-i386/Packages.gz
cd dists/stable
rm InRelease Release Release.gpg
cat > Release << EOF
Origin: fw16-led-matrixd
Label: fw16-led-matrixd
Suite: stable
Codename: stable
Version: ${version}
Architectures: amd64 i386
Components: main
Description: A repo containing a cross-platform daemon for controlling the Framework 16 LED Matrixes
Date: $(date -Ru)
EOF
echo "MD5Sum:" >> Release
for file in $(find -type f)
do
	# Don't hash the Release file
	if [[ "$file" = "./Release" ]]
	then
		continue
	fi
	echo " $(md5sum $file | cut -d' ' -f1) $(wc -c $file)" | sed 's/\.\///g' >> Release
done
echo "SHA1:" >> Release
for file in $(find -type f)
do
	# Don't hash the Release file
	if [[ "$file" = "./Release" ]]
	then
		continue
	fi
	echo " $(sha1sum $file | cut -d' ' -f1) $(wc -c $file)" | sed 's/\.\///g' >> Release
done
echo "SHA256:" >> Release
for file in $(find -type f)
do
	# Don't hash the Release file
	if [[ "$file" = "./Release" ]]
	then
		continue
	fi
	echo " $(sha256sum $file | cut -d' ' -f1) $(wc -c $file)" | sed 's/\.\///g' >> Release
done
gpg --armor --output Release.gpg --detach-sig Release
gpg --armor --output InRelease --detach-sig --clear-sign Release
cd ../../../../../../..

echo "Building zip file for Windows installation for version ${version}"
rm -r ${name}_windows.zip
cp ../target/x86_64-pc-windows-gnu/release/fw16-led-matrixd.exe windows/
cp ../target/x86_64-pc-windows-gnu/release/ledcli.exe windows/
cp ../sample-windows-config.toml windows/config.toml
cp ../LICENSE windows/
cd windows
7z a ../${name}_windows.zip *

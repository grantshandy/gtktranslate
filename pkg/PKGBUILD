# Maintainer: Grant Handy <grantshandy AT gmail DOT com>

_pkgname='gtktranslate'
pkgname=${_pkgname}-git
pkgver=0.3.0.r2.g780116c
pkgrel=1
pkgdesc="A Simple GTK Translator Using Libretranslate"
arch=('x86_64')
url="https://github.com/skylinecc/gtktranslate"
license=('GPL')
depends=('gtk4')
makedepends=('rust' 'git')
source=("${_pkgname}::git+https://github.com/skylinecc/gtktranslate.git")
md5sums=('SKIP')

pkgver() {
  cd "$srcdir/${_pkgname}"
  git describe --tags --long | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
  cd "$srcdir/${_pkgname}"
  cargo build --release
}

package() {
  cd "$srcdir/${_pkgname}"

  # Binary
  install -Dm755 target/release/$_pkgname $pkgdir/usr/bin/$_pkgname

  # Icons
  for i in 16x16 24x24 32x32 48x48 64x64 128x128 256x256; do
      echo copying icon ${i}
      install -Dm0644 -t "$pkgdir/usr/share/icons/hicolor/$i/apps/${_pkgname}.png" "data/icons/$i/gtktranslate.png"
  done

  install -Dm0644 "data/icons/128x128/gtktranslate.png" "$pkgdir/usr/share/pixmaps/gtktranslate.png"

  # Desktop file
  install -Dm0644 -t "$pkgdir/usr/share/applications" "data/gtktranslate.desktop"
}
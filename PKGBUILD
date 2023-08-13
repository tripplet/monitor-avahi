pkgname=monitor-avahi
pkgver=1.2.4
pkgrel=1
pkgdesc='Monitor/Restart avahi for invalid hostname'
url="https://github.com/tripplet/monitor-avahi"
arch=('x86_64' 'armv7h' 'aarch64')
depends=()
makedepends=(rust)

build() {
  cargo build --release --locked
  strip ../target/release/monitor-avahi
}

package()
{
  cd ${pkgdir}/../..
  install -Dm 755 "target/release/monitor-avahi" -t "${pkgdir}/usr/bin"
  install -Dm 644 "monitor-avahi.service" -t "${pkgdir}/usr/lib/systemd/system"
}

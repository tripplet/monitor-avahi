pkgname=monitor-avahi
pkgver=1.1.1
pkgrel=1
pkgdesc='Monitor/Restart avahi for invalid hostname'
url="https://github.com/tripplet/monitor-avahi"
arch=('x86_64' 'armv7h' 'aarch64')
depends=()
makedepends=(rust)
source=("monitor-avahi-${pkgver}-${pkgrel}.tar.gz::https://github.com/tripplet/monitor-avahi/archive/${pkgver}.tar.gz")
sha256sums=('346ac3102a6c5e2b63a1c930876e94e8613e2ac89a9810df667ec45b4e7d24eb')

build() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo build --release --locked
  strip target/release/monitor-avahi
}

package()
{
  cd "${srcdir}/${pkgname}-${pkgver}"
  install -Dm 755 "target/release/monitor-avahi" -t "${pkgdir}/usr/bin"
  install -Dm 644 "monitor-avahi.service" -t "${pkgdir}/usr/lib/systemd/system"
}

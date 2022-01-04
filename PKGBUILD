pkgname=monitor-avahi
pkgver=1.1.2
pkgrel=1
pkgdesc='Monitor/Restart avahi for invalid hostname'
url="https://github.com/tripplet/monitor-avahi"
arch=('x86_64' 'armv7h' 'aarch64')
depends=()
makedepends=(rust)
source=("monitor-avahi-${pkgver}-${pkgrel}.tar.gz::https://github.com/tripplet/monitor-avahi/archive/${pkgver}.tar.gz")
sha256sums=('0bc371647ea5c3932a2126879529b209cb0ece0d6f8040bb046b89726da9c28f')

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

include $(TOPDIR)/rules.mk

LUCI_TITLE:=luci-app-giwifi
LUCI_PKGARCH:=all

PKG_VERSION:=0.2.0
PKG_RELEASE:=2

PKG_LICENSE:=AGPL-3.0
PKG_MAINTAINER:=MCitem <mcitem@mcitem.com>


define Package/luci-app-giwifi
	TITLE:=luci-app-giwifi
	URL:=https://github.com/mcitem/giwifi
endef

define Package/luci-app-giwifi/description
	luci-app-giwifi is a LuCI application for login into giwifi.
endef

define Build/Compile
endef

include $(TOPDIR)/feeds/luci/luci.mk

$(eval $(call BuildPackage,luci-app-giwifi))


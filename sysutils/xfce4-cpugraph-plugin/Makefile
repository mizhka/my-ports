PORTNAME=	xfce4-cpugraph-plugin
PORTVERSION=	1.2.11
CATEGORIES=	sysutils xfce
MASTER_SITES=	XFCE/panel-plugins
DIST_SUBDIR=	xfce4

MAINTAINER=	xfce@FreeBSD.org
COMMENT=	Graphical representation of the CPU load
WWW=		https://docs.xfce.org/panel-plugins/xfce4-cpugraph-plugin/start

LICENSE=	GPLv2

LIB_DEPENDS=	libharfbuzz.so:print/harfbuzz

USES=		compiler:c++17-lang gettext-tools gmake gnome libtool pkgconfig \
		tar:bzip2 xfce
USE_CXXSTD=	c++17
USE_GNOME=	cairo gdkpixbuf gtk30
USE_XFCE=	libmenu panel xfconf

GNU_CONFIGURE=	yes
INSTALL_TARGET=	install-strip

OPTIONS_DEFINE=	NLS
OPTIONS_SUB=	yes

NLS_USES=		gettext-runtime
NLS_CONFIGURE_ENABLE=	nls

.include <bsd.port.mk>

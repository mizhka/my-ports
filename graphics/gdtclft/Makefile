PORTNAME=	Gdtclft
PORTVERSION=	2.2.5
PORTREVISION=	19
CATEGORIES=	graphics tcl
MASTER_SITES=	https://BSDforge.com/projects/source/graphics/gdtclft/
DISTNAME=	${PORTNAME}${PORTVERSION}

MAINTAINER=	portmaster@bsdforge.com
COMMENT=	TCL interface to the Thomas Boutell's Gd library
WWW=		http://graphviz.org/cgi-bin/man?gdtclft

LICENSE=	GRAPHVIZ
LICENSE_NAME=	GRAPHVIZ
LICENSE_FILE=	${WRKSRC}/license.terms
LICENSE_PERMS=	auto-accept

LIB_DEPENDS=	libgd.so:graphics/gd \
		libpng.so:graphics/png

USES=		tcl uidfix

MAKEFILE=	${FILESDIR}/Makefile.bsd
MAKE_ENV=	TCL_VER=${TCL_VER} MKDIR="${MKDIR}" \
			TCL_SHLIB_VER=${TCL_SHLIB_VER} \
			INSTALL_DATA="${INSTALL_DATA}" STAGEDIR="${STAGEDIR}"

MANNPAGE=	gdtclft.n
GDTCLDIR=	lib/tcl${TCL_VER}/gdtclft
PLIST_FILES=	${GDTCLDIR}/pkgIndex.tcl
PLIST_FILES+=	${GDTCLDIR}/libGdtclft2.so
PLIST_FILES+=	share/man/mann/${MANNPAGE}.gz

post-patch:
	${REINPLACE_CMD} -Ee 's,[[:space:]]+$$,,' ${WRKSRC}/${MANNPAGE}

post-install:
	${INSTALL_MAN} ${WRKSRC}/${MANNPAGE} ${STAGEDIR}${PREFIX}/share/man/mann

.include <bsd.port.mk>

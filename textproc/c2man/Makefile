PORTNAME=	c2man
PORTVERSION=	${VERSION}.${PATCHLEVEL}
PORTREVISION=	1
CATEGORIES=	textproc
MASTER_SITES=	http://www.ciselant.de/${PORTNAME}/
DISTNAME=	${PORTNAME}-${VERSION}@${PATCHLEVEL}

MAINTAINER=	lantw44@gmail.com
COMMENT=	Tool to generate documentation from C source code comments
WWW=		https://www.ciselant.de/c2man/c2man.html

LICENSE=	${PORTNAME}
LICENSE_NAME=	${PORTNAME}
LICENSE_FILE=	${WRKSRC}/README
LICENSE_PERMS=	dist-mirror dist-sell pkg-mirror pkg-sell auto-accept

DEPRECATED=	Not recommended for use in new projects
EXPIRATION_DATE=2025-04-30

VERSION=	2.0
PATCHLEVEL=	42

NO_WRKSUBDIR=	yes
NO_INSTALL=	yes

PLIST_FILES=	bin/c2man share/man/man1/c2man.1.gz

HAS_CONFIGURE=	yes
CONFIGURE_SCRIPT=Configure
CONFIGURE_ARGS=	-des -Dcc="${CC}" -Dccflags="${CFLAGS}" -Dldflags="${LDFLAGS}"

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/${PORTNAME} ${STAGEDIR}${PREFIX}/bin/
	${INSTALL_MAN} ${WRKSRC}/${PORTNAME}.1 ${STAGEDIR}${PREFIX}/share/man/man1/

.include <bsd.port.mk>

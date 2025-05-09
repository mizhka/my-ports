PORTNAME=	docbook2odf
PORTVERSION=	0.244
PORTREVISION=	4
CATEGORIES=	textproc perl5
MASTER_SITES=	http://open.comsultia.com/docbook2odf/dwn/

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Convert docbook document to Oasis Open Document
WWW=		http://open.comsultia.com/docbook2odf/

LICENSE=	GPLv2

BROKEN=		Unfetchable
DEPRECATED=	Upstream is gone and unmaintained in tree for years
EXPIRATION_DATE=2025-04-30

LIB_DEPENDS=	libsablot.so:textproc/sablotron \
		libxslt.so:textproc/libxslt
RUN_DEPENDS=	zip:archivers/zip \
		p5-Archive-Zip>=0:archivers/p5-Archive-Zip

USES=		desktop-file-utils magick:6,run perl5 shebangfix
SHEBANG_FILES=	utils/maketests utils/${PORTNAME}
USE_PERL5=	run
NO_BUILD=	yes

PORTEXAMPLES=	*
OPTIONS_DEFINE=	EXAMPLES

post-patch:
	@${REINPLACE_CMD} -e 's|#!/usr/bin/perl|#!${PERL}|' \
		${WRKSRC}/tests/tests/docbook-43.001.xml
	@${FIND} ${WRKSRC} -name "*.bak" -delete -or -name "*.cvsignore" -delete

do-install:
	${LN} -sf ${DATADIR}/utils/${PORTNAME} ${STAGEDIR}${PREFIX}/bin/
	@(cd ${WRKSRC} && ${COPYTREE_SHARE} "tests utils xsl" \
		${STAGEDIR}${DATADIR})
	${INSTALL_DATA} ${WRKSRC}/bindings/desktop/${PORTNAME}.desktop \
		${STAGEDIR}${DESKTOPDIR}
	${LN} -sf ${EXAMPLESDIR}/${PORTNAME}.png \
		${STAGEDIR}${PREFIX}/share/pixmaps/
	${INSTALL_MAN} ${WRKSRC}/docs/${PORTNAME}.1 \
		${STAGEDIR}${PREFIX}/share/man/man1/

	@(cd ${WRKSRC}/examples && ${COPYTREE_SHARE} . ${STAGEDIR}${EXAMPLESDIR})

.include <bsd.port.mk>

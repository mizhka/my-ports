PORTNAME=	FuzzyOcr
DISTVERSION=	3.6.0
PORTREVISION=	7
CATEGORIES=	mail perl5
MASTER_SITES=	http://users.own-hero.net/~decoder/${PORTNAME:tl}/
PKGNAMEPREFIX=	p5-
PKGNAMESUFFIX=	-devel
DISTNAME=	${PORTNAME:tl}-${DISTVERSION}

MAINTAINER=	ismail.yenigul@endersys.com.tr
COMMENT=	Plugin for SpamAssassin which scans image attachments for spam
WWW=		https://fuzzyocr.own-hero.net/

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	giftopnm:graphics/netpbm			\
		gifsicle:graphics/gifsicle			\
		gocr:graphics/gocr				\
		ocrad:graphics/ocrad			\
		giftext:graphics/giflib			\
		spamassassin:mail/spamassassin	\
		p5-MLDBM>=0:databases/p5-MLDBM	\
		p5-MLDBM-Sync>=0:databases/p5-MLDBM-Sync \
		p5-String-Approx>=0:devel/p5-String-Approx

USES=		perl5
NO_BUILD=	yes

WRKSRC=		${WRKDIR}/${PORTNAME}-${PORTVERSION}

PORTDOCS=	samples
PORTDOCS_F=	samples/README samples/ocr-animated.eml			\
		samples/ocr-gif.eml samples/ocr-jpg.eml samples/ocr-multi.eml	\
		samples/ocr-obfuscated.eml samples/ocr-png.eml samples/ocr-wrongext.eml
PORTEXAMPLES=	*

SUB_FILES=	pkg-message gifinter

SA_PLUGINDIR?=	Mail/SpamAssassin/Plugin
SA_CONFIGDIR?=	etc/mail/spamassassin

PLIST_FILES=	${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr.pm		\
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Config.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Deanimate.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Hashing.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Logging.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Misc.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Preprocessor.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Scanset.pm \
		${SITE_PERL}/${SA_PLUGINDIR}/FuzzyOcr/Scoring.pm \
		bin/gifinter

OPTIONS_DEFINE=	DOCS EXAMPLES
OPTIONS_SINGLE=	GIFINTER_EMULATION
OPTIONS_SINGLE_GIFINTER_EMULATION=IMAGEMAGICK GRAPHICSMAGICK
OPTIONS_DEFAULT=IMAGEMAGICK
OPTIONS_SUB=	please
IMAGEMAGICK_DESC=	Use ImageMagick's convert to process interlaced GIFs
GRAPHICSMAGICK_DESC=	Use GraphicsMagick to process interlaced GIFs
IMAGEMAGICK_RUN_DEPENDS=	convert:graphics/ImageMagick7
GRAPHICSMAGICK_RUN_DEPENDS=	gm:graphics/GraphicsMagick

pre-everything::
	@${ECHO_MSG} "+---------------------------------------------------+"
	@${ECHO_MSG} "|                                                   |"
	@${ECHO_MSG} "| Warning: FuzzyOcr now requires SpamAssassin 3.2.X |"
	@${ECHO_MSG} "|                                                   |"
	@${ECHO_MSG} "+---------------------------------------------------+"

post-patch:
	@${REINPLACE_CMD} -e 's,/usr/bin,${LOCALBASE}/bin,g'		\
	                  -e 's,/etc/mail,${LOCALBASE}/etc/mail,g'	\
	                  ${WRKSRC}/FuzzyOcr.??  ${WRKSRC}/FuzzyOcr/Config.pm
	@${REINPLACE_CMD} -e 's,FuzzyOcr::,Mail::SpamAssassin::Plugin::FuzzyOcr::,'			\
	                  ${WRKSRC}/FuzzyOcr.pm ${WRKSRC}/FuzzyOcr/*.pm
	@${REINPLACE_CMD} -e 's,FuzzyOcr;,Mail::SpamAssassin::Plugin::FuzzyOcr;,'			\
	                  ${WRKSRC}/FuzzyOcr.pm

do-install:
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR} ${STAGEDIR}${PREFIX}/${SITE_PERL_REL}/${SA_PLUGINDIR}/FuzzyOcr
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.pm ${STAGEDIR}${PREFIX}/${SITE_PERL_REL}/${SA_PLUGINDIR}
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr/*.pm ${STAGEDIR}${PREFIX}/${SITE_PERL_REL}/${SA_PLUGINDIR}/FuzzyOcr
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.cf ${STAGEDIR}${EXAMPLESDIR}
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.words ${STAGEDIR}${EXAMPLESDIR}/FuzzyOcr.words
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.scansets ${STAGEDIR}${EXAMPLESDIR}/FuzzyOcr.scansets
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.preps ${STAGEDIR}${EXAMPLESDIR}/FuzzyOcr.preps
	${INSTALL_DATA} ${WRKSRC}/FuzzyOcr.mysql ${STAGEDIR}${EXAMPLESDIR}/FuzzyOcr.mysql
	${INSTALL_SCRIPT} ${WRKDIR}/gifinter ${STAGEDIR}${PREFIX}/bin/
	@${MKDIR} ${STAGEDIR}${DOCSDIR}/samples
.for f in ${PORTDOCS_F}
	${INSTALL_DATA} ${WRKSRC}/${f} ${STAGEDIR}${DOCSDIR}/${f}
.endfor

.include <bsd.port.mk>

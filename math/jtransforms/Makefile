PORTNAME=	jtransforms
PORTVERSION=	2.4
PORTREVISION=	1
CATEGORIES=	math java
MASTER_SITES=	SF
DISTNAME=	${PORTNAME}-${PORTVERSION}-src

MAINTAINER=	olgeni@FreeBSD.org
COMMENT=	Multithreaded FFT library written in pure Java
WWW=		http://sites.google.com/site/piotrwendykier/software/jtransforms

LICENSE=	GPLv2 LGPL20 MPL11
LICENSE_COMB=	multi

USES=		java:ant,run tar:bzip2

WRKSRC=		${WRKDIR}/JTransforms

PLIST_FILES=	%%JAVAJARDIR%%/jtransforms.jar

do-install:
	@${INSTALL_DATA} ${WRKSRC}/jtransforms-2.4.jar ${STAGEDIR}${JAVAJARDIR}/jtransforms.jar

.include <bsd.port.mk>

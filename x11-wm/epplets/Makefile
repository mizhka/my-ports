PORTNAME=	epplets
PORTVERSION=	0.14
PORTREVISION=	12
CATEGORIES=	x11-wm
MASTER_SITES=	SF/enlightenment/${PORTNAME}/${PORTVERSION}

MAINTAINER=	enlightenment@FreeBSD.org
COMMENT=	Collection of Enlightenment dock applications

BUILD_DEPENDS=	e16:x11-wm/e16
LIB_DEPENDS=	libcdaudio.so:audio/libcdaudio
RUN_DEPENDS=	e16:x11-wm/e16

USES=		gl gmake libtool magick:6,run pkgconfig xorg
USE_XORG=	xext
USE_GL=		glut
USE_LDCONFIG=	yes
CPPFLAGS+=	-I${LOCALBASE}/include
LDFLAGS+=	-L${LOCALBASE}/lib
GNU_CONFIGURE=	yes
CONFIGURE_ARGS=	--disable-auto-respawn \
		--disable-esound

post-patch:
	@${REINPLACE_CMD} -e 's|<machine/soundcard\.h>|<sys/soundcard.h>|g' \
		${WRKSRC}/epplets/Emix.c
	@${REINPLACE_CMD} -e 's|<malloc\.h>|<stdlib.h>|g' \
		${WRKSRC}/epplets/mbox.c

.include <bsd.port.mk>

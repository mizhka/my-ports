PORTNAME=	megaglest
PORTVERSION=	3.13.0
PORTREVISION=	14
CATEGORIES=	games

MAINTAINER=	monwarez@mailoo.org
COMMENT=	Open source 3D real-time strategy game
WWW=		https://megaglest.org/

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/docs/gnu_gpl_3.0.txt

LIB_DEPENDS=	libcurl.so:ftp/curl \
		libpng.so:graphics/png \
		libftgl.so:graphics/ftgl \
		libircclient.so:irc/libircclient \
		libvorbis.so:audio/libvorbis \
		libfontconfig.so:x11-fonts/fontconfig \
		libfreetype.so:print/freetype2 \
		libminiupnpc.so:net/miniupnpc
BUILD_DEPENDS=	${LOCALBASE}/include/miniupnpc/miniupnpc.h:net/miniupnpc
RUN_DEPENDS=	${LOCALBASE}/${DATADIR_REL}/tutorials/2_basic_tutorial/2_basic_tutorial.xml:games/megaglest-data

USE_GITHUB=	yes
GH_ACCOUNT=	MegaGlest
GH_PROJECT=	megaglest-source

SUB_FILES=	pkg-message

USES=		cmake:insource compiler:c++11-lib display:build dos2unix jpeg lua \
		openal:al,alut pkgconfig ssl localbase:ldflags sdl gl xorg
USE_GL=		glew gl glu
USE_SDL=	sdl2
USE_XORG=	sm ice x11 xext
USE_CXXSTD=	c++11

LDFLAGS+=	-lssl -lcrypto
LDFLAGS_i386=	-Wl,-znotext

OPTIONS_DEFINE=	EDITOR DOCS VIEWER
OPTIONS_SUB=	yes
EDITOR_DESC=	Install MegaGlest Editor
VIEWER_DESC=	Install Megaglest G3D viewer

EDITOR_CMAKE_BOOL=	BUILD_MEGAGLEST_MAP_EDITOR
EDITOR_USE=		WX=3.0

VIEWER_CMAKE_BOOL=	BUILD_MEGAGLEST_MODEL_IMPORT_EXPORT_TOOLS \
			BUILD_MEGAGLEST_MODEL_VIEWER
VIEWER_USE=		WX=3.0

.include <bsd.port.pre.mk>

# The following is actually meant for lld 15.0 and later, but the ports
# framework does not support LINKER_TYPE and LINKER_VERSION yet.
.if ${COMPILER_TYPE} == "clang"
# Turn off checking of dynamic relocations, to avoid lld diagnostics about
# possibly incorrect addend values.
LDFLAGS_i386+=	-Wl,--no-check-dynamic-relocations
.endif

post-patch:
	@${REINPLACE_CMD} -e 's,/usr/local/include/lua51,${LUA_INCDIR},g ; \
		s,/usr/local/lib/lua51,${LUA_LIBDIR},g ; \
		s,lua5.2,lua-${LUA_VER},g' \
		${WRKSRC}/mk/cmake/Modules/FindLUA.cmake

do-install:
	@${MKDIR} ${STAGEDIR}${DATADIR}
	@${MKDIR} ${STAGEDIR}${PREFIX}/share/pixmaps
	${INSTALL_DATA} ${WRKSRC}/mk/shared/megaglest.ico ${STAGEDIR}${PREFIX}/share/pixmaps/
	${INSTALL_PROGRAM} ${WRKSRC}/source/glest_game/megaglest ${STAGEDIR}${PREFIX}/bin/${PORTNAME}
.for f in mk/linux/glest.ini mk/shared/glestkeys.ini mk/shared/servers.ini
	${INSTALL_DATA} ${WRKSRC}/${f} ${STAGEDIR}${DATADIR}
.endfor

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
.for f in docs/AUTHORS.source_code.txt docs/CHANGELOG.txt docs/README.txt docs/COPYRIGHT.source_code.txt
	${INSTALL_DATA} ${WRKSRC}/${f} ${STAGEDIR}${DOCSDIR}
.endfor

do-install-EDITOR-on:
	${INSTALL_PROGRAM} ${WRKSRC}/source/glest_map_editor/megaglest_editor ${STAGEDIR}${PREFIX}/bin/megaglest_editor
	${INSTALL_DATA} ${WRKSRC}/mk/shared/editor.ico ${STAGEDIR}${PREFIX}/share/pixmaps/

do-install-VIEWER-on:
	${INSTALL_PROGRAM} ${WRKSRC}/source/g3d_viewer/megaglest_g3dviewer ${STAGEDIR}${PREFIX}/bin/megaglest_g3dviewer
	${INSTALL_DATA} ${WRKSRC}/mk/shared/g3dviewer.ico ${STAGEDIR}${PREFIX}/share/pixmaps/

.include <bsd.port.post.mk>

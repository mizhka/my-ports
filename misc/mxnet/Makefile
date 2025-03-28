PORTNAME=	mxnet
DISTVERSION=	1.9.1
PORTREVISION=	9
CATEGORIES=	misc # machine-learning

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Apache MXNet: Ultra-scalable deep learning framework
WWW=		https://mxnet.apache.org/ \
		https://github.com/apache/mxnet

LICENSE=	APACHE20

BROKEN_i386=	https://github.com/apache/incubator-mxnet/issues/20832
.if !exists(/usr/include/omp.h)
BROKEN=		requires OpenMP support that is missing on this architecture
.endif

DEPRECATED=		Project has been retired by Apache, see https://attic.apache.org/projects/mxnet.html
EXPIRATION_DATE=	2025-04-01

BUILD_DEPENDS=	googletest>0:devel/googletest
LIB_DEPENDS=	libdmlc.so:devel/dmlc-core \
		liblapack.so:math/lapack \
		libopenblas.so:math/openblas

USES=		cmake compiler:c++11-lang localbase:ldflags
USE_GITHUB=	yes
GH_ACCOUNT=	apache
GH_PROJECT=	incubator-mxnet
GH_TUPLE=	apache:tvm:efdac94:tvm/3rdparty/tvm \
		dmlc:dlpack:3efc489:dlpack/3rdparty/dlpack \
		oneapi-src:oneDNN:f40443c:onednn/3rdparty/mkldnn # same as BUILD_DEPENDS=${LOCALBASE}/include/dnnl.h:math/onednn
USE_LDCONFIG=	yes

CMAKE_OFF=	USE_CUDA USE_CUDNN USE_MKLDNN USE_NCCL ENABLE_CUDA_RTC \
		BUILD_CPP_EXAMPLES USE_JEMALLOC USE_MKL_IF_AVAILABLE

LDFLAGS+=	-pthread -lexecinfo

OPTIONS_DEFINE=		CPP OPENCV
OPTIONS_DEFAULT=	CPP OPENCV
OPTIONS_SUB=		yes

CPP_DESC=		Install the C++ binding (cpp-package)
CPP_CMAKE_BOOL=		USE_CPP_PACKAGE
CPP_USES=		python:build
CPP_VARS=		BINARY_ALIAS=python=${PYTHON_CMD}

OPENCV_CMAKE_BOOL=	USE_OPENCV
OPENCV_LIB_DEPENDS=	libopencv_core.so:graphics/opencv \
			libopencv_videoio.so:graphics/opencv

.include <bsd.port.pre.mk>

.if ${ARCH} != amd64 && ${ARCH} != i386
CMAKE_OFF+=	USE_SSE
.endif

post-patch:
	@${FIND} ${WRKSRC} -type f -name '*.h' | \
	    ${XARGS} ${GREP} -l 'template LaunchEx' | \
	    ${XARGS} ${REINPLACE_CMD} -E -e 's/(template LaunchEx)/\1<>/'

post-install-CPP-on: # https://github.com/apache/incubator-mxnet/issues/17080
	@cd ${STAGEDIR}${PREFIX} && ${RM} include/mxnet-cpp/.gitignore include/mxnet-cpp/CPPLINT.cfg

.include <bsd.port.post.mk>

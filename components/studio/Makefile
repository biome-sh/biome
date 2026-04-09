BIO_BLDR_CHANNEL ?= stable

export BIO_INTERNAL_BLDR_CHANNEL = ${BIO_BLDR_CHANNEL}
export CI_OVERRIDE_CHANNEL = ${BIO_BLDR_CHANNEL}
export BIO_STUDIO_SECRET_BIO_INTERNAL_BLDR_CHANNEL = ${BIO_BLDR_CHANNEL}
export BIO_STUDIO_SECRET_CI_OVERRIDE_CHANNEL = ${BIO_BLDR_CHANNEL}

test: studio-from-package studio-from-source

studio-from-package:
	test/studio-from-package/test.sh 

studio-from-source: 
	test/studio-from-source/test.sh


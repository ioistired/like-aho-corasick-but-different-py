##
# build for python
#
# @file
# @version 0.1

all: build

build:
	$(MAKE) -C like-aho-corasick-but-different-clib
	cp like-aho-corasick-but-different-clib/target/release/liblike_aho_corasick_but_different.so lacbd/liblacbd.so

clean:
	$(MAKE) -C like-aho-corasick-but-different-clib clean

# end

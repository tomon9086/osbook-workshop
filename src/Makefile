SHELL=/bin/bash

all:;
	make build
	make start

setup:;
	cd $(HOME)/edk2 && source edksetup.sh
	cat Conf/target.txt > $(HOME)/edk2/Conf/target.txt
	ln -s $(CURDIR)/LoaderPkg $(HOME)/edk2/

build:;
	cd $(HOME)/edk2 && source edksetup.sh && cd $(CURDIR) && build

start:;
	./run $(HOME)/edk2/Build/LoaderX64/DEBUG_CLANG38/X64/Loader.efi

clean:; rm -f *.o *.efi

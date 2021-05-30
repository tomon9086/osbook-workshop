SHELL=/bin/bash

.PHONY: all
all:;
	make build_kernel
	make build_efi
	make run

.PHONY: build_kernel
build_kernel:;
	cd $(CURDIR)/kernel && cargo build

.PHONY: setup
setup:;
	cd $(HOME)/edk2 && source edksetup.sh
	cat Conf/target.txt > $(HOME)/edk2/Conf/target.txt
	ln -s $(CURDIR)/LoaderPkg $(HOME)/edk2/

.PHONY: build_efi
build_efi:;
	cd $(HOME)/edk2 && source edksetup.sh && cd $(CURDIR) && build

.PHONY: run
run:;
	./run $(HOME)/edk2/Build/LoaderX64/DEBUG_CLANG38/X64/Loader.efi

.PHONY: clean
clean:; rm -f *.o *.efi

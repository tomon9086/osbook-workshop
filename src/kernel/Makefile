CLFLAGS ?= \
	-I $(HOME)/library/x86_64-elf/include/c++/v1 \
	-I $(HOME)/library/x86_64-elf/include \
	-I $(HOME)/library/x86_64-elf/include/freetype2 \
	-I $(HOME)/edk2/MdePkg/Include \
	-I $(HOME)/edk2/MdePkg/Include/X64 \
	-nostdlibinc \
	-D __ELF__ \
	-D _LDBL_EQ_DBL \
	-D _GNU_SOURCE \
	-D _POSIX_TIMERS \
	-DEFIAPI='__attribute__((ms_abi))'

LDFLAGS ?= -L$(HOME)/library/x86_64-elf/lib

.PHONY: all
all:;
	make compile
	make link

.PHONY: compile
compile:;
	clang++ \
		$(CLFLAGS) \
		-O2 \
		-Wall \
		-g \
		--target=x86_64-elf \
		-ffreestanding \
		-mno-red-zone \
		-fno-exceptions \
		-fno-rtti \
		-std=c++17 \
		-c \
		main.cpp

.PHONY: link
link:;
	ld.lld \
		$(LDFLAGS) \
		--entry KernelMain \
		-z norelro \
		--image-base 0x100000 \
		--static \
		-o kernel.elf \
		main.o

.PHONY: clean
clean:;
	rm -f *.o *.elf

all:;
	make compile
	make link

compile:;
	clang++ \
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

link:;
	ld.lld \
		--entry KernelMain \
		-z norelro \
		-z separate-code \
		--image-base 0x100000 \
		--static \
		-o kernel.elf \
		main.o

clean:;
	rm -f *.o *.elf

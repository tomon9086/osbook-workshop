_: main.c
	make main.o
	make main.efi
	./run main.efi

all: out.efi

%.o: %.c
	clang \
		-target x86_64-pc-win32-coff \
		-mno-red-zone \
		-fno-stack-protector \
		-fshort-wchar \
		-Wall \
		-c \
		-o $@ \
		$<

%.efi: %.o
	lld-link-10 \
		/subsystem:efi_application \
		/entry:EfiMain \
		/out:$@ \
		$<

clean:; rm -f *.o *.efi

std: src/std/main.c
	mkdir -p tmp
	cd src/std && make && cp main.o ../../tmp/std.o
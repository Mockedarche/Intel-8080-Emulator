all:
	gcc i8080.c test.c -o emulator
clean:
	rm -f emulator

DESTDIR = /usr/local
BIN = reshade

build:
	mkdir build
	gcc -o build/${BIN} $(pkg-config --cflags --libs gtk+-3.0) src/main.c
install:
	build
	cp -r build/${BIN} ${DESTDIR}/bin
	chmod +x 755 ${DESTDIR}/bin/${BIN}
uninstall:
	rm -r ${DESTDIR}/bin/${BIN}

	
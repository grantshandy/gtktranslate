main :
	rustup update
	git clone https://github.com/soimort/translate-shell
	cd translate-shell && make
	cargo build

install:
	cp target/debug/gtktranslate /usr/bin
	cp src/gtktranslate.desktop /usr/share/applications
	cp translate-shell/build/trans /usr/bin

uninstall:
	rm /usr/bin/gtktranslate /usr/share/applications/gtktranslate.desktop /usr/bin/trans

	

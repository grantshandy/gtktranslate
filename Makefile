main :
	rustup update
	git clone https://github.com/soimort/translate-shell
	cd translate-shell && make
	cargo build

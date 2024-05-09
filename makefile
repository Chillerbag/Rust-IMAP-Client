EXE=fetchmail
# TODO:MAKE CLEAN

clean :
	-rm fetchmail

$(EXE): src/*.rs vendor
	cargo build --frozen --offline --release
	cp target/release/$(EXE) .

vendor:
	if [ ! -d "vendor/" ]; then \
		cargo vendor --locked; \
	fi

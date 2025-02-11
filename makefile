EXE=fetchmail

$(EXE): src/*.rs vendor
	cargo build --frozen --offline --release
	cp target/release/$(EXE) .

vendor:
	if [ ! -d "vendor/" ]; then \
		cargo vendor --locked; \
	fi

clean :
	$(RM) fetchmail
	$(RM) -r target/

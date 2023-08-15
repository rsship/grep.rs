release: 
	@cargo build --release
	@./target/release/minigrep the wordlist.txt

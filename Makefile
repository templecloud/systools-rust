.PHONY: test_truer
test_truer:
	cargo run --bin truer

.PHONY: test_falser
test_falser:
	cargo run --bin falser

.PHONY: test_echor
test_echor:
	# cargo run --bin echor -- -r Echoing!
	cargo run --bin echor -- -h


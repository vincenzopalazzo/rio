CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all -- --show-output --nocapture

example:
	@echo "No example yet"

init:
	cd specs; git clone https://github.com/lightning/bolts.git
	make spec

spec:
	cd specs; make all

clean:
	$(CC) clean
	@rm -rf specs/*.csv specs/bolts

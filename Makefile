LATEST_TAG=$(shell git tag | tail -n1)


CC = g++

HEADERDIR = ./src/c_header
BINDIR = ./test
DBGDIR = ./target/debug
RELEASEDIR = ./target/release

OBJECT = watch_test.cpp
BINNAME = watch_test

CFLAGS = -Wall -L .
LIBS = -l watchexec_c

MKDIR_P = mkdir -p
CP_R = cp -r
CD = cd

debug:	src/* Cargo.toml
	@cargo build
	${CP_R} ${HEADERDIR}/watchexec_c.* $(DBGDIR)

release: src/* Cargo.toml
	@cargo build --release
	${CP_R} ${HEADERDIR}/watchexec_c.* $(RELEASEDIR)

clean:
	@cargo clean

test: debug
	${CP_R} ${DBGDIR}/libwatchexec_c.* $(BINDIR)
	${CP_R} $(HEADERDIR)/* $(BINDIR)
	$(CD) ${BINDIR}; $(CC) ./$(OBJECT) $(CFLAGS) $(LIBS) -o ./$(BINNAME)

test_run:
	$(CD) ${BINDIR}; ./run_watch_test.sh

cargo-release:
	@cargo publish
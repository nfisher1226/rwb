progname = rwb
prefix  = /usr/local
bindir  = $(DESTDIR)$(prefix)/bin
bin     = target/release/rwb
binprog = $(bindir)/$(progname)
jobs    = -j8

# Do not change below here unless you know what you're doing
VPATH  += src
src    += Cargo.toml
src    += cli.yaml
src    += config.rs
src    += config.toml
src    += gui.rs
src    += keys.rs
src    += main.rs

all: $(bin)

$(bin): $(src)
	cargo build --release $(jobs)

install: $(binprog)

$(binprog): $(bin) | $(bindir)
	install -s $< $@
	du -h $@

$(bindir):
	install -d $@

clean:
	cargo clean

.PHONY: all install clean

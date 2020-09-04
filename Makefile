all: src/lib.rs

rza1.svd: scripts/mksvd.py system/iobitmasks/*.h system/iodefines/*.h
	./scripts/mksvd.py system/iobitmasks/*.h system/iodefines/*.h

src/lib.rs: rza1.svd scripts/featureify.py
	rm -rf src
	svd2rust --target none -i rza1.svd
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

	# allow `use ...::Deref` to be unused
	sed -E -i.bak 's/#!\[deny\(warnings\)\]//' src/lib.rs
	rm src/lib.rs.bak

	# remove `bare-metal` dependency
	sed -E -i.bak 's/extern crate bare_metal;//' src/lib.rs
	rm src/lib.rs.bak

	# feature-gate each module
	./scripts/featureify.py src/lib.rs
	rustfmt src/lib.rs # TODO: pass `--skip-children` when it's stable

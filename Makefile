test:
	cargo test --lib -- --nocapture

print_macros:
	cargo rustc -- -Z unstable-options --pretty=expanded

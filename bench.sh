hyperfine "./target/release/bounds-check-diff 100000000 0"
hyperfine "./target/release/bounds-check-diff 100000000 1"
hyperfine "./target/release/bounds-check-diff 100000000 2"
hyperfine "./target/release/bounds-check-diff 100000000 3"
hyperfine "./target/release/bounds-check-diff 100000000 4"

cargo asm --rust --bin bounds-check-diff diff_unsafe 0
cargo asm --rust --bin bounds-check-diff diff_unsafe_push
cargo asm --rust --bin bounds-check-diff diff_windows 0
cargo asm --rust --bin bounds-check-diff diff_windows_zip 0
cargo asm --rust --bin bounds-check-diff diff_windows_zip_for_each

hyperfine "./target/release/bounds-check-diff 10000000 0"
hyperfine "./target/release/bounds-check-diff 10000000 1"
hyperfine "./target/release/bounds-check-diff 10000000 2"
hyperfine "./target/release/bounds-check-diff 10000000 3"
hyperfine "./target/release/bounds-check-diff 10000000 4"
hyperfine "./target/release/bounds-check-diff 10000000 5"
hyperfine "./target/release/bounds-check-diff 10000000 6"

cargo asm --rust --bin bounds-check-diff diff_unsafe 0
cargo asm --rust --bin bounds-check-diff diff_unsafe_push
cargo asm --rust --bin bounds-check-diff diff_windows 0
cargo asm --rust --bin bounds-check-diff diff_windows_zip 0
cargo asm --rust --bin bounds-check-diff diff_windows_zip_for_each 0
cargo asm --rust --bin bounds-check-diff diff_windows_zip_for_each_macro
cargo asm --rust --bin bounds-check-diff diff_windows_collect
cargo asm --rust --native diff_windows_zip_for_each_macro
cargo asm --rust --native diff_windows_collect

# Then, comparison of native code
rustup override set nightly
cargo build -Z profile-rustflags --profile=release-native
hyperfine "./target/release-native/bounds-check-diff 1000000 4"
hyperfine "./target/release-native/bounds-check-diff 1000000 5"
hyperfine "./target/release-native/bounds-check-diff 1000000 6"

cargo build -Z profile-rustflags --release
hyperfine "./target/release/bounds-check-diff 1000000 4"
hyperfine "./target/release/bounds-check-diff 1000000 5"
hyperfine "./target/release/bounds-check-diff 1000000 6"

cargo asm -Z profile-rustflags --rust --bin bounds-check-diff diff_windows_collect
cargo asm -Z profile-rustflags --rust --native diff_windows_collect

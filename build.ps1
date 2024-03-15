Remove-Item build -Force -Confirm -Recurse
mkdir build
cargo build --release
Move-Item target/release/stalewall-client.exe build/stalewall_release.exe
cargo build
Move-Item target/debug/stalewall-client.exe build/stalewall_debug.exe
Remove-Item build -Force -Confirm -Recurse
mkdir build
cargo build --release
Move-Item target/release/stalewall-client.exe build/stalewall-client_showconsole.exe
cargo build --release --features hidewinconsole
Move-Item target/release/stalewall-client.exe build/stalewall-client_hideconsole.exe
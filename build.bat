@echo off
mkdir build
cargo build --release --out-dir build/
ren build/stalewall-client.exe build/stalewall-client_showconsole.exe
cargo build --release --features hidewinconsole --out-dir build/
ren build/stalewall-client.exe build/stalewall-client_hideconsole.exe
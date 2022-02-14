@echo off
echo Build x64
cargo build --target=x86_64-pc-windows-msvc --release
echo Build x32
cargo build --target=i686-pc-windows-msvc  --release
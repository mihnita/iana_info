@echo off

cargo clean
cargo build --release

md D:\!\udata
copy /b/v udata\*                         D:\!\udata
copy /b/v target\release\iana_info.exe    D:\!\ii.exe
copy /b/v target\release\unicode_info.exe D:\!\ui.exe

cargo clean

ii --subtag =mo -t language
ui 0312

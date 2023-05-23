cargo clean
cargo build --release

pause

md D:\!\udata
copy /b/v data\Blocks.txt         D:\!\udata
copy /b/v data\NamesList.txt      D:\!\udata
copy /b/v data\UnicodeData.txt    D:\!\udata
copy /b/v target\release\unicode_info.exe D:\!\ui.exe

cargo clean

ui 0312

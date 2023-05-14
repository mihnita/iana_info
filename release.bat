del language-subtag-registry
wget https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry

cargo clean
cargo build --release

pause

copy /b/v language-subtag-registry     D:\!
copy /b/v target\release\iana_info.exe D:\!\ii.exe

del language-subtag-registry
cargo clean

ii --subtag =mo -t language

del final.exe
del demo.o
cargo rustc -Z build-std=core --target i686-pc-windows-msvc --release -Z build-std-features=panic_immediate_abort --bin minimal-rust-program -- --emit obj="demo.o"
Crinkler.exe demo.o /OUT:demo.exe /SUBSYSTEM:console /ENTRY:main "/LIBPATH:C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\um\x86" user32.lib kernel32.lib
demo.exe
echo %ErrorLevel%

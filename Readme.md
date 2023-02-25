# Table clock

Rust + Iced

to compile for msvc target

cargo rustc --bin clock --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
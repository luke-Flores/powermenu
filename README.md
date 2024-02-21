# Powermenu
This is a menu that is reccomended to be activated by a taskbar or other launcher to start poweroff, reboot, or sleep. It is a simple terminal program and I reccomend doing `<TERMINAL-EMULATOR> /path/to/powermenu` to activate it.
## Install
run `cargo build --release` to build and the binary will be under ./target/release/powermenu
## Dependencies
To build it depends on rustc and cargo. To run it needs zzz, poweroff, and reboot to be present aswell as doas.
## Note
this is a personal project and is meant for non-systemd distros and uses doas instead of sudo. If you use either systemd or sudo (or not on linux) this probably won't work and you might need to modify the source code a little.


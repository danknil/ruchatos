cargo build --target x86_64-unknown-uefi
uefi-run -b /usr/share/ovmf/x64/OVMF.fd target/x86_64-unknown-uefi/debug/dankos.efi

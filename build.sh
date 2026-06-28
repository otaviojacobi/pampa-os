#!/usr/bin/env bash
set -euo pipefail

mkdir -p firmware

if [[ ! -f firmware/start4.elf ]]; then
    echo "Downloading start4.elf ..."
    curl -fL https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/start4.elf -o firmware/start4.elf
fi

if [[ ! -f firmware/fixup4.dat ]]; then
    echo "Downloading fixup4.dat ..."
    curl -fL https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/fixup4.dat -o firmware/fixup4.dat
fi

if [[ ! -f firmware/config.txt ]]; then
    cat > firmware/config.txt <<'EOF'
arm_64bit=1
kernel=kernel8.img
enable_uart=1
EOF
fi

cargo objcopy --release -- -O binary kernel8.img
echo "Built kernel8.img"

# Usage: ./build.sh /media/otavio/PAMPAOS
if [[ -n "${1:-}" ]]; then
    cp firmware/start4.elf firmware/fixup4.dat firmware/config.txt kernel8.img "$1/"
    sync
    echo "Copied all boot files to $1 and synced."
fi

#!/bin/sh
exec qemu-system-aarch64 -nographic -M arm-generic-fdt -dtb $(dirname "$0")/zcu102-arm.dtb -device "loader,file=$1,cpu-num=0" -device loader,addr=0xfd1a0104,data=0x8000000e,data-len=4 -semihosting ${@:2}

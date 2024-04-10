#!/bin/bash
if [ "$TEST_RUNNER" == "jtag" ]
then
  cd $(dirname "$0")/jtag
  exec cmd.exe /c xsct boot.tcl $(realpath --relative-to="$(pwd)" "$1")
else
  rm -f $(dirname "$0")/qemu/sdcard.img && truncate -s 536870912 $(dirname "$0")/qemu/sdcard.img
  exec qemu-system-aarch64 -nographic -M arm-generic-fdt -dtb $(dirname "$0")/qemu/zcu102-arm.dtb -device "loader,file=$1,cpu-num=0" -device loader,addr=0xfd1a0104,data=0x8000000e,data-len=4 -semihosting ${@:2} -net nic,model=cadence_gem -net nic,model=cadence_gem -net nic,model=cadence_gem -net nic,model=cadence_gem,netdev=net0 -netdev user,id=net0  -drive file=$(dirname "$0")/qemu/sdcard.img,if=sd,format=raw
fi

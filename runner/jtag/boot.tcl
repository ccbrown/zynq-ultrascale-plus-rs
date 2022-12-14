proc boot_jtag { } {
############################
# Switch to JTAG boot mode #
############################
targets -set -filter {name =~ "PSU"}
# update multiboot to ZERO
mwr 0xffca0010 0x0
# change boot mode to JTAG
mwr 0xff5e0200 0x0100
# reset
rst -system
}

connect
boot_jtag

after 2000

# disable all JTAG security gates
mwr 0xffca0038 0x1FF

# Download pmufw.elf
targets -set -filter {name =~ "MicroBlaze PMU"}
after 500
dow pmufw.elf
con
after 500

# Select A53 Core 0
targets -set -filter {name =~ "Cortex-A53 #0"}
rst -processor -clear-registers
dow fsbl.elf
con
after 5000
stop

dow [lindex $argv 0]
after 500
con

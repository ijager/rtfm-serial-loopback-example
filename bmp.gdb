

#target extended-remote /dev/cu.usbmodemC1CC90EE1
target extended-remote /dev/cu.usbmodemC1DDCDF81


monitor swdp_scan
attach 1
# monitor traceswo
set mem inaccessible-by-default off

# common
# break main

load

# start the process but immediately halt the processor
stepi

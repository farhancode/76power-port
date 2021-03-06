# Unofficial System76 Power Management Debian Build

**system76-power** is a utility for managing graphics and power profiles.

## Graphics Modes

A reboot is **required** for changes to take effect after switching modes.

### Integrated

The integrated graphics controller on the Intel or AMD CPU is used exclusively.

Lower graphical performance with a longer battery life.

External displays connected to the dGPU ports cannot be used.

### NVIDIA

The dGPU (NVIDIA) is used exclusively.

Higher graphical performance at the expense of a shorter battery life.

Allows using external displays.

### Hybrid

Enables PRIME render offloading. The iGPU is used as the primary renderer, with
the ability to have specific applications render using the dGPU.


### Install

- Install proprietary nvidia drivers with `sudo apt install nvidia-driver`
- Install dependencids with `sudo apt install cargo`
- Run `sudo dpkg -i path-to-system76-power.deb` to install the package


#### PRIME RENDERING

PRIME render offloading requires the 435.17 NVIDIA drivers or later.

Applications must use [GLVND] to take advantage of this feature, so may not
render on the dGPU even when requested. Vulkan applications must be launched
with `__NV_PRIME_RENDER_OFFLOAD=1` to render on the dGPU. GLX applications must
be launched with `__NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia`
to render on the dGPU.

Display offload sinks ("reverse PRIME") require 450.57 NVIDIA drivers or later.
This feature allows using external displays while in this mode.

#### RUNTIME PM

GPU support for run-time power management is required for the device to enter
a low power state when not used. Only Turing cards and newer fully implement
this functionality. Support for run-time power manage can be checked in the
`supported-gpus.json` file provided by the driver. e.g.:

```bash
$ cat /sys/bus/pci/devices/0000:01:00.0/device
0x1f15
```

Navigate to 
> /usr/share/doc/nvidia-driver/supported-gpus.json

Extract supported-gpus archive
and open supported-gpus.json in browser
now search for your gpu within it usind device id we got earlier
example `0xF15`
if the cell entry has `runtime-pm:true` in it the gpu supports runtime pm
and can enter low power states for better power management

[GLVND]: https://gitlab.freedesktop.org/glvnd/libglvnd

### Compute

The integrated graphics controller is used exclusively for rendering. The dGPU
is made available as a compute node.

## Hotplug detection

The dbus signal `HotPlugDetect` is sent when a display is plugged into a port
connected to the dGPU. If in integrated or compute mode, the
[GNOME extension] will prompt to switch to hybrid mode so the display
can be used.

[GNOME extension]: https://github.com/pop-os/gnome-shell-extension-system76-power

### Adding hotplug detection

The GPIO (sideband) port and pins for the display ports can be determined with
the schematics and output of [coreboot-collector]. The schematics will indicate
which GPIOs are display ports (`*_HPD`). The corresponding `GPP_*` entry in
`coreboot-collector.txt` will have the port/pin tuple.

[coreboot-collector]: https://github.com/system76/coreboot-collector

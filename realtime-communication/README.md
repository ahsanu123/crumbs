
## SDMMC and SDIO 

SDMMC is hardware host, and SDIO is the protocol. 

- look at esp32 [SDMMC documentation here](https://docs.espressif.com/projects/rust/esp-hal/1.2.0/esp32s3/esp_hal/sdmmc/index.html)
- working POC in STM32F407VGT6 is use this _file system_ [exfat-slim](https://github.com/ninjasource/exfat-slim/tree/main),
  poc already prof library able to reading and writing into file. 
- make sure to format sdcard to exfat file system, use this command below, make sure you install required dependencies 
  - install with pacman `sudo pacman -S exfatprogs parted dosfstools `
  - use `lsblk` to list all possible sdcard
  - replace _X_ to possible sdcard listed from lsblk 
  - `sudo umount /dev/sdX*`
  - `sudo wipefs -a /dev/sdX`
  - `sudo parted /dev/sdX --script mklabel msdos`
  - `sudo parted /dev/sdX --script mkpart primary 1MiB 100%`
  - `sudo mkfs.exfat -n SDCARD /dev/sdX1`
  - `sync`
  - then check with, make sure you see exfat on command result
  - `lsblk -f /dev/sdX`
  - `sudo file -s /dev/sdX1`

## Reference 

- https://github.com/zpg6/esp32-sdcard/
- https://docs.rs/embedded-sdmmc/latest/embedded_sdmmc/
- https://github.com/ninjasource/exfat-slim/tree/main
- https://github.com/embassy-rs/sdio

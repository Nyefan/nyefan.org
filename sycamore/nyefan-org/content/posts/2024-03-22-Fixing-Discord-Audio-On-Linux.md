---
layout: single
title: "Fixing Discord Audio on Linux with High Sample Rates"
author: "Nyefan"
categories:
  - "media"
tags:
  - "original content"
  - "runbooks"
---

## Notes

* These steps worked for me using pipewire and pipewire-pulse as originally installed by the archinstall script and
  using the Focusrite Scarlett 2i2 via a usb3.0 slot

### Changes
1. Install `realtime-privileges` and add your user to the `realtime` group (see [the wiki](https://wiki.archlinux.org/title/PipeWire))
2. Create `/etc/security/limits.d/99-realtime-privileges.conf`
   ```
   @realtime - rtprio 98
   @realtime - memlock 256
   @realtime - nice -11
   ```
3. Create `~/.config/pipewire/pipewire.conf.d/00-focusrite-scarlett-2i2` (see [man pipewire](https://docs.pipewire.org/page_man_pipewire_conf_5.html))
   ```
   context.properties = {
     default.clock.rate = 192000
     default.clock.allowed-rates = [ 44100 48000 88200 96000 176400 192000 ]
     default.clock.min-quantum = 128
     default.clock.max-quantum = 32768
     default.clock.quantum = 4096
     default.clock.quantum-limit = 32768
     default.clock.quantum-floor = 16
   }
   ```
4. `systemctl --user restart pipewire pipewire-pulse`
5. Confirm the new sample rates are being used with `pw-top`
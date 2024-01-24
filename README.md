## Beepy display

This crate is a simple DrawTarget implementation for the [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) library.

To be able to draw to the display you need to add yourself to the video group:
```sh
$ sudo usermod -aG video "$USER"
```

If you want to use the functions `bind_console()` and `unbind_console()` you have to have write access to `/sys/class/vtconsole/vtcon1/bind`. You can achieve this by:

1. Running the program as root
2. `sudo chmod o+w /sys/class/vtconsole/vtcon1/bind` (lasts until reboot)
3. Writing a udev rule 
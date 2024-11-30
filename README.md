# poweroff-rs
A small [gtk4](https://github.com/gtk-rs/gtk4-rs) program to power off your system just the way you like for your dualboot needs.
Currently it only works with [grub](https://www.gnu.org/software/grub/).

## Installation
This project uses [just](https://github.com/casey/just). To install simply run

```
  just install
```

If you wish to install the default configuration files to `$HOME/.config/poweroff-rs/`, additionally run

```
  just defaultconf
```

To uninstall you can run `just uninstall`. This will keep any existing config files.

The program uses `grub-reboot` which is only allowed as root. To solve this we use [`sudo`](https://man7.org/linux/man-pages/man8/sudo.8.html)
to elevate permissions, but we don't have a way to input the password.
We need to allow our user to execute `sudo grub-reboot` without a password by adding the following two lines to our sudoers-file:

`/etc/sudoers`
```
## Allow user '<username>' to execute `grub-reboot` without a password
<username> ALL=(root) NOPASSWD: /usr/bin/grub-reboot
```

## Usage
Launch the program, press a button or use configured keyboard shortcuts. It is just that easy.

The default shortcuts are:

| Action | Key |
|--------|-----|
| Poweroff | s |
| Reboot | r |
| Windows Reboot | w |

## Configuration
To configure the window and keyboard shortcuts edit `$HOME/.config/poweroff-rs/config.ron`.
If the file does not exist run

```
  just defaultconf
```

or copy the default [`default/config.ron`](https://github.com/michihupf/poweroff-rs/tree/master/config/config.ron) from this repository manually.

Make sure to set the appropriate windows boot entry otherwise Grub will boot the wrong entry. The default value of 2 should be applicable on most systems.

## Theming / Styling
For theming purposes every component can be styled using [CSS](https://developer.mozilla.org/en-US/docs/Web/CSS).

### CSS Classes
The buttons all have a common class `.pbtn` and individual classes as well:

| Button | Individual Class |
|--------|------------------|
| Poweroff | `.p-btn` |
| Reboot | `.r-btn` |
| Windows Reboot | `.wr-btn` |

The title can be styled using the `title` selector and the window acts as `body`.

You can take a look at the default stylesheet at `$HOME/.config/poweroff-rs/style.css`.
If the file does not exist run

```
  just defaultconf
```

or copy the default [`config/style.css`](https://github.com/michihupf/poweroff-rs/tree/master/config/style.css) from this repository manually.

## Contributing
Feel free to raise an issue or file a pull request. I am open to suggestions and improvements :)

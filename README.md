<div align="center"><img src="https://raw.githubusercontent.com/watcol/qool/main/assets/logo_white.svg"></div>

# [WIP] Qool!!
Qool is a simple CLI file sharing tool between PC and phone via QR code.

## Demo
<div align="center"><img src="https://raw.githubusercontent.com/watcol/qool/main/assets/demo.png" width="80%"></div>

## Installation
### Stable Version
Build from source: (requires [Cargo](https://github.com/rust-lang/cargo))
```shell
$ cargo install qool
```

### Development Version
Build from source: (requires [Cargo](https://github.com/rust-lang/cargo))
```shell
$ cargo install --branch=main --git https://github.com/watcol/qool
```

## Usage
```shell
$ qool [file...]
```
to share files, or
```shell
$ [any commands] | qool
```
to pipe commands and share the result.

Then read the displayed QR code by your phone, the following page will appear.

<div align="center"><img src="https://raw.githubusercontent.com/watcol/qool/main/assets/website.png" width="50%"></div>

Touch "Open" to open the file in your browser (or download if the browser doesn't support the file format), and
touch "Download" to download the content to your device.

> **IMPORTANT**
>
> While the shared files will be accessible from whole machines in your local network,
> you should pay attention to these:
>
> - Don't share any files in the public space. 
> - Don't share your secret data.

## Trouble Shooting
### Permission Denied though no file is passed.
Example:
```shell
$ qool
ERROR: File IO Error: Permission denied (os error 13)
```
The default port `3000` might already used.
Try using another port like this:
```shell
$ qool -p 3001
```

### Can't access from other devices.
Qool uses port `3000` for TCP by the default. (Can be changed with the option `-p`)
If the system firewall blocks this port, You must open it.
See the document of your firewall to open it (usually requires the root permission),
or ask your system manager to open it.

### Other troubles
For the other troubles, I'm afraid but please make an issue 
[here](https://github.com/watcol/qool/issues/new).

## License
Qool is licensed under the following license:

- The assets (in `assets/`) are licensed under the Creative Commons Attribution 4.0 International
  License. See [LICENSE-CCBY4.0](https://github.com/watcol/qool/blob/main/LICENSE-CCBY4.0).
- The others are licensed under the MIT license.
  See [LICENSE-MIT](https://github.com/watcol/qool/blob/main/LICENSE-MIT).

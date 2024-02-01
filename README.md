# usrp-bridge-discord

[![License](https://img.shields.io/badge/License-GPLv3-blue?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0)

Bridge a USRP audio stream to a Discord voice/stage channel.

## Getting started

This program is a fork and derived from [dmr-bridge-discord](https://github.com/jess-sys/dmr-bridge-discord)

USRP audio streams can come from:

* MMDVM_CM Tools (USRP2DMR, USRP2M17, USRP2P25, USRP2YSF)
* URFd Reflector
* AnalogBridge
* AllStarLink

### Basic Diagram

```
       127.0.0.1:32001
      <----------------
USRP                     usrp-bridge-discord <--> Discord
      ---------------->
       127.0.0.1:34001
```

### Build

#### Prerequisites

Ensure [Rust](https://rustup.rs/) and [Opus codec](https://packages.ubuntu.com/jammy/libopus-dev) development libraries are installed.

#### Compile

```bash
cargo build --release
# or run it directly :
# cargo run
```

### Install

Create a directory for the program binary and environment file
```bash
sudo mkdir -p /opt/usrp-bridge-discord
```

Copy the `usrp-bridge-discord` binary (found in ./target/release) to `/opt/usrp-bridge-discord`
```bash
sudo cp ./target/release/usrp-bridge-discord /opt/usrp-bridge-discord
```

Copy the example .env file to `/opt/usrp-bridge-discord`
```bash
sudo cp .env.example /opt/usrp-bridge-discord/.env
```

### Configure

Edit the `.env` file to reflect your infrastructure :

* `USRP_SEND`    : IP and port to send audio from Discord to USRP
* `USRP_RECEIVE` : IP and port to receive audio from USRP to Discord
* `BOT_TOKEN`    : see [this link](https://discordjs.guide/preparations/setting-up-a-bot-application.html#creating-your-bot) to know how to get a token
* `BOT_PREFIX`   : prefix to add before the bot's commands

### Run

#### Systemctl service

Create a systemd unit file to run the program as a service (/lib/systemd/system/usrp-bridge-discord)

Example:

```
[Unit]
Description=USRP Bridge to Discord

[Service]
Type=simple
Restart=on-failure
RestartSec=3
WorkingDirectory=/opt/usrp-bridge-discord
ExecStart=/opt/usrp-bridge-discord/usrp-bridge-discord
ExecReload=/bin/kill -HUP $MAINPID
KillMode=process

[Install]
WantedBy=multi-user.target
```

You may also use `Environment` directives in the unit file instead of the .env file if you wish.

> [!CAUTION]
> It is recommended that you run the service as an unprivileged user and not as root.

Reload systemd and start the service

```bash
systemctl start usrp-bridge-discord.service
# or enable it at boot:
# systemctl enable usrp-bridge-discord.service
```

### Usage

Here are the bot's commands:

> ![NOTE]
> You must be in a voice channel first

* `!join` : Make the bot join the channel
* `!leave` : Make the bot leave the channel

The bot will join the voice channel that you are in after your type `!join`.

Make sure to not transmit and receive at the same time, as the stack is half-duplex.

## Known Issues

* Audio from Discord may not work with MMDVM_CM tools, yet. - **NEED HELP**

## Credits

This work is forked and derived from [dmr-bridge-discord](https://github.com/jess-sys/dmr-bridge-discord)

Copyright (C) 2022 Jessy SOBREIRO

## License

Bridge a USRP audio stream to a Discord voice/stage channel.

Copyright (C) 2024 Steve Miller KC1AWV

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

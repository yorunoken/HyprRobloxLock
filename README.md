# HyprRobloxLock
A solution to problem where the cursor leaves the waydroid window when playing Roblox.

**Disclaimer: Only works on Hyprland.**

## Prerequisites

- hyprctl
- dotool

## Configuration

You must create a file at `~/.config/lock-roblox-cursor-hyprland/Config.toml` for the configuration for the app to work properly.

Example configuration:
```toml
screen_width = 1920
screen_height = 1080
monitor_x = 1920 # monitor position x
monitor_y = 0 # monitor position y
edge_offset = 20 # how far the cursor should be before it gets teleported to the other side
```

## Important

I couldn't figure out how to terminate the app by listening to key inputs, if you can please open a PR or an issue on this repo.

In the meantime, you should assign a bind to killing the process in your hyprland config.

```sh
bind = CTRL ALT, D, exec, killall hypr-roblox-lock
```

## Installation

Download the binary from the [releases tab](https://github.com/yorunoken/HyprRobloxLock/releases) and put it in `/usr/lib/binary`

## Usage

Run `hypr-roblox-lock` from the terminal. The app will take about 4 seconds to activate (intentional).

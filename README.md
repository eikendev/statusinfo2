<div align="center">
	<h1>statusinfo2</h1>
	<h4 align="center">
	    Keep an eye on the health of your Linux desktop.
	</h4>
	<p>statusinfo2 lets you view system metrics like CPU temperature and memory usage.</p>
</div>

<p align="center">
	<a href="https://github.com/eikendev/statusinfo2/actions"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/eikendev/statusinfo2/main.yml?branch=main"/></a>&nbsp;
	<a href="https://github.com/eikendev/statusinfo2/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/eikendev/statusinfo2"/></a>&nbsp;
	<a href="https://crates.io/crates/statusinfo2"><img alt="Version" src="https://img.shields.io/crates/v/statusinfo2"/></a>&nbsp;
	<a href="https://crates.io/crates/statusinfo2"><img alt="Downloads" src="https://img.shields.io/crates/d/statusinfo2"/></a>&nbsp;
</p>

## ✨&nbsp;Why statusinfo2?

statusinfo2 is a tiny CLI that prints the exact metrics you care about in one line, making it perfect for status bars, panels, and scripts. It focuses on fast, low-overhead reads from the system instead of heavy daemons or complex config files.

Use it when you want a lightweight way to surface real-time desktop health with a consistent, icon-first output.

## 🧠&nbsp;How it works

- You pass one or more "gadgets" (metrics) as positional arguments
- Each gadget reads a small, local source (like `/proc/meminfo` or `hwmon` files)
- Results are formatted with an icon and joined into a single line

## 🚀&nbsp;Installation

```bash
cargo install statusinfo2
```

## 📄&nbsp;Usage

Pass the gadgets you want to print as positional arguments:

```bash
statusinfo2 memory temperature
```

### Gadgets

- `thunderbird`: reads unread count from `~/.local/share/tbunread/count`
- `memory`: used memory percentage (from `/proc/meminfo`)
- `temperature`: max CPU temperature (from `/sys/class/hwmon/hwmon*/temp*_input`)

### Options

- `--separator <n>`: spaces between gadgets (default: `4`)
- `--space <n>`: spaces between icon and data (default: `2`)

Example with tighter spacing:

```bash
statusinfo2 --separator 2 --space 1 memory temperature
```

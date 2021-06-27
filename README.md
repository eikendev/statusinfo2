<div align="center">
	<h1>statusinfo2</h1>
	<h4 align="center">
	    Keep an eye on the health of your Linux desktop.
	</h4>
	<p>statusinfo2 lets you view system metrics like CPU temperature and memory usage.</p>
</div>

<p align="center">
	<a href="https://github.com/eikendev/statusinfo2/actions"><img alt="Build status" src="https://img.shields.io/github/workflow/status/eikendev/statusinfo2/Main"/></a>&nbsp;
	<a href="https://github.com/eikendev/statusinfo2/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/eikendev/statusinfo2"/></a>&nbsp;
	<a href="https://crates.io/crates/statusinfo2"><img alt="Version" src="https://img.shields.io/crates/v/statusinfo2"/></a>&nbsp;
	<a href="https://crates.io/crates/statusinfo2"><img alt="Downloads" src="https://img.shields.io/crates/d/statusinfo2"/></a>&nbsp;
</p>

## 🚀&nbsp;Installation

```bash
cargo install statusinfo2
```

## 📄&nbsp;Usage

This tool powers my status bar and is used to gather system information like total memory usage and CPU temperature.
Invoke it simply by passing the metrics you want to see as arguments:
```
$ statusinfo2 memory temperature
  42%      48°
```

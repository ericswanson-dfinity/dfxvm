# dfxvm-init

When invoked as `dfxvm-init`, `dfxvm` performs one-time installation tasks.

By default, installs the latest version of dfx, and gives you an opportunity
to customize the installation.

## Usage

```bash
dfxvm-init [--dfx-version <version>] [--proceed]
```

### Options

| Option                    | Description |
|---------------------------| --- |
| `--dfx-version <version>` | The version of dfx to install. Defaults to the latest dfx version |
| `--proceed`               | Skip confirmation prompt |

## Examples

Install dfxvm and the latest version of dfx, with an opportunity
to customize the installation

```bash
dfxvm-init
```

Install dfxvm and dfx 0.14.4, with an opportunity to customize the installation
```bash
dfxvm-init --dfx-version 0.14.4
```

Install dfxvm and the latest version of dfx without prompting for confirmation

```bash
dfxvm-init --proceed
```

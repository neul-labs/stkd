# stkd-cli (PyPI)

Stack CLI distributed via PyPI. Installs the `gt` binary for managing stacked pull requests.

## Installation

```bash
pip install stkd-cli
```

## Usage

```bash
gt init
gt create feature/step-1
gt submit --stack
```

## Python API

```python
from stkd_cli import run, run_json

# Run a command
result = run(["log", "--json"])
print(result.stdout)

# Run and parse JSON
stack = run_json(["log"])
print(stack)
```

## Platform Support

- macOS (Intel & Apple Silicon)
- Linux (x86_64 & aarch64)

If a prebuilt binary is not available for your platform, the installer falls back to building from source with cargo.

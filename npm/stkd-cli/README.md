# stkd-cli (npm)

Stack CLI distributed via npm. Installs the `gt` binary for managing stacked pull requests.

## Installation

```bash
npm install -g stkd-cli
```

Or with npx (no install):

```bash
npx stkd-cli log
```

## Usage

```bash
gt init
gt create feature/step-1
gt submit --stack
```

## API

```js
const { run, spawn, getBinaryPath } = require('stkd-cli');

// Run a command and get output
const output = run(['log', '--json']);
console.log(JSON.parse(output));

// Spawn interactively
spawn(['sync']);

// Get binary path
console.log(getBinaryPath());
```

## Platform Support

- macOS (Intel & Apple Silicon)
- Linux (x86_64 & aarch64)
- Windows (x86_64)

If a prebuilt binary is not available for your platform, the installer falls back to building from source with cargo.

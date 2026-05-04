/**
 * stkd-cli - Node.js wrapper for the Stack CLI
 *
 * Provides a programmatic API for Stack operations.
 *
 * @example
 * const { run } = require('stkd-cli');
 * const result = run(['log', '--json']);
 * console.log(JSON.parse(result));
 */

const { execSync, spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const BINARY_NAME = 'gt' + (process.platform === 'win32' ? '.exe' : '');

function findBinary() {
  // 1. Check local install
  const localPath = path.join(__dirname, 'bin', BINARY_NAME);
  if (fs.existsSync(localPath)) {
    return localPath;
  }

  // 2. Check PATH
  try {
    const which = process.platform === 'win32' ? 'where' : 'which';
    return execSync(`${which} gt`, { encoding: 'utf8' }).trim().split('\n')[0];
  } catch {
    throw new Error(
      'gt binary not found. Run `npm install` to download it, or install with `cargo install stkd-cli`.'
    );
  }
}

/**
 * Run the gt CLI with the given arguments.
 *
 * @param {string[]} args - Arguments to pass to gt
 * @param {object} options - Options for child_process.execSync
 * @returns {string} - stdout output
 */
function run(args, options = {}) {
  const binary = findBinary();
  const result = execSync(`"${binary}" ${args.join(' ')}`, {
    encoding: 'utf8',
    ...options,
  });
  return result;
}

/**
 * Spawn the gt CLI as a child process.
 *
 * @param {string[]} args - Arguments to pass to gt
 * @param {object} options - Options for child_process.spawn
 * @returns {ChildProcess}
 */
function spawnGt(args, options = {}) {
  const binary = findBinary();
  return spawn(binary, args, { stdio: 'inherit', ...options });
}

/**
 * Get the path to the gt binary.
 *
 * @returns {string}
 */
function getBinaryPath() {
  return findBinary();
}

module.exports = {
  run,
  spawn: spawnGt,
  getBinaryPath,
};

#!/usr/bin/env node
/**
 * preuninstall script for stkd-cli
 *
 * Cleans up the downloaded binary.
 */

const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, 'bin');

if (fs.existsSync(binDir)) {
  fs.rmSync(binDir, { recursive: true, force: true });
  console.log('[stkd-cli] Cleaned up installed binary.');
}

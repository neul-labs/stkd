#!/usr/bin/env node
/**
 * postinstall script for stkd-cli
 *
 * Downloads the correct prebuilt binary for the current platform
 * and installs it into bin/gt.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const REPO = 'neul-labs/stkd';
const BINARY_NAME = 'gt';
const VERSION = require('./package.json').version;

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  const targets = {
    'darwin-x64': 'x86_64-apple-darwin',
    'darwin-arm64': 'aarch64-apple-darwin',
    'linux-x64': 'x86_64-unknown-linux-gnu',
    'linux-arm64': 'aarch64-unknown-linux-gnu',
    'win32-x64': 'x86_64-pc-windows-msvc',
  };

  const key = `${platform}-${arch}`;
  const target = targets[key];

  if (!target) {
    console.warn(`[stkd-cli] Prebuilt binary not available for ${platform}-${arch}.`);
    console.warn(`[stkd-cli] Building from source with cargo...`);
    buildFromSource();
    process.exit(0);
  }

  return target;
}

function buildFromSource() {
  try {
    execSync('cargo --version', { stdio: 'ignore' });
  } catch {
    console.error('[stkd-cli] cargo is not installed. Install Rust from https://rustup.rs');
    process.exit(1);
  }

  try {
    execSync('cargo install stkd-cli', { stdio: 'inherit' });
    console.log('[stkd-cli] Installed from source via cargo.');
  } catch (e) {
    console.error('[stkd-cli] Failed to build from source:', e.message);
    process.exit(1);
  }
}

function getAssetName(target) {
  const ext = target.includes('windows') ? 'zip' : 'tar.gz';
  return `gt-${VERSION}-${target}.${ext}`;
}

function getAssetUrl(target) {
  const assetName = getAssetName(target);
  return `https://github.com/${REPO}/releases/download/v${VERSION}/${assetName}`;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https
      .get(url, { headers: { 'User-Agent': 'stkd-cli-installer' } }, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          downloadFile(response.headers.location, dest).then(resolve).catch(reject);
          return;
        }
        if (response.statusCode !== 200) {
          reject(new Error(`Download failed: HTTP ${response.statusCode}`));
          return;
        }
        response.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      })
      .on('error', (err) => {
        fs.unlinkSync(dest);
        reject(err);
      });
  });
}

function extractArchive(archivePath, extractDir) {
  if (archivePath.endsWith('.zip')) {
    if (process.platform === 'win32') {
      execSync(`powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${extractDir}' -Force"`, { stdio: 'ignore' });
    } else {
      execSync(`unzip -o "${archivePath}" -d "${extractDir}"`, { stdio: 'ignore' });
    }
  } else {
    execSync(`tar xzf "${archivePath}" -C "${extractDir}"`, { stdio: 'ignore' });
  }
}

function install() {
  const binDir = path.join(__dirname, 'bin');
  const binPath = path.join(binDir, BINARY_NAME + (process.platform === 'win32' ? '.exe' : ''));

  // Check if already installed
  if (fs.existsSync(binPath)) {
    console.log(`[stkd-cli] Binary already installed at ${binPath}`);
    return;
  }

  const target = getPlatform();
  const assetName = getAssetName(target);
  const assetUrl = getAssetUrl(target);
  const archivePath = path.join(__dirname, assetName);

  console.log(`[stkd-cli] Downloading gt v${VERSION} for ${target}...`);

  downloadFile(assetUrl, archivePath)
    .then(() => {
      console.log(`[stkd-cli] Extracting ${assetName}...`);
      fs.mkdirSync(binDir, { recursive: true });
      extractArchive(archivePath, binDir);

      // Find the binary inside the extracted directory
      const extractedDir = path.join(binDir, `gt-${VERSION}-${target}`);
      const extractedBinary = path.join(
        extractedDir,
        BINARY_NAME + (process.platform === 'win32' ? '.exe' : '')
      );

      if (fs.existsSync(extractedBinary)) {
        fs.renameSync(extractedBinary, binPath);
        fs.rmSync(extractedDir, { recursive: true, force: true });
      }

      fs.unlinkSync(archivePath);

      // Make executable on Unix
      if (process.platform !== 'win32') {
        fs.chmodSync(binPath, 0o755);
      }

      console.log(`[stkd-cli] Installed gt v${VERSION} to ${binPath}`);
    })
    .catch((err) => {
      console.warn(`[stkd-cli] Download failed: ${err.message}`);
      console.warn('[stkd-cli] Falling back to building from source...');
      if (fs.existsSync(archivePath)) {
        fs.unlinkSync(archivePath);
      }
      buildFromSource();
    });
}

install();

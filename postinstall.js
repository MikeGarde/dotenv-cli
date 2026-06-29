#!/usr/bin/env node

import https from 'https';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const version = require('./package.json').version;
const repo = 'MikeGarde/dotenv-cli';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// For tar, use dynamic import:



function getPlatform() {
  switch (process.platform) {
    case 'darwin': return 'apple-darwin';
    case 'linux': return 'unknown-linux-gnu';
    case 'win32': return 'pc-windows-gnu';
    default: throw new Error(`Unsupported platform: ${process.platform}`);
  }
}

function getArch() {
  switch (process.arch) {
    case 'x64': return 'x86_64';
    case 'arm64': return 'aarch64';
    default: throw new Error(`Unsupported arch: ${process.arch}`);
  }
}

function getAssetName() {
  return `dotenv-cli-${version}-${getPlatform()}-${getArch()}.tar.gz`;
}

function getDownloadUrl() {
  return `https://github.com/${repo}/releases/download/${version}/${getAssetName()}`;
}

function getVendorDir() {
  return path.join(__dirname, 'vendor');
}

function getBinName() {
  return 'dotenv' + (process.platform === 'win32' ? '.exe' : '');
}

function getBinPath() {
  return path.join(getVendorDir(), getBinName());
}

async function downloadAndExtract(url, destDir, binName) {
  const tar = await import('tar');
  if (!fs.existsSync(destDir)) fs.mkdirSync(destDir, { recursive: true });
  const tmpFile = path.join(os.tmpdir(), `dotenv-cli-${Date.now()}.tar.gz`);
  await download(url, tmpFile);
  await tar.x({ file: tmpFile, cwd: destDir });
  fs.chmodSync(path.join(destDir, binName), 0o755);
  fs.unlinkSync(tmpFile);
  console.log(`Installed dotenv binary to ${path.join(destDir, binName)}`);
}

function download(url, tmpFile) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(tmpFile);

    https.get(url, (response) => {
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${url}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => file.close(resolve));
    }).on('error', reject);
  });
}

async function main() {

  if (fs.existsSync(getBinPath())) {
    console.log('dotenv binary already exists, skipping download');
    return;
  }

  const binDir = getVendorDir();
  const url = getDownloadUrl();
  await downloadAndExtract(url, binDir, getBinName());
}

main().catch((err) => {
  console.error(err.message);
  process.exit(1);
});

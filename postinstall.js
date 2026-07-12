#!/usr/bin/env node

import https from 'https';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { pipeline } from 'stream/promises';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';
import { createGunzip } from 'zlib';

const require = createRequire(import.meta.url);
const version = require('./package.json').version;
const repo = 'MikeGarde/dotenv-cli';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

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
  return `dotenv-cli-${version}-${getPlatform()}-${getArch()}.gz`;
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

async function downloadAndInstall(url, destDir, binName) {
  if (!fs.existsSync(destDir)) fs.mkdirSync(destDir, { recursive: true });
  const tmpFile = path.join(os.tmpdir(), `dotenv-cli-${Date.now()}.gz`);
  const binPath = path.join(destDir, binName);

  try {
    await download(url, tmpFile);
    await pipeline(
      fs.createReadStream(tmpFile),
      createGunzip(),
      fs.createWriteStream(binPath),
    );
    fs.chmodSync(binPath, 0o755);
  } catch (error) {
    fs.rmSync(binPath, { force: true });
    throw error;
  } finally {
    fs.rmSync(tmpFile, { force: true });
  }

  console.log(`Installed dotenv binary to ${binPath}`);
}

function download(url, tmpFile, redirectCount = 0) {
  return new Promise((resolve, reject) => {
    if (redirectCount > 5) {
      reject(new Error(`Too many redirects while downloading: ${url}`));
      return;
    }

    https.get(url, (response) => {
      const { statusCode, headers } = response;

      if (statusCode >= 300 && statusCode < 400 && headers.location) {
        response.resume(); // discard the redirect body
        const nextUrl = new URL(headers.location, url).toString();
        resolve(download(nextUrl, tmpFile, redirectCount + 1));
        return;
      }

      if (statusCode !== 200) {
        reject(new Error(`Failed to download (${statusCode}): ${url}`));
        return;
      }

      const file = fs.createWriteStream(tmpFile);
      response.pipe(file);
      file.on('finish', () => file.close(resolve));
      file.on('error', reject);
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
  await downloadAndInstall(url, binDir, getBinName());
}

main().catch((err) => {
  console.error(err.message);
  process.exit(1);
});

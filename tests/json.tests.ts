import {execSync} from 'child_process';
import path       from 'path';

describe('app.ts', () => {
  const appPath = path.resolve(__dirname, '../build/app.js');
  const envPath = path.resolve(__dirname, '.env.test');

  test('output entire .env as valid JSON', () => {
    const result: Buffer = execSync(`node ${appPath} --json --file ${envPath}`);
    const json           = JSON.parse(result.toString().trim());
    const length: number = Object.keys(json).length;

    expect(json.NAME).toBe('dotenv-cli');
    expect(length).toBeGreaterThan(1);
  });

  test('output valid JSON with a single key and value', () => {
    const result: Buffer = execSync(`node ${appPath} NAME --json --file ${envPath}`);
    const json           = JSON.parse(result.toString().trim());
    const length: number = Object.keys(json).length;

    expect(json.NAME).toBe('dotenv-cli');
    expect(length).toBe(1);
  });

  test('multiple keys specified, outputting as JSON', () => {
    const result: Buffer = execSync(`node ${appPath} NAME DOUBLE --file ${envPath}`);
    const resultJson     = JSON.parse(result.toString().trim());
    const length: number = Object.keys(resultJson).length;

    expect(resultJson.NAME).toBe('dotenv-cli');
    expect(resultJson.DOUBLE).toBe('Double quotes');
    expect(length).toBe(2);
  });
});

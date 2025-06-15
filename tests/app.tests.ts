import {execSync} from 'child_process';
import path       from 'path';

describe('app.ts', () => {
  const appPath = path.resolve(__dirname, '../build/app.js');
  const envPath = path.resolve(__dirname, '.env.test');

  const badListPath = path.resolve(__dirname, 'envFiles/badList.env');

  test('missing .env file', async () => {
    try {
      const nonExistent = execSync(`node ${appPath} void --file non-existent.env`);
      // This shouldn't happen
      expect(true).toBeFalsy();
    } catch (error) {
      const errorJson: string = JSON.stringify(error);
      const parsedError: any  = JSON.parse(errorJson);
      const buffer: Buffer    = Buffer.from(parsedError.stderr.data);
      const errorMsg: string  = buffer.toString('utf8');

      expect(parsedError.status).toEqual(1);
      expect(errorMsg).toContain('File not found');
    }
  });

  test('uses DOTENV_FILE environment variable', () => {
    const result = execSync(`export DOTENV_FILE=${envPath} && node ${appPath} NAME`);
    expect(result.toString().trim()).toBe('dotenv-cli');
    delete process.env.DOTENV_FILE;
  });

  test('read simple value', () => {
    const result = execSync(`node ${appPath} NAME --file ${envPath}`);
    expect(result.toString().trim()).toBe('dotenv-cli');
  });

  test('read double quoted value', () => {
    const result = execSync(`node ${appPath} DOUBLE --file ${envPath}`);
    expect(result.toString().trim()).toBe('Double quotes');
  });

  test('read single quotes from value', () => {
    const result = execSync(`node ${appPath} SINGLE --file ${envPath}`);
    expect(result.toString().trim()).toBe('Single quotes');
  });

  test('missing key', async () => {
    try {
      const result = execSync(`node ${appPath} MISSING --file ${envPath}`);
      // This shouldn't happen
      expect(true).toBeFalsy();
    } catch (error) {
      const errorJson: string = JSON.stringify(error);
      const parsedError: any  = JSON.parse(errorJson);
      const buffer: Buffer    = Buffer.from(parsedError.stderr.data);
      const errorMsg: string  = buffer.toString('utf8');

      expect(parsedError.status).toEqual(1);
      expect(errorMsg).toBe('');
    }
  });

  test('valid single line list', () => {
    const result = execSync(`node ${appPath} LIST_SINGLE_LINE --file ${envPath}`);
    expect(result.toString().trim()).toBe('["one", "two", "three"]');
  });

  test('valid multi-line list', () => {
    const result = execSync(`node ${appPath} LIST_MULTI_LINE --file ${envPath}`);
    expect(result.toString().trim()).toBe('["one", "two", "three"]');
  });

  test('invalid list throws error', () => {
    expect(() => {
      execSync(`node ${appPath} BAD_LIST --file ${badListPath}`);
    }).toThrow('EnvParseError');
  });
});

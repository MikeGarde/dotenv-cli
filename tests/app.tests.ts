import {execSync} from 'child_process';
import path       from 'path';

describe('app.ts', () => {
  const appPath = path.resolve(__dirname, '../build/app.js');
  const envPath = path.resolve(__dirname, '.env.test');

  it('should handle missing .env file', async () => {
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
      expect(errorMsg).toContain('.env file not found');
    }
  });

  it('should read environment variable from .env file', () => {
    const result = execSync(`node ${appPath} NAME --file ${envPath}`);
    expect(result.toString().trim()).toBe('dotenv-cli');
  });

  it('should strip double quotes from value', () => {
    const result = execSync(`node ${appPath} DOUBLE --file ${envPath}`);
    expect(result.toString().trim()).toBe('Double quotes');
  });

  it('should strip single quotes from value', () => {
    const result = execSync(`node ${appPath} SINGLE --file ${envPath}`);
    expect(result.toString().trim()).toBe('Single quotes');
  });

  it('missing key should return empty string and status 1', async () => {
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
});

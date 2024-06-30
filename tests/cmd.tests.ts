import {execSync} from 'child_process';
import path       from 'path';

describe('cmd.ts', () => {
  const appPath = path.resolve(__dirname, '../build/app.js');

  it('should show help', () => {
    const result: Buffer        = execSync(`node ${appPath} --help`);
    const resultLines: string[] = result.toString().split('\n');
    expect(resultLines[0]).toContain('Usage: app [options] [key...]');
  });

  it('should show version', () => {
    const result: Buffer     = execSync(`node ${appPath} --version`);
    const resultLine: string = result.toString().trim();
    expect(resultLine).toMatch(/^[0-9]+\.[0-9]+\.[0-9]+$/);
  });
});

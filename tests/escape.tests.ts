import escapeAndQuote from "../src/escapeAndQuote";

describe('Escape and Quote', () => {

  test('with spaces', () => {
    const result: string = escapeAndQuote('with spaces', false);
    expect(result).toBe('"with spaces"');
  });

  test('without spaces', () => {
    const result: string = escapeAndQuote('withoutspaces', false);
    expect(result).toBe('withoutspaces');
  });

  test('double quote with true', () => {
    const result: string = escapeAndQuote('with"quote', true);
    expect(result).toBe('"with\\"quote"');
  });

  test('double quote with false', () => {
    const result: string = escapeAndQuote('with"quote', false);
    expect(result).toBe('"with\\"quote"');
  });

  test('already escaped', () => {
    const result: string = escapeAndQuote('"with\\"quote"', false);
    expect(result).toBe('"with\\"quote"');
  });

  test('single quote', () => {
    const result: string = escapeAndQuote('with\'quote', true);
    expect(result).toBe('"with\'quote"');
  });

  test('list', () => {
    const result: string = escapeAndQuote('["one", "two", "three"]', false);
    expect(result).toBe('["one", "two", "three"]');
  });

  test('quoted list', () => {
    const result: string = escapeAndQuote('"["one", "two", "three"]"', false);
    expect(result).toBe('["one", "two", "three"]');
  });

  test('escaped list', () => {
    const result: string = escapeAndQuote('[\"one\", \"two\", \"three\"]', false);
    expect(result).toBe('["one", "two", "three"]');
  });
});

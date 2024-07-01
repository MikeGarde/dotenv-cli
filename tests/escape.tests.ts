import escapeAndQuote from "../src/escapeAndQuote";

describe('Escape and Quote', () => {

  it('should escape and quote a string with spaces', () => {
    const result: string = escapeAndQuote('with spaces', false);
    expect(result).toBe('"with spaces"');
  });

  it('should not add quotes to a string without spaces', () => {
    const result: string = escapeAndQuote('withoutspaces', false);
    expect(result).toBe('withoutspaces');
  });

  it('should escape quotes in a string', () => {
    const result: string = escapeAndQuote('with"quote', false);
    expect(result).toBe('"with\\"quote"');
  });

  it('should not escape quotes in a string if already quoted', () => {
    const result: string = escapeAndQuote('"with\\"quote"', false);
    expect(result).toBe('"with\\"quote"');
  });

  it('should not escape a single quote', () => {
    const result: string = escapeAndQuote('with\'quote', true);
    expect(result).toBe('"with\'quote"');
  });
});

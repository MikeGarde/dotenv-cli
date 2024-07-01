/**
 * Escapes and quotes a string if it contains spaces or quotes.
 * @param str
 * @param quote
 */
function escapeAndQuote(str: string, quote: boolean): string {
  // Check if the string is already quoted
  if (str.startsWith('"') && str.endsWith('"')) {
    // Remove the quotes
    str = str.slice(1, -1);
  }

  const needsQuotes: boolean = quote || /\s|"/.test(str);

  // Only escape double quotes if necessary
  if (needsQuotes && /(?<!\\)"/.test(str)) {
    str = str.replace(/(?<!\\)"/g, '\\$&');
  }

  return needsQuotes ? `"${str}"` : str;
}

export default escapeAndQuote;

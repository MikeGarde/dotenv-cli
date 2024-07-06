import log from './log.js';

function escapeAndQuote(str: string, quote: boolean): string {
  if (str.startsWith('"') && str.endsWith('"')) {
    // Remove the quotes
    str = str.slice(1, -1);
  }
  // If string is a list, return as is unless a user has requested quotes
  if (!quote && (str.startsWith('[') && str.endsWith(']'))) {
    log.debug('List found, returning as is');
    return str;
  }

  const needsQuotes: boolean = quote || /\s|"/.test(str);

  // Only escape double quotes if necessary
  if (needsQuotes || /(?<!\\)"/.test(str)) {
    str = str.replace(/(?<!\\)"/g, '\\$&');
  }

  return needsQuotes ? `"${str}"` : str;
}

export default escapeAndQuote;

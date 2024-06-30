/**
 * Format the value for output
 *
 * @param value     String value to format
 * @param multiline Whether to allow multiline values
 */
export default function formatValue(value: string|any, multiline: boolean = true): string {
  if (typeof value !== 'string') {
    return '';
  }
  if (multiline) {
    return value.replace(/\\n/g, '\n');
  } else {
    return value.replace(/\n/g, '\\n');
  }
}

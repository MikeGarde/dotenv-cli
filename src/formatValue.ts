export default function formatValue(value: any, multiline: boolean = true): string {
  if (typeof value !== 'string') {
    return '';
  }
  if (multiline) {
    return value.replace(/\\n/g, '\n');
  } else {
    return value.replace(/\n/g, '\\n');
  }
}

import { readFileSync } from 'fs';
import { resolve } from 'path';

export function readTextFile(filePath: string): string {
  try {
    const absolutePath = resolve(process.cwd(), filePath);
    return readFileSync(absolutePath, 'utf-8');
  } catch (error) {
    console.error(`Error reading file ${filePath}:`, error);
    return ''; // Return empty string or handle error as appropriate
  }
}

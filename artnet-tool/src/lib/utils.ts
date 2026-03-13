// Pure utility functions (no side effects, no Tauri calls)
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * shadcn/ui class name utility — merges Tailwind classes conflict-free.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Format a raw DMX channel value (0-255) for display.
 * Returns a 3-digit zero-padded string.
 */
export function formatDmxValue(value: number): string {
  return value.toString().padStart(3, '0');
}

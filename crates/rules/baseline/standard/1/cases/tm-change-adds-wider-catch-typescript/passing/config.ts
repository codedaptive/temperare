import { readFileSync } from "node:fs";

/** Reads the settings file; with no file, the defaults stand. */
export function readSettings<T>(path: string, defaults: T): T {
  try {
    return JSON.parse(readFileSync(path, "utf8")) as T;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    return defaults;
  }
}

/** Reads the settings file; a file that cannot be read is reported, and the defaults stand. */
export function readSettingsOrReport<T>(path: string, defaults: T, report: (error: unknown) => void): T {
  let settings = defaults;
  try {
    settings = JSON.parse(readFileSync(path, "utf8")) as T;
  } catch (error) {
    /* the caller decides what a bad file means */
    report(error);
    /* settings keeps the defaults */
  }
  return settings;
}

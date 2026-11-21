import { readFileSync } from "node:fs";

/** Reads the settings file; with no file, the defaults stand. */
export function readSettings<T>(path: string, defaults: T): T {
  let settings = defaults;
  try {
    settings = JSON.parse(readFileSync(path, "utf8")) as T;
  } catch {}
  return settings;
}

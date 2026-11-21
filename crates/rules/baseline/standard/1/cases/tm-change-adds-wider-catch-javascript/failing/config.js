import { readFileSync } from "node:fs";

/** Reads the settings file; with no file, the defaults stand. */
export function readSettings(path, defaults) {
  let settings = defaults;
  try {
    settings = JSON.parse(readFileSync(path, "utf8"));
  } catch {}
  return settings;
}

import { readFileSync } from "node:fs";

/** Reads the settings file; with no file, the defaults stand. */
export function readSettings(path, defaults) {
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    return defaults;
  }
}

/** Reads the settings file; a file that cannot be read is reported, and the defaults stand. */
export function readSettingsOrReport(path, defaults, report) {
  let settings = defaults;
  try {
    settings = JSON.parse(readFileSync(path, "utf8"));
  } catch (error) {
    /* the caller decides what a bad file means */
    report(error);
    /* settings keeps the defaults */
  }
  return settings;
}

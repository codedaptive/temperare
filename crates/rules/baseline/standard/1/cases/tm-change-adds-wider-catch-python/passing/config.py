import json


def read_settings(path, defaults):
    """Read the settings file; with no file, the defaults stand."""
    try:
        with open(path, encoding="utf-8") as file:
            return json.load(file)
    except FileNotFoundError:
        return defaults

"""Settings read from a JSON file."""

import json


def load_settings(path):
    """The settings in the JSON file at `path`."""
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)

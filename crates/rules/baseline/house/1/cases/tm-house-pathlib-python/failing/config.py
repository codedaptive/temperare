"""Where the tool's settings file lives."""

import os


def config_path(home: str) -> str:
    return os.path.join(home, ".config", "tool.toml")

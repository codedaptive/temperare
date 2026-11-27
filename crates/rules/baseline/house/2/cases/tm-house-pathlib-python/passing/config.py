"""Where the tool's settings file lives."""

from pathlib import Path


def config_path(home: Path) -> Path:
    return home / ".config" / "tool.toml"

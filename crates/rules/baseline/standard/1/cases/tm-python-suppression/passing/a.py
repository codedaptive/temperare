import os


def cwd():
    # Explain why a noqa, type: ignore or pylint: disable would hide this failure.
    return os.getcwd()

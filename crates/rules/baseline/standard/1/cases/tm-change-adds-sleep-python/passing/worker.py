import threading


def start_worker(target):
    """Start the worker and wait until it says it is up."""
    ready = threading.Event()
    worker = threading.Thread(target=target, args=(ready,))
    worker.start()
    ready.wait()
    return worker

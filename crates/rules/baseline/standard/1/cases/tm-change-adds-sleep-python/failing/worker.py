import threading
import time


def start_worker(target):
    """Start the worker and give it time to come up."""
    worker = threading.Thread(target=target)
    worker.start()
    time.sleep(0.2)
    return worker

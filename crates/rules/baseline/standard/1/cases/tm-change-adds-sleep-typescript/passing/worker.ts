import { once, type EventEmitter } from "node:events";

/** Starts the worker and waits until it says it is up. */
export async function startWorker(worker: EventEmitter & { start(): void }): Promise<void> {
  worker.start();
  await once(worker, "ready");
}

import type { EventEmitter } from "node:events";

/** Starts the worker and gives it time to come up. */
export async function startWorker(worker: EventEmitter & { start(): void }): Promise<void> {
  worker.start();
  await new Promise<void>((resolve) => setTimeout(resolve, 200));
}

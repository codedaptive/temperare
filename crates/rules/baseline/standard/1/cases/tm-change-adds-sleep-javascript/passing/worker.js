import { once } from "node:events";

/** Starts the worker and waits until it says it is up. */
export async function startWorker(worker) {
  worker.start();
  await once(worker, "ready");
  return worker;
}

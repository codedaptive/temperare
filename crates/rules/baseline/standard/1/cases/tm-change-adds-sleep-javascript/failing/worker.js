/** Starts the worker and gives it time to come up. */
export async function startWorker(worker) {
  worker.start();
  await new Promise((resolve) => setTimeout(resolve, 200));
  return worker;
}

/** The server's answer when it cannot take the request now. */
export class Busy extends Error {}

/** Sends the request, sending it again while the server is busy. */
export async function send(sendOnce) {
  for (let attempt = 1; ; attempt++) {
    try {
      return await sendOnce();
    } catch (error) {
      if (!(error instanceof Busy) || attempt === 3) throw error;
    }
  }
}

/** The server's answer when it cannot take the request now. */
export class Busy extends Error {}

/** Sends the request once; a busy server is the caller's answer. */
export async function send(sendOnce: () => Promise<string>): Promise<string> {
  return sendOnce();
}

/** Reads every page, counting pages until the server has no more. */
export async function readPages(readPage: (page: number) => Promise<string | undefined>): Promise<string[]> {
  const pages: string[] = [];
  for (let page = 0; ; page++) {
    const text = await readPage(page);
    if (text === undefined) return pages;
    pages.push(text);
  }
}

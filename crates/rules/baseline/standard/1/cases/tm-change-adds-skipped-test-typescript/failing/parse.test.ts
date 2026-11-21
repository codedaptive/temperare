function parsePort(text: string): number | undefined {
  const port = Number(text);
  return port < 65536 ? port : undefined;
}

it("reads a port", () => {
  expect(parsePort("8080")).toBe(8080);
});

it.skip("refuses a port out of range", () => {
  expect(parsePort("70000")).toBeUndefined();
});

function parsePort(text: string): number | undefined {
  const port = Number(text);
  return port < 65536 ? port : undefined;
}

it("reads a port", () => {
  expect(parsePort("8080")).toBe(8080);
});

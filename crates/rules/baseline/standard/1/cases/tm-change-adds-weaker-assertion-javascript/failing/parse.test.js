function parsePort(text) {
  const port = Number(text);
  return port < 65536 ? port : undefined;
}

test("reads a port", () => {
  expect(parsePort("8080")).toBeDefined();
});

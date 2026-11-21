function parsePort(text) {
  const port = Number(text);
  return port < 65536 ? port : undefined;
}

test("reads a port", () => {
  expect(parsePort("8080")).toBe(8080);
});

test("refuses a port out of range", () => {
  expect(parsePort("70000")).toBeUndefined();
});

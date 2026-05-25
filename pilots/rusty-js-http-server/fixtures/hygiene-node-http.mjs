import http from "node:http";

const server = http.createServer((req, res) => {
  const headerName = { toString() { return "x-agent-object"; } };
  const headerValue = { toString() { return "coerced-header"; } };
  const body = { toString() { return "coerced-body"; } };
  res.setHeader(headerName, headerValue);
  res.end(body);
  server.close();
});

console.log(
  "HS_SENTINEL_KEYS:" +
    Object.keys(server).filter((key) => key.startsWith("__cruftless_http_")).join(",")
);

server.listen(0, "127.0.0.1", () => {
  console.log(`HS_PORT:${server.address().port}`);
});

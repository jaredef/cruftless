import http from "node:http";

const server = http.createServer((req, res) => {
  res.writeHead(200, { "content-type": "text/plain" });
  res.end("hello");
  server.close();
});

server.listen(0, "127.0.0.1", () => {
  console.log(`HS_PORT:${server.address().port}`);
});

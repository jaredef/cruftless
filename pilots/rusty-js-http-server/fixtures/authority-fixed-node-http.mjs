import http from "node:http";

const server = http.createServer((req, res) => {
  res.end("authority-ok");
  server.close();
});

server.listen(39731, "127.0.0.1");

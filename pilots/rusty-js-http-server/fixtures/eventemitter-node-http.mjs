import http from "node:http";

let seen = 0;

const server = http.createServer();

server.once("request", (req, res) => {
  seen += 1;
  res.setHeader("x-agent-event", "once");
  res.end(`event:${seen}:${req.url}`);
  server.close();
});

server.on("ignored", () => {
  throw new Error("ignored event should not fire");
});

server.listen(0, "127.0.0.1", () => {
  console.log(`HS_EVENT_PORT:${server.address().port}`);
});

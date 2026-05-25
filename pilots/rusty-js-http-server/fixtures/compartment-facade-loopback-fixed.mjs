const http = __cruftless_makeHttpFacade();
const c = new Compartment({ globals: { http, marker: "realm-ok" } });

c.evaluate(`
  const server = http.createServer();
  server.once("request", (req, res) => {
    res.end(marker);
    server.close();
  });
  server.listen(39732, "127.0.0.1");
`);

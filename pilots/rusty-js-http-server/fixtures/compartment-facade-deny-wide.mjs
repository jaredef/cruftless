const http = __cruftless_makeHttpFacade();
const c = new Compartment({ globals: { http } });

c.evaluate(`
  http.createServer((req, res) => res.end("wide-bind")).listen(0, "0.0.0.0");
`);

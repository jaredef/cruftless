import express from "express";
import http from "node:http";

const app = express();

app.use((req, res, next) => {
  res.set("x-mid", "seen");
  req.seenByMiddleware = "yes";
  next();
});

app.get("/user/:id", (req, res) => {
  res
    .status(201)
    .set("x-route", req.params.id)
    .send(`user:${req.params.id}:q=${req.query.q}:mid=${req.seenByMiddleware}`);
});

const server = http.createServer(app);

server.listen(39734, "127.0.0.1");

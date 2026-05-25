import express from "express";
import http from "node:http";

const app = express();

app.get("/", (req, res) => {
  res.status(200).set("x-from", "express").send("hello express");
});

const server = http.createServer(app);

server.listen(39733, "127.0.0.1");

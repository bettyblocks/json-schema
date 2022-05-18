const express = require("express");
const fs = require("fs");
const path = require("path");

const SCHEMA = "http://json-schema.org/draft-07/schema";
const HOST = "https://raw.githubusercontent.com";
const PREFIX = "bettyblocks/json-schema/master/";
const PORT = 9797;

const app = express();

const serve = (request, response, file) => {
  console.log("Request for: ", request.url);
  console.log("Serving: ", file);

  fs.readFile(file, function (error, content) {
    if (error) {
      if (error.code == "ENOENT") {
        response.status(404).send("Not Found");
      } else {
        response.status(500).send("Internal Error: " + error.code + " ..\n");
      }
    } else {
      const hostname = `${request.protocol}://${request.headers.host}`;
      const json = content
        .toString()
        .replaceAll(HOST, hostname)
        .replaceAll(PREFIX, "")
        .replaceAll(SCHEMA, `${hostname}/schema`);
      response.status(200).contentType("application/json").send(json);
    }
  });
};

app.get("/schema", (request, response) =>
  serve(request, response, path.join(process.cwd(), "schema.json"))
);

app.get("*", (request, response) => {
  serve(request, response, path.join(process.cwd(), request.url));
});

app.listen(PORT, () =>
  console.log(`JSON Schema Server running at 127.0.0.1:${PORT}`)
);

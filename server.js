const http = require('http');
const fs = require('fs');
const path = require('path');
const port = 9797;

const pathPrefix = '/bettyblocks/json-schema/master';

http.createServer((request, response) => {

  console.log('Request for: ', request.url);

  filePath = request.url.replace(pathPrefix, '');
  fullPath = path.join(process.cwd(), filePath);

  console.log('Serving: ', fullPath);

  fs.readFile(fullPath, function(error, content) {
    if (error) {
      if (error.code == 'ENOENT') {
        response.writeHead(404);
        response.end('Not Found');
      } else {
        response.writeHead(500);
        response.end('Internal Error: ' + error.code + ' ..\n');
      }
    } else {
      response.writeHead(200);
      response.end(content, 'utf-8');
    }
  });

}).listen(port);

console.log(`Server running at http://127.0.0.1:${port}`);

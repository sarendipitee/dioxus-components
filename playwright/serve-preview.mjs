import { createReadStream, existsSync, readFileSync, statSync } from "node:fs";
import { extname, join, normalize, resolve } from "node:path";
import { createServer } from "node:http";

const rootArg = process.argv[2];
const portArg = process.argv[3];

if (!rootArg || !portArg) {
  console.error("Usage: node serve-preview.mjs <public-dir> <port>");
  process.exit(1);
}

const rootDir = resolve(rootArg);
const port = Number.parseInt(portArg, 10);

if (!Number.isInteger(port) || port <= 0) {
  console.error(`Invalid port: ${portArg}`);
  process.exit(1);
}

if (!existsSync(rootDir) || !statSync(rootDir).isDirectory()) {
  console.error(`Preview directory does not exist: ${rootDir}`);
  process.exit(1);
}

const indexHtml = readFileSync(join(rootDir, "index.html"));

const mimeTypes = new Map([
  [".css", "text/css; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "application/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".png", "image/png"],
  [".svg", "image/svg+xml"],
  [".wasm", "application/wasm"],
  [".woff", "font/woff"],
  [".woff2", "font/woff2"],
]);

const server = createServer((req, res) => {
  const url = new URL(req.url ?? "/", `http://${req.headers.host ?? "127.0.0.1"}`);
  const pathname = url.pathname === "/" ? "/index.html" : url.pathname;
  const safePath = normalize(pathname).replace(/^(\.\.(\/|\\|$))+/, "");
  const filePath = join(rootDir, safePath);

  if (existsSync(filePath) && statSync(filePath).isFile()) {
    res.writeHead(200, {
      "Content-Type": mimeTypes.get(extname(filePath)) ?? "application/octet-stream",
      "Cache-Control": "no-cache",
    });
    createReadStream(filePath).pipe(res);
    return;
  }

  res.writeHead(200, {
    "Content-Type": "text/html; charset=utf-8",
    "Cache-Control": "no-cache",
  });
  res.end(indexHtml);
});

server.listen(port, "127.0.0.1", () => {
  console.log(`Preview server ready at http://127.0.0.1:${port}`);
});

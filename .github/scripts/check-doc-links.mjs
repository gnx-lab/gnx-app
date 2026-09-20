import fs from "node:fs";
import path from "node:path";

const root = path.resolve(process.argv[2] || "_site");
const basePath = (process.env.PAGES_BASE_PATH || "").replace(/\/$/, "");
const failures = [];
let checked = 0;

function filesUnder(directory) {
  const entries = fs.readdirSync(directory, { withFileTypes: true });
  return entries.flatMap(entry => {
    const full = path.join(directory, entry.name);
    return entry.isDirectory() ? filesUnder(full) : [full];
  });
}

function safeDecode(value) {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}

function candidatesFor(sourceFile, rawPath) {
  let urlPath = safeDecode(rawPath.split(/[?#]/, 1)[0]);
  if (!urlPath) return [];
  if (urlPath.startsWith("/")) {
    if (basePath && urlPath.startsWith(`${basePath}/`)) urlPath = urlPath.slice(basePath.length);
    if (urlPath === basePath) urlPath = "/";
    return [path.join(root, urlPath), path.join(root, urlPath, "index.html")];
  }
  const local = path.resolve(path.dirname(sourceFile), urlPath);
  return [local, path.join(local, "index.html"), `${local}.html`];
}

function existsAsTarget(candidate) {
  if (fs.existsSync(candidate) && fs.statSync(candidate).isFile()) return true;
  if (fs.existsSync(candidate) && fs.statSync(candidate).isDirectory()) {
    return fs.existsSync(path.join(candidate, "index.html"));
  }
  return false;
}

for (const file of filesUnder(root).filter(file => file.endsWith(".html"))) {
  const html = fs.readFileSync(file, "utf8");
  const links = html.matchAll(/\b(?:href|src)=["']([^"']+)["']/gi);
  for (const match of links) {
    const href = match[1].trim();
    if (!href || href.startsWith("#") || /^(?:https?:|mailto:|tel:|data:|javascript:)/i.test(href)) continue;
    checked += 1;
    const candidates = candidatesFor(file, href);
    if (!candidates.length || !candidates.some(existsAsTarget)) {
      failures.push(`${path.relative(root, file)} -> ${href}`);
    }
  }
}

if (failures.length) {
  console.error(`Broken documentation links (${failures.length}):`);
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log(`Documentation links checked: ${checked}; broken: 0`);

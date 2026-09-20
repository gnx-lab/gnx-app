const CACHE = "gnx-app-v6";
const APP_SHELL = [
  "./",
  "./index.html",
  "./styles.css",
  "./app.js",
  "./sw.js",
  "./manifest.webmanifest",
  "./assets/icon-192.png",
  "./assets/icon-512.png",
  "./assets/tray-icon.ico",
  "./assets/tray-icon.png",
  "./assets/branding-install-logo.png"
];

const updateCache = (request, response) => {
  if (!response || !response.ok || response.type === "opaque") return response;
  return caches.open(CACHE)
    .then(cache => cache.put(request, response.clone()))
    .then(() => response);
};

self.addEventListener("install", event => {
  event.waitUntil(
    caches.open(CACHE)
      .then(cache => cache.addAll(APP_SHELL))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener("activate", event => {
  event.waitUntil(
    caches.keys()
      .then(keys => Promise.all(
        keys.filter(key => key.startsWith("gnx-app-") && key !== CACHE)
          .map(key => caches.delete(key))
      ))
      .then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", event => {
  const { request } = event;
  if (request.method !== "GET" || new URL(request.url).origin !== self.location.origin) return;

  if (request.mode === "navigate") {
    event.respondWith(
      fetch(request)
        .then(response => updateCache(request, response))
        .catch(() => caches.match("./index.html"))
    );
    return;
  }

  event.respondWith(
    caches.match(request).then(cached => {
      const fresh = fetch(request)
        .then(response => updateCache(request, response))
        .catch(() => Response.error());
      return cached || fresh;
    })
  );
});

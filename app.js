const $ = selector => document.querySelector(selector);
const isLocal = ["localhost", "127.0.0.1", "::1"].includes(location.hostname);
const SESSION_KEY = "gnx.demo.session";
const SERVICE_URL = "http://127.0.0.1:17890/status";

const modeBadge = $("#modeBadge");
const intro = $("#intro");
const sessionState = $("#sessionState");
const pwaState = $("#pwaState");
const serviceRow = $("#serviceRow");
const serviceState = $("#serviceState");
const sessionBtn = $("#sessionBtn");
const installBtn = $("#installBtn");
const verifyBtn = $("#verifyBtn");
const handoffLabel = $("#handoffLabel");
const handoffCode = $("#handoffCode");
const status = $("#status");
let deferredPrompt;

const setStatus = message => { status.textContent = message; };
const readSession = () => localStorage.getItem(SESSION_KEY);
const createSession = () => {
  const bytes = crypto.getRandomValues(new Uint8Array(18));
  return `gnx_${Array.from(bytes, byte => byte.toString(16).padStart(2, "0")).join("")}`;
};

function configureMode() {
  modeBadge.textContent = isLocal ? "Local · PWA + servicio" : "Vista pública";
  intro.textContent = isLocal
    ? "Frontend local conectado al servicio Windows instalado."
    : "Esta publicación no expone el servicio local.";
  serviceRow.hidden = !isLocal;
  sessionBtn.textContent = isLocal ? "Usar sesión" : "Generar sesión";
  if (isLocal) {
    handoffLabel.hidden = false;
    handoffCode.readOnly = false;
    handoffCode.placeholder = "Pega aquí un código de sesión";
  }
}

function updateSession() {
  const token = readSession();
  sessionState.textContent = token ? "Disponible" : "No generada";
  if (token && !isLocal) {
    handoffLabel.hidden = false;
    handoffCode.value = token;
  }
}

function updatePwaState() {
  const installed = window.matchMedia("(display-mode: standalone)").matches || window.navigator.standalone;
  pwaState.textContent = installed ? "Instalada" : "Navegador";
  return installed;
}

async function checkService() {
  if (!isLocal) return;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 2000);
  try {
    const response = await fetch(SERVICE_URL, { signal: controller.signal, cache: "no-store" });
    if (!response.ok) throw new Error("service status failed");
    const state = await response.json();
    const checks = [state.serviceRunning, state.urlAvailable, state.manifestAvailable, state.serviceWorkerAvailable];
    serviceState.textContent = checks.every(Boolean) ? "Correcto" : "Revisar URL";
    setStatus(checks.every(Boolean) ? "Servicio local y PWA disponibles." : "El servicio responde; alguna comprobación de la URL necesita atención.");
  } catch {
    serviceState.textContent = "No conectado";
    setStatus("No se encontró el servicio local en 127.0.0.1:17890.");
  } finally {
    clearTimeout(timeout);
  }
}

sessionBtn.addEventListener("click", () => {
  const value = handoffCode.value.trim();
  if (isLocal && value) localStorage.setItem(SESSION_KEY, value);
  if (!isLocal && !readSession()) localStorage.setItem(SESSION_KEY, createSession());
  updateSession();
  setStatus(readSession() ? "Sesión disponible para esta demo." : "No se pudo generar la sesión.");
});
handoffCode.addEventListener("change", () => {
  if (isLocal && handoffCode.value.trim()) {
    localStorage.setItem(SESSION_KEY, handoffCode.value.trim());
    updateSession();
  }
});
verifyBtn.addEventListener("click", async () => {
  const installed = updatePwaState();
  if (isLocal) await checkService();
  else setStatus(installed ? "La demo está instalada como PWA." : "Instala la demo desde el menú del navegador.");
});
installBtn.addEventListener("click", async () => {
  if (!deferredPrompt) return;
  deferredPrompt.prompt();
  const { outcome } = await deferredPrompt.userChoice;
  deferredPrompt = null;
  installBtn.hidden = true;
  setStatus(outcome === "accepted" ? "Instalación aceptada." : "Instalación cancelada.");
});
window.addEventListener("beforeinstallprompt", event => {
  event.preventDefault();
  deferredPrompt = event;
  installBtn.hidden = false;
});
window.addEventListener("appinstalled", () => {
  deferredPrompt = null;
  installBtn.hidden = true;
  updatePwaState();
  setStatus("PWA instalada correctamente.");
});

configureMode();
updateSession();
updatePwaState();
if (isLocal) {
  checkService();
  setInterval(checkService, 30000);
}
if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => navigator.serviceWorker.register("./sw.js", { updateViaCache: "none" })
    .catch(() => setStatus("El Service Worker no está disponible.")));
}

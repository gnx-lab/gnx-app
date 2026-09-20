/* GnX App: un único frontend con dos modos explícitos. */
const $ = selector => document.querySelector(selector);
const isLocal = ["localhost", "127.0.0.1", "::1"].includes(location.hostname);
const API_BASE_URL = window.GNX_API_URL || "";
const SESSION_KEY = "gnx.session";

const modeBadge = $("#modeBadge");
const intro = $("#intro");
const sessionState = $("#sessionState");
const pwaState = $("#pwaState");
const sourcesState = $("#sourcesState");
const status = $("#status");
const sessionBtn = $("#sessionBtn");
const installBtn = $("#installBtn");
const verifyBtn = $("#verifyBtn");
const handoffLabel = $("#handoffLabel");
const handoffCode = $("#handoffCode");
let deferredPrompt;

const setStatus = message => { status.textContent = message; };
const readSession = () => sessionStorage.getItem(SESSION_KEY) || localStorage.getItem(SESSION_KEY);
const setSession = token => {
  sessionStorage.setItem(SESSION_KEY, token);
  if (isLocal) localStorage.setItem(SESSION_KEY, token);
};
const newSessionCode = () => {
  const bytes = crypto.getRandomValues(new Uint8Array(18));
  return `gnx_${Array.from(bytes, byte => byte.toString(16).padStart(2, "0")).join("")}`;
};

function configureMode() {
  modeBadge.textContent = isLocal ? "Privado · localhost" : "Público · GitHub Pages";
  intro.textContent = isLocal
    ? "Sesión local para operar contra las fuentes configuradas."
    : "Genera una sesión y verifica que esta PWA está instalada.";
  if (!isLocal) {
    $(".private-only").hidden = true;
    sessionBtn.textContent = "Generar sesión";
  } else {
    sessionBtn.textContent = "Comprobar sesión";
    verifyBtn.textContent = "Comprobar fuentes";
  }
}

function updateSessionView() {
  const token = readSession();
  sessionState.textContent = token ? (isLocal ? "Disponible" : "Generada") : "No iniciada";
  if (isLocal) {
    handoffLabel.hidden = false;
    handoffCode.readOnly = false;
    handoffCode.placeholder = "Pega aquí el código público";
  } else if (token) {
    handoffLabel.hidden = false;
    handoffCode.value = token;
  }
}

async function verifySession() {
  const token = readSession();
  if (!token) {
    sessionState.textContent = "No iniciada";
    setStatus(isLocal ? "Pega un código generado en la página pública." : "Genera una sesión para comenzar.");
    return false;
  }
  if (!isLocal || !API_BASE_URL) {
    sessionState.textContent = isLocal ? "Pendiente de API" : "Generada";
    setStatus(isLocal ? "Sesión local preparada; la API aún no está configurada." : "Código generado. La autenticación real la confirma la API.");
    return true;
  }
  try {
    const response = await fetch(`${API_BASE_URL}/session/verify`, {
      method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` }, body: "{}"
    });
    if (!response.ok) throw new Error("session verification failed");
    sessionState.textContent = "Autenticada";
    setStatus("Sesión autenticada por la API.");
    return true;
  } catch {
    sessionState.textContent = "No verificada";
    setStatus("La API no pudo verificar la sesión.");
    return false;
  }
}

async function checkSources() {
  if (!isLocal) return;
  if (!API_BASE_URL) {
    sourcesState.textContent = "API no configurada";
    return;
  }
  try {
    const response = await fetch(`${API_BASE_URL}/health`, { credentials: "include" });
    sourcesState.textContent = response.ok ? "Disponibles" : "No disponibles";
  } catch {
    sourcesState.textContent = "No disponibles";
  }
}

window.addEventListener("beforeinstallprompt", event => {
  event.preventDefault();
  deferredPrompt = event;
  installBtn.hidden = false;
  pwaState.textContent = "Disponible";
});
window.addEventListener("appinstalled", () => {
  deferredPrompt = null;
  installBtn.hidden = true;
  pwaState.textContent = "Instalada";
  setStatus("PWA instalada correctamente.");
});

handoffCode.addEventListener("change", () => {
  if (isLocal && handoffCode.value.trim()) {
    setSession(handoffCode.value.trim());
    updateSessionView();
  }
});

sessionBtn.addEventListener("click", async () => {
  if (isLocal && handoffCode.value.trim()) setSession(handoffCode.value.trim());
  if (!isLocal && !readSession()) setSession(newSessionCode());
  updateSessionView();
  await verifySession();
  await checkSources();
});
verifyBtn.addEventListener("click", async () => {
  if (isLocal) { await verifySession(); await checkSources(); return; }
  const standalone = window.matchMedia("(display-mode: standalone)").matches || window.navigator.standalone;
  pwaState.textContent = standalone ? "Instalada" : "No instalada";
  setStatus(standalone ? "La PWA está ejecutándose instalada." : "Abre el menú del navegador para instalar la PWA.");
});
installBtn.addEventListener("click", async () => {
  if (!deferredPrompt) return;
  deferredPrompt.prompt();
  const { outcome } = await deferredPrompt.userChoice;
  deferredPrompt = null;
  installBtn.hidden = true;
  setStatus(outcome === "accepted" ? "Instalación aceptada." : "Instalación cancelada.");
});

configureMode();
updateSessionView();
if (isLocal) checkSources();
if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => navigator.serviceWorker.register("./sw.js", { updateViaCache: "none" })
    .catch(() => setStatus("El modo sin conexión no está disponible.")));
}

---
layout: default
title: Roadmap
description: Evolución propuesta, ordenada por evidencia y no por promesas.
status: Propuesta
updated: 2026-09-20
permalink: /roadmap/
---

## Ahora: documentación y operación local

- [x] Publicar únicamente Markdown desde `/docs`.
- [x] Navegación lateral, búsqueda de navegación y diseño responsive.
- [x] Documentar la PWA local y el monitor Windows Rust.
- [x] Marcar por separado evidencia, historial y propuesta.
- [ ] Añadir validación automática de enlaces internos en CI.

## Después: contrato de conexión

Antes de tocar un botón `Conectar`:

- [ ] Acordar quién autentica y cómo expira una sesión.
- [ ] Publicar el contrato de `compute.gnx`.
- [ ] Definir estados de conexión, reintentos y cancelación.
- [ ] Definir qué datos puede enviar `app.gnx`.
- [ ] Probar un flujo extremo a extremo sin secretos en logs.

## Más adelante: producto distribuido

- [ ] Integrar la entrada pública con una API autorizada.
- [ ] Separar monitor de disponibilidad de health semántico.
- [ ] Añadir rollback y migración del servicio con evidencia.
- [ ] Publicar una matriz de compatibilidad de navegador, Windows y runtime.

<div class="callout warning">
<strong>La prioridad no es añadir más páginas.</strong> La siguiente entrega debe cerrar decisiones y pruebas de la conexión remota antes de presentar `compute.gnx` como una capacidad real.
</div>

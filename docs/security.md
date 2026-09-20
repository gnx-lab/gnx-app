---
layout: default
title: Seguridad y límites
description: El contrato de confianza que la documentación puede afirmar sin exagerar capacidades.
status: Verificado contra el diseño actual
updated: 2026-09-20
permalink: /security/
---

## Principios

- **Local por defecto:** el servicio escucha en `127.0.0.1`; la PWA no expone ese endpoint a Internet.
- **Sin secretos en Pages:** GitHub Pages solo recibe Markdown y genera HTML estático.
- **Separación de contextos:** la documentación externa, la PWA local y cualquier futuro compute remoto son superficies diferentes.
- **Evidencia antes que promesa:** un enlace o una pantalla no demuestra autenticación, disponibilidad ni autorización.
- **Fallo explícito:** un monitor que no puede comprobar una URL debe reportar el estado como fallido o pendiente, no como listo.

## Qué no hace el sitio de docs

Este sitio no:

- recibe tokens ni códigos de sesión;
- consulta `127.0.0.1:17890` desde el servidor;
- conecta `app.gnx` con `compute.gnx`;
- publica el frontend PWA o el MSI;
- autentica usuarios;
- almacena datos de equipos.

El enlace a <a href="https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/" target="_blank" rel="noopener noreferrer">gnx-obsidiana.mayas-alas-yx.chatgpt.site</a> es solo contexto navegable y está marcado como externo.

## Superficies y controles

| Superficie | Riesgo principal | Control documentado |
|---|---|---|
| Markdown/Jekyll | Publicar un secreto o un archivo de runtime por accidente | El workflow copia solo `/docs` y verifica nombres prohibidos en el artefacto. |
| PWA local | Asumir que un código local es una credencial | La sesión de demo se mantiene local y no se presenta como autenticación. |
| Monitor Windows | Escuchar fuera de loopback | El servidor HTTP se enlaza a `127.0.0.1`. |
| Futuro `compute.gnx` | Inventar un contrato o enviar datos sin consentimiento | Especificación de identidad, transporte, alcance y pruebas antes del botón. |

## Revisión antes de publicar

```powershell
rg -n "(API_KEY|SECRET|PASSWORD|TOKEN=|BEGIN PRIVATE KEY)" docs README.md
rg -n "mayas-alas\.github\.io|gnx-lab\.github\.io" docs README.md .github || exit 0
```

Las coincidencias de palabras en una explicación no son por sí mismas secretos; cada revisión debe inspeccionar el contexto. Nunca pegues credenciales reales en Markdown, logs, ejemplos o capturas.

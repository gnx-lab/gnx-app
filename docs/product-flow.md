---
layout: default
title: Mapa del producto
description: Qué sabemos, qué heredamos del historial y qué debe validarse antes de llamar a una integración real.
status: Mixto: histórico + propuesta explícita
updated: 2026-09-20
permalink: /product-flow/
---

## El flujo solicitado

El flujo de producto que esta versión de la documentación debe hacer legible es:

```text
Usuario → https://app.gnx → «Conectar» → https://compute.gnx
```

La intención es clara: `app.gnx` funciona como punto de entrada y `compute.gnx` como destino de cómputo. Sin embargo, el checkout actual no contiene el botón `Conectar`, no define un cliente remoto ni registra `compute.gnx` en sus tres commits. Por eso este diagrama es **propuesta de producto**, no evidencia de una integración activa.

## Evidencia del historial

El historial disponible es corto:

| Commit | Evidencia |
|---|---|
| `2688c53` | Publica una PWA y configura `https://app.gnx` como URL de instalación/documentación del instalador. |
| `6558e12` | Añade el script completo de build del instalador. |
| `a06aa23` | Empaqueta el monitor self-contained en el MSI. |

`app.gnx` sí aparece en README, arquitectura e instalador históricos. `compute.gnx` no aparece en ninguna ruta, texto o commit inspeccionado. La documentación no inventa un contrato HTTP, OAuth, WebSocket, CLI o esquema de sesión para ese dominio.

## Tres capas que no deben mezclarse

1. **Entrada histórica:** `https://app.gnx` es una referencia del producto anterior. Úsala para hablar de intención y de la experiencia esperada, no como prueba de que el dominio esté publicado desde este repo.
2. **Cómputo propuesto:** `https://compute.gnx` es un destino nombrado por el brief actual. Antes de implementarlo hay que acordar identidad, autorización, transporte, health checks, límites y manejo de errores.
3. **Implementación presente:** el checkout ejecutable es una PWA local y un monitor Windows en `127.0.0.1:17890`; no realiza una conexión remota a `compute.gnx`.

## Contrato mínimo que falta definir

Antes de conectar un botón real, registrar estas decisiones:

| Pregunta | Decisión requerida |
|---|---|
| Identidad | ¿La sesión se crea en `app.gnx`, en un proveedor externo o localmente? |
| Transporte | HTTPS, WebSocket, SSE o protocolo específico; nunca asumirlo por el nombre del dominio. |
| Autorización | Token, OAuth, sesión de navegador o enlace de un solo uso. |
| Estado | Estados de conexión, expiración, reintento, cancelación y pérdida de red. |
| Datos | Qué sale de `app.gnx`, qué llega de `compute.gnx` y qué nunca se persiste. |
| Operación | Health, observabilidad, rate limits, rollback y soporte. |

<div class="callout warning">
<strong>No es una integración todavía.</strong> Si una pantalla usa el texto «Conectar a compute.gnx», debe mostrar también estado, origen y límites hasta que exista una especificación y una prueba de extremo a extremo autorizada.
</div>

## Fuente de contexto externo

Para la exploración conceptual se puede consultar <a href="https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/" target="_blank" rel="noopener noreferrer">gnx-obsidiana.mayas-alas-yx.chatgpt.site</a>. Esa referencia no autentica, no enruta tráfico hacia `compute.gnx` y no debe tratarse como API de GnX App.

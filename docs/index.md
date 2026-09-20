---
layout: default
title: GnX Docs
description: El mapa verificable para entender GnX App, probar su PWA local y operar el monitor Windows.
status: Verificado contra el checkout actual
updated: 2026-09-20
permalink: /
---

<div class="callout">
<strong>Una documentación con dos reglas.</strong>
<p>Todo lo que está marcado como <strong>verificado</strong> tiene evidencia en este repositorio. Todo lo que pertenece a una visión, un producto externo o un historial incompleto aparece etiquetado como tal.</p>
</div>

GnX App es una PWA local conectada a un servicio Windows de monitorización. Esta documentación convierte el repositorio en una experiencia de lectura tipo GitBook: empieza por tu intención, sigue una ruta corta y vuelve a la referencia cuando necesites precisión.

<div class="card-grid">
<a class="doc-card" href="{{ '/quickstart/' | relative_url }}"><strong>Quiero empezar</strong><span>Levanta la PWA local, verifica el endpoint y entiende qué queda fuera.</span></a>
<a class="doc-card" href="{{ '/product-flow/' | relative_url }}"><strong>Quiero entender el producto</strong><span>Separa la evidencia del repo del flujo histórico app.gnx → compute.gnx.</span></a>
<a class="doc-card" href="{{ '/architecture/' | relative_url }}"><strong>Quiero entender la arquitectura</strong><span>Frontend, Service Worker, MSI, Rust y contrato loopback en una sola vista.</span></a>
<a class="doc-card" href="{{ '/operations/' | relative_url }}"><strong>Quiero operar o diagnosticar</strong><span>Comandos de build, verificación y rutas de troubleshooting sin secretos.</span></a>
</div>

## El mapa en 60 segundos

| Si vienes por… | Lee primero | Después |
|---|---|---|
| Una prueba local | [Inicio rápido]({{ '/quickstart/' | relative_url }}) | [Troubleshooting]({{ '/troubleshooting/' | relative_url }}) |
| La visión del producto | [Mapa del producto]({{ '/product-flow/' | relative_url }}) | [Recursos]({{ '/resources/' | relative_url }}) |
| Una decisión técnica | [Arquitectura]({{ '/architecture/' | relative_url }}) | [Referencia]({{ '/reference/' | relative_url }}) |
| Una instalación Windows | [Operación]({{ '/operations/' | relative_url }}) | [Instalador]({{ '/installer-worklog/' | relative_url }}) |

## Fuentes relacionadas

El contexto editorial externo vive en <a href="https://gnx-obsidiana.mayas-alas-yx.chatgpt.site/" target="_blank" rel="noopener noreferrer">gnx-obsidiana.mayas-alas-yx.chatgpt.site</a>. Es una referencia de exploración y no es un backend, un canal de autenticación ni una dependencia operativa de GnX App.

Para comparar la experiencia editorial, consulta <a href="https://gnx.gitbook.io/docs" target="_blank" rel="noopener noreferrer">la documentación GitBook de referencia</a>. Este sitio no copia su contenido: adopta sus mejores patrones de navegación, jerarquía y lectura en una implementación propia dentro de `/docs`.

## Estado actual

- **Verificado:** la PWA local, el contrato `GET /status`, el build WiX y el monitor Rust están representados por archivos del checkout actual.
- **Histórico:** `https://app.gnx` aparece en el historial como URL pública de la PWA y valor por defecto del MSI.
- **Por validar:** `https://compute.gnx` forma parte del flujo de producto solicitado, pero no aparece en los tres commits disponibles; no se presenta como integración implementada.

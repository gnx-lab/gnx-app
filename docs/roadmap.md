---
layout: default
title: Roadmap
description: Evolución propuesta después de cerrar la feature de documentación.
status: Producto futuro
updated: 2026-09-20
permalink: /roadmap/
---

> La feature de documentación ya está cerrada. Consulta la [auditoría de documentación]({{ '/documentation-audit/' | relative_url }}) para ver criterios, evidencia y alcance entregado.

## Próximo: contrato de conexión

Antes de tocar un botón `Conectar`:

- [ ] Acordar quién autentica y cómo expira una sesión.
- [ ] Publicar el contrato de `compute.gnx`.
- [ ] Definir estados de conexión, reintentos y cancelación.
- [ ] Definir qué datos puede enviar `app.gnx`.
- [ ] Probar un flujo extremo a extremo sin secretos en logs.

## Producto distribuido

- [ ] Integrar la entrada pública con una API autorizada.
- [ ] Separar monitor de disponibilidad de health semántico.
- [ ] Añadir rollback y migración del servicio con evidencia.
- [ ] Publicar una matriz de compatibilidad de navegador, Windows y runtime.

## Regla de priorización

No se presenta `compute.gnx` como una capacidad real hasta tener contrato, identidad, límites, pruebas autorizadas y evidencia reproducible. La documentación seguirá registrando esos cambios, pero no volverá a cargar una feature terminada como pendiente.

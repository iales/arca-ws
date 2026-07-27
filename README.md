# arca-ws

SDK en Rust (no oficial) para ARCA (ex-AFIP) — facturación electrónica vía WSAA/WSFEv1.

## Prerequisitos: certificado de homologación (testing)

Para hablar con ARCA (vía WSAA) necesitás un certificado X.509 propio, asociado a tu CUIT. No es algo que este SDK pueda generar por vos — es un trámite que hacés una única vez por CUIT, con tu Clave Fiscal.

Esta guía cubre **solo el ambiente de homologación** (testing, vía WSASS) — es el que usa este SDK durante desarrollo.

> **Producción: WIP.** Es un trámite distinto (no usa WSASS, otra URL de WSAA, otro proceso de alta) — todavía no lo documentamos acá. Referencia para cuando lo abordemos: [manual oficial de generación de certificados para producción](https://www.afip.gob.ar/ws/wsaa/wsaa.obtenercertificado.pdf).

1. Generá tu clave privada (nunca la compartas ni la subas a ningún repositorio):
   ```bash
   openssl genrsa -out privada.key 2048
   ```
2. Generá el CSR (pedido de certificado) a partir de esa clave. El campo `serialNumber` es obligatorio y debe llevar tu CUIT (11 dígitos, sin guiones):
   ```bash
   openssl req -new -key privada.key -subj "/C=AR/O=TuNombre/CN=UnAlias/serialNumber=CUIT 20123456789" -out pedido.csr
   ```
3. Entrá a ARCA con tu Clave Fiscal, andá al servicio **"Administrador de Relaciones de Clave Fiscal"** → **"Administración de Certificados Digitales"**, y subí el contenido de `pedido.csr` (WSASS, ambiente de homologación/testing).
4. Descargá el `.crt` que te da ARCA — ese es tu certificado.

Guardá `privada.key` y el `.crt` fuera del control de versiones (el `.gitignore` de este repo ya excluye `*.key`, `*.crt`, `*.csr` y `*.pem` por defecto).

## Documentación oficial

- [Documentación de Web Services SOAP (índice)](https://www.afip.gob.ar/ws/documentacion/ws-factura-electronica.asp)
- [WSAA — Manual del Desarrollador](https://www.afip.gob.ar/ws/WSAA/WSAAmanualDev.pdf)
- [WSAA — Especificación Técnica 1.2.2](https://www.arca.gob.ar/ws/WSAA/Especificacion_Tecnica_WSAA_1.2.2.pdf)
- [Manuales para el desarrollador — Facturación RG 4291 (Proyecto FE v4.5)](https://www.arca.gob.ar/ws/documentacion/manuales/manual-desarrollador-ARCA-COMPG.pdf)
- [Manuales para el desarrollador — Facturación Electrónica v4.0](https://arca.gob.ar/ws/documentacion/manuales/manual-desarrollador-ARCA-COMPG-v4-0.pdf)

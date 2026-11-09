---
title: Plugins exec
description: Cómo inicia sesión Homeport en clústeres de EKS, GKE y AKS con el plugin de credenciales exec que indica su kubeconfig, y qué se ejecuta dónde.
---

Los kubeconfigs de la nube no contienen una contraseña ni un certificado: su usuario indica un **plugin exec**, un programa de tu equipo que escribe un token de corta duración cuando se le pide. Homeport ejecuta el mismo programa que ejecutaría `kubectl`, en este equipo, siguiendo el protocolo `ExecCredential` de Kubernetes (`client.authentication.k8s.io/v1` y `v1beta1`).

## Proveedores habituales

| Proveedor | El usuario del kubeconfig ejecuta    | Instalación                                                                       |
| --------- | ------------------------------------ | --------------------------------------------------------------------------------- |
| EKS       | `aws eks get-token --cluster-name …` | la CLI de AWS, versión 2                                                          |
| GKE       | `gke-gcloud-auth-plugin`             | la CLI de Google Cloud y luego `gcloud components install gke-gcloud-auth-plugin` |
| AKS       | `kubelogin get-token …`              | `az aks install-cli`, o kubelogin desde sus releases                              |

Escribe el kubeconfig como siempre (`aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` seguido de `kubelogin convert-kubeconfig`) y luego [añade sus contextos](/docs/clusters/).

## Qué se ejecuta dónde

- El plugin se ejecuta **en este equipo**, como tú, con los argumentos y las variables de entorno que le da el kubeconfig (`AWS_PROFILE`, por ejemplo). Homeport nunca le pide que sea interactivo y no le da ninguna entrada.
- Un lanzador de escritorio inicia Homeport con un `PATH` corto, así que Homeport también busca en los lugares de instalación habituales de las CLI de AWS, Google Cloud y Azure y de kubelogin (Homebrew, la carpeta propia del SDK, `~/.local/bin`, `~/.krew/bin`, Program Files…). Un plugin indicado con su ruta completa en el kubeconfig se ejecuta desde ahí.
- El token o el certificado que escribe se guarda **solo en memoria**, hasta poco antes de que caduque; entonces el plugin vuelve a ejecutarse. Se ejecuta una vez por cada vida de token, no una vez por solicitud. Cuando la API rechaza un token, Homeport le pide uno nuevo al plugin una vez antes de mostrar **Acceso denegado**.
- Las cachés propias del plugin (`~/.kube/cache`, `~/.aws/sso/cache`, la configuración de gcloud) son cosa suya; Homeport no escribe nada ahí.

## Cuando no puede iniciar sesión

- **«… no está instalado en este equipo»**: no se encontró el programa. Instálalo (el mensaje dice cómo, usando el `installHint` del kubeconfig cuando lo tiene), comprueba que funciona en un terminal y luego elige **Volver a conectar**.
- **«… necesita que inicies sesión, y no puede hacerlo dentro de Homeport»**: el plugin quiere un navegador o un código de dispositivo (AWS SSO, el inicio de sesión de dispositivo de Azure, una sesión de `gcloud` caducada). Ejecuta `kubectl` una vez en un terminal con ese contexto, inicia sesión ahí y luego elige **Volver a conectar**: el plugin encontrará entonces su inicio de sesión en caché.
- **«… no pudo obtener credenciales para …»**: se ejecutó pero falló; **Detalles** muestra el final de lo que escribió. Homeport vuelve a intentarlo por sí solo.

Consulta [Solución de problemas](/docs/reference/troubleshooting/#clusters) para los demás mensajes.

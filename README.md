<article align="center"><h1>Redis Server</h1></article>
Taller 1 - Proyecto para armar un servidor Redis en Rust

Grupo Stormtroopers

## Secciones
 - [Instalación 🔧](#instalación)
 - [Referencia rápida](#referencia-rápida)
 - [Comenzando 🚀](#comenzando)
 - [Documentación 📖](#documentación)
 - [Versionado 📌](#versionado)
 - [Autores ✒️](#autores)

## Stack tecnológico 
Los frameworks y librerías que utilizaremos son:
 - Rustc >= rustc 1.52.0
 - Cargo >= 1.52.0
 - git

## Instalación 
 - Cargo: https://www.rust-lang.org/es/tools/install
 - git: https://git-scm.com/downloads

### Clonando el repositorio  

Clonar el repositorio por HTTPS.

```bash
$ git clone https://github.com/taller-1-fiuba-rust/Stormtroopers.git
```

```bash
$ cd Stormtroopers
```

**Note:** Recuerda reemplazar las configuraciones de git en tu repositorio con el usuario y email que estás utilizando en github. Para ello ejecutar:
```bash
$ git config user.name '$tu-nombre' && git config user.email '$tu-email'
```
Puedes verificar las variables con el comando: `$git config -l`

## Comenzando

Compila el proyecto:
```bash
cargo build
```
Correr los tests:
```bash
cargo test
```
Ejecuta el archivo binario compilado:
```bash
./target/debug/proyecto_taller_1
```

## Documentación
* **Corrida inicial:** Con `cargo run` arranca el proyecto. Para conectarse a través de un cliente se puede ingresar, por ejemplo `nc localhost 8081` si el puerto no fue seteado (la configuración original está en ese puerto).
* **Seteo de la configuración:** En el archivo `redis.config` se pueden setear distintos parámetros, como el puerto de la conexión, el nombre del filedump, el nombre del logger, entre otros.
* **Ruta del archivo de configuración:** Se puede modificar con `cargo run --ruta_de_acceso`
* **Seteo de cantidad de clientes disponibles:** Se tiene una constante `THREAD_POOL_COUNT` que se puede modificar. Tener en cuenta que cada cliente establece dos conexiones del ThreadPool (una para enviar y otra para enviar información a través del Servidor de Redis).
* **Distribución de los módulos:** Distribuimos los distintos módulos a través de las carpetas `command`, `data_base`, `errors`, `handles`, `server`, además de la ejecución del main en primera instancia.
* **Documentación de cada entidad:** En el código se puede encontrar para qué sirve cada entidad creada, junto con algunos ejemplos de ciertos comandos, funciones o comportamiento general.
* **Diagramas:** En la carpeta `docs` se encuentran los distintos diagramas de clases y secuencias elaborados. 

## Autores
* **Cristian Queirolo** - *Trabajo Inicial* - [cristianqueirolo](https://github.com/cqueirolo)
* **Gonzalo Sabatino** - *Trabajo Inicial* - [gonzalosabatino](https://github.com/gsabatino9)
* **Lucas Verón** - *Trabajo Inicial* - [lucasveron](https://github.com/lucasveron)

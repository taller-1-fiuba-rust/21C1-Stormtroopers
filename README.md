<article align="center"><h1>Redis Server</h1></article>
Taller 1 - Proyecto para armar un servidor Redis en Rust

Grupo Stormtroopers

## Secciones
 - [Instalaci贸n 馃敡](#instalaci贸n)
 - [Referencia r谩pida](#referencia-r谩pida)
 - [Comenzando 馃殌](#comenzando)
 - [Documentaci贸n 馃摉](#documentaci贸n)
 - [Versionado 馃搶](#versionado)
 - [Autores 鉁掞笍](#autores)

## Stack tecnol贸gico 
Los frameworks y librer铆as que utilizaremos son:
 - Rustc >= rustc 1.52.0
 - Cargo >= 1.52.0
 - git

## Instalaci贸n 
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

**Note:** Recuerda reemplazar las configuraciones de git en tu repositorio con el usuario y email que est谩s utilizando en github. Para ello ejecutar:
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

## Documentaci贸n
* **Corrida inicial:** Con `cargo run` arranca el proyecto. Para conectarse a trav茅s de un cliente se puede ingresar, por ejemplo `nc localhost 8081` si el puerto no fue seteado (la configuraci贸n original est谩 en ese puerto).
* **Seteo de la configuraci贸n:** En el archivo `redis.config` se pueden setear distintos par谩metros, como el puerto de la conexi贸n, el nombre del filedump, el nombre del logger, entre otros.
* **Ruta del archivo de configuraci贸n:** Se puede modificar con `cargo run --ruta_de_acceso`
* **Seteo de cantidad de clientes disponibles:** Se tiene una constante `THREAD_POOL_COUNT` que se puede modificar. Tener en cuenta que cada cliente establece dos conexiones del ThreadPool (una para enviar y otra para enviar informaci贸n a trav茅s del Servidor de Redis).
* **Distribuci贸n de los m贸dulos:** Distribuimos los distintos m贸dulos a trav茅s de las carpetas `command`, `data_base`, `errors`, `handles`, `server`, adem谩s de la ejecuci贸n del main en primera instancia.
* **Documentaci贸n de cada entidad:** En el c贸digo se puede encontrar para qu茅 sirve cada entidad creada, junto con algunos ejemplos de ciertos comandos, funciones o comportamiento general.
* **Diagramas:** En la carpeta `docs` se encuentran los distintos diagramas de clases y secuencias elaborados. 

## Autores
* **Cristian Queirolo** - *Trabajo Inicial* - [cristianqueirolo](https://github.com/cqueirolo)
* **Gonzalo Sabatino** - *Trabajo Inicial* - [gonzalosabatino](https://github.com/gsabatino9)
* **Lucas Ver贸n** - *Trabajo Inicial* - [lucasveron](https://github.com/lucasveron)

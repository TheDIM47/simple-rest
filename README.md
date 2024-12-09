### Simple REST server.

Eng: Sometimes you need to just quick test some external api without access to it.
Just run `simple-rest` server and create required response files under `resources` directory.

Rus: Иногда вам нужно по-быстрому протестировать взаимодействие с каким-то внешним API, но доступ к нему по тем или иным причинам отсутствует.
Запускаете `simple-rest` и добавляете нужные ответы `json`-ы по нужному пути. Все.

#### Help
```bash
$ target/release/simple-rest --help
Simple REST server. Accept any requests and return resource content, if any

Usage: simple-rest [OPTIONS]

Options:
  -r, --resources <DIR>      Sets a custom resource directory [default: ./resources]
      --host <HOST>          Bind to host [default: 0.0.0.0]
      --port <PORT>          Http port [default: 8080]
  -s, --tls                  Enable https support
  -t, --tls-port <TLS_PORT>  Https port (only if https server enabled) [default: 8443]
  -h, --help                 Print help
  -V, --version              Print version
```

#### Examples

Valid query
```sh
$ curl -v http://localhost:8080/api/json/currency
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/json/currency HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.81.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 172
< content-type: application/json
< date: Mon, 06 Dec 2024 15:46:24 GMT
< 
{
  "status": "success",
  "data": {
    "value": [
      {"id": "1", "text": "Dollar"},
      {"id": "2", "text": "Euro"},
      {"id": "3", "text": "Rouble"}
    ]
  }
}
```

Invalid query
```sh
$ curl -v http://localhost:8080/invalid/path
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /invalid/path HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.81.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 500 Internal Server Error
< content-length: 35
< content-type: text/plain; charset=utf-8
< date: Mon, 06 Dec 2024 15:46:30 GMT
< 
* Connection #0 to host localhost left intact
Path /invalid/path/ does not exists
```

`$ docker build -f Dockerfile.simple -t simplerest .`

`$ docker run -p 8080:8080 simplerest`

Use `-s` or `--tls` keys to enable `https` server.

Default port for `https` is `8443`, `http` - `8080`.

`key.pem` and `cert.pem` are generated with `$ openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

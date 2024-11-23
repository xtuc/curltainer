# curltainer

> curl anything running inside a container, even with exposed ports

## Install

```
cargo install curltainer
```

## Usage

```
Usage: curltainer --container-name <CONTAINER_NAME> [CURL_ARGS]...

Arguments:
  [CURL_ARGS]...

Options:
  -c, --container-name <CONTAINER_NAME>
  -h, --help                             Print help
  -V, --version                          Print version

```

## How it works?

Start a container with a HTTP server, with no ports exposed to the host:
```
$ docker run -d python python -m http.server 9000
a8c8e97ff4f06e8e92c10d41f3764c96a900a098559a946e070c4845219a5007
```

curl the server running inside the container from the host:
```
$ sudo curltainer -c a8c8e97ff4f06e8e92c10d41f3764c96a900a098559a946e070c4845219a5007 localhost:9000  -v

*   Trying 127.0.0.1:9000...
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET / HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.81.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
* HTTP 1.0, assume close after body
< HTTP/1.0 200 OK
< Server: SimpleHTTP/0.6 Python/3.13.0
< Date: Sat, 23 Nov 2024 20:16:00 GMT
< Content-type: text/html; charset=utf-8
< Content-Length: 877
<
<!DOCTYPE HTML>
<html lang="en">
...
```

## esper

flipper flips root domains to www subdomains!

It is powered by the most excellent event driven
[hyper](https://github.com/hyperium/hyper) library.

### Usage

```bash
curl -v -X GET http://localhost:3000/ -H "example.com"
> GET / HTTP/1.1
> User-Agent: curl/7.35.0
> Accept: */*
> Host: example.com
>
< HTTP/1.1 308 Permanent Redirect
< Location: www.example.com
< Date: Wed, 20 Jul 2016 02:12:20 GMT
< Transfer-Encoding: chunked
```

### Deploying

Flipper ships as a standalone executable with a small set of command-line
options. Here is the flipper's help screen for more details on the
supported options:

```
$ flipper --help
flipper - HTTP domain name flipper, powered by hyper.

Usage:
  esper [--bind=<bind>] [--port=<port>] [--threads=<st>]
  esper (-h | --help)
  esper --version

Options:
  -h --help          Show this screen.
  --version          Show version.
  -b --bind=<bind>   Bind to specific IP [default: 127.0.0.1]
  -p --port=<port>   Run on a specific port number [default: 3000]
  -t --threads=<st>  Number of server threads [default: 2].
```

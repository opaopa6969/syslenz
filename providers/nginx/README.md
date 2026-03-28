# syslenz nginx Provider

Monitors an nginx server via the `stub_status` module.

## What it monitors

- **Active connections**: current client connections including waiting
- **Accepts**: total accepted connections since start
- **Handled**: total handled connections (should equal accepts unless resource limits hit)
- **Requests**: total client requests served
- **Reading**: connections currently reading request headers
- **Writing**: connections where nginx is writing a response
- **Waiting**: keep-alive connections waiting for the next request

## Requirements

- `curl` installed and on `$PATH`
- nginx compiled with `--with-http_stub_status_module`
- A `stub_status` location configured in nginx, for example:

```nginx
server {
    listen 80;
    location /nginx_status {
        stub_status;
        allow 127.0.0.1;
        deny all;
    }
}
```

## Configuration

| Variable           | Default                          | Description                  |
|--------------------|----------------------------------|------------------------------|
| `NGINX_STATUS_URL` | `http://localhost/nginx_status`  | Full URL to stub_status page |

## Installation

```bash
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-nginx ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-nginx

# If status is on a non-default URL:
export NGINX_STATUS_URL=http://localhost:8080/status

syslenz
```

## Testing

```bash
./syslenz-provider-nginx | jq .
```

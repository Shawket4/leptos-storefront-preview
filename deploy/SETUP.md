# Deploying the storefront (one-time setup)

The pipeline (`.github/workflows/deploy.yaml`) is MadarRust's:
1. A version tag (`git tag v0.1.0 && git push origin v0.1.0`) builds the Docker image.
2. The image is pushed to `ghcr.io/shawket4/leptos-storefront-preview`.
3. The workflow copies `docker-compose.yml` to the VPS over SSH and restarts the one container.
4. It polls `/health`, then publishes a GitHub Release.

Nothing deploys on a push to `main`.

## 1. GitHub (repository settings)

- **Environment:** create `production` (Settings → Environments).
- **Secrets** (Settings → Secrets and variables → Actions). The names match MadarRust's, so the
  same values work:

  | Secret | What |
  |---|---|
  | `MADAR_SHARED_TOKEN` | A token with read access to `Shawket4/madar-shared`. The pricing and VAT crates are a private git dependency. |
  | `GHCR_TOKEN` | Lets the VPS `docker login ghcr.io` and pull this repo's private image (`read:packages`). |
  | `SSH_PRIVATE_KEY` | The deploy key. |
  | `SSH_KNOWN_HOSTS` | Output of `ssh-keyscan -p <port> <host>`. |
  | `SSH_HOST`, `SSH_PORT`, `SSH_USER` | The VPS. |

- **Variables (optional):**
  - `STOREFRONT_PORT`: the loopback port. Default `8090`; MadarRust uses 8081 and 8083, and apex
    uses nginx :8095.
  - `STOREFRONT_URL`: a public shop URL the workflow checks after deploying.

## 2. DNS and TLS

The shop is the **first label of the hostname**, as with Madar's per-shop subdomains. Pick a base
domain, e.g. `shop.example.com`:
- Add DNS `A` records (or one wildcard `*.shop.example.com`) pointing at the VPS for
  `nile.shop.example.com` and `souq.shop.example.com`.
- Issue a certificate for those names: `certbot certonly --nginx -d nile.shop.example.com -d
  souq.shop.example.com`, or a wildcard through a DNS challenge.

## 3. nginx (TLS, HTTP/2, proxy to the container)

`/etc/nginx/sites-available/leptos-storefront.conf`, linked into `sites-enabled`:

```nginx
server {
    listen 80;
    server_name nile.shop.example.com souq.shop.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    http2 on;
    server_name nile.shop.example.com souq.shop.example.com;
    ssl_certificate     /etc/letsencrypt/live/nile.shop.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/nile.shop.example.com/privkey.pem;

    # The app compresses its own responses (brotli/gzip); don't compress twice.
    gzip off;

    location / {
        proxy_pass http://127.0.0.1:8090;
        proxy_http_version 1.1;
        # The shop comes from the Host header.
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        # Pages are streamed in order: pass them through as they're written.
        proxy_buffering off;
    }
}
```

Then run `nginx -t && systemctl reload nginx`. HTTP/2 matters: the preview's tailnet link was plain
HTTP/1.1, which serialises requests over six connections.

## 4. The VPS

Docker with the compose plugin, which is already present for MadarRust. The workflow creates
`/opt/leptos-storefront/` itself. The app needs **no `.env` and no secrets**:
- the catalogue and images are baked into the image;
- carts live in the shopper's cookie;
- orders are kept in memory (it's a preview).

## 5. First release

1. Set the secrets above.
2. Tag and push: `git tag v0.1.0 && git push origin v0.1.0`.
3. Watch the Actions tab. The workflow ends by curling `http://127.0.0.1:<port>/health` on the box
   and, if `STOREFRONT_URL` is set, the public page.

To build the image locally:

```bash
GH_TOKEN=$(gh auth token) docker build --secret id=madar_shared_token,env=GH_TOKEN -t storefront .
docker run --rm -p 8090:8080 storefront     # then http://nile.localhost:8090
```

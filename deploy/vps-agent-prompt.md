# Prompt for the VPS-side agent

Copy everything below the line into the Claude session running on the VPS. Before pasting, replace
`<duck>` with the DuckDNS subdomain you minted, e.g. `storefront-test` for
`storefront-test.duckdns.org`.

---

You're preparing this VPS and the GitHub repository for the first deploy of a **new, separate
project**: `Shawket4/leptos-storefront-preview` (private). It's a two-shop storefront preview: one
Rust binary (Leptos SSR on Axum) with a small TypeScript client, shipped as one Docker image.
**It is not part of Madar.** Madar's services already run on this box, and you must not change
them. You'll *read* Madar's setup to reuse its conventions.

**Do not deploy anything yourself.** The deploy is a GitHub Actions workflow
(`.github/workflows/deploy.yaml`, modelled on MadarRust's `deploy.yml`), triggered later by the
owner with a version tag. Your job is to make everything it needs ready and verified, then report.

## What the workflow expects (read `deploy.yaml` and `deploy/SETUP.md` in the repo first)

- It SSHes in with `SSH_PRIVATE_KEY` / `SSH_KNOWN_HOSTS` / `SSH_HOST` / `SSH_PORT` / `SSH_USER`.
- It runs `mkdir -p /opt/leptos-storefront` and copies `docker-compose.yml` there.
- It runs `docker login ghcr.io` with `GHCR_TOKEN`, then pulls
  `ghcr.io/shawket4/leptos-storefront-preview:sha-<commit>` (a private image).
- It runs `docker compose -f /opt/leptos-storefront/docker-compose.yml up -d --no-deps --pull never
  storefront`.
  - The container listens on 8080 inside, published on **`127.0.0.1:${STOREFRONT_PORT:-8090}`**.
- It polls `curl -sf http://127.0.0.1:<port>/health` on the box.
- If the repo variable `STOREFRONT_URL` is set, it then checks that public URL for a 200 that
  carries the page's script.
- The app needs **no `.env`, no database and no volumes**: the catalogue and images are in the
  image, and carts live in the shopper's cookie.
- The **shop is the first label of the hostname**: `nile.…` and `souq.…` are the two shops. nginx
  must pass `Host` through unchanged.

## 1. Look at how Madar is deployed here (read-only)

Find and note, changing nothing:
- `/opt/madar-rust` and its `docker-compose.yml`: which **user** owns it and runs the deploys.
- That deploy user's `~/.ssh/authorized_keys`: the key comment of Madar's GitHub Actions key.
- Is that user in the `docker` group? Which `docker compose` version is installed?
- Which ports are already taken (`ss -ltnp`). Madar uses 8081 and 8083; another app is on nginx
  :8095. **Confirm 8090 is free**, or pick a free one.
- Madar's nginx sites (`/etc/nginx/sites-enabled/*`) and snippets (`/etc/nginx/snippets/`, e.g.
  `madar-security-headers.conf`, `madar-csp-public.conf`). Note how TLS, HTTP/2, `gzip_static`,
  real IP and logging are done.
- How certificates are issued and renewed (certbot: nginx plugin or webroot, its renewal timer).

## 2. Deploy access for this repo (suggested: a dedicated key, so it can be revoked on its own)

- Generate a **new ed25519 key pair** for this repo's workflow. Add the public key to the same
  deploy user Madar uses, with the comment `leptos-storefront-preview deploy`.
- Create `/opt/leptos-storefront`, owned by that user.
- Make sure the user can run `docker` without sudo, and that `curl` is installed.
- `SSH_KNOWN_HOSTS` = `ssh-keyscan -p <port> <host>` against the exact host and port the workflow
  will use as `SSH_HOST`/`SSH_PORT`. Use the same host form Madar's workflow uses, most likely the
  public IP.

## 3. DNS: DuckDNS for the testing phase

- The owner minted `<duck>.duckdns.org`. Point it at this VPS's public IP if it isn't already:
  `curl "https://www.duckdns.org/update?domains=<duck>&token=<duckdns token>&ip="`. An empty `ip`
  means "the caller's IP". Ask the owner for the token if you don't have it, and never print it.
- DuckDNS answers every sub-name of your domain, so **`nile.<duck>.duckdns.org` and
  `souq.<duck>.duckdns.org` already resolve**. Verify with `dig +short` for both.
- **This is temporary:** Cloudflare replaces it later (see section 7). Keep the nginx config easy
  to re-point: one `server_name` line and one certificate path.

## 4. TLS

- Issue one Let's Encrypt certificate covering both names, through the same certbot method Madar
  uses: `certbot --nginx -d nile.<duck>.duckdns.org -d souq.<duck>.duckdns.org`, or webroot if
  that's what Madar does.
- Use HTTP-01; a wildcard isn't needed for two names. `duckdns.org` is on the Public Suffix List,
  so rate limits are per DuckDNS subdomain.
- Confirm renewal is covered (`certbot renew --dry-run`).

## 5. nginx

Add `/etc/nginx/sites-available/leptos-storefront.conf`, link it into `sites-enabled`, then
`nginx -t` and reload. **Don't edit any Madar file.** Base it on this (from `deploy/SETUP.md`) and
add what Madar's sites do well:

```nginx
server {
    listen 80;
    server_name nile.<duck>.duckdns.org souq.<duck>.duckdns.org;
    return 301 https://$host$request_uri;          # keep certbot's ACME location if it needs one
}

server {
    listen 443 ssl;
    http2 on;
    server_name nile.<duck>.duckdns.org souq.<duck>.duckdns.org;
    ssl_certificate     /etc/letsencrypt/live/nile.<duck>.duckdns.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/nile.<duck>.duckdns.org/privkey.pem;

    gzip off;                 # the app compresses its own responses (brotli/gzip)
    client_max_body_size 64k;

    location / {
        proxy_pass http://127.0.0.1:8090;
        proxy_http_version 1.1;
        proxy_set_header Host $host;                       # the shop comes from Host
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_buffering off;  # pages stream in order; pass them through as they're written
    }
}
```

What I suggest beyond that:
- **Security headers:** include Madar's `madar-security-headers.conf` if it's generic (HSTS,
  `nosniff`, `Referrer-Policy`, `frame-ancestors`). Check it sets nothing Madar-specific first.
- **Don't include Madar's CSP snippet as is.** This app's pages carry `<script type="module"
  src="/js/app-<hash>.js">`, JSON data blocks (`type="application/json"`, which aren't executed),
  and one tiny inline Leptos script. The `/showcase/stream` demo relies on inline scripts. Use a
  CSP like `default-src 'self'; img-src 'self' data: https:; style-src 'self' 'unsafe-inline';
  script-src 'self' 'unsafe-inline'; connect-src 'self'; frame-ancestors 'none'`, or leave CSP
  off for the testing phase and say so in your report.
- **Caching:** don't add `proxy_cache` or `expires` here. The app sends exact `Cache-Control`
  values: hashed `/js` and `/pkg` assets are `immutable`, pages are `no-cache`, and personal pages
  are `private, no-store`.
- **Before the first deploy, a 502 from the shop names is expected.** Nothing listens on 8090 yet.

## 6. GitHub: environment, secrets, variables

If `gh` on this box is logged in with admin access to `Shawket4/leptos-storefront-preview`, set
them with `gh`. **Never print secret values.** Pipe them in, e.g. `gh secret set SSH_PRIVATE_KEY
-R Shawket4/leptos-storefront-preview < keyfile`, then delete the private key file from the box.
If not, give the owner the exact commands to run.

- **Environment:**
  `gh api -X PUT repos/Shawket4/leptos-storefront-preview/environments/production`.
- **Secrets:**

  | Secret | Value |
  |---|---|
  | `SSH_PRIVATE_KEY` | the new private key from step 2 |
  | `SSH_KNOWN_HOSTS` | from step 2 |
  | `SSH_HOST`, `SSH_PORT`, `SSH_USER` | as the workflow should reach this box (same form as Madar's) |
  | `GHCR_TOKEN` | a token with **`read:packages`**, so the box can pull the private image. Madar's workflow uses the same secret name; if the owner wants to reuse that token, they set it. Verify on the box that `docker login ghcr.io -u Shawket4` succeeds with it. |
  | `MADAR_SHARED_TOKEN` | read access to the private `Shawket4/madar-shared` repo, which the image build fetches. Suggested: a fine-grained PAT with Contents: read on that one repo. **Ask the owner for it**; you can't read Madar's existing secret. |

- **Variables:**
  - `STOREFRONT_PORT` = the port from step 1 (8090 unless taken);
  - `STOREFRONT_URL` = `https://nile.<duck>.duckdns.org`.

## 7. Notes for the Cloudflare switch later (don't do now; include them in your report)

- Move DNS to Cloudflare with `A` records for the shop names, or a wildcard, **proxied**.
- Set SSL to **Full (strict)** and use a Cloudflare Origin Certificate on nginx, or keep Let's
  Encrypt.
- nginx should take the real client IP from `CF-Connecting-IP` (`set_real_ip_from` the Cloudflare
  ranges), and may then accept only Cloudflare's IPs.
- Caching:
  - The app already sends `CDN-Cache-Control: public, s-maxage=300, stale-while-revalidate=60` on
    catalogue pages, and Cloudflare honours it.
  - Add a **Cache Rule with a custom cache key that includes the `lang`, `num` and `theme`
    cookies**. Display preferences are cookies, and without this, shoppers would get pages in the
    wrong language.
  - Pages also carry `Cache-Tag: t<shop>, t<shop>-p<id>, …` for targeted purges. Check purge-by-tag
    on the account's plan.
- Then update the `server_name`, the certificate and the `STOREFRONT_URL` variable.

## 8. Verify, then report (names and results only, never secret values)

Check each of these:
- `dig` resolves both shop names to this box;
- `nginx -t` is OK;
- `curl -I https://nile.<duck>.duckdns.org` gets a valid certificate and HTTP/2 (502 is expected
  before the first deploy);
- the deploy user can run `docker ps`;
- `docker login ghcr.io` with `GHCR_TOKEN` works;
- `ssh -i <new key> -p <port> <user>@<host> 'docker compose version'` works, run from somewhere
  other than the box if you can;
- `gh secret list` and `gh variable list` show every name above, and the `production` environment
  exists.

Then report:
- what you reused from Madar;
- the chosen port;
- the nginx file path;
- the certificate path and how it renews;
- which secrets and variables are set and which the owner still has to provide;
- the CSP decision;
- the Cloudflare notes.

The owner will then tell the lead to tag `v0.1.0`.

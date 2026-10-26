#!/usr/bin/env python3
"""Expose loopback-only dev servers on the tailnet IP (so a phone can watch them live).

    python3 -I tools/dev/tailnet_forward.py 100.123.21.13 3101 3102 3103 3104 3201 3202 3203 3204

Listens on <ip>:<port> and pipes each connection to 127.0.0.1:<port>. Binding the tailnet IP
(not 0.0.0.0) avoids clashing with the dev server's own 127.0.0.1 bind and keeps it off the LAN.
A dev server that isn't running just refuses the connection.
"""

import asyncio
import sys


async def pipe(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
    try:
        while data := await reader.read(65536):
            writer.write(data)
            await writer.drain()
    except (ConnectionError, OSError):
        pass
    finally:
        try:
            writer.close()
        except Exception:
            pass


async def handle(port: int, reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
    try:
        up_r, up_w = await asyncio.open_connection("127.0.0.1", port)
    except OSError:
        writer.close()
        return
    await asyncio.gather(pipe(reader, up_w), pipe(up_r, writer))


async def main() -> None:
    ip, ports = sys.argv[1], [int(p) for p in sys.argv[2:]]
    servers = []
    for p in ports:
        servers.append(await asyncio.start_server(lambda r, w, p=p: handle(p, r, w), ip, p))
        print(f"forwarding {ip}:{p} -> 127.0.0.1:{p}", flush=True)
    await asyncio.gather(*(s.serve_forever() for s in servers))


if __name__ == "__main__":
    asyncio.run(main())

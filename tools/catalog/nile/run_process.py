#!/usr/bin/env python3
"""Run tools/media/process.py unchanged on this machine's Pillow 9.0.1.

process.py uses ``PIL.Image.Resampling`` (Pillow >= 9.1). Pillow 9.0 has the same filters as
module constants, so alias them and hand over to process.py with the same arguments:

    python3 -I tools/catalog/nile/run_process.py --batch jobs.jsonl
"""

from __future__ import annotations

import os
import runpy
import sys
import types

from PIL import Image

if not hasattr(Image, "Resampling"):
    Image.Resampling = types.SimpleNamespace(
        NEAREST=Image.NEAREST, BOX=Image.BOX, BILINEAR=Image.BILINEAR,
        HAMMING=Image.HAMMING, BICUBIC=Image.BICUBIC, LANCZOS=Image.LANCZOS,
    )

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
PROCESS = os.path.join(ROOT, "tools", "media", "process.py")
sys.argv = [PROCESS] + sys.argv[1:]
runpy.run_path(PROCESS, run_name="__main__")

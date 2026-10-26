"""Licence metadata for web-sourced images (Wikimedia Commons via Openverse), keyed by local file stem.

Local originals live in data/media/_src/nile/commons/<stem>.png (or openverse/ for heroes).
"""

LICENSES = {
    "by": ("CC BY", "https://creativecommons.org/licenses/by/{v}/"),
    "by-sa": ("CC BY-SA", "https://creativecommons.org/licenses/by-sa/{v}/"),
    "cc0": ("CC0", "https://creativecommons.org/publicdomain/zero/1.0/"),
}

FILES = {
    "ps5-dualsense": dict(
        title="PlayStation 5 and DualSense with transparent background",
        creator="Soberian (retouched file), Osh33m (original photograph)",
        license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=97325012",
        url="https://upload.wikimedia.org/wikipedia/commons/1/1b/PlayStation_5_and_DualSense_with_transparent_background.png"),
    "ps5-digital": dict(
        title="PS5DigitalEdition", creator="Wfrmsf", license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=99716176",
        url="https://upload.wikimedia.org/wikipedia/commons/0/03/PS5DigitalEdition.png"),
    "dualsense": dict(
        title="Playstation DualSense Controller", creator="Alex Cochrane", license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=99051832",
        url="https://upload.wikimedia.org/wikipedia/commons/3/3c/Playstation_DualSense_Controller.png"),
    "dualsense-gow": dict(
        title="God of War Ragnarök themed DualSense", creator="Godofwarfan333", license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=125511050",
        url="https://upload.wikimedia.org/wikipedia/commons/d/d8/God_of_War_Ragnar%C3%B6k_themed_DualSense.png"),
    "xsx-2": dict(
        title="Xbox Series X 2 (transparent background)", creator="Der. Bellemer", license="by", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=163247277",
        url="https://upload.wikimedia.org/wikipedia/commons/2/25/Xbox_Series_X_2_%28transparent_background%29.png"),
    "xsx-controller": dict(
        title="Xbox Series X mit Controller (transparent background)", creator="Der. Bellemer", license="by",
        version="4.0", landing="https://commons.wikimedia.org/w/index.php?curid=163274421",
        url="https://upload.wikimedia.org/wikipedia/commons/0/09/Xbox_Series_X_mit_Controller_%28transparent_background%29.png"),
    "xsx-xss": dict(
        title="Xbox Series X and S with controllers (transparent background)", creator="Newfiebluejay",
        license="by", version="4.0", landing="https://commons.wikimedia.org/w/index.php?curid=163247278",
        url="https://upload.wikimedia.org/wikipedia/commons/b/b0/Xbox_Series_X_and_S_with_controllers_%28transparent_background%29.png"),
    "switch-oled-pad": dict(
        title="Nintendo Switch OLED", creator="Padgriffin", license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=114717535",
        url="https://upload.wikimedia.org/wikipedia/commons/f/fe/Nintendo_Switch_OLED.png"),
    "switch-oled-docked": dict(
        title="Nintendo Switch – OLED-Modell mit gedockter Konsole 20230506 HOF01624 RAW-Export",
        creator="PantheraLeo1359531", license="by", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=131606693",
        url="https://upload.wikimedia.org/wikipedia/commons/d/da/Nintendo_Switch_%E2%80%93_OLED-Modell_mit_gedockter_Konsole_20230506_HOF01624_RAW-Export.png"),
    "switch2-equip": dict(
        title="Nintendo Switch 2-Spieleequipment 20250605", creator="PantheraLeo1359531", license="by",
        version="4.0", landing="https://commons.wikimedia.org/w/index.php?curid=166895606",
        url="https://upload.wikimedia.org/wikipedia/commons/f/f2/Nintendo_Switch_2-Spieleequipment_20250605.png"),
    "switch2-handheld-cc0": dict(
        title="Nintendo Switch 2 console in handheld mode", creator="AOMAF2024 from Wedel, Germany",
        license="cc0", version="1.0", landing="https://commons.wikimedia.org/w/index.php?curid=166907266",
        url="https://upload.wikimedia.org/wikipedia/commons/8/89/Nintendo_Switch_2_console_in_handheld_mode.png"),
    "steamdeck-front": dict(
        title="Steam Deck (front)", creator="Liam Dawe/GamingOnLinux, PNG version by VulcanSphere",
        license="by-sa", version="4.0", landing="https://commons.wikimedia.org/w/index.php?curid=115603258",
        url="https://upload.wikimedia.org/wikipedia/commons/5/5d/Steam_Deck_%28front%29.png"),
    "iphone16promax-rear": dict(
        title="IPhone 16 Pro Max Desert Titanium Rear", creator="Padgriffin", license="by", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=171391719",
        url="https://upload.wikimedia.org/wikipedia/commons/e/e2/IPhone_16_Pro_Max_Desert_Titanium_Rear.png"),
    # derivatives made with tools/catalog/nile/cutout.py (same licence, changes noted)
    "derived/xss-crop": dict(
        title="Xbox Series X and S with controllers (transparent background)", creator="Newfiebluejay",
        license="by", version="4.0", landing="https://commons.wikimedia.org/w/index.php?curid=163247278",
        url="https://upload.wikimedia.org/wikipedia/commons/b/b0/Xbox_Series_X_and_S_with_controllers_%28transparent_background%29.png",
        changes="cropped to the Series S and its controller"),
    "derived/dualsense-cut": dict(
        title="Playstation DualSense Controller", creator="Alex Cochrane", license="by-sa", version="4.0",
        landing="https://commons.wikimedia.org/w/index.php?curid=99051832",
        url="https://upload.wikimedia.org/wikipedia/commons/3/3c/Playstation_DualSense_Controller.png",
        changes="white backdrop removed (transparent background)"),
}


def license_label(meta: dict) -> str:
    name, _ = LICENSES[meta["license"]]
    return f"{name} {meta['version']}"


def license_url(meta: dict) -> str:
    _, url = LICENSES[meta["license"]]
    return url.format(v=meta["version"])

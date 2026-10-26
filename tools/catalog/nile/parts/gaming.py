"""Gaming (category 13) and the iPhone 16 Pro Max: lead-owned products with Wikimedia Commons photos.

Photos are transparent-background PNGs from Commons (CC BY / CC BY-SA / CC0, see commons_credits.py).
"""

F, D, N, B, X = "featured", "deal", "new", "bestseller", "express"

SKELETON = {
    1301: (13, ("commons", "ps5-dualsense"), "sony", "PlayStation 5 Console — Disc Edition", [F, B],
           "White, with DualSense controller. EGP 29,999."),
    1302: (13, ("commons", "xsx-2", "xsx-controller"), "xbox", "Xbox Series X 1 TB", [D],
           "Black tower with controller. EGP 31,999 compare 36,999."),
    1303: (13, ("commons", "switch2-handheld-cc0", "switch2-equip"), "nintendo", "Nintendo Switch 2", [F, N],
           "Handheld + docked with Joy-Con 2 grip. EGP 28,999."),
    1304: (13, ("commons", "switch-oled-pad", "switch-oled-docked"), "nintendo", "Nintendo Switch — OLED Model", [B],
           "White Joy-Con, 7-inch OLED. EGP 15,999."),
    1305: (13, ("commons", "steamdeck-front"), "valve", "Valve Steam Deck OLED", [],
           "Handheld PC. 512 GB EGP 32,999."),
    1306: (13, ("commons", "ps5-digital"), "sony", "PlayStation 5 Console — Digital Edition", [],
           "All-digital PS5. EGP 25,999."),
    1307: (13, ("commons", "derived/dualsense-cut"), "sony", "DualSense Wireless Controller — White", [X],
           "Standard white PS5 controller. EGP 3,799."),
    1308: (13, ("commons", "derived/xss-crop"), "xbox", "Xbox Series S 512 GB", [],
           "White, with white controller. EGP 17,999."),
}

PS5_SPECS = [
    (("Processor", "المعالج"), ("8-core AMD Zen 2, up to 3.5 GHz", "AMD Zen 2 ثماني النوى حتى 3.5 جيجاهرتز")),
    (("Graphics", "الرسوميات"), ("10.28 TFLOPS AMD RDNA 2 with ray tracing",
                                 "AMD RDNA 2 بقوة 10.28 تيرافلوبس مع تتبّع الأشعة")),
    (("Storage", "التخزين"), ("825 GB custom SSD (667 GB usable) + M.2 expansion slot",
                              "SSD سعة 825 جيجابايت (667 جيجابايت متاحة) مع منفذ توسعة M.2")),
    (("Video output", "إخراج الفيديو"), ("Up to 4K at 120 Hz with HDR, HDMI 2.1", "حتى 4K بتردد 120 هرتز مع HDR عبر HDMI 2.1")),
]

PRODUCTS = [
    dict(
        id=1301,
        name=("PlayStation 5 Console — Disc Edition", "جهاز بلايستيشن 5 — إصدار الأقراص"),
        subtitle=("4K gaming · ultra-fast SSD · Blu-ray drive", "لعب 4K وتحميل في ثواني، ومعاه دراع DualSense"),
        description=(
            "Sony's PS5 with the Ultra HD Blu-ray drive, so you can play game discs and watch 4K films. The custom "
            "SSD loads games in seconds, ray tracing and up to 120 fps make them look next-gen, and the DualSense "
            "controller's haptic feedback and adaptive triggers put every tackle and gear change in your hands.",
            "بلايستيشن 5 من سوني بمشغّل أقراص Ultra HD Blu-ray، فتلعب بالأسطوانات وتشاهد أفلام 4K. وحدة SSD "
            "المصممة خصيصًا تفتح الألعاب في ثوانٍ، وتتبّع الأشعة مع معدل يصل إلى 120 إطارًا يمنحك صورة الجيل "
            "الجديد، بينما ينقل لك دراع DualSense بالاهتزاز اللمسي والأزرار التكيفية كل التحام وكل تغيير سرعة."),
        highlights=[
            ("Ultra HD Blu-ray disc drive", "مشغّل أقراص Ultra HD Blu-ray"),
            ("Custom SSD for near-instant loading", "وحدة SSD سريعة لتحميل شبه فوري"),
            ("DualSense with haptics and adaptive triggers", "دراع DualSense باهتزاز لمسي وأزرار تكيفية"),
            ("1-year Nile warranty", "ضمان نايل لمدة سنة"),
        ],
        specs=PS5_SPECS + [
            (("In the box", "محتويات العلبة"), ("Console, DualSense controller, stand, HDMI and power cables",
                                                "الجهاز ودراع DualSense والقاعدة وكابلا HDMI والكهرباء")),
        ],
        options=[dict(id="bundle", name=("Bundle", "الباقة"), kind="chip", values=[
            dict(id="console", label=("Console only", "الجهاز فقط"), code="CON"),
            dict(id="fc26", label=("With EA SPORTS FC 26", "مع لعبة EA SPORTS FC 26"), code="FC26"),
        ])],
        variants=[dict(opts=["console"], price=29999), dict(opts=["fc26"], price=32499)],
        modifier_groups=[dict(id="addons", name=("Add-ons", "إضافات"), selection="multi", min=0, max=3, required=False,
                              options=[
                                  dict(id="pad", name=("Extra DualSense controller, white", "دراع DualSense إضافي أبيض"),
                                       price=3799),
                                  dict(id="psplus", name=("PlayStation Plus Essential, 12 months (UAE account)",
                                                          "اشتراك PlayStation Plus Essential لمدة 12 شهرًا (حساب إماراتي)"),
                                       price=3199),
                                  dict(id="warranty", name=("Extra year of warranty", "سنة ضمان إضافية"), price=1499),
                              ])],
        rating=4.8, reviews=2634,
        alt=("PlayStation 5 Disc Edition standing upright with a DualSense controller",
             "جهاز بلايستيشن 5 بمشغّل الأقراص واقفًا مع دراع DualSense"),
    ),
    dict(
        id=1302,
        name=("Xbox Series X 1 TB", "إكس بوكس سيريس X سعة 1 تيرابايت"),
        subtitle=("The most powerful Xbox · 4K up to 120 fps", "أقوى إكس بوكس: 4K لحد 120 فريم"),
        description=(
            "Microsoft's most powerful console: 12 teraflops of graphics, a 1 TB custom NVMe SSD and Quick Resume "
            "to jump between several games in seconds. It plays thousands of titles across four generations of "
            "Xbox, many of them enhanced for 4K, and the 4K Blu-ray drive doubles as your film player.",
            "أقوى أجهزة مايكروسوفت: قدرة رسومية 12 تيرافلوبس، ووحدة SSD سعة 1 تيرابايت، وخاصية Quick Resume "
            "للتنقل بين أكثر من لعبة في ثوانٍ. يشغّل آلاف الألعاب من أربعة أجيال من إكس بوكس، كثير منها محسّن "
            "لدقة 4K، ومشغّل أقراص Blu-ray بدقة 4K يجعله مشغّل أفلامك أيضًا."),
        highlights=[
            ("12 TFLOPS, 4K at up to 120 fps", "12 تيرافلوبس و4K حتى 120 إطارًا في الثانية"),
            ("1 TB NVMe SSD with Quick Resume", "SSD سعة 1 تيرابايت مع Quick Resume"),
            ("4K UHD Blu-ray drive", "مشغّل أقراص 4K UHD Blu-ray"),
            ("Plays thousands of older Xbox games", "يشغّل آلاف ألعاب إكس بوكس السابقة"),
        ],
        specs=[
            (("Processor", "المعالج"), ("8-core AMD Zen 2 at 3.8 GHz", "AMD Zen 2 ثماني النوى بتردد 3.8 جيجاهرتز")),
            (("Graphics", "الرسوميات"), ("12 TFLOPS, 52 compute units, RDNA 2", "12 تيرافلوبس و52 وحدة حوسبة بمعمارية RDNA 2")),
            (("Memory", "الذاكرة"), ("16 GB GDDR6", "16 جيجابايت GDDR6")),
            (("Storage", "التخزين"), ("1 TB custom NVMe SSD", "SSD سعة 1 تيرابايت")),
            (("In the box", "محتويات العلبة"), ("Console, Xbox Wireless Controller, HDMI 2.1 and power cables",
                                                "الجهاز ودراع إكس بوكس اللاسلكي وكابلا HDMI 2.1 والكهرباء")),
        ],
        variants=[dict(opts=[], price=31999, compare_at=36999)],
        modifier_groups=[dict(id="addons", name=("Add-ons", "إضافات"), selection="multi", min=0, max=2, required=False,
                              options=[
                                  dict(id="gpu", name=("Game Pass Ultimate, 3 months", "اشتراك Game Pass Ultimate لمدة 3 أشهر"),
                                       price=1999),
                                  dict(id="pad", name=("Extra Xbox Wireless Controller", "دراع إكس بوكس لاسلكي إضافي"),
                                       price=3299),
                              ])],
        rating=4.7, reviews=418,
        alt=("Xbox Series X console with a black wireless controller", "جهاز إكس بوكس سيريس X مع دراع لاسلكي أسود"),
        views=[("front", "من الأمام"), ("angled, controller beside it", "من الزاوية والدراع بجانبه")],
    ),
    dict(
        id=1303,
        name=("Nintendo Switch 2", "نينتندو سويتش 2"),
        subtitle=("Bigger 7.9\" screen · up to 4K on the TV", "شاشة أكبر وJoy-Con بالمغناطيس، والعب على التلفزيون"),
        description=(
            "The next Nintendo Switch: a 7.9-inch 1080p screen with HDR and up to 120 Hz in handheld mode, and up "
            "to 4K when docked to your TV. The new Joy-Con 2 controllers attach magnetically and double as a mouse, "
            "GameChat lets friends talk and share screens, and most Nintendo Switch games play on it too.",
            "الجيل الجديد من نينتندو سويتش: شاشة 7.9 بوصة بدقة 1080p مع HDR وتردد حتى 120 هرتز في الوضع "
            "المحمول، وحتى 4K عند توصيله بالتلفزيون عبر القاعدة. أذرع Joy-Con 2 الجديدة تُثبَّت بالمغناطيس وتعمل "
            "كفأرة أيضًا، وميزة GameChat تتيح الدردشة ومشاركة الشاشة مع أصحابك، ومعظم ألعاب سويتش تعمل عليه."),
        highlights=[
            ("7.9\" 1080p HDR screen, up to 120 Hz", "شاشة 7.9 بوصة 1080p مع HDR حتى 120 هرتز"),
            ("Up to 4K on the TV via the dock", "حتى 4K على التلفزيون عبر القاعدة"),
            ("Magnetic Joy-Con 2 with mouse controls", "أذرع Joy-Con 2 مغناطيسية تعمل كفأرة"),
            ("Plays most Nintendo Switch games", "يشغّل معظم ألعاب نينتندو سويتش"),
        ],
        specs=[
            (("Screen", "الشاشة"), ("7.9\" LCD, 1920 × 1080, HDR10, up to 120 Hz",
                                   "7.9 بوصة LCD بدقة 1920 × 1080 مع HDR10 حتى 120 هرتز")),
            (("TV mode", "وضع التلفزيون"), ("Up to 4K at 60 fps; 1440p/1080p at up to 120 fps",
                                            "حتى 4K بمعدل 60 إطارًا، و1440p أو 1080p حتى 120 إطارًا")),
            (("Storage", "التخزين"), ("256 GB, expandable with microSD Express", "256 جيجابايت قابلة للتوسعة بكروت microSD Express")),
            (("Battery", "البطارية"), ("About 2 to 6.5 hours", "من ساعتين إلى 6.5 ساعات تقريبًا")),
            (("In the box", "محتويات العلبة"), ("Console, Joy-Con 2 (L/R), Joy-Con 2 grip, dock, HDMI cable, USB-C power adapter",
                                                "الجهاز وذراعا Joy-Con 2 ومقبض Joy-Con 2 والقاعدة وكابل HDMI ومحوّل شحن USB-C")),
        ],
        options=[dict(id="bundle", name=("Bundle", "الباقة"), kind="chip", values=[
            dict(id="console", label=("Console only", "الجهاز فقط"), code="CON"),
            dict(id="mkw", label=("Mario Kart World bundle", "باقة Mario Kart World"), code="MKW"),
        ])],
        variants=[dict(opts=["console"], price=28999, image=0), dict(opts=["mkw"], price=31999, image=0)],
        rating=4.8, reviews=57,
        alt=("Nintendo Switch 2", "نينتندو سويتش 2"),
        views=[("console in handheld mode", "الجهاز في الوضع المحمول"),
               ("docked, with the Joy-Con 2 grip", "في القاعدة مع مقبض Joy-Con 2")],
    ),
    dict(
        id=1304,
        name=("Nintendo Switch — OLED Model, White", "نينتندو سويتش — إصدار OLED، أبيض"),
        subtitle=("7\" OLED screen · play at home or on the go", "شاشة OLED ألوانها تفتح النفس، في البيت وبره"),
        description=(
            "The Switch with a 7-inch OLED screen: vivid colours and deep blacks in handheld and tabletop play, a "
            "wide adjustable stand, 64 GB of storage and a dock with a wired LAN port. Slide the white Joy-Con off "
            "to share a game, or dock it and play on the big screen.",
            "سويتش بشاشة OLED مقاس 7 بوصات بألوان زاهية وأسود عميق في اللعب المحمول ووضع الطاولة، مع حامل "
            "عريض قابل للضبط وذاكرة 64 جيجابايت وقاعدة بمنفذ إنترنت سلكي. افصل أذرع Joy-Con البيضاء لتلعب مع "
            "صاحبك، أو ركّبه في القاعدة والعب على الشاشة الكبيرة."),
        highlights=[
            ("7\" OLED screen", "شاشة OLED مقاس 7 بوصات"),
            ("Handheld, tabletop and TV modes", "أوضاع اللعب المحمول والطاولة والتلفزيون"),
            ("64 GB storage, dock with LAN port", "ذاكرة 64 جيجابايت وقاعدة بمنفذ LAN"),
        ],
        specs=[
            (("Screen", "الشاشة"), ("7\" OLED, 1280 × 720", "7 بوصات OLED بدقة 1280 × 720")),
            (("TV output", "إخراج التلفزيون"), ("Up to 1080p", "حتى 1080p")),
            (("Storage", "التخزين"), ("64 GB, microSD up to 2 TB", "64 جيجابايت مع دعم microSD حتى 2 تيرابايت")),
            (("Battery", "البطارية"), ("About 4.5 to 9 hours", "من 4.5 إلى 9 ساعات تقريبًا")),
            (("In the box", "محتويات العلبة"), ("Console, white Joy-Con (L/R), dock, Joy-Con grip, straps, HDMI cable, AC adapter",
                                                "الجهاز وذراعا Joy-Con بيضاوان والقاعدة ومقبض Joy-Con وأحزمة وكابل HDMI ومحوّل الكهرباء")),
        ],
        variants=[dict(opts=[], price=15999)],
        rating=4.8, reviews=1873,
        alt=("Nintendo Switch OLED Model with white Joy-Con", "نينتندو سويتش إصدار OLED بأذرع Joy-Con بيضاء"),
        views=[("handheld, front", "الوضع المحمول من الأمام"), ("in the dock", "داخل القاعدة")],
    ),
    dict(
        id=1305,
        name=("Valve Steam Deck OLED", "فالف ستيم ديك OLED"),
        subtitle=("Your Steam library, in your hands", "مكتبة ستيم كلها في إيدك"),
        description=(
            "A handheld gaming PC that plays your Steam library anywhere: a 7.4-inch HDR OLED screen at up to 90 Hz, "
            "a custom AMD APU, full-size sticks and two trackpads, and a 50 Wh battery. SteamOS makes it "
            "plug-and-play, and it turns into a Linux desktop when you need one. Imported, with a 1-year Nile warranty.",
            "كمبيوتر ألعاب محمول يشغّل مكتبة ستيم في أي مكان: شاشة OLED بتقنية HDR مقاس 7.4 بوصة حتى 90 هرتز، "
            "ومعالج AMD مخصص، وعصا تحكم بالحجم الكامل ولوحتا لمس، وبطارية سعتها 50 واط/ساعة. نظام SteamOS "
            "يجعله جاهزًا للعب فورًا، ويتحول إلى سطح مكتب لينكس عند الحاجة. مستورد بضمان نايل لمدة سنة."),
        highlights=[
            ("7.4\" HDR OLED, up to 90 Hz", "شاشة OLED بتقنية HDR مقاس 7.4 بوصة حتى 90 هرتز"),
            ("Plays your Steam library on the go", "شغّل ألعاب ستيم في أي مكان"),
            ("50 Wh battery and Wi-Fi 6E", "بطارية 50 واط/ساعة وشبكة Wi-Fi 6E"),
        ],
        specs=[
            (("Screen", "الشاشة"), ("7.4\" HDR OLED, 1280 × 800, up to 90 Hz", "7.4 بوصة OLED بتقنية HDR بدقة 1280 × 800 حتى 90 هرتز")),
            (("Processor", "المعالج"), ("Custom AMD APU, Zen 2 CPU + RDNA 2 GPU", "معالج AMD مخصص بأنوية Zen 2 ورسوميات RDNA 2")),
            (("Memory", "الذاكرة"), ("16 GB LPDDR5", "16 جيجابايت LPDDR5")),
            (("Battery", "البطارية"), ("50 Wh, about 3 to 12 hours", "50 واط/ساعة، من 3 إلى 12 ساعة تقريبًا")),
            (("Weight", "الوزن"), ("About 640 g", "حوالي 640 جرامًا")),
        ],
        options=[dict(id="storage", name=("Storage", "السعة التخزينية"), kind="chip", values=[
            dict(id="512", label=("512 GB", "512 جيجابايت")),
            dict(id="1tb", label=("1 TB", "1 تيرابايت"), code="1TB"),
        ])],
        variants=[dict(opts=["512"], price=32999), dict(opts=["1tb"], price=38999)],
        rating=4.7, reviews=92,
        alt=("Valve Steam Deck handheld, front view", "جهاز ستيم ديك المحمول من الأمام"),
    ),
    dict(
        id=1306,
        name=("PlayStation 5 Console — Digital Edition", "جهاز بلايستيشن 5 — الإصدار الرقمي"),
        subtitle=("Full PS5 power, no disc drive", "نفس قوة PS5 من غير أقراص وبسعر أوفر"),
        description=(
            "All the speed and graphics of the PS5 without the disc drive: buy and download your games from "
            "PlayStation Store and keep the same custom SSD, ray tracing and DualSense controller. A slimmer, "
            "lighter console for players whose library is already digital.",
            "كل سرعة بلايستيشن 5 ورسومياته لكن من غير مشغّل أقراص: اشترِ ألعابك وحمّلها من متجر بلايستيشن، "
            "واستمتع بنفس وحدة SSD وتتبّع الأشعة ودراع DualSense. جهاز أنحف وأخف لمن أصبحت مكتبته رقمية بالكامل."),
        highlights=[
            ("Same performance as the disc edition", "نفس أداء إصدار الأقراص"),
            ("Custom SSD for near-instant loading", "وحدة SSD سريعة لتحميل شبه فوري"),
            ("DualSense controller included", "دراع DualSense داخل العلبة"),
        ],
        specs=PS5_SPECS + [
            (("In the box", "محتويات العلبة"), ("Console, DualSense controller, stand, HDMI and power cables",
                                                "الجهاز ودراع DualSense والقاعدة وكابلا HDMI والكهرباء")),
        ],
        variants=[dict(opts=[], price=25999)],
        rating=4.7, reviews=612,
        alt=("PlayStation 5 Digital Edition console", "جهاز بلايستيشن 5 الإصدار الرقمي"),
    ),
    dict(
        id=1307,
        name=("DualSense Wireless Controller — White", "دراع DualSense اللاسلكي — أبيض"),
        subtitle=("Haptic feedback · adaptive triggers · built-in mic", "إحساس اللعب في إيدك بالاهتزاز والأزرار التكيفية"),
        description=(
            "The standard PS5 controller: haptic feedback that lets you feel every surface and impact, adaptive "
            "triggers with real resistance, a built-in microphone with a mute button and a rechargeable battery "
            "that charges over USB-C. It also pairs with PC, Mac and phones over Bluetooth.",
            "دراع بلايستيشن 5 الأساسي: اهتزاز لمسي يجعلك تشعر بكل سطح وكل ارتطام، وأزرار خلفية تكيفية بمقاومة "
            "حقيقية، وميكروفون مدمج بزر كتم، وبطارية قابلة للشحن عبر USB-C. ويعمل أيضًا مع الكمبيوتر والماك "
            "والموبايل عبر البلوتوث."),
        highlights=[
            ("Haptic feedback and adaptive triggers", "اهتزاز لمسي وأزرار تكيفية"),
            ("Built-in microphone with mute button", "ميكروفون مدمج بزر كتم الصوت"),
            ("Rechargeable, USB-C charging", "بطارية قابلة للشحن عبر USB-C"),
        ],
        specs=[
            (("Compatibility", "التوافق"), ("PlayStation 5; PC, Mac and phones via Bluetooth",
                                            "بلايستيشن 5، والكمبيوتر والماك والموبايل عبر البلوتوث")),
            (("Battery", "البطارية"), ("Built-in, about 6 to 12 hours", "مدمجة، من 6 إلى 12 ساعة تقريبًا")),
            (("Connection", "الاتصال"), ("Bluetooth 5.1, USB-C", "بلوتوث 5.1 ومنفذ USB-C")),
            (("Audio", "الصوت"), ("Built-in speaker, mic and 3.5 mm headset jack",
                                  "سماعة وميكروفون مدمجان ومنفذ سماعة 3.5 مم")),
            (("Weight", "الوزن"), ("About 280 g", "حوالي 280 جرامًا")),
        ],
        variants=[dict(opts=[], price=3799)],
        rating=4.8, reviews=1214,
        alt=("White DualSense wireless controller for PlayStation 5", "دراع DualSense اللاسلكي الأبيض لبلايستيشن 5"),
    ),
    dict(
        id=1308,
        name=("Xbox Series S 512 GB — Robot White", "إكس بوكس سيريس S سعة 512 جيجابايت — أبيض"),
        subtitle=("Next-gen speed in the smallest Xbox", "أصغر إكس بوكس بسرعة الجيل الجديد"),
        description=(
            "The compact, all-digital Xbox: next-gen loading and Quick Resume in a console about 60% smaller than "
            "Series X. Games run at up to 1440p and 120 fps, films and apps stream in 4K, and the 512 GB custom "
            "SSD keeps your favourites ready. Comes with the white Xbox Wireless Controller.",
            "إكس بوكس الرقمي المدمج: تحميل الجيل الجديد وخاصية Quick Resume في جهاز أصغر من سيريس X بنحو 60%. "
            "يشغّل الألعاب حتى 1440p بمعدل 120 إطارًا، ويعرض الأفلام والتطبيقات بدقة 4K، ووحدة SSD سعة 512 "
            "جيجابايت تحفظ ألعابك المفضلة جاهزة. مع دراع إكس بوكس اللاسلكي الأبيض."),
        highlights=[
            ("Up to 1440p at 120 fps", "حتى 1440p بمعدل 120 إطارًا"),
            ("512 GB SSD with Quick Resume", "SSD سعة 512 جيجابايت مع Quick Resume"),
            ("All-digital and about 60% smaller than Series X", "رقمي بالكامل وأصغر من سيريس X بنحو 60%"),
        ],
        specs=[
            (("Processor", "المعالج"), ("8-core AMD Zen 2 at 3.6 GHz", "AMD Zen 2 ثماني النوى بتردد 3.6 جيجاهرتز")),
            (("Graphics", "الرسوميات"), ("4 TFLOPS, RDNA 2", "4 تيرافلوبس بمعمارية RDNA 2")),
            (("Memory", "الذاكرة"), ("10 GB GDDR6", "10 جيجابايت GDDR6")),
            (("Storage", "التخزين"), ("512 GB custom NVMe SSD", "SSD سعة 512 جيجابايت")),
            (("In the box", "محتويات العلبة"), ("Console, white Xbox Wireless Controller, HDMI and power cables",
                                                "الجهاز ودراع إكس بوكس اللاسلكي الأبيض وكابلا HDMI والكهرباء")),
        ],
        variants=[dict(opts=[], price=17999)],
        rating=4.6, reviews=389,
        alt=("White Xbox Series S console with a white wireless controller", "جهاز إكس بوكس سيريس S الأبيض مع دراع لاسلكي أبيض"),
    ),
]

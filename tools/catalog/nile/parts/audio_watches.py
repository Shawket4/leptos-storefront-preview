"""Audio & Accessories (1201–1213) and Watches (1401–1412)."""

PRODUCTS = [
    # ------------------------------------------------------------------ Audio & Accessories
    dict(
        id=1201,
        name=("Apple AirPods 4", "سماعات أبل إيربودز 4"),
        subtitle=("H2 chip · USB-C case · ANC option", "صوت أنقى وراحة طول اليوم، وفيه نسخة بعزل الضوضاء"),
        description=(
            "Apple's most comfortable open-fit AirPods, redesigned around the H2 chip for richer sound, "
            "Personalised Spatial Audio and clearer calls with Voice Isolation. Pick the version with Active "
            "Noise Cancellation to add Transparency mode, Adaptive Audio and wireless charging for the case.",
            "أكثر سماعات إيربودز راحةً بتصميمها المفتوح الجديد، وشريحة H2 تمنحك صوتًا أغنى وصوتًا محيطيًا "
            "مخصصًا ومكالمات أوضح بفضل خاصية عزل الصوت. اختر نسخة إلغاء الضوضاء النشط لتحصل كذلك على وضع "
            "الشفافية والصوت التكيفي وشحن العلبة لاسلكيًا.",
        ),
        highlights=[
            ("H2 chip with Personalised Spatial Audio", "شريحة H2 مع صوت محيطي مخصص"),
            ("Up to 30 hours of listening with the USB-C case", "حتى 30 ساعة استماع مع علبة الشحن USB-C"),
            ("IP54 dust, sweat and water resistance", "مقاومة للغبار والعرق والماء بمعيار IP54"),
        ],
        specs=[
            (("Chip", "الشريحة"), ("Apple H2", "Apple H2")),
            (("Battery", "البطارية"), ("Up to 5 h listening; 30 h with case", "حتى 5 ساعات استماع، و30 ساعة مع العلبة")),
            (("Noise control", "التحكم في الضوضاء"),
             ("ANC, Transparency and Adaptive Audio on the ANC model", "إلغاء الضوضاء والشفافية والصوت التكيفي في نسخة ANC")),
            (("Charging", "الشحن"),
             ("USB-C; wireless and Apple Watch charger on the ANC model", "USB-C، ولاسلكيًا أو بشاحن ساعة أبل في نسخة ANC")),
            (("Water resistance", "مقاومة الماء"), ("IP54 (earbuds and case)", "IP54 للسماعات والعلبة")),
        ],
        options=[
            dict(id="model", name=("Model", "الموديل"), kind="chip", values=[
                dict(id="std", label=("AirPods 4", "إيربودز 4"), code="STD"),
                dict(id="anc", label=("With Active Noise Cancellation", "بإلغاء الضوضاء النشط"), code="ANC"),
            ]),
        ],
        variants=[
            dict(opts=["std"], price=7499, image=0),
            dict(opts=["anc"], price=9999, image=0),
        ],
        rating=4.7, reviews=1186,
        alt=("Apple AirPods 4 with charging case", "سماعات أبل إيربودز 4 مع علبة الشحن"),
        views=[
            ("case open with the earbuds inside", "العلبة مفتوحة والسماعات بداخلها"),
            ("earbuds lifted out of the case", "السماعات خارج العلبة"),
            ("pair of earbuds", "زوج السماعات"),
        ],
    ),
    dict(
        id=1202,
        name=("Apple AirPods Max — Silver", "سماعة أبل إيربودز ماكس — فضي"),
        subtitle=("Over-ear ANC · Spatial Audio · 20-hour battery", "عزل ضوضاء بجد وصوت يحسسك إنك في سينما"),
        description=(
            "Apple's over-ear headphones pair a 40 mm dynamic driver with the H1 chip in each ear cup for Active "
            "Noise Cancellation, Transparency mode and Spatial Audio with head tracking. The knit-mesh canopy and "
            "memory-foam cushions spread the weight, and the Digital Crown gives precise volume control. Silver "
            "Lightning model, supplied with the Smart Case.",
            "سماعة أبل الرأسية تجمع بين مشغّل ديناميكي 40 مم وشريحة H1 في كل أذن، لتمنحك إلغاء ضوضاء نشطًا "
            "ووضع الشفافية وصوتًا محيطيًا يتتبع حركة رأسك. الحزام الشبكي والوسائد الإسفنجية يوزّعان الوزن براحة، "
            "والتاج الرقمي يتحكم في مستوى الصوت بدقة. اللون الفضي بمنفذ لايتنينج، ومعها الحافظة الذكية.",
        ),
        highlights=[
            ("Active Noise Cancellation and Transparency mode", "إلغاء ضوضاء نشط ووضع الشفافية"),
            ("Spatial Audio with dynamic head tracking", "صوت محيطي مع تتبّع حركة الرأس"),
            ("Up to 20 hours of listening per charge", "حتى 20 ساعة استماع بالشحنة الواحدة"),
        ],
        specs=[
            (("Driver", "المشغّل الصوتي"), ("40 mm Apple dynamic driver", "مشغّل ديناميكي من أبل 40 مم")),
            (("Chip", "الشريحة"), ("Apple H1 in each ear cup", "Apple H1 في كل أذن")),
            (("Battery", "البطارية"), ("Up to 20 h with ANC on", "حتى 20 ساعة مع تشغيل إلغاء الضوضاء")),
            (("Charging", "الشحن"), ("Lightning", "لايتنينج")),
            (("Weight", "الوزن"), ("384.8 g", "384.8 جرام")),
            (("In the box", "محتويات العلبة"), ("Smart Case, Lightning to USB-C cable", "الحافظة الذكية وكابل لايتنينج إلى USB-C")),
        ],
        variants=[dict(opts=[], price=29999, compare_at=34999, image=0)],
        rating=4.6, reviews=412,
        alt=("Apple AirPods Max headphones in Silver", "سماعة أبل إيربودز ماكس باللون الفضي"),
        views=[],
    ),
    dict(
        id=1203,
        name=("Beats Flex Wireless Earphones — Yuzu Yellow", "سماعات بيتس فليكس اللاسلكية — أصفر يوزو"),
        subtitle=("Magnetic earbuds · 12-hour battery", "خفيفة وبتقعد معاك اليوم كله"),
        description=(
            "Everyday wireless earphones with Apple's W1 chip for one-tap pairing and a steady Class 1 Bluetooth "
            "connection. The magnetic earbuds snap together around your neck and pause your music automatically, "
            "and a 10-minute Fast Fuel top-up gives 1.5 hours of playback.",
            "سماعات لاسلكية للاستخدام اليومي بشريحة W1 من أبل للاقتران بلمسة واحدة واتصال بلوتوث ثابت. "
            "السماعتان تلتصقان مغناطيسيًا حول رقبتك وتوقفان الموسيقى تلقائيًا، وشحن سريع لعشر دقائق يكفيك "
            "ساعة ونصف من التشغيل.",
        ),
        highlights=[
            ("Apple W1 chip for one-tap pairing", "شريحة Apple W1 للاقتران بلمسة واحدة"),
            ("Up to 12 hours of listening", "حتى 12 ساعة استماع"),
            ("Magnetic earbuds auto-pause your music", "سماعات مغناطيسية توقف الموسيقى تلقائيًا"),
        ],
        specs=[
            (("Chip", "الشريحة"), ("Apple W1", "Apple W1")),
            (("Battery", "البطارية"), ("Up to 12 hours", "حتى 12 ساعة")),
            (("Fast charge", "الشحن السريع"), ("10 min = 1.5 h playback", "10 دقائق = ساعة ونصف تشغيل")),
            (("Charging port", "منفذ الشحن"), ("USB-C", "USB-C")),
            (("Colour", "اللون"), ("Yuzu Yellow", "أصفر يوزو")),
        ],
        variants=[dict(opts=[], price=3299, compare_at=3999, image=0)],
        rating=4.4, reviews=655,
        alt=("Beats Flex wireless earphones in Yuzu Yellow", "سماعات بيتس فليكس اللاسلكية باللون الأصفر"),
        views=[],
    ),
    dict(
        id=1204,
        name=("Apple HomePod mini — Midnight", "سماعة أبل هوم بود ميني الذكية — منتصف الليل"),
        subtitle=("360° sound · Siri · smart-home hub", "صغيرة في حجمها بس صوتها مالي الأوضة"),
        description=(
            "A 9.8 cm sphere that fills the room with 360-degree sound, tuned in real time by computational audio "
            "on the S5 chip. Ask Siri for music, timers and reminders, pair two for stereo, and use it as a Thread "
            "hub for smart-home accessories. Midnight finish, 20 W USB-C adapter included.",
            "كرة صغيرة قطرها 9.8 سم تملأ الغرفة بصوت 360 درجة يضبطه الصوت الحوسبي على شريحة S5 لحظة بلحظة. "
            "اطلب من Siri الموسيقى والمؤقتات والتذكيرات، واربط جهازين معًا لصوت ستيريو، واستخدمه مركزًا لأجهزة "
            "البيت الذكي بتقنية Thread. بلون منتصف الليل، ومعه شاحن USB-C بقدرة 20 واط.",
        ),
        highlights=[
            ("360-degree sound with computational audio", "صوت 360 درجة مع الصوت الحوسبي"),
            ("Siri, Intercom and stereo pairing", "Siri والاتصال الداخلي والربط الستيريو"),
            ("Smart-home hub with Thread", "مركز للبيت الذكي بتقنية Thread"),
        ],
        specs=[
            (("Chip", "الشريحة"), ("Apple S5", "Apple S5")),
            (("Audio", "الصوت"), ("Full-range driver, 2 passive radiators", "مشغّل كامل النطاق ومشعّان سلبيان")),
            (("Connectivity", "الاتصال"), ("Wi-Fi 4, Bluetooth 5.0, Thread", "Wi-Fi 4 وبلوتوث 5.0 وThread")),
            (("Size", "الأبعاد"), ("84.3 × 97.9 mm, 345 g", "84.3 × 97.9 مم، 345 جرام")),
            (("Power", "الطاقة"), ("20 W USB-C adapter included", "شاحن USB-C بقدرة 20 واط مرفق")),
        ],
        variants=[dict(opts=[], price=5999, image=0)],
        rating=4.6, reviews=238,
        alt=("Apple HomePod mini in Midnight", "سماعة أبل هوم بود ميني بلون منتصف الليل"),
        views=[],
    ),
    dict(
        id=1205,
        name=("Amazon Echo (3rd Gen) Smart Speaker — Heather Grey", "سماعة أمازون إيكو الذكية (الجيل الثالث) — رمادي فاتح"),
        subtitle=("Alexa smart speaker · rich 360° sound", "قول لأليكسا تشغّل اللي نفسك فيه"),
        description=(
            "Amazon's fabric-covered smart speaker with a 3-inch woofer and 0.8-inch tweeter for full, balanced "
            "360° sound. Ask Alexa to play music, set alarms, read the news or control compatible smart plugs and "
            "bulbs, and hook up another speaker through the 3.5 mm line in/out.",
            "سماعة أمازون الذكية بغطاء قماشي، بمكبر صوت جهير 3 بوصات وتويتر 0.8 بوصة لصوت ممتلئ ومتوازن في كل "
            "الاتجاهات. اطلب من أليكسا تشغيل الموسيقى أو ضبط المنبه أو قراءة الأخبار أو التحكم في المقابس "
            "واللمبات الذكية المتوافقة، ووصّل سماعة أخرى عبر مدخل ومخرج الصوت 3.5 مم.",
        ),
        highlights=[
            ("3-inch woofer and 0.8-inch tweeter with Dolby processing", "مكبر جهير 3 بوصات وتويتر 0.8 بوصة مع معالجة Dolby"),
            ("Hands-free Alexa for music, alarms and smart plugs", "أليكسا بدون استخدام اليدين للموسيقى والمنبهات والمقابس الذكية"),
            ("3.5 mm line in/out", "مدخل ومخرج صوت 3.5 مم"),
        ],
        specs=[
            (("Speakers", "السماعات"), ("3.0-inch woofer, 0.8-inch tweeter", "مكبر جهير 3.0 بوصة وتويتر 0.8 بوصة")),
            (("Voice assistant", "المساعد الصوتي"), ("Alexa (English)", "أليكسا (باللغة الإنجليزية)")),
            (("Connectivity", "الاتصال"), ("Dual-band Wi-Fi, Bluetooth, 3.5 mm in/out", "Wi-Fi ثنائي النطاق وبلوتوث ومنفذ 3.5 مم")),
            (("Size", "الأبعاد"), ("148 × 99 mm, 780 g", "148 × 99 مم، 780 جرام")),
            (("Colour", "اللون"), ("Heather Grey fabric", "قماش رمادي فاتح")),
        ],
        variants=[dict(opts=[], price=3999, compare_at=4999, image=0)],
        rating=4.4, reviews=317,
        alt=("Amazon Echo smart speaker in Heather Grey", "سماعة أمازون إيكو الذكية باللون الرمادي الفاتح"),
        views=[
            ("front", "من الأمام"),
            ("top with light ring and buttons", "من أعلى مع حلقة الإضاءة والأزرار"),
        ],
    ),
    dict(
        id=1206,
        name=("Apple 5W USB Power Adapter + Lightning Cable (1 m)", "شاحن أبل USB بقدرة 5 واط مع كابل لايتنينج (1 متر)"),
        subtitle=("Original Apple charger for Lightning iPhones", "الشاحن الأصلي لأي آيفون بمدخل لايتنينج"),
        description=(
            "The genuine Apple 5 W USB power adapter with a European two-pin plug, bundled with Apple's 1 m "
            "USB-to-Lightning cable. The right everyday charger for Lightning iPhones — including the renewed "
            "iPhones on Nile — AirPods with Lightning cases and older iPads.",
            "شاحن أبل الأصلي بقدرة 5 واط بفيشة أوروبية بسنّين، ومعه كابل أبل من USB إلى لايتنينج بطول متر. "
            "الشاحن اليومي المناسب لأجهزة آيفون بمنفذ لايتنينج، ومنها الآيفونات المجددة على نايل، وكذلك علب "
            "إيربودز وأجهزة آيباد القديمة.",
        ),
        highlights=[
            ("Genuine Apple adapter with European two-pin plug", "شاحن أبل أصلي بفيشة أوروبية بسنّين"),
            ("1 m USB-A to Lightning cable included", "كابل USB-A إلى لايتنينج بطول متر مرفق"),
            ("For Lightning iPhones, AirPods and iPads", "لأجهزة آيفون وإيربودز وآيباد بمنفذ لايتنينج"),
        ],
        specs=[
            (("Output", "القدرة"), ("5 W (5 V ⎓ 1 A)", "5 واط (5 فولت ⎓ 1 أمبير)")),
            (("Plug", "الفيشة"), ("European two-pin (Type C)", "أوروبية بسنّين (Type C)")),
            (("Cable", "الكابل"), ("USB-A to Lightning, 1 m", "USB-A إلى لايتنينج، 1 متر")),
            (("Compatibility", "التوافق"),
             ("iPhone 5 to iPhone 14, Lightning AirPods and iPads", "من آيفون 5 حتى آيفون 14، وإيربودز وآيباد بمنفذ لايتنينج")),
        ],
        variants=[dict(opts=[], price=999, image=0)],
        rating=4.6, reviews=2412,
        alt=("Apple 5W USB power adapter with Lightning cable", "شاحن أبل 5 واط مع كابل لايتنينج"),
        views=[
            ("adapter and coiled cable", "الشاحن والكابل ملفوفًا"),
            ("two-pin plug and cable", "الفيشة بسنّين والكابل"),
        ],
    ),
    dict(
        id=1207,
        name=("Belkin BoostCharge Dual Wireless Charging Pad (10 W)", "قاعدة شحن لاسلكي مزدوجة بلكين بوست تشارج (10 واط)"),
        subtitle=("Charge phone and earbuds together · 10 W", "اشحن الموبايل والسماعة مع بعض من غير سلك"),
        description=(
            "Two charging spots on one slim pad, so your phone and wireless-charging earbuds top up side by side "
            "overnight. Delivers up to 10 W to supported Android phones and 7.5 W to iPhone, with LED indicators "
            "and a non-slip finish; wall adapter included.",
            "مكانان للشحن على قاعدة واحدة نحيفة، لتشحن الموبايل والسماعة اللاسلكية جنبًا إلى جنب طوال الليل. "
            "تصل القدرة إلى 10 واط لهواتف أندرويد المتوافقة و7.5 واط للآيفون، مع لمبات بيان وسطح مانع "
            "للانزلاق، ومعها محوّل الكهرباء.",
        ),
        highlights=[
            ("Charges two devices at once", "يشحن جهازين في نفس الوقت"),
            ("Up to 10 W fast wireless charging", "شحن لاسلكي سريع حتى 10 واط"),
            ("Wall adapter included", "محوّل الكهرباء مرفق"),
        ],
        specs=[
            (("Output", "القدرة"), ("Up to 10 W (7.5 W for iPhone)", "حتى 10 واط (7.5 واط للآيفون)")),
            (("Charging spots", "أماكن الشحن"), ("2 Qi spots", "مكانان بمعيار Qi")),
            (("Works with", "يعمل مع"), ("Qi phones and wireless earbud cases", "الموبايلات وعلب السماعات الداعمة لمعيار Qi")),
            (("Colour", "اللون"), ("White", "أبيض")),
        ],
        variants=[dict(opts=[], price=2299, image=0)],
        rating=4.3, reviews=164,
        alt=("Belkin dual wireless charging pad in white", "قاعدة شحن لاسلكي مزدوجة من بلكين باللون الأبيض"),
        views=[],
    ),
    dict(
        id=1208,
        name=("Apple MagSafe Battery Pack", "بطارية أبل MagSafe الخارجية"),
        subtitle=("Snap-on battery for iPhone 12 and later", "لزّقها في ضهر الآيفون وكمّل يومك من غير شاحن"),
        description=(
            "Apple's slim battery snaps magnetically onto the back of iPhone 12 and later and charges it wirelessly "
            "on the go at up to 7.5 W. Plug it into a 20 W charger and it recharges itself and your iPhone together; "
            "its charge level shows in the iPhone Batteries widget.",
            "بطارية أبل النحيفة تثبت مغناطيسيًا على ظهر آيفون 12 وما بعده وتشحنه لاسلكيًا وأنت في الطريق بقدرة "
            "حتى 7.5 واط. وصّلها بشاحن 20 واط لتشحن نفسها والآيفون معًا، وتابع نسبة شحنها من أداة البطاريات "
            "على الآيفون.",
        ),
        highlights=[
            ("Snaps on magnetically — no cables", "تثبت مغناطيسيًا بدون أسلاك"),
            ("Charges iPhone at up to 7.5 W on the go", "تشحن الآيفون حتى 7.5 واط أثناء التنقل"),
            ("Charge level in the iPhone widget", "نسبة الشحن تظهر في أداة البطاريات على الآيفون"),
        ],
        specs=[
            (("Capacity", "السعة"), ("1,460 mAh (11.13 Wh)", "1460 مللي أمبير (11.13 واط ساعة)")),
            (("Wireless output", "الشحن اللاسلكي"), ("Up to 7.5 W", "حتى 7.5 واط")),
            (("Input port", "منفذ الشحن"), ("Lightning", "لايتنينج")),
            (("Compatibility", "التوافق"), ("iPhone 12 and later", "آيفون 12 وما بعده")),
        ],
        variants=[dict(opts=[], price=4499, compare_at=5499, image=0)],
        rating=4.2, reviews=389,
        alt=("Apple MagSafe Battery Pack", "بطارية أبل MagSafe الخارجية"),
        views=[
            ("back with the Apple logo", "الظهر بشعار أبل"),
            ("magnetic MagSafe side", "الوجه المغناطيسي MagSafe"),
        ],
    ),
    dict(
        id=1209,
        name=("Apple Silicone Case with MagSafe for iPhone 15 — Plum", "جراب أبل سيليكون بتقنية MagSafe لهاتف آيفون 15 — برقوقي"),
        subtitle=("Silky silicone · microfibre lining · MagSafe", "ملمس ناعم وحماية شيك، وشغال مع MagSafe"),
        description=(
            "Apple's silicone case wraps iPhone 15 in a silky, soft-touch finish, with a microfibre lining that "
            "protects the back. Built-in MagSafe magnets keep chargers and wallets aligned, and it slips on and off "
            "easily. Plum colour.",
            "جراب أبل السيليكون يغلّف آيفون 15 بملمس حريري ناعم، مع بطانة داخلية من الألياف الدقيقة تحمي ظهر "
            "الموبايل. مغناطيسات MagSafe المدمجة تثبّت الشواحن والمحافظ في مكانها، ويُركَّب ويُفك بسهولة. "
            "باللون البرقوقي.",
        ),
        highlights=[
            ("Built-in MagSafe magnets", "مغناطيسات MagSafe مدمجة"),
            ("Silky silicone with microfibre lining", "سيليكون حريري الملمس مع بطانة من الألياف الدقيقة"),
            ("Fits iPhone 15 (6.1-inch)", "مناسب لآيفون 15 مقاس 6.1 بوصة"),
        ],
        specs=[
            (("Compatibility", "التوافق"), ("iPhone 15", "آيفون 15")),
            (("Material", "الخامة"), ("Silicone, microfibre lining", "سيليكون مع بطانة من الألياف الدقيقة")),
            (("MagSafe support", "دعم MagSafe"), ("Yes", "نعم")),
            (("Colour", "اللون"), ("Plum", "برقوقي")),
        ],
        variants=[dict(opts=[], price=2299, image=0)],
        rating=4.5, reviews=296,
        alt=("Apple silicone case with MagSafe in Plum", "جراب أبل سيليكون بتقنية MagSafe باللون البرقوقي"),
        views=[
            ("back", "من الخلف"),
            ("angled view", "بزاوية جانبية"),
            ("microfibre lining", "البطانة الداخلية"),
            ("side buttons", "الأزرار الجانبية"),
        ],
    ),
    dict(
        id=1210,
        name=("UGREEN Selfie Stick Tripod with Bluetooth Remote", "عصا سيلفي وحامل ثلاثي من يوجرين مع ريموت بلوتوث"),
        subtitle=("Selfie stick + tripod · detachable remote", "عصا سيلفي وترايبود في واحد، والريموت بيتشال"),
        description=(
            "A selfie stick that unfolds into a stable tripod, with a 360° rotating phone clamp and a detachable "
            "Bluetooth remote for group shots from up to 10 m away. The aluminium shaft extends to 102 cm and folds "
            "down small enough for a backpack pocket.",
            "عصا سيلفي تتحول إلى حامل ثلاثي ثابت، بماسك موبايل يدور 360 درجة وريموت بلوتوث قابل للفصل لتصوير "
            "الصور الجماعية من مسافة تصل إلى 10 أمتار. الذراع من الألومنيوم يمتد حتى 102 سم ويُطوى ليدخل جيب "
            "حقيبة الظهر.",
        ),
        highlights=[
            ("Selfie stick and tripod in one", "عصا سيلفي وحامل ثلاثي في جهاز واحد"),
            ("Detachable Bluetooth remote, 10 m range", "ريموت بلوتوث قابل للفصل بمدى 10 أمتار"),
            ("360° rotating phone clamp", "ماسك موبايل يدور 360 درجة"),
        ],
        specs=[
            (("Extended length", "الطول الممتد"), ("102 cm", "102 سم")),
            (("Folded length", "الطول مطويًا"), ("19 cm", "19 سم")),
            (("Phone width", "عرض الموبايل"), ("55–90 mm", "55–90 مم")),
            (("Remote", "الريموت"), ("Bluetooth 5.0, CR1632 battery", "بلوتوث 5.0، بطارية CR1632")),
            (("Weight", "الوزن"), ("180 g", "180 جرام")),
        ],
        variants=[dict(opts=[], price=899, image=0)],
        rating=4.6, reviews=1874,
        alt=("UGREEN selfie stick tripod", "عصا سيلفي وحامل ثلاثي من يوجرين"),
        views=[
            ("standing as a tripod", "مفتوحة كحامل ثلاثي"),
            ("angled view", "بزاوية جانبية"),
        ],
    ),
    dict(
        id=1211,
        name=("10-inch LED Ring Light with Tripod Stand", "حلقة إضاءة LED مقاس 10 بوصات مع حامل ثلاثي"),
        subtitle=("3 light modes · stand up to 160 cm", "إضاءة حلوة لصورك ولايفاتك من أي زاوية"),
        description=(
            "A 10-inch dimmable LED ring with warm, neutral and cool modes and 10 brightness levels, powered from "
            "any USB port or power bank. The tripod stand adjusts from 50 to 160 cm and the phone holder rotates for "
            "portrait or landscape — made for TikToks, live streams and video calls.",
            "حلقة إضاءة LED مقاس 10 بوصات بثلاثة أوضاع للإضاءة (دافئة ومحايدة وباردة) و10 درجات للسطوع، تعمل "
            "من أي منفذ USB أو باور بانك. الحامل الثلاثي يتراوح ارتفاعه بين 50 و160 سم، وماسك الموبايل يدور "
            "للتصوير الطولي أو العرضي، مثالية لتيك توك والبث المباشر ومكالمات الفيديو.",
        ),
        highlights=[
            ("3 colour temperatures × 10 brightness levels", "3 درجات لون للإضاءة و10 مستويات سطوع"),
            ("Stand adjusts from 50 to 160 cm", "ارتفاع الحامل من 50 إلى 160 سم"),
            ("USB powered — works from a power bank", "يعمل بمنفذ USB حتى من الباور بانك"),
        ],
        specs=[
            (("Ring size", "مقاس الحلقة"), ("10 inches (26 cm)", "10 بوصات (26 سم)")),
            (("Light modes", "أوضاع الإضاءة"), ("Warm, neutral, cool (3000–6000 K)", "دافئ ومحايد وبارد (3000–6000 كلفن)")),
            (("Power", "الطاقة"), ("USB 5 V, 10 W", "USB بجهد 5 فولت، 10 واط")),
            (("Stand height", "ارتفاع الحامل"), ("50–160 cm", "50–160 سم")),
        ],
        variants=[dict(opts=[], price=749, image=0)],
        rating=4.3, reviews=528,
        alt=("10-inch LED ring light on a tripod stand", "حلقة إضاءة LED مقاس 10 بوصات على حامل ثلاثي"),
        views=[],
    ),
    dict(
        id=1212,
        name=("Baseus Extendable Selfie Stick", "عصا سيلفي باسيوس قابلة للتطويل"),
        subtitle=("Pocket-size · extends to 80 cm", "صغيرة تدخل في الجيب وتطول لحد 80 سم"),
        description=(
            "A pocket-size aluminium selfie stick that extends from 19 to 80 cm, with a padded clamp for phones "
            "56–85 mm wide and a head that tilts through 270°. It pairs over Bluetooth, so the shutter button in the "
            "handle works with any camera app.",
            "عصا سيلفي صغيرة من الألومنيوم تمتد من 19 إلى 80 سم، بماسك مبطّن يناسب الموبايلات بعرض 56 إلى 85 مم "
            "ورأس يميل 270 درجة. تتصل بالبلوتوث، فيعمل زر التصوير الموجود في المقبض مع أي تطبيق كاميرا.",
        ),
        highlights=[
            ("Extends from 19 to 80 cm", "تمتد من 19 إلى 80 سم"),
            ("Bluetooth shutter button in the handle", "زر تصوير بلوتوث في المقبض"),
            ("270° tilting head", "رأس يميل 270 درجة"),
        ],
        specs=[
            (("Length", "الطول"), ("19–80 cm", "19–80 سم")),
            (("Phone width", "عرض الموبايل"), ("56–85 mm", "56–85 مم")),
            (("Material", "الخامة"), ("Aluminium alloy", "سبيكة ألومنيوم")),
            (("Weight", "الوزن"), ("125 g", "125 جرام")),
        ],
        variants=[dict(opts=[], price=499, image=0)],
        rating=4.2, reviews=241,
        alt=("Baseus selfie stick holding a phone", "عصا سيلفي باسيوس تمسك موبايل"),
        views=[],
    ),
    dict(
        id=1213,
        name=("Professional Video Tripod with Dolly", "حامل كاميرا فيديو احترافي ثلاثي بعجلات"),
        subtitle=("Fluid head · 12 kg payload · wheeled dolly", "حركة ناعمة للكاميرا جوه الاستوديو"),
        description=(
            "A studio-grade aluminium tripod with a fluid pan-and-tilt head, telescopic pan bar and 75 mm bowl, "
            "rated for cameras up to 12 kg. The three-wheel dolly with brakes gives smooth moves on studio floors, "
            "and the height adjusts from 75 to 180 cm.",
            "حامل ثلاثي احترافي من الألومنيوم برأس هيدروليكي للحركة الأفقية والرأسية وذراع تحكم قابلة للتطويل "
            "وقاعدة 75 مم، ويحمل كاميرات حتى 12 كجم. العجلات الثلاث بفرامل تتيح حركة سلسة على أرضية الاستوديو، "
            "والارتفاع قابل للضبط من 75 إلى 180 سم.",
        ),
        highlights=[
            ("Fluid pan-and-tilt head with pan bar", "رأس هيدروليكي للحركة الأفقية والرأسية مع ذراع تحكم"),
            ("Carries cameras up to 12 kg", "يحمل كاميرات حتى 12 كجم"),
            ("Three-wheel dolly with brakes", "قاعدة بثلاث عجلات بفرامل"),
        ],
        specs=[
            (("Payload", "الحمولة القصوى"), ("12 kg", "12 كجم")),
            (("Height", "الارتفاع"), ("75–180 cm", "75–180 سم")),
            (("Head", "الرأس"), ("Fluid head, 75 mm bowl", "رأس هيدروليكي بقاعدة 75 مم")),
            (("Legs", "الأرجل"), ("Aluminium with mid-level spreader", "ألومنيوم مع مثبّت في المنتصف")),
            (("Weight", "الوزن"), ("8.6 kg with dolly", "8.6 كجم مع العجلات")),
        ],
        variants=[dict(opts=[], price=12999, image=0)],
        rating=4.7, reviews=12,
        alt=("Video tripod with fluid head and wheeled dolly", "حامل فيديو برأس هيدروليكي وقاعدة بعجلات"),
        views=[],
    ),

    # ------------------------------------------------------------------ Watches
    dict(
        id=1401,
        name=("Longines Master Collection Moonphase Chronograph — 18K Yellow Gold",
              "ساعة لونجين ماستر كولكشن كرونوغراف بطور القمر — ذهب أصفر عيار 18"),
        subtitle=("Triple calendar · moon phase · 18K gold", "تحفة سويسرية بالدهب الأصفر وطور القمر"),
        description=(
            "Longines' classic triple-calendar chronograph in 18K yellow gold: day and month windows, pointer date, "
            "moon phase at 6 o'clock and a 12-hour chronograph, driven by the automatic column-wheel calibre L687 "
            "with a 54-hour power reserve. Silvered barleycorn dial with blued hands and painted Arabic numerals, on "
            "a brown alligator strap.",
            "كرونوغراف لونجين الكلاسيكي بتقويم ثلاثي في علبة من الذهب الأصفر عيار 18: نافذتان لليوم والشهر، "
            "وعقرب للتاريخ، وطور القمر عند الساعة 6، وكرونوغراف 12 ساعة، تحركها آلية L687 الأوتوماتيكية "
            "باحتياطي طاقة 54 ساعة. ميناء فضي بنقش حبات الشعير وعقارب زرقاء وأرقام مطبوعة، على سوار من جلد "
            "التمساح البني.",
        ),
        highlights=[
            ("Triple calendar with moon phase", "تقويم ثلاثي مع طور القمر"),
            ("L687 automatic chronograph, 54-hour reserve", "كرونوغراف أوتوماتيكي L687 باحتياطي 54 ساعة"),
            ("40 mm case in 18K yellow gold", "علبة 40 مم من الذهب الأصفر عيار 18"),
        ],
        specs=[
            (("Case", "العلبة"), ("40 mm, 18K yellow gold", "40 مم، ذهب أصفر عيار 18")),
            (("Movement", "الحركة"), ("Automatic calibre L687, 54 h reserve", "أوتوماتيكية عيار L687، احتياطي 54 ساعة")),
            (("Functions", "الوظائف"), ("Chronograph, day, date, month, moon phase", "كرونوغراف، اليوم، التاريخ، الشهر، طور القمر")),
            (("Crystal", "الزجاج"), ("Sapphire, anti-reflective", "سافير مضاد للانعكاس")),
            (("Strap", "السوار"), ("Brown alligator leather", "جلد تمساح بني")),
            (("Water resistance", "مقاومة الماء"), ("30 m", "30 متر")),
        ],
        variants=[dict(opts=[], price=789000, image=0)],
        rating=4.9, reviews=3,
        alt=("Longines Master Collection moonphase chronograph in yellow gold",
             "ساعة لونجين ماستر كولكشن بطور القمر من الذهب الأصفر"),
        views=[
            ("dial", "الميناء"),
            ("angled, showing the buckle", "بزاوية تُظهر الإبزيم"),
            ("case profile", "جانب العلبة"),
        ],
    ),
    dict(
        id=1402,
        name=("Longines Master Collection Moonphase Chronograph — Steel",
              "ساعة لونجين ماستر كولكشن كرونوغراف بطور القمر — ستانلس ستيل"),
        subtitle=("Triple calendar · moon phase · automatic", "كرونوغراف سويسري كلاسيكي بطور القمر"),
        description=(
            "The celebrated Master Collection complication in a 40 mm stainless-steel case on a matching steel "
            "bracelet: triple calendar, moon phase and 12-hour chronograph powered by the automatic calibre L687 "
            "(54-hour reserve). The black dial with silver-toned hands reads crisply in any light.",
            "نفس كرونوغراف ماستر كولكشن الشهير في علبة ستانلس ستيل 40 مم مع سوار معدني متناسق: تقويم ثلاثي "
            "وطور القمر وكرونوغراف 12 ساعة بآلية L687 الأوتوماتيكية واحتياطي طاقة 54 ساعة. الميناء الأسود "
            "بعقارب فضية واضح القراءة في أي إضاءة.",
        ),
        highlights=[
            ("Triple calendar with moon phase", "تقويم ثلاثي مع طور القمر"),
            ("Automatic L687 chronograph movement", "حركة كرونوغراف أوتوماتيكية L687"),
            ("Steel bracelet with folding clasp", "سوار ستيل بمشبك قابل للطي"),
        ],
        specs=[
            (("Case", "العلبة"), ("40 mm, stainless steel", "40 مم، ستانلس ستيل")),
            (("Movement", "الحركة"), ("Automatic calibre L687, 54 h reserve", "أوتوماتيكية عيار L687، احتياطي 54 ساعة")),
            (("Dial", "الميناء"), ("Black, silver-toned hands", "أسود بعقارب فضية")),
            (("Crystal", "الزجاج"), ("Sapphire, anti-reflective", "سافير مضاد للانعكاس")),
            (("Bracelet", "السوار"), ("Stainless steel, folding clasp", "ستانلس ستيل بمشبك قابل للطي")),
            (("Water resistance", "مقاومة الماء"), ("30 m", "30 متر")),
        ],
        variants=[dict(opts=[], price=214900, image=0)],
        rating=4.8, reviews=9,
        alt=("Longines Master Collection moonphase chronograph in steel with black dial",
             "ساعة لونجين ماستر كولكشن ستيل بميناء أسود"),
        views=[
            ("dial", "الميناء"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
            ("case and bracelet profile", "جانب العلبة والسوار"),
        ],
    ),
    dict(
        id=1403,
        name=("Rolex Cellini Date 39 mm — Pre-owned", "ساعة رولكس تشيليني ديت 39 مم — مستعملة"),
        subtitle=("2019 · box & papers · authenticated", "مستعملة بحالة ممتازة وبالعلبة والورق"),
        description=(
            "A 2019 Rolex Cellini Date (ref. 50515) in 18 ct Everose gold, with the date shown by a hand on the "
            "sub-dial at 3 o'clock and a black guilloché dial. Pre-owned in excellent condition with original box "
            "and papers, authenticated and serviced by Nile's Swiss-trained watch partner and backed by a 12-month "
            "warranty.",
            "رولكس تشيليني ديت موديل 2019 (المرجع 50515) من ذهب إيفروز عيار 18، يظهر فيها التاريخ بعقرب على "
            "الميناء الفرعي عند الساعة 3، بميناء أسود منقوش. مستعملة بحالة ممتازة مع العلبة والأوراق الأصلية، "
            "وتم فحصها وصيانتها لدى شريك نايل المتخصص في الساعات السويسرية، مع ضمان 12 شهرًا.",
        ),
        highlights=[
            ("2019, original box and papers", "موديل 2019 بالعلبة والأوراق الأصلية"),
            ("Authenticated and serviced, 12-month warranty", "مفحوصة ومُصانة مع ضمان 12 شهرًا"),
            ("18 ct Everose gold, black guilloché dial", "ذهب إيفروز عيار 18 بميناء أسود منقوش"),
        ],
        specs=[
            (("Reference", "المرجع"), ("50515", "50515")),
            (("Case", "العلبة"), ("39 mm, 18 ct Everose gold", "39 مم، ذهب إيفروز عيار 18")),
            (("Movement", "الحركة"), ("Calibre 3165, automatic, 48 h reserve", "عيار 3165 أوتوماتيكي، احتياطي 48 ساعة")),
            (("Condition", "الحالة"), ("Pre-owned, excellent (2019)", "مستعملة، حالة ممتازة (2019)")),
            (("Water resistance", "مقاومة الماء"), ("50 m", "50 متر")),
        ],
        variants=[dict(opts=[], price=689000, image=0)],
        rating=4.8, reviews=2,
        alt=("Rolex Cellini Date in Everose gold with black dial", "ساعة رولكس تشيليني ديت من ذهب إيفروز بميناء أسود"),
        views=[
            ("dial", "الميناء"),
            ("angled view", "بزاوية جانبية"),
            ("case profile and strap", "جانب العلبة والسوار"),
        ],
    ),
    dict(
        id=1404,
        name=("Rolex Cellini Moonphase 39 mm — Everose Gold", "ساعة رولكس تشيليني بطور القمر 39 مم — ذهب إيفروز"),
        subtitle=("Unworn · Everose gold · blue enamel moon", "قمر أزرق على ميناء أبيض، أناقة رولكس الهادية"),
        description=(
            "Rolex's dress watch with a moon phase: a white lacquer dial, a blue enamel moon disc set with a "
            "meteorite full moon, and the date read by a central hand around the edge. 39 mm 18 ct Everose case on "
            "a brown alligator strap, powered by the in-house calibre 3195 (48-hour reserve). Unworn, with the full "
            "set and Rolex warranty card.",
            "ساعة رولكس الرسمية بطور القمر: ميناء أبيض مطلي باللك، وقرص قمر من المينا الزرقاء يتوسطه بدر من حجر "
            "نيزكي، وتاريخ يُقرأ بعقرب مركزي على حافة الميناء. علبة 39 مم من ذهب إيفروز عيار 18 على سوار من جلد "
            "التمساح البني، بآلية رولكس 3195 واحتياطي طاقة 48 ساعة. جديدة لم تُلبس، مع العلبة الكاملة وبطاقة "
            "ضمان رولكس.",
        ),
        highlights=[
            ("Unworn, full set with warranty card", "جديدة لم تُلبس بالعلبة الكاملة وبطاقة الضمان"),
            ("Blue enamel moon disc with meteorite full moon", "قرص قمر من المينا الزرقاء ببدر من حجر نيزكي"),
            ("Calibre 3195, 48-hour power reserve", "آلية 3195 باحتياطي طاقة 48 ساعة"),
        ],
        specs=[
            (("Reference", "المرجع"), ("50535", "50535")),
            (("Case", "العلبة"), ("39 mm, 18 ct Everose gold", "39 مم، ذهب إيفروز عيار 18")),
            (("Movement", "الحركة"), ("Calibre 3195, automatic", "عيار 3195 أوتوماتيكي")),
            (("Dial", "الميناء"), ("White lacquer, blue enamel moon disc", "أبيض مطلي باللك مع قرص قمر أزرق")),
            (("Strap", "السوار"), ("Brown alligator, Everose buckle", "جلد تمساح بني بإبزيم إيفروز")),
            (("Water resistance", "مقاومة الماء"), ("50 m", "50 متر")),
        ],
        variants=[dict(opts=[], price=1349000, image=0)],
        modifier_groups=[
            dict(id="delivery", name=("Delivery", "التوصيل"), selection="single", min=0, max=1, required=False,
                 options=[dict(id="white-glove", name=("Insured white-glove delivery", "توصيل مؤمَّن بخدمة خاصة"),
                               price=1500)]),
        ],
        rating=4.9, reviews=4,
        alt=("Rolex Cellini Moonphase in Everose gold", "ساعة رولكس تشيليني بطور القمر من ذهب إيفروز"),
        views=[
            ("dial with the blue moon disc", "الميناء وقرص القمر الأزرق"),
            ("angled view", "بزاوية جانبية"),
            ("case profile and strap", "جانب العلبة والسوار"),
        ],
    ),
    dict(
        id=1405,
        name=("Rolex Datejust 41 — Oystersteel & Yellow Gold", "ساعة رولكس ديت جاست 41 — ستيل وذهب أصفر"),
        subtitle=("Jubilee bracelet · slate Roman dial · 41 mm", "الأيقونة الكلاسيكية بالستيل والدهب"),
        description=(
            "The Datejust 41 in Rolesor — Oystersteel with an 18 ct yellow-gold fluted bezel, crown and Jubilee "
            "centre links — with a slate dial and Roman numerals. Calibre 3235 gives a 70-hour power reserve, the "
            "Oyster case is waterproof to 100 m, and the Cyclops lens magnifies the date at 3 o'clock.",
            "ديت جاست 41 بخامة الرولسور: ستيل أويستر مع إطار مضلع وتاج وحلقات وسطى من الذهب الأصفر عيار 18 في "
            "سوار جوبيلي، بميناء رمادي داكن وأرقام رومانية. آلية 3235 باحتياطي طاقة 70 ساعة، وعلبة أويستر مقاومة "
            "للماء حتى 100 متر، والتاريخ مكبَّر بعدسة سايكلوبس عند الساعة 3.",
        ),
        highlights=[
            ("Rolesor: Oystersteel and 18 ct yellow gold", "رولسور: ستيل أويستر وذهب أصفر عيار 18"),
            ("Calibre 3235, 70-hour power reserve", "آلية 3235 باحتياطي طاقة 70 ساعة"),
            ("Waterproof to 100 m", "مقاومة للماء حتى 100 متر"),
        ],
        specs=[
            (("Reference", "المرجع"), ("126333", "126333")),
            (("Case", "العلبة"), ("41 mm Oyster, Oystersteel and yellow gold", "أويستر 41 مم، ستيل وذهب أصفر")),
            (("Dial", "الميناء"), ("Slate, Roman numerals", "رمادي داكن بأرقام رومانية")),
            (("Bracelet", "السوار"), ("Jubilee, Oysterclasp", "جوبيلي بمشبك Oysterclasp")),
            (("Movement", "الحركة"), ("Calibre 3235, automatic, 70 h", "عيار 3235 أوتوماتيكي، 70 ساعة")),
            (("Water resistance", "مقاومة الماء"), ("100 m", "100 متر")),
        ],
        variants=[dict(opts=[], price=789000, image=0)],
        rating=4.9, reviews=7,
        alt=("Rolex Datejust 41 two-tone with Jubilee bracelet", "ساعة رولكس ديت جاست 41 بلونين بسوار جوبيلي"),
        views=[
            ("dial", "الميناء"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
            ("case profile", "جانب العلبة"),
        ],
    ),
    dict(
        id=1406,
        name=("Rolex Submariner Date 41 — Oystersteel & Yellow Gold", "ساعة رولكس صب مارينر ديت 41 — ستيل وذهب أصفر"),
        subtitle=("Dive icon · 300 m · Cerachrom bezel", "ساعة الغطس الأشهر بالستيل والدهب"),
        description=(
            "The two-tone Submariner Date (ref. 126613LN) pairs Oystersteel with 18 ct yellow gold, a black dial and "
            "a black Cerachrom ceramic bezel with gold-filled numerals. 41 mm Oyster case waterproof to 300 m, "
            "calibre 3235 with a 70-hour reserve, and the Glidelock clasp for quick bracelet adjustment. Sold out "
            "for now — ask our concierge to add you to the waiting list.",
            "صب مارينر ديت بلونين (المرجع 126613LN) يجمع ستيل أويستر مع الذهب الأصفر عيار 18، بميناء أسود وإطار "
            "سيراكروم خزفي أسود بأرقام مطلية بالذهب. علبة أويستر 41 مم مقاومة للماء حتى 300 متر، وآلية 3235 "
            "باحتياطي طاقة 70 ساعة، ومشبك جلايدلوك لضبط طول السوار بسهولة. نفدت الكمية حاليًا، ويمكنك التسجيل "
            "في قائمة الانتظار لنتواصل معك فور وصولها.",
        ),
        highlights=[
            ("Waterproof to 300 m", "مقاومة للماء حتى 300 متر"),
            ("Black Cerachrom bezel with gold numerals", "إطار سيراكروم أسود بأرقام ذهبية"),
            ("Glidelock clasp for quick sizing", "مشبك جلايدلوك لضبط المقاس بسرعة"),
        ],
        specs=[
            (("Reference", "المرجع"), ("126613LN", "126613LN")),
            (("Case", "العلبة"), ("41 mm Oyster, Oystersteel and yellow gold", "أويستر 41 مم، ستيل وذهب أصفر")),
            (("Movement", "الحركة"), ("Calibre 3235, automatic, 70 h", "عيار 3235 أوتوماتيكي، 70 ساعة")),
            (("Bezel", "الإطار"), ("Unidirectional, black Cerachrom", "دوّار باتجاه واحد، سيراكروم أسود")),
            (("Water resistance", "مقاومة الماء"), ("300 m", "300 متر")),
        ],
        variants=[dict(opts=[], price=829000, image=0)],
        rating=4.9, reviews=5,
        alt=("Rolex Submariner Date two-tone with black bezel", "ساعة رولكس صب مارينر ديت بلونين بإطار أسود"),
        views=[
            ("dial and bezel", "الميناء والإطار"),
            ("side with the Glidelock clasp", "الجانب مع مشبك جلايدلوك"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
        ],
    ),
    dict(
        id=1407,
        name=("Rolex Cellini Moonphase 39 mm — Pre-owned", "ساعة رولكس تشيليني بطور القمر 39 مم — مستعملة"),
        subtitle=("2021 · full set · authenticated", "مستعملة نضيفة بالعلبة والورق ومتفحوصة"),
        description=(
            "A 2021 Rolex Cellini Moonphase (ref. 50535) in 18 ct Everose gold with the white lacquer dial and blue "
            "enamel moon disc. Pre-owned with light signs of wear and sold with the full set, it has been "
            "authenticated and serviced by Nile's watch partner and carries a 12-month warranty.",
            "رولكس تشيليني بطور القمر موديل 2021 (المرجع 50535) من ذهب إيفروز عيار 18، بالميناء الأبيض المطلي "
            "وقرص القمر من المينا الزرقاء. مستعملة بآثار استخدام خفيفة وتأتي بالعلبة والأوراق كاملة، وتم فحصها "
            "وصيانتها لدى شريك نايل للساعات، مع ضمان 12 شهرًا.",
        ),
        highlights=[
            ("2021, full set", "موديل 2021 بالعلبة والأوراق كاملة"),
            ("Authenticated and serviced, 12-month warranty", "مفحوصة ومُصانة مع ضمان 12 شهرًا"),
            ("Everose gold with a blue enamel moon", "ذهب إيفروز مع قمر من المينا الزرقاء"),
        ],
        specs=[
            (("Reference", "المرجع"), ("50535", "50535")),
            (("Case", "العلبة"), ("39 mm, 18 ct Everose gold", "39 مم، ذهب إيفروز عيار 18")),
            (("Movement", "الحركة"), ("Calibre 3195, automatic, 48 h", "عيار 3195 أوتوماتيكي، 48 ساعة")),
            (("Condition", "الحالة"), ("Pre-owned, very good (2021)", "مستعملة، حالة جيدة جدًا (2021)")),
            (("Water resistance", "مقاومة الماء"), ("50 m", "50 متر")),
        ],
        variants=[dict(opts=[], price=1049000, image=0)],
        rating=4.8, reviews=2,
        alt=("Rolex Cellini Moonphase in Everose gold", "ساعة رولكس تشيليني بطور القمر من ذهب إيفروز"),
        views=[
            ("dial with the blue moon disc", "الميناء وقرص القمر الأزرق"),
            ("angled view", "بزاوية جانبية"),
            ("case profile and strap", "جانب العلبة والسوار"),
        ],
    ),
    dict(
        id=1408,
        name=("Rolex Datejust 36 — Oystersteel & Yellow Gold", "ساعة رولكس ديت جاست 36 — ستيل وذهب أصفر"),
        subtitle=("Classic 36 mm · Jubilee · slate Roman dial", "ديت جاست بمقاس 36 مم المريح على المعصم"),
        description=(
            "The original Datejust size: a 36 mm Oyster case in Rolesor — Oystersteel and 18 ct yellow gold — with "
            "a fluted bezel, slate Roman-numeral dial and five-link Jubilee bracelet. Calibre 3235 (70-hour "
            "reserve) and waterproof to 100 m; it sits beautifully on smaller wrists.",
            "المقاس الأصلي لساعة ديت جاست: علبة أويستر 36 مم بخامة الرولسور من ستيل أويستر والذهب الأصفر عيار 18، "
            "بإطار مضلع وميناء رمادي داكن بأرقام رومانية وسوار جوبيلي بخمس حلقات. آلية 3235 باحتياطي طاقة 70 "
            "ساعة ومقاومة للماء حتى 100 متر، ومثالية للمعصم الأنحف.",
        ),
        highlights=[
            ("Classic 36 mm size", "المقاس الكلاسيكي 36 مم"),
            ("Rolesor with a fluted gold bezel", "رولسور بإطار ذهبي مضلع"),
            ("Calibre 3235, 70-hour reserve", "آلية 3235 باحتياطي 70 ساعة"),
        ],
        specs=[
            (("Reference", "المرجع"), ("126233", "126233")),
            (("Case", "العلبة"), ("36 mm Oyster, Oystersteel and yellow gold", "أويستر 36 مم، ستيل وذهب أصفر")),
            (("Dial", "الميناء"), ("Slate, Roman numerals", "رمادي داكن بأرقام رومانية")),
            (("Bracelet", "السوار"), ("Jubilee, Crownclasp", "جوبيلي بمشبك Crownclasp")),
            (("Movement", "الحركة"), ("Calibre 3235, automatic, 70 h", "عيار 3235 أوتوماتيكي، 70 ساعة")),
            (("Water resistance", "مقاومة الماء"), ("100 m", "100 متر")),
        ],
        variants=[dict(opts=[], price=639000, image=0)],
        rating=4.9, reviews=4,
        alt=("Rolex Datejust 36 two-tone with Jubilee bracelet", "ساعة رولكس ديت جاست 36 بلونين بسوار جوبيلي"),
        views=[
            ("dial", "الميناء"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
            ("case profile", "جانب العلبة"),
        ],
    ),
    dict(
        id=1409,
        name=("IWC Ingenieur Automatic 40 mm — Black Dial", "ساعة آي دبليو سي إنجينيور أوتوماتيك 40 مم — ميناء أسود"),
        subtitle=("Swiss automatic · 120 m · black dial", "ساعة المهندسين السويسرية، عملية وشيك"),
        description=(
            "IWC's engineer's watch in a 40 mm brushed-and-polished steel case on an integrated steel bracelet, with "
            "a clean black dial and luminous hands and indices. The self-winding calibre 35111 runs for 42 hours, "
            "and the case is water-resistant to 120 m.",
            "ساعة المهندسين من آي دبليو سي في علبة ستيل 40 مم بلمسات مصقولة ومطفية وسوار معدني متصل بالعلبة، "
            "بميناء أسود بسيط وعقارب ومؤشرات مضيئة. آلية 35111 الأوتوماتيكية تعمل 42 ساعة، والعلبة مقاومة للماء "
            "حتى 120 مترًا.",
        ),
        highlights=[
            ("Swiss automatic, 42-hour reserve", "حركة سويسرية أوتوماتيكية باحتياطي 42 ساعة"),
            ("Water-resistant to 120 m", "مقاومة للماء حتى 120 مترًا"),
            ("Integrated steel bracelet", "سوار ستيل متصل بالعلبة"),
        ],
        specs=[
            (("Case", "العلبة"), ("40 mm stainless steel", "ستانلس ستيل 40 مم")),
            (("Movement", "الحركة"), ("Calibre 35111, automatic, 42 h", "عيار 35111 أوتوماتيكي، 42 ساعة")),
            (("Dial", "الميناء"), ("Black, luminous hands", "أسود بعقارب مضيئة")),
            (("Crystal", "الزجاج"), ("Sapphire", "سافير")),
            (("Water resistance", "مقاومة الماء"), ("120 m", "120 متر")),
        ],
        variants=[dict(opts=[], price=459000, image=0)],
        rating=4.7, reviews=3,
        alt=("IWC Ingenieur Automatic in steel with black dial", "ساعة آي دبليو سي إنجينيور أوتوماتيك ستيل بميناء أسود"),
        views=[
            ("dial", "الميناء"),
            ("angled view", "بزاوية جانبية"),
            ("bracelet profile", "جانب السوار"),
        ],
    ),
    dict(
        id=1410,
        name=("Daniel Klein Premium Women's Watch — Gold-tone, Navy Dial",
              "ساعة دانيال كلاين بريميوم النسائية — ذهبية بميناء كحلي"),
        subtitle=("Gold-tone bracelet · navy Roman dial", "لمسة دهب على إيدك بسعر حلو"),
        description=(
            "A dressy women's watch with a gold-tone stainless-steel case and link bracelet, a navy sunray dial with "
            "Roman numerals and a red seconds hand. Japanese quartz movement, mineral glass and 3 ATM splash "
            "resistance — it arrives gift-boxed.",
            "ساعة نسائية أنيقة بعلبة وسوار من الستانلس ستيل بلون ذهبي، وميناء كحلي لامع بأرقام رومانية وعقرب "
            "ثوانٍ أحمر. آلية كوارتز يابانية وزجاج معدني ومقاومة لرذاذ الماء 3 ATM، وتصلك في علبة هدايا.",
        ),
        highlights=[
            ("Navy sunray dial with Roman numerals", "ميناء كحلي لامع بأرقام رومانية"),
            ("Japanese quartz movement", "آلية كوارتز يابانية"),
            ("Gift box included", "علبة هدايا مرفقة"),
        ],
        specs=[
            (("Case", "العلبة"), ("34 mm, gold-tone stainless steel", "34 مم، ستانلس ستيل بلون ذهبي")),
            (("Movement", "الحركة"), ("Japanese quartz", "كوارتز ياباني")),
            (("Glass", "الزجاج"), ("Mineral", "معدني")),
            (("Water resistance", "مقاومة الماء"), ("3 ATM (splash resistant)", "3 ATM (مقاومة للرذاذ)")),
            (("Bracelet", "السوار"), ("Gold-tone steel links", "حلقات ستيل بلون ذهبي")),
        ],
        variants=[dict(opts=[], price=1899, compare_at=2499, image=0)],
        rating=4.3, reviews=341,
        alt=("Daniel Klein gold-tone women's watch with navy dial", "ساعة دانيال كلاين النسائية الذهبية بميناء كحلي"),
        views=[
            ("dial", "الميناء"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
            ("case profile", "جانب العلبة"),
        ],
    ),
    dict(
        id=1411,
        name=("Casio Enticer Analogue Watch — Steel, Black Dial", "ساعة كاسيو إنتايسر أنالوج — ستانلس ستيل بميناء أسود"),
        subtitle=("Everyday steel watch · 50 m water resistant", "ساعة يومية بسيطة تستحمل مشاويرك"),
        description=(
            "A no-fuss steel analogue watch from Casio's Enticer line: black dial with baton indices, mineral glass "
            "and a stainless-steel bracelet with a fold-over clasp. Quartz accuracy, about three years on one "
            "battery and 50 m water resistance for everyday wear.",
            "ساعة كاسيو أنالوج بسيطة من مجموعة إنتايسر: ميناء أسود بمؤشرات عصوية، وزجاج معدني، وسوار ستانلس "
            "ستيل بمشبك قابل للطي. دقة الكوارتز، وبطارية تدوم نحو 3 سنوات، ومقاومة للماء حتى 50 مترًا "
            "للاستخدام اليومي.",
        ),
        highlights=[
            ("50 m water resistance", "مقاومة للماء حتى 50 مترًا"),
            ("About 3 years of battery life", "بطارية تدوم نحو 3 سنوات"),
            ("Steel bracelet with fold-over clasp", "سوار ستيل بمشبك قابل للطي"),
        ],
        specs=[
            (("Case", "العلبة"), ("40 mm, stainless steel", "40 مم، ستانلس ستيل")),
            (("Movement", "الحركة"), ("Quartz", "كوارتز")),
            (("Glass", "الزجاج"), ("Mineral", "معدني")),
            (("Water resistance", "مقاومة الماء"), ("50 m", "50 متر")),
            (("Battery", "البطارية"), ("SR626SW, approx. 3 years", "SR626SW، نحو 3 سنوات")),
        ],
        variants=[dict(opts=[], price=2199, compare_at=2699, image=0)],
        rating=4.5, reviews=864,
        alt=("Casio Enticer steel watch with black dial", "ساعة كاسيو إنتايسر ستيل بميناء أسود"),
        views=[
            ("dial", "الميناء"),
            ("angled bracelet view", "السوار بزاوية جانبية"),
            ("case profile", "جانب العلبة"),
        ],
    ),
    dict(
        id=1412,
        name=("Apple Watch SE 3 (GPS) — Starlight Aluminium", "ساعة أبل SE 3 (GPS) — ألومنيوم بلون ضوء النجوم"),
        subtitle=("Always-on display · S10 chip · sleep score", "تابع صحتك ونومك وردّ على إشعاراتك من إيدك"),
        description=(
            "Apple's most affordable watch now has an always-on Retina display and the S10 chip, with double tap "
            "and wrist-flick gestures, sleep score and wrist-temperature sensing. Get heart-rate notifications, fall "
            "and crash detection, and up to 18 hours of battery that charges up to twice as fast. Starlight "
            "aluminium case with the Sport Loop pictured.",
            "أوفر ساعة من أبل أصبحت بشاشة Retina تعمل دائمًا وشريحة S10، مع إيماءة النقر المزدوج وتدوير المعصم، "
            "وتقييم النوم، واستشعار حرارة المعصم. تنبيهات لنبض القلب واكتشاف السقوط والحوادث، وبطارية حتى 18 "
            "ساعة تُشحن أسرع حتى الضعف. علبة ألومنيوم بلون ضوء النجوم مع سوار Sport Loop الظاهر في الصورة.",
        ),
        highlights=[
            ("Always-on Retina display and S10 chip", "شاشة Retina تعمل دائمًا وشريحة S10"),
            ("Sleep score and wrist-temperature sensing", "تقييم النوم واستشعار حرارة المعصم"),
            ("Fall and crash detection with Emergency SOS", "اكتشاف السقوط والحوادث مع خدمة الطوارئ SOS"),
            ("Water-resistant to 50 m", "مقاومة للماء حتى 50 مترًا"),
        ],
        specs=[
            (("Case", "العلبة"), ("40 or 44 mm aluminium, Starlight", "ألومنيوم 40 أو 44 مم بلون ضوء النجوم")),
            (("Chip", "الشريحة"), ("S10 SiP", "S10 SiP")),
            (("Display", "الشاشة"), ("Always-on Retina OLED", "Retina OLED تعمل دائمًا")),
            (("Battery", "البطارية"), ("Up to 18 h (32 h in Low Power Mode)", "حتى 18 ساعة (32 ساعة في وضع الطاقة المنخفضة)")),
            (("Band", "السوار"), ("Sport Loop, adjustable hook-and-loop", "Sport Loop بإغلاق لاصق قابل للضبط")),
            (("Water resistance", "مقاومة الماء"), ("50 m", "50 متر")),
        ],
        options=[
            dict(id="size", name=("Case size", "مقاس العلبة"), kind="chip", values=[
                dict(id="40mm", label=("40 mm", "40 مم"), code="40"),
                dict(id="44mm", label=("44 mm", "44 مم"), code="44"),
            ]),
        ],
        variants=[
            dict(opts=["40mm"], price=13499, image=0),
            dict(opts=["44mm"], price=14999, image=0),
        ],
        modifier_groups=[
            dict(id="band", name=("Add a second band", "أضف سوارًا إضافيًا"), selection="single", min=0, max=1,
                 required=False, options=[
                     dict(id="sport-band", name=("Sport Band", "سوار Sport Band"), price=2499),
                     dict(id="sport-loop", name=("Sport Loop", "سوار Sport Loop"), price=2499),
                 ]),
        ],
        rating=4.7, reviews=948,
        alt=("Apple Watch SE 3 in Starlight aluminium with Sport Loop",
             "ساعة أبل SE 3 بعلبة ألومنيوم بلون ضوء النجوم وسوار Sport Loop"),
        views=[
            ("front", "من الأمام"),
            ("angled, with the Sport Loop", "بزاوية مع السوار"),
            ("side with the Digital Crown", "الجانب مع التاج الرقمي"),
        ],
    ),
]

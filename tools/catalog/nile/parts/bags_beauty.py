"""Bags & Accessories (1701–1713) and Beauty & Fragrance (1801–1813)."""

FRONT = ("front", "من الأمام")
ANGLED = ("angled view", "بزاوية جانبية")
OTHER = ("angled from the other side", "من الزاوية المقابلة")
SIDE = ("side view", "من الجانب")

GIFT_PERFUME = dict(
    id="gift", name=("Gift options", "خيارات الهدية"), selection="multi", min=0, max=2, required=False,
    options=[
        dict(id="wrap", name=("Gift wrapping", "تغليف هدية"), price=75),
        dict(id="card", name=("Greeting card with your message", "بطاقة تهنئة برسالتك"), price=25),
    ],
)


def size(*ml):
    return dict(id="size", name=("Size", "الحجم"), kind="chip",
                values=[dict(id=f"{m}ml", label=(f"{m} ml", f"{m} مل")) for m in ml])


PRODUCTS = [
    # ------------------------------------------------------------------ Bags
    dict(
        id=1701,
        name=("Charles & Keith Double-Handle Tote Bag — Navy", "حقيبة توت بمقبضين من تشارلز آند كيث — كحلي"),
        subtitle=("Structured navy tote for work and weekends", "شنطة كحلي شيك تشيل كل حاجتك للشغل والخروج"),
        description=(
            "A clean, structured tote in deep navy with two rolled top handles and a roomy main compartment "
            "for everything your day needs. The smooth finish wipes clean, and the firm shape stays put even "
            "when the bag is full.",
            "حقيبة توت أنيقة بلون كحلي غامق، بمقبضين ملفوفين وجيب رئيسي واسع يستوعب أغراض يومك كلها. "
            "خامتها الناعمة سهلة التنظيف، وشكلها المتماسك يبقى ثابتًا حتى وهي ممتلئة.",
        ),
        highlights=[
            ("Holds its shape even when full", "تحافظ على شكلها حتى وهي ممتلئة"),
            ("Roomy main compartment with inner zip pocket", "جيب رئيسي واسع وجيب داخلي بسحّاب"),
            ("Two comfortable rolled top handles", "مقبضان ملفوفان مريحان في اليد"),
        ],
        specs=[
            (("Material", "الخامة"), ("Smooth faux leather (PU)", "جلد صناعي ناعم (PU)")),
            (("Colour", "اللون"), ("Navy", "كحلي")),
            (("Dimensions", "الأبعاد"), ("33 × 24 × 14 cm", "33 × 24 × 14 سم")),
            (("Handle drop", "ارتفاع المقبض"), ("12 cm", "12 سم")),
            (("Closure", "الإغلاق"), ("Magnetic snap", "قفل مغناطيسي")),
        ],
        variants=[dict(opts=[], price=4199)],
        rating=4.6, reviews=148,
        alt=("Charles & Keith double-handle tote bag in navy", "حقيبة توت كحلية بمقبضين من تشارلز آند كيث"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1702,
        name=("ALDO Leather Hobo Bag — Cognac", "حقيبة هوبو من الجلد الطبيعي من ألدو — كونياك"),
        subtitle=("Soft slouchy leather with a front zip pocket", "جلد طبيعي طري بلون الكونياك، يمشي مع كل لبس"),
        description=(
            "A relaxed hobo shape in soft cognac leather, finished with gold-tone hardware and a front zip pocket "
            "for your phone and keys. The rolled handle sits comfortably on the shoulder, and the leather only "
            "gets better with wear.",
            "حقيبة هوبو بتصميم مريح من الجلد الطبيعي الناعم بلون الكونياك، مزينة بإكسسوارات ذهبية وجيب أمامي "
            "بسحّاب للهاتف والمفاتيح. يستقر مقبضها الملفوف براحة على الكتف، ويزداد الجلد جمالًا مع الاستخدام.",
        ),
        highlights=[
            ("Genuine cowhide leather", "جلد بقري طبيعي"),
            ("Front zip pocket for quick-grab essentials", "جيب أمامي بسحّاب للأغراض السريعة"),
            ("Fits a 10-inch tablet", "تتسع لتابلت مقاس 10 بوصات"),
        ],
        specs=[
            (("Material", "الخامة"), ("Genuine leather, fabric lining", "جلد طبيعي وبطانة قماش")),
            (("Colour", "اللون"), ("Cognac", "كونياك")),
            (("Dimensions", "الأبعاد"), ("34 × 27 × 13 cm", "34 × 27 × 13 سم")),
            (("Handle drop", "ارتفاع المقبض"), ("22 cm", "22 سم")),
            (("Closure", "الإغلاق"), ("Top zip", "سحّاب علوي")),
        ],
        variants=[dict(opts=[], price=3799)],
        rating=4.4, reviews=96,
        alt=("ALDO cognac leather hobo bag", "حقيبة هوبو من الجلد بلون الكونياك من ألدو"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1703,
        name=("Prada Galleria Medium Saffiano Leather Bag — Celeste",
              "حقيبة برادا جاليريا المتوسطة من جلد سافيانو — أزرق سماوي"),
        subtitle=("The icon in crosshatch Saffiano leather", "الشنطة الأيقونية من برادا بجلد سافيانو المميز"),
        description=(
            "Prada's Galleria in the medium size, crafted from scratch-resistant Saffiano leather in a soft celeste "
            "blue. Twin rolled handles, gold-tone hardware, the enamel triangle logo and a detachable shoulder strap "
            "take it from the office to dinner.",
            "حقيبة جاليريا الشهيرة من برادا بالمقاس المتوسط، مصنوعة من جلد سافيانو المقاوم للخدوش بلون أزرق سماوي "
            "هادئ. تأتي بمقبضين ملفوفين وإكسسوارات ذهبية وشعار برادا المثلث، مع حزام كتف قابل للفصل يناسب العمل "
            "والسهرة.",
        ),
        highlights=[
            ("Scratch-resistant Saffiano calf leather", "جلد عجل سافيانو مقاوم للخدوش"),
            ("Three compartments with a central zip pocket", "ثلاثة أقسام مع جيب أوسط بسحّاب"),
            ("Authenticity card and dust bag included", "بطاقة أصالة وكيس حماية مرفقان"),
        ],
        specs=[
            (("Material", "الخامة"), ("Saffiano leather, leather lining", "جلد سافيانو وبطانة جلد")),
            (("Colour", "اللون"), ("Celeste (light blue)", "أزرق سماوي")),
            (("Size", "المقاس"), ("Medium — 33 × 23 × 15 cm", "متوسط — 33 × 23 × 15 سم")),
            (("Hardware", "الإكسسوارات المعدنية"), ("Gold-tone metal", "معدن بلون ذهبي")),
            (("Made in", "بلد الصنع"), ("Italy", "إيطاليا")),
        ],
        variants=[dict(opts=[], price=199000)],
        rating=4.9, reviews=7,
        alt=("Prada Galleria Saffiano leather bag in celeste blue", "حقيبة برادا جاليريا من جلد سافيانو باللون الأزرق السماوي"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1704,
        name=("DeFacto Faux-Leather Mini Backpack — White", "حقيبة ظهر صغيرة من الجلد الصناعي من ديفاكتو — أبيض"),
        subtitle=("A clean white mini backpack for uni and weekends", "شنطة ضهر صغيرة بيضا، خفيفة للجامعة والخروجات"),
        description=(
            "A compact backpack in smooth white faux leather with a fold-over flap, gold-tone bar detail and a "
            "drawstring top. Adjustable straps and a grab handle make it easy to carry, and the size keeps it light "
            "for everyday errands.",
            "حقيبة ظهر صغيرة من الجلد الصناعي الأبيض الناعم، بغطاء أمامي وتفصيلة معدنية ذهبية وإغلاق علوي برباط. "
            "أحزمتها قابلة للتعديل ولها مقبض علوي، ومقاسها العملي يجعلها خفيفة لمشاوير اليوم.",
        ),
        highlights=[
            ("Drawstring top under a magnetic flap", "إغلاق برباط تحت غطاء مغناطيسي"),
            ("Adjustable shoulder straps and top handle", "أحزمة كتف قابلة للتعديل ومقبض علوي"),
            ("Wipe-clean faux leather", "جلد صناعي يُنظَّف بسهولة"),
        ],
        specs=[
            (("Material", "الخامة"), ("Faux leather (PU), polyester lining", "جلد صناعي (PU) وبطانة بوليستر")),
            (("Colour", "اللون"), ("White", "أبيض")),
            (("Dimensions", "الأبعاد"), ("28 × 22 × 11 cm", "28 × 22 × 11 سم")),
            (("Pockets", "الجيوب"), ("1 inner slip pocket", "جيب داخلي واحد")),
        ],
        variants=[dict(opts=[], price=799, compare_at=1099)],
        rating=4.3, reviews=214,
        alt=("DeFacto white faux-leather mini backpack", "حقيبة ظهر صغيرة بيضاء من الجلد الصناعي من ديفاكتو"),
        views=[FRONT, ("back with straps", "من الخلف مع الأحزمة"), SIDE],
    ),
    dict(
        id=1705,
        name=("Max Structured Top-Handle Bag — Black", "حقيبة يد بمقبض علوي من ماكس — أسود"),
        subtitle=("Polished black satchel with a belted front", "شنطة إيد سودا كلاسيك بتفاصيل دهبي، تمشي مع أي لبس"),
        description=(
            "A structured satchel in black faux leather with a belted front strap, gold-tone studs and flared sides "
            "that open wide for easy access. Carry it by the double top handles or clip on the included crossbody "
            "strap.",
            "حقيبة يد متماسكة من الجلد الصناعي الأسود، بحزام أمامي وتفاصيل معدنية ذهبية وجوانب واسعة تسهّل الوصول "
            "إلى أغراضك. احمليها من المقبضين العلويين أو بالحزام الطويل المرفق.",
        ),
        highlights=[
            ("Double top handles plus detachable crossbody strap", "مقبضان علويان وحزام كروس قابل للفصل"),
            ("Zip pocket and two slip pockets inside", "جيب بسحّاب وجيبان مفتوحان من الداخل"),
            ("Gold-tone hardware", "إكسسوارات معدنية ذهبية"),
        ],
        specs=[
            (("Material", "الخامة"), ("Faux leather", "جلد صناعي")),
            (("Colour", "اللون"), ("Black", "أسود")),
            (("Dimensions", "الأبعاد"), ("30 × 23 × 15 cm", "30 × 23 × 15 سم")),
            (("Strap", "الحزام"), ("Detachable, adjustable up to 120 cm", "قابل للفصل والتعديل حتى 120 سم")),
        ],
        variants=[dict(opts=[], price=1299)],
        rating=4.4, reviews=187,
        alt=("Max black structured top-handle bag", "حقيبة يد سوداء بمقبض علوي من ماكس"),
        views=[FRONT, ANGLED, OTHER],
    ),
    # ------------------------------------------------------------------ Eyewear
    dict(
        id=1706,
        name=("Ray-Ban Clubmaster Classic RB3016 — Havana / G-15 Green",
              "نظارة شمسية راي بان كلوب ماستر كلاسيك RB3016 — هافانا / عدسات G-15 خضراء"),
        subtitle=("The browline that never goes out of style", "الشكل الكلاسيك اللي عمره ما يقدم"),
        description=(
            "The 1950s browline icon: a Havana acetate brow over gold-tone metal rims, fitted with Ray-Ban's G-15 "
            "green glass lenses for natural colours and full UV protection. Comes in the original Ray-Ban case with "
            "a cleaning cloth.",
            "أيقونة الخمسينيات بحاجب علوي من الأسيتات بلون هافانا وحواف معدنية ذهبية، مع عدسات G-15 الزجاجية "
            "الخضراء الشهيرة من راي بان التي تحافظ على الألوان الطبيعية وتحمي بالكامل من الأشعة فوق البنفسجية. "
            "تصل في علبة راي بان الأصلية مع قطعة تنظيف.",
        ),
        highlights=[
            ("G-15 crystal lenses, 100% UV protection", "عدسات G-15 كريستال بحماية كاملة من الأشعة فوق البنفسجية"),
            ("Acetate brow with gold-tone metal rims", "حاجب من الأسيتات مع حواف معدنية ذهبية"),
            ("Original case and cleaning cloth", "علبة أصلية وقطعة تنظيف"),
        ],
        specs=[
            (("Model", "الموديل"), ("RB3016 W0366", "RB3016 W0366")),
            (("Size", "المقاس"), ("51-21-145 mm", "51-21-145 مم")),
            (("Frame", "الإطار"), ("Acetate & metal, Havana/gold", "أسيتات ومعدن، هافانا وذهبي")),
            (("Lenses", "العدسات"), ("G-15 green glass, non-polarised", "زجاج G-15 أخضر، غير مستقطب")),
            (("UV protection", "الحماية من الأشعة"), ("UV400 (100%)", "UV400 (حماية 100%)")),
        ],
        variants=[dict(opts=[], price=9499)],
        rating=4.8, reviews=412,
        alt=("Ray-Ban Clubmaster Classic sunglasses, Havana with green lenses",
             "نظارة شمسية راي بان كلوب ماستر كلاسيك بلون هافانا وعدسات خضراء"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1707,
        name=("Ray-Ban Aviator Gradient RB3025 — Gold / Blue Gradient",
              "نظارة شمسية راي بان أفياتور RB3025 — ذهبي / عدسات زرقاء متدرجة"),
        subtitle=("Classic pilot shape, soft blue gradient", "الأفياتور الأصلية بعدسات زرقا متدرجة تليق على كل وش"),
        description=(
            "The original pilot sunglasses, with a thin gold-tone metal frame, double bridge and adjustable nose "
            "pads. Blue-to-clear gradient lenses are darker at the top to cut glare and lighter below, so reading "
            "your phone or the dashboard stays easy.",
            "نظارة الطيارين الأصلية بإطار معدني رفيع بلون ذهبي وجسر مزدوج ووسادات أنف قابلة للتعديل. عدساتها "
            "المتدرجة من الأزرق إلى الشفاف أغمق من أعلى لتقليل الوهج وأفتح من أسفل، فتبقى قراءة الهاتف أو تابلوه "
            "السيارة سهلة.",
        ),
        highlights=[
            ("Gradient lenses with 100% UV protection", "عدسات متدرجة بحماية كاملة من الأشعة فوق البنفسجية"),
            ("Light metal frame with adjustable nose pads", "إطار معدني خفيف بوسادات أنف قابلة للتعديل"),
            ("Original case and cleaning cloth", "علبة أصلية وقطعة تنظيف"),
        ],
        specs=[
            (("Model", "الموديل"), ("RB3025", "RB3025")),
            (("Size", "المقاس"), ("58-14-135 mm", "58-14-135 مم")),
            (("Frame", "الإطار"), ("Metal, gold-tone", "معدن بلون ذهبي")),
            (("Lenses", "العدسات"), ("Blue gradient, non-polarised", "أزرق متدرج، غير مستقطب")),
            (("UV protection", "الحماية من الأشعة"), ("UV400 (100%)", "UV400 (حماية 100%)")),
        ],
        variants=[dict(opts=[], price=9999)],
        rating=4.7, reviews=365,
        alt=("Ray-Ban Aviator sunglasses, gold frame with blue gradient lenses",
             "نظارة شمسية راي بان أفياتور بإطار ذهبي وعدسات زرقاء متدرجة"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1708,
        name=("Polaroid Polarized Pilot Sunglasses — Black / Green",
              "نظارة شمسية بولارويد بايلوت مستقطبة — أسود / عدسات خضراء"),
        subtitle=("Polarised lenses that cut road and sea glare", "عدسات بولارايزد تريّح عينك من لمعة الطريق والبحر"),
        description=(
            "A modern take on the pilot shape with a slim black metal frame, flat double bridge and geometric green "
            "lenses. Polaroid's polarised lenses block reflected glare from the road, sea and sand — made for the "
            "Sahel drive.",
            "لمسة عصرية على شكل نظارة الطيار بإطار معدني أسود رفيع وجسر مزدوج مستقيم وعدسات خضراء هندسية. عدسات "
            "بولارويد المستقطبة تحجب الوهج المنعكس من الطريق والبحر والرمال، ومثالية لمشوار الساحل.",
        ),
        highlights=[
            ("Polarised lenses, UV400", "عدسات مستقطبة بحماية UV400"),
            ("Lightweight stainless-steel frame", "إطار خفيف من الستانلس ستيل"),
            ("Case and microfibre cloth included", "علبة وقطعة مايكروفايبر مرفقتان"),
        ],
        specs=[
            (("Frame", "الإطار"), ("Stainless steel, matte black", "ستانلس ستيل أسود مطفي")),
            (("Lenses", "العدسات"), ("Green, polarised", "خضراء مستقطبة")),
            (("Size", "المقاس"), ("58-15-145 mm", "58-15-145 مم")),
            (("UV protection", "الحماية من الأشعة"), ("UV400 (100%)", "UV400 (حماية 100%)")),
        ],
        variants=[dict(opts=[], price=2799, compare_at=3499)],
        rating=4.5, reviews=538,
        alt=("Polaroid polarised pilot sunglasses, black frame with green lenses",
             "نظارة شمسية بولارويد مستقطبة بإطار أسود وعدسات خضراء"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1709,
        name=("Pixel 'Deal With It' Party Glasses", "نظارة حفلات بتصميم البكسل"),
        subtitle=("8-bit meme shades for photos and parties", "نظارة الميمز الشهيرة لصور الحفلات والضحك"),
        description=(
            "The 8-bit meme sunglasses in real life: a black pixelated frame with tinted pink lenses and white "
            "pixel highlights. Lightweight plastic with folding arms — perfect for birthdays, graduation photos "
            "and gaming nights.",
            "نظارة الميمز الشهيرة بتصميم البكسل على أرض الواقع: إطار أسود بشكل مربعات وعدسات وردية مع لمعة بكسل "
            "بيضاء. خفيفة الوزن بأذرع قابلة للطي، ومثالية لأعياد الميلاد وصور التخرج وسهرات الألعاب.",
        ),
        highlights=[
            ("Novelty eyewear — not for driving", "نظارة للتسلية وغير مخصصة للقيادة"),
            ("One size fits most", "مقاس واحد يناسب معظم الأعمار"),
            ("Folding arms, lightweight plastic", "أذرع قابلة للطي وبلاستيك خفيف"),
        ],
        specs=[
            (("Frame", "الإطار"), ("Plastic, black", "بلاستيك أسود")),
            (("Lenses", "العدسات"), ("Tinted acrylic, pink", "أكريليك ملوّن وردي")),
            (("Size", "المقاس"), ("One size", "مقاس واحد")),
            (("Age", "العمر"), ("8+", "من 8 سنوات")),
        ],
        variants=[dict(opts=[], price=149)],
        rating=4.2, reviews=129,
        alt=("Black pixel-art party sunglasses with pink lenses", "نظارة حفلات سوداء بتصميم البكسل وعدسات وردية"),
        views=[FRONT, ANGLED, OTHER],
    ),
    dict(
        id=1710,
        name=("Oversized Clear-Frame Blue-Light Glasses", "نظارة بإطار شفاف كبير وعدسات للحماية من الضوء الأزرق"),
        subtitle=("Screen-time glasses with a statement frame", "نظارة للشاشات بإطار شفاف شيك يلفت الأنظار"),
        description=(
            "Oversized square glasses in a crystal-clear frame, finished with tiny gold-tone star studs at the "
            "hinges. The non-prescription lenses filter part of the blue light from phones and laptops to make long "
            "screen sessions easier on the eyes.",
            "نظارة مربعة كبيرة بإطار شفاف تمامًا، مزينة بنجوم معدنية ذهبية صغيرة عند المفصلات. عدساتها بدون مقاس "
            "طبي وتُرشّح جزءًا من الضوء الأزرق الصادر من الهاتف واللابتوب لتقليل إجهاد العين أثناء الاستخدام الطويل.",
        ),
        highlights=[
            ("Blue-light filtering lenses, no prescription", "عدسات تُرشّح الضوء الأزرق بدون مقاس طبي"),
            ("Lightweight TR90 frame", "إطار TR90 خفيف الوزن"),
            ("Any optician can fit prescription lenses", "يمكن تركيب عدسات طبية لها عند أي محل نظارات"),
        ],
        specs=[
            (("Frame", "الإطار"), ("Transparent TR90", "TR90 شفاف")),
            (("Lenses", "العدسات"), ("Clear, blue-light filter", "شفافة بفلتر للضوء الأزرق")),
            (("Size", "المقاس"), ("56-17-145 mm", "56-17-145 مم")),
            (("Details", "التفاصيل"), ("Gold-tone star studs", "نجوم معدنية ذهبية")),
        ],
        variants=[dict(opts=[], price=449)],
        rating=4.1, reviews=76,
        alt=("Oversized clear-frame blue-light glasses with gold star studs",
             "نظارة بإطار شفاف كبير ونجوم ذهبية للحماية من الضوء الأزرق"),
        views=[FRONT, ANGLED, OTHER],
    ),
    # ------------------------------------------------------------------ Jewellery
    dict(
        id=1711,
        name=("Emerald Crystal Drop Earrings — Gold-Plated", "أقراط متدلية بكريستال زمردي — مطلية بالذهب"),
        subtitle=("Faceted green drops for weddings and soirées", "حلق زمردي يلمع في الفرح والسهرة"),
        description=(
            "Faceted emerald-green crystal drops hang beneath a round green crystal and a polished gold-plated cone, "
            "all on a secure hinged huggie hoop. Elegant enough for an engagement party, light enough to wear all "
            "evening.",
            "دلّايات من الكريستال الأخضر الزمردي المقطوع بعناية تتدلى أسفل حبة كريستال مستديرة ومخروط لامع مطلي "
            "بالذهب، على حلقة صغيرة بقفل آمن. أنيقة بما يكفي لحفلة خطوبة، وخفيفة بما يكفي لارتدائها طوال السهرة.",
        ),
        highlights=[
            ("18K gold plating over brass", "طلاء ذهب عيار 18 على النحاس"),
            ("Nickel-free", "خالية من النيكل"),
            ("Secure hinged huggie closure", "قفل حلقي آمن بمفصلة"),
        ],
        specs=[
            (("Material", "الخامة"), ("Gold-plated brass, glass crystal", "نحاس مطلي بالذهب وكريستال زجاجي")),
            (("Length", "الطول"), ("5.5 cm", "5.5 سم")),
            (("Weight", "الوزن"), ("6 g per pair", "6 جرام للزوج")),
            (("Closure", "الإغلاق"), ("Hinged hoop", "حلقة بمفصلة")),
        ],
        variants=[dict(opts=[], price=899)],
        modifier_groups=[dict(
            id="gift", name=("Gift options", "خيارات الهدية"), selection="single", min=0, max=1, required=False,
            options=[dict(id="giftbox", name=("Gift box with ribbon", "علبة هدايا بشريطة"), price=99)],
        )],
        rating=4.6, reviews=88,
        alt=("Gold-plated earrings with faceted emerald-green crystal drops",
             "أقراط مطلية بالذهب بدلّايات من الكريستال الأخضر الزمردي"),
        views=[FRONT, ("side view showing the hinged hoop", "من الجانب مع القفل الحلقي"), ANGLED],
    ),
    dict(
        id=1712,
        name=("Checkerboard Acrylic Hoop Earrings — Mint", "أقراط حلقية من الأكريليك بنقشة الشطرنج — أخضر نعناعي"),
        subtitle=("Playful retro check, featherlight acrylic", "نقشة شطرنج ريترو وخفيفة جدًا على الودن"),
        description=(
            "Statement hoops in a wavy mint-and-cream checkerboard, cut from lightweight acrylic and hung from a "
            "matching round stud. They add instant colour to a plain tee or a summer dress without weighing your "
            "ears down.",
            "أقراط حلقية لافتة بنقشة شطرنج متموجة بالأخضر النعناعي والكريمي، مقطوعة من الأكريليك الخفيف ومعلّقة في "
            "دبوس دائري بنفس اللون. تضيف لمسة لونية فورية للتيشيرت السادة أو الفستان الصيفي دون أن تُثقل الأذن.",
        ),
        highlights=[
            ("Featherlight acrylic, about 4 g each", "أكريليك خفيف جدًا، حوالي 4 جرام للقطعة"),
            ("Hypoallergenic stainless-steel posts", "دبابيس ستانلس ستيل مضادة للحساسية"),
            ("Matching round studs", "دبوس دائري بنفس اللون"),
        ],
        specs=[
            (("Material", "الخامة"), ("Acrylic, stainless-steel post", "أكريليك ودبوس ستانلس ستيل")),
            (("Length", "الطول"), ("6 cm", "6 سم")),
            (("Colour", "اللون"), ("Mint & cream", "أخضر نعناعي وكريمي")),
            (("Closure", "الإغلاق"), ("Push-back", "دبوس بظهر ضاغط")),
        ],
        variants=[dict(opts=[], price=349)],
        rating=4.5, reviews=23,
        alt=("Mint and cream checkerboard acrylic hoop earrings", "أقراط حلقية من الأكريليك بنقشة شطرنج نعناعية وكريمية"),
        views=[FRONT, ANGLED, ("side profile", "من الجانب")],
    ),
    dict(
        id=1713,
        name=("Monstera Leaf Statement Earrings — Gold-Tone", "أقراط بشكل ورقة المونستيرا — ذهبية اللون"),
        subtitle=("Tropical leaf drops on a polished gold disc", "ورقة مونستيرا دهبي، لوك صيفي مختلف"),
        description=(
            "A sculpted monstera leaf in warm matte gold tone dangles from a polished disc stud for an easy tropical "
            "statement. Light enough to wear all day, from the office to a beach dinner in El Gouna.",
            "ورقة مونستيرا منحوتة بلون ذهبي دافئ مطفي تتدلى من دبوس دائري لامع، لإطلالة استوائية لافتة بسهولة. "
            "خفيفة بما يكفي لارتدائها طوال اليوم، من المكتب إلى عشاء على البحر في الجونة.",
        ),
        highlights=[
            ("Lightweight zinc-alloy leaf", "ورقة من سبيكة زنك خفيفة"),
            ("Nickel-free posts", "دبابيس خالية من النيكل"),
            ("Tarnish-resistant finish", "طلاء مقاوم للبهتان"),
        ],
        specs=[
            (("Material", "الخامة"), ("Gold-tone zinc alloy, stainless-steel post", "سبيكة زنك ذهبية اللون ودبوس ستانلس ستيل")),
            (("Length", "الطول"), ("6.5 cm", "6.5 سم")),
            (("Weight", "الوزن"), ("7 g each", "7 جرام للقطعة")),
            (("Closure", "الإغلاق"), ("Push-back", "دبوس بظهر ضاغط")),
        ],
        variants=[dict(opts=[], price=399)],
        rating=4.4, reviews=17,
        alt=("Gold-tone monstera leaf drop earrings", "أقراط متدلية بشكل ورقة المونستيرا بلون ذهبي"),
        views=[FRONT, ANGLED, ("side profile", "من الجانب")],
    ),
    # ------------------------------------------------------------------ Make-up
    dict(
        id=1801,
        name=("essence Lash Princess False Lash Effect Mascara — Black",
              "ماسكارا إيسنس لاش برنسيس بتأثير الرموش الصناعية — أسود"),
        subtitle=("The cult volume-and-length mascara", "الماسكارا اللي كل البنات بتحلف بيها"),
        description=(
            "essence's best-selling mascara with a conic fibre brush that separates, lengthens and builds dramatic "
            "volume for a false-lash look. The deep black formula stays put all day without flaking.",
            "الماسكارا الأكثر مبيعًا من إيسنس، بفرشاة مخروطية من الألياف تفصل الرموش وتطيلها وتمنحها كثافة درامية "
            "تشبه الرموش الصناعية. تركيبتها السوداء العميقة تثبت طوال اليوم دون أن تتفتت.",
        ),
        highlights=[
            ("Conic fibre brush for volume and length", "فرشاة مخروطية للكثافة والطول"),
            ("Smudge-resistant, long-wearing formula", "تركيبة ثابتة ومقاومة للتلطخ"),
            ("Not tested on animals", "لم تُختبر على الحيوانات"),
        ],
        specs=[
            (("Volume", "الحجم"), ("12 ml", "12 مل")),
            (("Shade", "اللون"), ("Black", "أسود")),
            (("Brush", "الفرشاة"), ("Conic fibre brush", "فرشاة ألياف مخروطية")),
            (("Effect", "النتيجة"), ("Volume & length", "كثافة وطول")),
        ],
        variants=[dict(opts=[], price=259)],
        rating=4.4, reviews=4268,
        alt=("essence Lash Princess mascara, tube and brush", "ماسكارا إيسنس لاش برنسيس بالعبوة والفرشاة"),
        views=[("tube and wand", "العبوة والفرشاة")],
    ),
    dict(
        id=1802,
        name=("Flormar 12-Shade Eyeshadow Palette with Mirror — Warm Berry",
              "باليت ظلال عيون فلورمار 12 لونًا بمرآة — توت دافئ"),
        subtitle=("Warm nudes, berries and bronze shimmer in one palette",
                  "نود وتوت ولمعة برونزي، لوك النهار والسهرة في باليت واحدة"),
        description=(
            "Twelve buttery shades that run from soft champagne and peach to berry, plum and bronze shimmer, so one "
            "palette covers the office, a wedding and a night out. A full-size mirror and slim magnetic case make it "
            "easy to take along.",
            "اثنا عشر لونًا بملمس ناعم تبدأ من الشامبين والخوخي وتصل إلى التوتي والبرقوقي والبرونزي اللامع، لتغطي "
            "إطلالة العمل والفرح والسهرة بباليت واحدة. مرآة كاملة وعلبة رفيعة بغلق مغناطيسي تجعلها سهلة الحمل.",
        ),
        highlights=[
            ("12 matte and shimmer shades", "12 لونًا بين المطفي واللامع"),
            ("Full-size mirror", "مرآة بحجم كامل"),
            ("Richly pigmented and easy to blend", "ألوان كثيفة وسهلة الدمج"),
        ],
        specs=[
            (("Shades", "عدد الألوان"), ("12 (8 matte, 4 shimmer)", "12 لونًا (8 مطفي و4 لامع)")),
            (("Net weight", "الوزن الصافي"), ("12 × 1.5 g", "12 × 1.5 جرام")),
            (("Finish", "اللمسة النهائية"), ("Matte & shimmer", "مطفي ولامع")),
            (("Made in", "بلد الصنع"), ("Turkey", "تركيا")),
        ],
        variants=[dict(opts=[], price=599, compare_at=799)],
        rating=4.3, reviews=642,
        alt=("Flormar 12-shade warm eyeshadow palette with mirror", "باليت ظلال عيون فلورمار بـ12 لونًا دافئًا ومرآة"),
        views=[("open palette with mirror", "الباليت مفتوحة بالمرآة")],
    ),
    dict(
        id=1803,
        name=("Golden Rose Pressed Compact Powder", "بودرة مضغوطة من جولدن روز"),
        subtitle=("Silky matte finish that sets make-up", "بودرة ناعمة تثبّت الميك أب وتقلل اللمعة طول اليوم"),
        description=(
            "A finely milled pressed powder that sets foundation, softens shine and blurs pores with a natural matte "
            "finish. The slim compact with mirror and puff slips into any bag for quick touch-ups.",
            "بودرة مضغوطة مطحونة بنعومة تثبّت كريم الأساس وتقلل اللمعة وتخفي المسام بلمسة نهائية مطفية طبيعية. "
            "علبتها الرفيعة بالمرآة والإسفنجة تدخل أي حقيبة للتعديل السريع.",
        ),
        highlights=[
            ("Natural matte finish", "لمسة نهائية مطفية طبيعية"),
            ("Mirror and puff included", "مرآة وإسفنجة مرفقتان"),
            ("Good for oily and combination skin", "مناسبة للبشرة الدهنية والمختلطة"),
        ],
        specs=[
            (("Net weight", "الوزن الصافي"), ("11 g", "11 جرام")),
            (("Finish", "اللمسة النهائية"), ("Matte", "مطفي")),
            (("Shades", "الدرجات"), ("3", "3 درجات")),
            (("Made in", "بلد الصنع"), ("Turkey", "تركيا")),
        ],
        options=[dict(id="shade", name=("Shade", "الدرجة"), kind="swatch", values=[
            dict(id="01-ivory", label=("01 Ivory", "01 عاجي"), swatch="#EED9C4", code="01"),
            dict(id="03-natural-beige", label=("03 Natural Beige", "03 بيج طبيعي"), swatch="#E2C3A2", code="03"),
            dict(id="05-sand", label=("05 Sand", "05 رملي"), swatch="#D2AE8A", code="05"),
        ])],
        variants=[
            dict(opts=["01-ivory"], price=329, image=0),
            dict(opts=["03-natural-beige"], price=329, image=0),
            dict(opts=["05-sand"], price=329, image=0),
        ],
        rating=4.2, reviews=318,
        alt=("Golden Rose pressed compact powder", "بودرة مضغوطة من جولدن روز"),
        views=[("open compact with mirror", "العلبة مفتوحة بالمرآة")],
    ),
    dict(
        id=1804,
        name=("Flormar Matte Lipstick", "أحمر شفاه مطفي من فلورمار"),
        subtitle=("Bold red, velvety matte, long wear", "أحمر جريء بلمسة مطفية مخملية وثبات طويل"),
        description=(
            "A creamy lipstick that glides on in one stroke and sets to a velvety matte without drying your lips. "
            "Three reds — a true ruby, a bright cherry and a warm brick — suit every skin tone and occasion.",
            "أحمر شفاه كريمي ينزلق على الشفاه من أول لمسة ويثبت بلمسة مطفية مخملية دون أن يجففها. اختاري من ثلاث "
            "درجات للأحمر: روبي كلاسيكي، وكرزي مشرق، وطوبي دافئ، لتناسب كل لون بشرة وكل مناسبة.",
        ),
        highlights=[
            ("Velvet matte finish", "لمسة نهائية مطفية مخملية"),
            ("Enriched with vitamin E", "غني بفيتامين E"),
            ("Colour that lasts up to 8 hours", "ثبات اللون حتى 8 ساعات"),
        ],
        specs=[
            (("Net weight", "الوزن الصافي"), ("4 g", "4 جرام")),
            (("Finish", "اللمسة النهائية"), ("Matte", "مطفي")),
            (("Shades", "الدرجات"), ("3 reds", "3 درجات من الأحمر")),
            (("Made in", "بلد الصنع"), ("Turkey", "تركيا")),
        ],
        options=[dict(id="shade", name=("Shade", "الدرجة"), kind="swatch", values=[
            dict(id="ruby", label=("Ruby Red", "أحمر روبي"), swatch="#9B111E", code="RBY"),
            dict(id="cherry", label=("Cherry", "كرزي"), swatch="#C8102E", code="CHR"),
            dict(id="brick", label=("Brick Red", "أحمر طوبي"), swatch="#A23B2C", code="BRK"),
        ])],
        variants=[
            dict(opts=["ruby"], price=299, image=0),
            dict(opts=["cherry"], price=299, image=0),
            dict(opts=["brick"], price=299, image=0),
        ],
        rating=4.5, reviews=2846,
        alt=("Flormar red matte lipstick", "أحمر شفاه مطفي من فلورمار"),
        views=[("lipstick and cap", "أحمر الشفاه مع الغطاء")],
    ),
    dict(
        id=1805,
        name=("Golden Rose Rich Color Nail Lacquer", "طلاء أظافر جولدن روز ريتش كولور"),
        subtitle=("Glossy, chip-resistant colour in two coats", "لمعة قوية ولون غني من طبقتين بس"),
        description=(
            "A rich, high-gloss nail lacquer with a wide brush that covers evenly in two coats. Quick-drying and "
            "chip-resistant, in three essential reds from classic to deep wine.",
            "طلاء أظافر بلون غني ولمعة عالية، بفرشاة عريضة تغطي الظفر بالتساوي من طبقتين. سريع الجفاف ومقاوم "
            "للتقشر، ومتوفر بثلاث درجات أساسية من الأحمر الكلاسيكي حتى النبيتي الداكن.",
        ),
        highlights=[
            ("Full coverage in two coats", "تغطية كاملة من طبقتين"),
            ("Quick-dry, high-shine formula", "تركيبة سريعة الجفاف بلمعة عالية"),
            ("Wide flat brush", "فرشاة عريضة مسطحة"),
        ],
        specs=[
            (("Volume", "الحجم"), ("10.5 ml", "10.5 مل")),
            (("Finish", "اللمسة النهائية"), ("High gloss", "لامع")),
            (("Shades", "الدرجات"), ("3 reds", "3 درجات من الأحمر")),
            (("Made in", "بلد الصنع"), ("Turkey", "تركيا")),
        ],
        options=[dict(id="shade", name=("Shade", "الدرجة"), kind="swatch", values=[
            dict(id="classic-red", label=("Classic Red", "أحمر كلاسيكي"), swatch="#C1121F", code="CLR"),
            dict(id="scarlet", label=("Scarlet", "قرمزي"), swatch="#E2231A", code="SCR"),
            dict(id="wine", label=("Wine", "نبيتي"), swatch="#6D0F1F", code="WIN"),
        ])],
        variants=[
            dict(opts=["classic-red"], price=119, image=0),
            dict(opts=["scarlet"], price=119, image=0),
            dict(opts=["wine"], price=119, image=0),
        ],
        rating=4.3, reviews=640,
        alt=("Golden Rose Rich Color nail lacquer in red", "طلاء أظافر جولدن روز ريتش كولور باللون الأحمر"),
        views=[("bottle and brush", "الزجاجة والفرشاة")],
    ),
    # ------------------------------------------------------------------ Fragrance
    dict(
        id=1806,
        name=("Calvin Klein CK One Eau de Toilette", "عطر كالفن كلاين CK One أو دو تواليت"),
        subtitle=("The fresh unisex classic", "الريحة الفريش اللي بيحبها الكل، رجالي وحريمي"),
        description=(
            "The original shared fragrance: a clean, citrus-green opening of bergamot, cardamom and pineapple, a heart "
            "of green tea and jasmine, and a soft musk and amber base. Light enough for Cairo summers and easy to wear "
            "every day.",
            "العطر المشترك الأصلي: افتتاحية منعشة من البرغموت والهيل والأناناس، وقلب من الشاي الأخضر والياسمين، "
            "وقاعدة ناعمة من المسك والعنبر. خفيف ومناسب لصيف القاهرة وللاستخدام اليومي.",
        ),
        highlights=[
            ("Unisex citrus-aromatic scent", "عطر حمضي عطري للجنسين"),
            ("Ideal for daytime and summer", "مثالي للنهار والصيف"),
            ("Iconic frosted flask with screw cap", "زجاجة مطفية أيقونية بغطاء محكم"),
        ],
        specs=[
            (("Concentration", "التركيز"), ("Eau de Toilette", "أو دو تواليت")),
            (("Top notes", "المقدمة"), ("Bergamot, cardamom, pineapple", "برغموت، هيل، أناناس")),
            (("Heart notes", "القلب"), ("Green tea, jasmine", "شاي أخضر، ياسمين")),
            (("Base notes", "القاعدة"), ("Musk, amber", "مسك، عنبر")),
            (("For", "مناسب لـ"), ("Unisex", "الجنسين")),
        ],
        options=[size(100, 200)],
        variants=[
            dict(opts=["100ml"], price=2999, compare_at=3699, image=2),
            dict(opts=["200ml"], price=4299, compare_at=4999, image=1),
        ],
        rating=4.7, reviews=3104,
        alt=("Calvin Klein CK One eau de toilette", "عطر كالفن كلاين CK One أو دو تواليت"),
        views=[("bottle", "الزجاجة"), ("200 ml bottle with box", "زجاجة 200 مل مع العلبة"),
               ("100 ml bottle with box", "زجاجة 100 مل مع العلبة")],
    ),
    dict(
        id=1807,
        name=("CHANEL Coco Noir Eau de Parfum", "عطر شانيل كوكو نوار أو دو بارفان"),
        subtitle=("Rose and patchouli in noir elegance", "أناقة سودا بريحة الورد والباتشولي، للسهرات الخاصة"),
        description=(
            "Coco Noir opens with sparkling grapefruit and bergamot, unfolds into a heart of rose and jasmine, and "
            "settles on a deep, sensual trail of patchouli, sandalwood, tonka bean and vanilla. A mysterious, "
            "long-lasting signature for evenings.",
            "يبدأ كوكو نوار بانتعاش الجريب فروت والبرغموت، ثم يتفتح قلبه على الورد والياسمين، ويستقر على أثر دافئ "
            "وعميق من الباتشولي وخشب الصندل والتونكا والفانيليا. توقيع غامض يدوم طويلًا ويليق بالسهرات.",
        ),
        highlights=[
            ("Oriental-floral, long-lasting", "عطر شرقي زهري يدوم طويلًا"),
            ("Black lacquered bottle with gold-edged label", "زجاجة سوداء لامعة بملصق بإطار ذهبي"),
            ("Authentic, in its sealed box", "أصلي ومغلّف بعلبته"),
        ],
        specs=[
            (("Concentration", "التركيز"), ("Eau de Parfum", "أو دو بارفان")),
            (("Top notes", "المقدمة"), ("Grapefruit, bergamot", "جريب فروت، برغموت")),
            (("Heart notes", "القلب"), ("Rose, jasmine", "ورد، ياسمين")),
            (("Base notes", "القاعدة"), ("Patchouli, sandalwood, tonka, vanilla", "باتشولي، صندل، تونكا، فانيليا")),
            (("For", "مناسب لـ"), ("Women", "النساء")),
        ],
        options=[size(50, 100)],
        variants=[
            dict(opts=["50ml"], price=8499, image=1),
            dict(opts=["100ml"], price=11499, image=2),
        ],
        modifier_groups=[GIFT_PERFUME],
        rating=4.8, reviews=214,
        alt=("CHANEL Coco Noir eau de parfum", "عطر شانيل كوكو نوار أو دو بارفان"),
        views=[("bottle", "الزجاجة"), ("50 ml bottle with box", "زجاجة 50 مل مع العلبة"),
               ("100 ml box and bottle", "علبة وزجاجة 100 مل")],
    ),
    dict(
        id=1808,
        name=("Dior J'adore Eau de Parfum", "عطر ديور جادور أو دو بارفان"),
        subtitle=("A luminous bouquet of ylang-ylang, rose and jasmine", "باقة ورد وياسمين مشرقة، أنوثة صافية"),
        description=(
            "J'adore blends ylang-ylang from the Comoros, Damascus rose and Grasse jasmine sambac into a radiant, "
            "golden floral bouquet. The amphora bottle with its gold neck is as iconic as the scent — a gift that "
            "never misses.",
            "يمزج جادور زهرة اليلانغ يلانغ من جزر القمر والوردة الدمشقية وياسمين السمباك من غراس في باقة زهرية "
            "ذهبية مشرقة. زجاجته على شكل الأمفورا بعنقها الذهبي أيقونة مثل العطر نفسه، وهدية لا تخيب.",
        ),
        highlights=[
            ("Floral bouquet for women", "باقة زهرية للنساء"),
            ("Iconic amphora bottle with gold neck", "زجاجة الأمفورا الأيقونية بعنق ذهبي"),
            ("Authentic, in its sealed box", "أصلي ومغلّف بعلبته"),
        ],
        specs=[
            (("Concentration", "التركيز"), ("Eau de Parfum", "أو دو بارفان")),
            (("Key notes", "المكونات الرئيسية"), ("Ylang-ylang, Damascus rose, jasmine sambac",
                                                  "يلانغ يلانغ، ورد دمشقي، ياسمين سمباك")),
            (("Fragrance family", "عائلة العطر"), ("Floral", "زهري")),
            (("For", "مناسب لـ"), ("Women", "النساء")),
        ],
        options=[size(50, 100)],
        variants=[
            dict(opts=["50ml"], price=7999),
            dict(opts=["100ml"], price=10999),
        ],
        modifier_groups=[GIFT_PERFUME],
        rating=4.8, reviews=356,
        alt=("Dior J'adore eau de parfum", "عطر ديور جادور أو دو بارفان"),
        views=[("bottle", "الزجاجة"), ("box and bottle", "العلبة والزجاجة"),
               ("bottle with its Eau de Parfum box", "الزجاجة مع علبة أو دو بارفان")],
    ),
    dict(
        id=1809,
        name=("Dolce&Gabbana Dolce Shine Eau de Parfum", "عطر دولتشي آند غابانا دولتشي شاين أو دو بارفان"),
        subtitle=("Juicy mango and jasmine — pure sunshine", "مانجا وياسمين، ريحة مبهجة زي شمس الصيف"),
        description=(
            "Dolce Shine is a joyful fruity floral: juicy mango, grapefruit and bergamot up top, a heart of jasmine, "
            "and a warm base of blond woods and musk. The bottle with its yellow flower cap and black bow is made for "
            "gifting.",
            "دولتشي شاين عطر فاكهي زهري مبهج: مانجا عصيرية وجريب فروت وبرغموت في الافتتاحية، وقلب من الياسمين، "
            "وقاعدة دافئة من الأخشاب الشقراء والمسك. زجاجته بغطاء الوردة الصفراء والفيونكة السوداء جاهزة كهدية.",
        ),
        highlights=[
            ("Fruity-floral, perfect for daytime", "عطر فاكهي زهري مثالي للنهار"),
            ("Signature flower cap and bow", "غطاء الوردة والفيونكة المميزان"),
            ("Authentic, in its sealed box", "أصلي ومغلّف بعلبته"),
        ],
        specs=[
            (("Concentration", "التركيز"), ("Eau de Parfum", "أو دو بارفان")),
            (("Top notes", "المقدمة"), ("Mango, grapefruit, bergamot", "مانجا، جريب فروت، برغموت")),
            (("Heart notes", "القلب"), ("Jasmine", "ياسمين")),
            (("Base notes", "القاعدة"), ("Blond woods, musk", "أخشاب شقراء، مسك")),
            (("For", "مناسب لـ"), ("Women", "النساء")),
        ],
        options=[size(50, 75)],
        variants=[
            dict(opts=["50ml"], price=4999),
            dict(opts=["75ml"], price=6299),
        ],
        rating=4.6, reviews=188,
        alt=("Dolce&Gabbana Dolce Shine eau de parfum", "عطر دولتشي آند غابانا دولتشي شاين أو دو بارفان"),
        views=[("bottle", "الزجاجة"), ("box and bottle", "العلبة والزجاجة"), ("bottle and box", "الزجاجة والعلبة")],
    ),
    dict(
        id=1810,
        name=("Gucci Bloom Eau de Parfum", "عطر غوتشي بلوم أو دو بارفان"),
        subtitle=("A garden of tuberose and jasmine", "حديقة مسك الروم والياسمين في زجاجة وردي"),
        description=(
            "Gucci Bloom is a rich white floral built around natural tuberose and jasmine bud, lifted by the rare "
            "Rangoon creeper. The powder-pink lacquered bottle and botanical-print box make it a gift that feels "
            "personal.",
            "غوتشي بلوم عطر زهري أبيض غني يتمحور حول مسك الروم الطبيعي وبراعم الياسمين، مع لمسة نادرة من زهرة "
            "رانغون كريبر. زجاجته الوردية اللامعة وعلبته بنقشة الزهور تجعله هدية بلمسة شخصية.",
        ),
        highlights=[
            ("White-floral scent for women", "عطر زهري أبيض للنساء"),
            ("Powder-pink lacquered bottle", "زجاجة وردية لامعة"),
            ("Authentic, in its sealed box", "أصلي ومغلّف بعلبته"),
        ],
        specs=[
            (("Concentration", "التركيز"), ("Eau de Parfum", "أو دو بارفان")),
            (("Key notes", "المكونات الرئيسية"), ("Tuberose, jasmine, Rangoon creeper",
                                                  "مسك الروم، ياسمين، رانغون كريبر")),
            (("Fragrance family", "عائلة العطر"), ("White floral", "زهري أبيض")),
            (("For", "مناسب لـ"), ("Women", "النساء")),
        ],
        options=[size(50, 100)],
        variants=[
            dict(opts=["50ml"], price=6499, compare_at=7999),
            dict(opts=["100ml"], price=8999, compare_at=10499, image=1),
        ],
        modifier_groups=[GIFT_PERFUME],
        rating=4.7, reviews=241,
        alt=("Gucci Bloom eau de parfum", "عطر غوتشي بلوم أو دو بارفان"),
        views=[("bottle", "الزجاجة"), ("100 ml box and bottle", "علبة وزجاجة 100 مل"), ("box and bottle", "العلبة والزجاجة")],
    ),
    # ------------------------------------------------------------------ Body care
    dict(
        id=1811,
        name=("ATTITUDE Super Leaves Natural Hand Soap — Lemon Leaves, 473 ml",
              "صابون يدين طبيعي أتيتيود سوبر ليفز — أوراق الليمون، 473 مل"),
        subtitle=("Plant-based clean with a fresh lemon scent", "صابون طبيعي بريحة الليمون الفريش، لطيف على الإيدين"),
        description=(
            "A gentle, plant-based hand soap with a fresh lemon-leaves scent that cleans without stripping. "
            "Hypoallergenic, dermatologically tested and EWG Verified — safe for the whole family's hands.",
            "صابون يدين لطيف بمكونات نباتية ورائحة أوراق الليمون المنعشة، ينظف دون أن يسبب جفاف اليدين. مضاد "
            "للحساسية ومُختبر جلديًا وحاصل على اعتماد EWG، وآمن لأيدي العائلة كلها.",
        ),
        highlights=[
            ("EWG Verified, plant-based ingredients", "مكونات نباتية معتمدة من EWG"),
            ("Hypoallergenic, dermatologically tested", "مضاد للحساسية ومُختبر جلديًا"),
            ("Vegan and not tested on animals", "نباتي ولم يُختبر على الحيوانات"),
        ],
        specs=[
            (("Volume", "الحجم"), ("473 ml (16 fl oz)", "473 مل")),
            (("Scent", "الرائحة"), ("Lemon leaves", "أوراق الليمون")),
            (("Skin type", "نوع البشرة"), ("All skin types", "كل أنواع البشرة")),
            (("Made in", "بلد الصنع"), ("Canada", "كندا")),
        ],
        variants=[dict(opts=[], price=549)],
        rating=4.6, reviews=74,
        alt=("ATTITUDE Super Leaves lemon leaves hand soap pump bottle",
             "عبوة صابون اليدين أتيتيود سوبر ليفز بأوراق الليمون"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1812,
        name=("Olay Ultra Moisture Body Wash with Shea Butter, 887 ml",
              "غسول الجسم أولاي ألترا مويستشر بزبدة الشيا، 887 مل"),
        subtitle=("Creamy shea-butter wash that locks in moisture", "شاور جل كريمي بزبدة الشيا، يسيب جسمك ناعم وطري"),
        description=(
            "A rich, creamy body wash with shea butter and Olay's lock-in-moisture formula that cleanses while "
            "leaving skin soft and hydrated. The big 887 ml pump bottle lasts the whole family for weeks.",
            "غسول جسم كريمي غني بزبدة الشيا وتركيبة أولاي التي تحبس الترطيب، ينظف البشرة ويتركها ناعمة ورطبة. "
            "عبوة كبيرة بمضخة سعة 887 مل تكفي العائلة لأسابيع.",
        ),
        highlights=[
            ("Shea butter with lock-in-moisture formula", "زبدة الشيا مع تركيبة تحبس الترطيب"),
            ("Big 887 ml pump bottle", "عبوة كبيرة بمضخة سعة 887 مل"),
            ("Rich, creamy lather", "رغوة كريمية غنية"),
        ],
        specs=[
            (("Volume", "الحجم"), ("887 ml (30 fl oz)", "887 مل")),
            (("Key ingredient", "المكوّن الرئيسي"), ("Shea butter", "زبدة الشيا")),
            (("Skin type", "نوع البشرة"), ("Dry to normal skin", "البشرة الجافة والعادية")),
            (("Format", "العبوة"), ("Pump bottle", "عبوة بمضخة")),
        ],
        variants=[dict(opts=[], price=549)],
        rating=4.6, reviews=263,
        alt=("Olay Ultra Moisture shea butter body wash pump bottle", "عبوة غسول الجسم أولاي ألترا مويستشر بزبدة الشيا"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1813,
        name=("Vaseline Men Fast Absorbing Body & Face Lotion, 600 ml",
              "لوشن فازلين للرجال سريع الامتصاص للجسم والوجه، 600 مل"),
        subtitle=("Non-greasy hydration that absorbs instantly", "ترطيب سريع من غير أي إحساس بالدهون"),
        description=(
            "Made for men's skin, this non-greasy lotion absorbs instantly and helps heal dry skin with "
            "micro-droplets of Vaseline Jelly. Use it on face and body after shaving or a long day in the sun.",
            "لوشن مصمم لبشرة الرجال، غير دهني ويمتص فورًا، ويساعد على علاج جفاف البشرة بفضل قطرات دقيقة من فازلين "
            "جيلي. مناسب للوجه والجسم بعد الحلاقة أو بعد يوم طويل في الشمس.",
        ),
        highlights=[
            ("Absorbs instantly, non-greasy", "يمتص فورًا وغير دهني"),
            ("For face and body", "للوجه والجسم"),
            ("With micro-droplets of Vaseline Jelly", "بقطرات دقيقة من فازلين جيلي"),
        ],
        specs=[
            (("Volume", "الحجم"), ("600 ml (20.3 fl oz)", "600 مل")),
            (("Skin type", "نوع البشرة"), ("Dry skin", "البشرة الجافة")),
            (("Texture", "القوام"), ("Light, non-greasy", "خفيف وغير دهني")),
            (("Format", "العبوة"), ("Pump bottle", "عبوة بمضخة")),
        ],
        variants=[dict(opts=[], price=349)],
        rating=4.6, reviews=1486,
        alt=("Vaseline Men fast absorbing body and face lotion pump bottle",
             "عبوة لوشن فازلين للرجال سريع الامتصاص للجسم والوجه"),
        views=[FRONT, ANGLED, SIDE],
    ),
]

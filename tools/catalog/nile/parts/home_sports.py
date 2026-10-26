"""Home & Furniture (1901–1911) and Sports & Fitness (2101–2118) content for the Nile catalogue."""

FRONT = ("front view", "منظر أمامي")
ANGLED = ("angled view", "منظر مائل")
SIDE = ("side angle", "منظر جانبي")

PRODUCTS = [
    # ------------------------------------------------------------------ Home & Furniture
    dict(
        id=1901,
        name=("Classic Beech-Wood Double Bed", "سرير مزدوج كلاسيكي من خشب الزان"),
        subtitle=("Handmade in Damietta · curved headboard", "شغل دمياطي أصلي · بضهر منحني"),
        description=(
            "A classic double bed handcrafted by Damietta carpenters from solid beech and finished in a warm "
            "walnut stain. The gently curved headboard and turned legs give the bedroom a timeless look, while "
            "a slatted base keeps your mattress well ventilated. Sold as a bed frame only; mattress and bedding "
            "are not included.",
            "سرير مزدوج كلاسيكي صنعه حرفيو دمياط يدويًا من خشب الزان المصمت بتشطيب جوزي دافئ. يمنح الظهر "
            "المنحني والأرجل المخروطة غرفة النوم طابعًا لا يفقد أناقته، بينما تحافظ قاعدة الشرائح الخشبية على "
            "تهوية المرتبة. السعر للسرير فقط بدون مرتبة أو مفروشات.",
        ),
        highlights=[
            ("Solid beech, handmade in Damietta", "خشب زان مصمت بصناعة دمياطية يدوية"),
            ("Slatted base for better mattress airflow", "قاعدة شرائح خشبية لتهوية أفضل للمرتبة"),
            ("Delivered in parts — add assembly at checkout", "يُسلَّم مفكوكًا ويمكنك إضافة خدمة التركيب عند الطلب"),
            ("Bed frame only — mattress not included", "السرير فقط بدون مرتبة"),
        ],
        specs=[
            (("Material", "الخامة"), ("Solid beech wood, walnut stain", "خشب زان مصمت بتشطيب جوزي")),
            (("Mattress size", "مقاس المرتبة"), ("160 × 200 cm or 180 × 200 cm", "160 × 200 سم أو 180 × 200 سم")),
            (("Headboard height", "ارتفاع الظهر"), ("115 cm", "115 سم")),
            (("Footprint", "المساحة الكلية"), ("Mattress size + 16 cm each way", "مقاس المرتبة + 16 سم في الطول والعرض")),
            (("Maximum load", "أقصى حمولة"), ("300 kg", "300 كجم")),
            (("Made in", "بلد الصنع"), ("Damietta, Egypt", "دمياط، مصر")),
        ],
        options=[
            dict(id="size", name=("Size", "المقاس"), kind="chip", values=[
                dict(id="160", label=("160 × 200 cm", "160 × 200 سم"), code="160"),
                dict(id="180", label=("180 × 200 cm", "180 × 200 سم"), code="180"),
            ]),
        ],
        variants=[
            dict(opts=["160"], price=32999),
            dict(opts=["180"], price=36999),
        ],
        modifier_groups=[
            dict(id="services", name=("Services", "خدمات"), selection="multi", min=0, max=2, required=False,
                 options=[
                     dict(id="assembly", name=("Assembly by a Nile carpenter", "التركيب بواسطة نجّار نايل"), price=750),
                     dict(id="removal", name=("Old bed removal", "فك ونقل السرير القديم"), price=500),
                 ]),
        ],
        rating=4.6, reviews=84,
        alt=("Classic beech-wood double bed with a curved headboard in walnut finish",
             "سرير مزدوج كلاسيكي من خشب الزان بظهر منحني وتشطيب جوزي"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1902,
        name=("Teak-Frame 3-Seater Sofa with Linen Cushions", "أريكة 3 مقاعد بإطار من خشب التيك ووسائد كتان"),
        subtitle=("Solid teak frame · removable linen covers", "خشب تيك متين وكسوة كتان تتفك وتتغسل"),
        description=(
            "A relaxed three-seater built around an open teak frame with wide armrests that double as side "
            "shelves. Deep seat cushions are covered in an off-white linen blend with zipped, removable covers, "
            "and two black-and-white accent pillows are included. Delivered by our furniture team in Cairo, Giza "
            "and Alexandria within 5–7 working days.",
            "أريكة بثلاثة مقاعد بإطار مفتوح من خشب التيك ومساند عريضة تصلح رفًّا جانبيًا. الوسائد العميقة "
            "مكسوّة بقماش كتان مخلوط بلون أوف وايت بسحّابات لسهولة الفك والغسيل، ومعها وسادتان للديكور "
            "بالأبيض والأسود. التوصيل عبر فريق الأثاث في القاهرة والجيزة والإسكندرية خلال 5–7 أيام عمل.",
        ),
        highlights=[
            ("Kiln-dried solid teak frame", "إطار من خشب التيك المصمت المجفّف بالأفران"),
            ("Removable, washable linen-blend covers", "كسوة كتان مخلوط قابلة للفك والغسيل"),
            ("Two accent pillows included", "وسادتان للديكور مرفقتان"),
        ],
        specs=[
            (("Dimensions", "الأبعاد"), ("220 × 90 × 78 cm (W × D × H)", "220 × 90 × 78 سم (العرض × العمق × الارتفاع)")),
            (("Seat height", "ارتفاع المقعد"), ("44 cm", "44 سم")),
            (("Frame", "الإطار"), ("Solid teak", "خشب تيك مصمت")),
            (("Upholstery", "التنجيد"), ("Linen-cotton blend over high-density foam", "قماش كتان وقطن فوق إسفنج عالي الكثافة")),
            (("Seats", "عدد المقاعد"), ("3", "3 مقاعد")),
        ],
        variants=[dict(opts=[], price=38999)],
        modifier_groups=[
            dict(id="services", name=("Services", "خدمات"), selection="single", min=0, max=1, required=False,
                 options=[
                     dict(id="placement", name=("Assembly & placement in your room", "التركيب والوضع في المكان المطلوب"),
                          price=600),
                 ]),
        ],
        rating=4.5, reviews=112,
        alt=("Teak-frame three-seater sofa with off-white linen cushions",
             "أريكة ثلاثية بإطار من خشب التيك ووسائد كتان أوف وايت"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1903,
        name=("Curved Cherry-Wood Bedside Table", "طاولة سرير جانبية منحنية من خشب الكرز"),
        subtitle=("One drawer, an open shelf and curved sides", "درج ورف مفتوح بتصميم منحني شيك"),
        description=(
            "A bedside table with gracefully curved sides in a deep cherry finish, with a soft-close drawer for "
            "your phone and glasses and an open shelf for books. The top easily fits a lamp, an alarm clock and "
            "a glass of water, and it pairs beautifully with our classic beech-wood bed.",
            "طاولة سرير جانبية بجوانب منحنية أنيقة وتشطيب كرزي داكن، بها درج بغلق هادئ للهاتف والنظارة ورف "
            "مفتوح للكتب. يتسع سطحها لمصباح ومنبّه وكوب ماء، وتتناسق تمامًا مع سريرنا الكلاسيكي من خشب الزان.",
        ),
        highlights=[
            ("Solid cherry wood, hand-finished", "خشب كرز مصمت بتشطيب يدوي"),
            ("Soft-close drawer", "درج بغلق هادئ"),
            ("Open shelf for books", "رف مفتوح للكتب"),
        ],
        specs=[
            (("Dimensions", "الأبعاد"), ("50 × 40 × 62 cm (W × D × H)", "50 × 40 × 62 سم (العرض × العمق × الارتفاع)")),
            (("Material", "الخامة"), ("Solid cherry wood", "خشب كرز مصمت")),
            (("Finish", "التشطيب"), ("Dark cherry lacquer", "ورنيش كرزي داكن")),
            (("Storage", "التخزين"), ("1 drawer + 1 open shelf", "درج واحد ورف مفتوح")),
            (("Assembly", "التركيب"), ("Delivered assembled", "يُسلَّم مُجمّعًا")),
        ],
        variants=[dict(opts=[], price=5499)],
        rating=4.4, reviews=57,
        alt=("Dark cherry-wood bedside table with curved sides, a drawer and an open shelf",
             "طاولة سرير جانبية من خشب الكرز الداكن بجوانب منحنية ودرج ورف مفتوح"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1904,
        name=("Mobica Tulip Executive Swivel Armchair", "كرسي مكتب دوّار توليب من موبيكا"),
        subtitle=("Mid-century shell · 5-star swivel base", "شكل كلاسيكي من الخمسينات وقاعدة بتلف بعجل"),
        description=(
            "Mobica's Tulip armchair brings the classic mid-century executive silhouette to your office or home "
            "desk: a moulded shell upholstered in red fabric, with a padded seat cushion and generous armrests. "
            "The polished aluminium five-star base swivels 360° on smooth castors that are kind to tiles and "
            "parquet. Made in Egypt with a 2-year Mobica warranty.",
            "يقدّم كرسي توليب من موبيكا الطابع التنفيذي الكلاسيكي لمنتصف القرن الماضي لمكتبك في العمل أو البيت: "
            "هيكل مقولب منجّد بقماش أحمر مع مقعد مبطّن ومساند ذراع واسعة. تدور القاعدة الخماسية من الألومنيوم "
            "المصقول 360 درجة على عجلات ناعمة لا تخدش السيراميك أو الباركيه. صناعة مصرية بضمان موبيكا لمدة سنتين.",
        ),
        highlights=[
            ("360° swivel on smooth castors", "دوران 360 درجة على عجلات ناعمة"),
            ("Upholstered moulded shell with padded seat", "هيكل مقولب منجّد ومقعد مبطّن"),
            ("2-year Mobica warranty", "ضمان موبيكا لمدة سنتين"),
        ],
        specs=[
            (("Dimensions", "الأبعاد"), ("68 × 60 × 82 cm (W × D × H)", "68 × 60 × 82 سم (العرض × العمق × الارتفاع)")),
            (("Seat height", "ارتفاع المقعد"), ("47 cm", "47 سم")),
            (("Upholstery", "التنجيد"), ("Red woven fabric", "قماش منسوج أحمر")),
            (("Base", "القاعدة"), ("Polished aluminium, 5 castors", "ألومنيوم مصقول بخمس عجلات")),
            (("Maximum load", "أقصى حمولة"), ("120 kg", "120 كجم")),
        ],
        variants=[dict(opts=[], price=18999)],
        rating=4.7, reviews=23,
        alt=("Red upholstered swivel armchair with a polished five-star base",
             "كرسي دوّار منجّد باللون الأحمر بقاعدة خماسية مصقولة"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1905,
        name=("Ideal Standard 60 cm Bathroom Vanity with Mirror", "وحدة حمام أيديال ستاندرد 60 سم مع مرآة"),
        subtitle=("Cabinet, basin, mirror and two lights", "دولاب وحوض ومراية ولمبتين — الطقم كامل"),
        description=(
            "A complete freestanding vanity set: a black two-door cabinet on raised feet, a white ceramic basin "
            "top with a single tap hole, and a framed mirror flanked by two wall sconces. Moisture-resistant "
            "boards and a sealed lacquer finish are made for steamy bathrooms. Mixer tap not included; "
            "installation by a plumber is recommended.",
            "طقم حمام متكامل: دولاب أسود ببابين على أرجل مرتفعة، وسطح بحوض سيراميك أبيض بفتحة خلاط واحدة، "
            "ومرآة بإطار يحيط بها كشّافان للحائط. الألواح مقاومة للرطوبة والتشطيب بورنيش محكم يتحمل بخار "
            "الحمّام. الخلاط غير مرفق، ونوصي بالتركيب بواسطة سبّاك.",
        ),
        highlights=[
            ("Cabinet, ceramic basin, mirror and two sconces", "دولاب وحوض سيراميك ومرآة وكشّافان"),
            ("Moisture-resistant carcass", "هيكل مقاوم للرطوبة"),
            ("Soft-close doors", "أبواب بغلق هادئ"),
        ],
        specs=[
            (("Cabinet", "الدولاب"), ("60 × 46 × 85 cm", "60 × 46 × 85 سم")),
            (("Mirror", "المرآة"), ("60 × 70 cm, framed", "60 × 70 سم بإطار")),
            (("Basin", "الحوض"), ("Vitreous china, 1 tap hole", "سيراميك صحي بفتحة خلاط واحدة")),
            (("Finish", "اللون"), ("Matte black / white", "أسود مطفي / أبيض")),
            (("In the box", "محتويات العبوة"), ("Cabinet, basin, mirror, 2 sconces (no tap)",
                                              "دولاب وحوض ومرآة وكشّافان (بدون خلاط)")),
        ],
        variants=[dict(opts=[], price=21999)],
        rating=4.3, reviews=31,
        alt=("Black 60 cm bathroom vanity with a white basin and a framed mirror with two sconces",
             "وحدة حمام سوداء 60 سم بحوض أبيض ومرآة بإطار وكشّافين"),
        views=[FRONT, ANGLED, SIDE],
    ),
    dict(
        id=1906,
        name=("Hanging Egg Chair with Stand", "كرسي أرجوحة بيضاوي معلّق بحامل"),
        subtitle=("Your balcony's new favourite seat", "ركنك المفضل الجديد في البلكونة"),
        description=(
            "A black hand-woven wicker egg chair that hangs from a powder-coated steel stand, so there's no "
            "need to drill into the ceiling. The thick white cushion is water-repellent and the PE rattan is "
            "UV-resistant, so it's just as happy in the living room as on a sunny balcony. Holds up to 120 kg "
            "and assembles in about 20 minutes.",
            "كرسي بيضاوي من الخوص الصناعي الأسود المنسوج يدويًا، معلّق على حامل من الصلب المطلي فلا حاجة "
            "لتثقيب السقف. الوسادة البيضاء السميكة طاردة للماء والخوص مقاوم لأشعة الشمس، ليناسب الصالة أو "
            "البلكونة المشمسة. يتحمل حتى 120 كجم ويُركَّب في نحو 20 دقيقة.",
        ),
        highlights=[
            ("Freestanding steel stand — no ceiling drilling", "حامل مستقل من الصلب بدون تثقيب السقف"),
            ("UV-resistant PE rattan, water-repellent cushion", "خوص مقاوم للشمس ووسادة طاردة للماء"),
            ("Holds up to 120 kg", "يتحمل حتى 120 كجم"),
        ],
        specs=[
            (("Overall height", "الارتفاع الكلي"), ("195 cm", "195 سم")),
            (("Basket", "السلة"), ("105 × 70 × 65 cm", "105 × 70 × 65 سم")),
            (("Materials", "الخامات"), ("PE rattan, powder-coated steel", "خوص صناعي وصلب مطلي")),
            (("Maximum load", "أقصى حمولة"), ("120 kg", "120 كجم")),
            (("Weight", "الوزن"), ("28 kg", "28 كجم")),
        ],
        variants=[dict(opts=[], price=11999, compare_at=14999)],
        rating=4.7, reviews=286,
        alt=("Black rattan hanging egg chair with a white cushion on a steel stand",
             "كرسي بيضاوي معلّق من الخوص الأسود بوسادة بيضاء على حامل معدني"),
        views=[FRONT, ANGLED, ("side view", "منظر جانبي")],
    ),
    dict(
        id=1907,
        name=("Family Tree Wall Photo Frame Set", "طقم إطارات صور شجرة العائلة للحائط"),
        subtitle=("10 frames on a tree — your family's story", "10 براويز على شكل شجرة تحكي حكاية العيلة"),
        description=(
            "Turn a blank wall into a family gallery: a black tree-branch wall decal with ten black photo frames "
            "in mixed sizes and a 'Family' plaque. The frames have glass fronts and fit standard 10 × 15 cm and "
            "13 × 18 cm prints. Comes with a placement template and all the hanging hardware.",
            "حوّل الحائط الفارغ إلى معرض صور للعائلة: ملصق جداري لشجرة بفروع سوداء مع عشرة إطارات صور سوداء "
            "بمقاسات مختلفة ولوحة مكتوب عليها «Family». الإطارات بواجهة زجاجية وتناسب الصور المطبوعة مقاس "
            "10 × 15 سم و13 × 18 سم، ومعها قالب لتحديد الأماكن وكل مستلزمات التعليق.",
        ),
        highlights=[
            ("10 glass-front frames plus tree decal", "10 إطارات بواجهة زجاجية مع ملصق الشجرة"),
            ("Placement template included", "قالب تحديد الأماكن مرفق"),
            ("Fits 10 × 15 and 13 × 18 cm prints", "يناسب صور 10 × 15 و13 × 18 سم"),
        ],
        specs=[
            (("Overall size", "المساحة الكلية"), ("150 × 120 cm", "150 × 120 سم")),
            (("Frames", "الإطارات"), ("6 × (10 × 15 cm), 4 × (13 × 18 cm)", "6 إطارات 10 × 15 سم و4 إطارات 13 × 18 سم")),
            (("Materials", "الخامات"), ("MDF frames, glass, vinyl decal", "إطارات MDF وزجاج وملصق فينيل")),
            (("Colour", "اللون"), ("Black", "أسود")),
        ],
        variants=[dict(opts=[], price=1299)],
        rating=4.5, reviews=143,
        alt=("Family tree wall decal with ten black photo frames",
             "ملصق شجرة العائلة على الحائط مع عشرة إطارات صور سوداء"),
    ),
    dict(
        id=1908,
        name=("Artificial Boxwood Plant on Metal Stand", "نبات بوكسوود صناعي على حامل معدني"),
        subtitle=("Year-round green, zero watering", "خضرة طول السنة من غير ما تسقيها"),
        description=(
            "A lush artificial boxwood shrub set in a black bowl on a tall square-frame metal stand, ideal for "
            "an entrance, a corner or beside the TV unit. UV-treated leaves keep their colour near sunny windows, "
            "and it never needs watering or trimming — just wipe it with a damp cloth now and then.",
            "شجيرة بوكسوود صناعية كثيفة في وعاء أسود فوق حامل معدني مربع طويل، مثالية للمدخل أو الأركان أو "
            "بجوار وحدة التلفزيون. أوراقها معالجة ضد الأشعة فوق البنفسجية فتحتفظ بلونها قرب النوافذ المشمسة، "
            "ولا تحتاج إلى ري أو تقليم، ويكفي مسحها بقطعة قماش مبللة من وقت لآخر.",
        ),
        highlights=[
            ("UV-treated leaves that don't fade", "أوراق معالجة ضد الشمس لا يبهت لونها"),
            ("Black metal stand included", "حامل معدني أسود مرفق"),
            ("No watering, no mess", "بدون ري أو اتساخ"),
        ],
        specs=[
            (("Total height", "الارتفاع الكلي"), ("110 cm", "110 سم")),
            (("Stand", "الحامل"), ("30 × 30 × 75 cm, powder-coated iron", "30 × 30 × 75 سم، حديد مطلي")),
            (("Plant", "النبات"), ("PE leaves, UV-treated", "أوراق PE معالجة ضد الشمس")),
            (("Use", "الاستخدام"), ("Indoor and covered outdoor", "داخلي وخارجي مظلل")),
        ],
        variants=[dict(opts=[], price=1599)],
        rating=4.4, reviews=76,
        alt=("Artificial boxwood plant in a black bowl on a tall metal stand",
             "نبات بوكسوود صناعي في وعاء أسود على حامل معدني طويل"),
        views=[FRONT, ANGLED, ("side view", "منظر جانبي")],
    ),
    dict(
        id=1909,
        name=("Artificial Potted Plant in Matte Black Planter", "نبات صناعي في أصيص أسود مطفي"),
        subtitle=("Desk-size greenery that never wilts", "زرعة صغيرة للمكتب عمرها ما تدبل"),
        description=(
            "A dense, round-leaf artificial plant in a matte black ceramic-look planter, sized for desks, shelves "
            "and coffee tables. Soft-touch leaves with natural colour variation look real from across the room, "
            "and the weighted base keeps it steady.",
            "نبات صناعي كثيف بأوراق مستديرة في أصيص أسود مطفي بمظهر السيراميك، بحجم مناسب للمكاتب والأرفف "
            "وطاولات القهوة. أوراقه ناعمة الملمس وبدرجات لون طبيعية فتبدو حقيقية من بعيد، وقاعدته المثقّلة "
            "تبقيه ثابتًا.",
        ),
        highlights=[
            ("Natural-looking soft-touch leaves", "أوراق ناعمة بمظهر طبيعي"),
            ("Weighted planter that won't tip over", "أصيص مثقّل لا ينقلب"),
            ("Zero care needed", "لا يحتاج أي عناية"),
        ],
        specs=[
            (("Height", "الارتفاع"), ("38 cm", "38 سم")),
            (("Planter diameter", "قطر الأصيص"), ("22 cm", "22 سم")),
            (("Materials", "الخامات"), ("PE leaves, polyresin planter", "أوراق PE وأصيص من البولي ريزن")),
            (("Colour", "اللون"), ("Green / matte black", "أخضر / أسود مطفي")),
        ],
        variants=[dict(opts=[], price=899)],
        rating=4.3, reviews=98,
        alt=("Round-leaf artificial plant in a matte black planter",
             "نبات صناعي بأوراق مستديرة في أصيص أسود مطفي"),
        views=[FRONT, ANGLED, SIDE, ("from above", "منظر علوي")],
    ),
    dict(
        id=1910,
        name=("Palm-Tree Table Lamp with Amber Shade", "مصباح طاولة بقاعدة نخلة وغطاء كهرماني"),
        subtitle=("A warm amber glow with a tropical twist", "نور دافي بلمسة استوائية"),
        description=(
            "A statement table lamp with a hand-painted resin base sculpted as a palm trunk and fronds, topped "
            "with a bell shade that casts a warm amber glow. Perfect on a side table or console in a classic "
            "living room. Takes an E27 bulb (up to 40 W or LED equivalent) and has an inline on/off switch.",
            "مصباح طاولة لافت بقاعدة من الريزن المرسوم يدويًا على شكل جذع نخلة وسعفها، يعلوه غطاء جرسي يمنح "
            "إضاءة كهرمانية دافئة. مثالي على طاولة جانبية أو كونسول في صالون كلاسيكي. يعمل بلمبة E27 حتى 40 واط "
            "أو ما يعادلها من LED، مع مفتاح تشغيل على السلك.",
        ),
        highlights=[
            ("Hand-painted resin palm base", "قاعدة ريزن مرسومة يدويًا على شكل نخلة"),
            ("Warm amber fabric shade", "غطاء قماش بلون كهرماني دافئ"),
            ("E27 socket with inline switch", "دواية E27 ومفتاح على السلك"),
        ],
        specs=[
            (("Height", "الارتفاع"), ("68 cm", "68 سم")),
            (("Shade diameter", "قطر الغطاء"), ("35 cm", "35 سم")),
            (("Bulb", "اللمبة"), ("E27, max 40 W (not included)", "E27 بحد أقصى 40 واط (غير مرفقة)")),
            (("Cable", "السلك"), ("1.8 m with 2-pin plug", "1.8 متر بفيشة ثنائية")),
        ],
        variants=[dict(opts=[], price=1899, compare_at=2499)],
        rating=4.2, reviews=64,
        alt=("Table lamp with a palm-tree resin base and an amber bell shade",
             "مصباح طاولة بقاعدة ريزن على شكل نخلة وغطاء جرسي كهرماني"),
    ),
    dict(
        id=1911,
        name=("Stone-Effect Tissue Box Cover", "غطاء علبة مناديل بملمس الحجر"),
        subtitle=("Hide the cardboard, keep the tissues", "خبّي علبة الكرتون بشياكة"),
        description=(
            "A charcoal resin cover with a sculpted wave texture that slips over any standard rectangular tissue "
            "box. Felt pads underneath protect your furniture, and the open base makes refills a five-second "
            "job. Looks just as good on a bathroom counter as on an office desk.",
            "غطاء من الريزن بلون الفحم وملمس موجي محفور، يوضع فوق أي علبة مناديل مستطيلة عادية. تحمي وسادات "
            "اللباد في الأسفل أسطح الأثاث، والقاعدة المفتوحة تجعل تغيير العلبة في ثوانٍ. أنيق على رخامة الحمّام "
            "كما على مكتب العمل.",
        ),
        highlights=[
            ("Fits standard rectangular tissue boxes", "يناسب علب المناديل المستطيلة القياسية"),
            ("Felt pads protect surfaces", "وسادات لباد لحماية الأسطح"),
            ("Tissues not included", "المناديل غير مرفقة"),
        ],
        specs=[
            (("Outer size", "المقاس الخارجي"), ("26 × 14 × 10 cm", "26 × 14 × 10 سم")),
            (("Fits boxes up to", "يناسب علبًا حتى"), ("24 × 12 × 9 cm", "24 × 12 × 9 سم")),
            (("Material", "الخامة"), ("Polyresin", "بولي ريزن")),
            (("Colour", "اللون"), ("Charcoal", "رمادي فحمي")),
        ],
        variants=[dict(opts=[], price=449)],
        rating=4.4, reviews=39,
        alt=("Charcoal stone-effect tissue box cover with a wave texture",
             "غطاء علبة مناديل بلون الفحم بملمس حجري موجي"),
        views=[("front angle", "منظر أمامي مائل"), SIDE],
    ),

    # ------------------------------------------------------------------ Sports & Fitness
    dict(
        id=2101,
        name=("Wilson Official-Size American Football", "كرة قدم أمريكية ويلسون بالمقاس الرسمي"),
        subtitle=("Classic leather-look ball with white stripes", "الكورة الكلاسيكية بالخطوط البيضا"),
        description=(
            "An official-size American football with a pebbled composite cover for a sure grip, raised laces "
            "and painted white stripes. Built for flag-football leagues, school teams and weekend throws on "
            "the beach.",
            "كرة قدم أمريكية بالمقاس الرسمي بغلاف مركّب محبّب يمنح قبضة ثابتة، مع أربطة بارزة وخطوط بيضاء. "
            "مناسبة لدوريات الفلاج فوتبول وفرق المدارس ورميات نهاية الأسبوع على الشاطئ.",
        ),
        highlights=[
            ("Official size 9, ages 14+", "المقاس الرسمي 9 من سن 14 سنة"),
            ("Pebbled composite cover for grip", "غلاف مركّب محبّب لقبضة أفضل"),
            ("Raised laces for tight spirals", "أربطة بارزة لرميات حلزونية دقيقة"),
        ],
        specs=[
            (("Size", "المقاس"), ("Official (No. 9)", "رسمي (رقم 9)")),
            (("Cover", "الغلاف"), ("Composite leather", "جلد مركّب")),
            (("Bladder", "المثانة"), ("Butyl", "بيوتيل")),
            (("Recommended age", "السن المناسب"), ("14+", "من 14 سنة")),
        ],
        variants=[dict(opts=[], price=999)],
        rating=4.5, reviews=58,
        alt=("Brown Wilson American football with white stripes", "كرة قدم أمريكية بنية من ويلسون بخطوط بيضاء"),
    ),
    dict(
        id=2102,
        name=("Leather Baseball", "كرة بيسبول من الجلد"),
        subtitle=("Official size with raised red seams", "مقاس رسمي بخياطة حمرا بارزة"),
        description=(
            "A regulation 9-inch baseball with a full-grain leather cover, a cushioned cork centre and raised red "
            "seams for a confident grip on every pitch. Great for practice sessions, school clubs and league "
            "training. Sold individually.",
            "كرة بيسبول قانونية بمحيط 9 بوصات وغلاف من الجلد الطبيعي ومركز من الفلين المبطّن، مع خياطة حمراء "
            "بارزة لقبضة واثقة في كل رمية. مناسبة للتدريبات وأندية المدارس وتمارين الدوريات، وتُباع بالقطعة.",
        ),
        highlights=[
            ("Full-grain leather cover", "غلاف من الجلد الطبيعي"),
            ("Cushioned cork centre", "مركز من الفلين المبطّن"),
            ("Raised red seams", "خياطة حمراء بارزة"),
        ],
        specs=[
            (("Circumference", "المحيط"), ("9 in (23 cm)", "9 بوصات (23 سم)")),
            (("Weight", "الوزن"), ("145 g", "145 جرامًا")),
            (("Cover", "الغلاف"), ("Leather", "جلد طبيعي")),
            (("Core", "القلب"), ("Cork and rubber", "فلين ومطاط")),
        ],
        variants=[dict(opts=[], price=249)],
        rating=4.3, reviews=27,
        alt=("White leather baseball with red stitching", "كرة بيسبول بيضاء من الجلد بخياطة حمراء"),
    ),
    dict(
        id=2103,
        name=("Rawlings Leather Baseball Glove, 11.5-inch", "قفاز بيسبول جلد من رولينجز مقاس 11.5 بوصة"),
        subtitle=("Infield glove that feels broken-in", "جوانتي للملعب الداخلي، طري من أول ماتش"),
        description=(
            "An 11.5-inch infield glove made from oiled cowhide that's soft enough to play with on day one. The "
            "basket-weave web and padded palm help you scoop grounders and turn quick double plays. Worn on the "
            "left hand, for right-handed throwers.",
            "قفاز للملعب الداخلي مقاس 11.5 بوصة من جلد البقر المزيّت، طري بما يكفي للعب من أول يوم. تساعدك "
            "الشبكة بنسيج السلة وراحة القفاز المبطّنة على التقاط الكرات الأرضية وتنفيذ اللعبات السريعة. يُلبس "
            "في اليد اليسرى للاعبين الذين يرمون باليمنى.",
        ),
        highlights=[
            ("Oiled cowhide, game-ready", "جلد بقري مزيّت جاهز للعب"),
            ("Basket-weave web", "شبكة بنسيج السلة"),
            ("Left-hand fit for right-handed throwers", "لليد اليسرى للاعبين أصحاب اليد اليمنى"),
        ],
        specs=[
            (("Size", "المقاس"), ("11.5 in", "11.5 بوصة")),
            (("Position", "المركز"), ("Infield", "الملعب الداخلي")),
            (("Material", "الخامة"), ("Cowhide leather", "جلد بقري")),
            (("Hand", "اليد"), ("Worn on left hand", "يُلبس في اليد اليسرى")),
        ],
        variants=[dict(opts=[], price=2499)],
        rating=4.6, reviews=22,
        alt=("Brown leather Rawlings baseball glove", "قفاز بيسبول جلد بني من رولينجز"),
        views=[("pocket view", "من ناحية الجيب"), ANGLED, ("back view", "منظر خلفي")],
    ),
    dict(
        id=2104,
        name=("Wilson NBA Authentic Series Basketball, Size 7", "كرة سلة ويلسون NBA أوثنتيك مقاس 7"),
        subtitle=("Official size 7 · indoor and outdoor", "مقاس 7 الرسمي للصالة والملعب المفتوح"),
        description=(
            "The feel of the NBA game ball in a tough composite cover that handles parquet courts and outdoor "
            "asphalt alike. Deep channels and a moisture-wicking surface keep your grip sure through the fourth "
            "quarter. Official size 7 and weight for men's play.",
            "إحساس كرة دوري NBA بغلاف مركّب متين يتحمل أرضيات الصالات الخشبية وملاعب الأسفلت المفتوحة. تحافظ "
            "القنوات العميقة والسطح الماص للعرق على ثبات قبضتك حتى آخر ربع. مقاس 7 ووزن رسميان لمباريات الرجال.",
        ),
        highlights=[
            ("Indoor/outdoor composite leather", "جلد مركّب للصالات والملاعب المفتوحة"),
            ("Moisture-wicking grip", "سطح ماص للعرق لقبضة ثابتة"),
            ("Official size 7 (29.5 in)", "المقاس الرسمي 7 (29.5 بوصة)"),
        ],
        specs=[
            (("Size", "المقاس"), ("7 (75 cm circumference)", "7 (محيط 75 سم)")),
            (("Weight", "الوزن"), ("600 g", "600 جرام")),
            (("Cover", "الغلاف"), ("Composite leather", "جلد مركّب")),
            (("Court", "الملعب"), ("Indoor and outdoor", "صالات وملاعب مفتوحة")),
        ],
        variants=[dict(opts=[], price=1999)],
        rating=4.8, reviews=412,
        alt=("Orange Wilson NBA basketball", "كرة سلة ويلسون NBA برتقالية"),
    ),
    dict(
        id=2105,
        name=("Spalding Basketball Rim with Net", "طوق سلة سبالدينج مع الشبكة"),
        subtitle=("Regulation 45 cm steel rim, all-weather net", "طوق صلب 45 سم وشبكة تستحمل الشمس"),
        description=(
            "A regulation 45 cm (18-inch) steel rim with a weather-resistant orange powder coat and an all-weather "
            "nylon net. It fits most backboards with the standard 4-bolt pattern, making it an easy upgrade for "
            "school courts, clubs and home driveways. Mounting bolts included.",
            "طوق قانوني مقاس 45 سم (18 بوصة) من الصلب بطلاء برتقالي مقاوم للعوامل الجوية وشبكة نايلون تتحمل كل "
            "الأجواء. يناسب معظم اللوحات ذات فتحات التثبيت الأربع القياسية، فتركيبه سهل في ملاعب المدارس والأندية "
            "والبيوت. مسامير التثبيت مرفقة.",
        ),
        highlights=[
            ("Regulation 45 cm steel rim", "طوق صلب قانوني 45 سم"),
            ("All-weather nylon net", "شبكة نايلون لكل الأجواء"),
            ("Standard 4-bolt mounting, bolts included", "تثبيت قياسي بأربعة مسامير مرفقة"),
        ],
        specs=[
            (("Rim diameter", "قطر الطوق"), ("45 cm (18 in)", "45 سم (18 بوصة)")),
            (("Material", "الخامة"), ("Powder-coated steel", "صلب مطلي")),
            (("Net", "الشبكة"), ("All-weather nylon, 12 loops", "نايلون لكل الأجواء، 12 حلقة")),
            (("Mounting", "التثبيت"), ("4-bolt pattern", "4 مسامير")),
        ],
        variants=[dict(opts=[], price=2999)],
        rating=4.4, reviews=35,
        alt=("Orange Spalding basketball rim with a white net", "طوق سلة سبالدينج برتقالي مع شبكة بيضاء"),
    ),
    dict(
        id=2106,
        name=("Kookaburra Menace Cricket Ball, 156 g", "كرة كريكيت كوكابورا مينيس 156 جرامًا"),
        subtitle=("Red leather match ball for club cricket", "كورة جلد حمرا لماتشات الأندية"),
        description=(
            "A hand-stitched red leather ball from Kookaburra, built for club matches and serious net sessions. "
            "The four-piece construction and pronounced seam give bowlers real movement, while the cork core "
            "keeps its shape over long spells. Men's regulation weight of 156 g.",
            "كرة من الجلد الأحمر مخيطة يدويًا من كوكابورا، مصممة لمباريات الأندية وجلسات التدريب الجادة. يمنح "
            "تصميمها من أربع قطع وخياطتها البارزة الرامي حركة حقيقية للكرة، ويحافظ قلبها من الفلين على شكلها "
            "طوال الشوط. بالوزن القانوني للرجال 156 جرامًا.",
        ),
        highlights=[
            ("Hand-stitched 4-piece leather", "جلد من 4 قطع مخيط يدويًا"),
            ("Pronounced seam for swing", "خياطة بارزة لحركة أفضل للكرة"),
            ("Regulation 156 g", "الوزن القانوني 156 جرامًا"),
        ],
        specs=[
            (("Weight", "الوزن"), ("156 g", "156 جرامًا")),
            (("Cover", "الغلاف"), ("Alum-tanned leather, 4-piece", "جلد مدبوغ من 4 قطع")),
            (("Core", "القلب"), ("Cork and rubber", "فلين ومطاط")),
            (("Colour", "اللون"), ("Red", "أحمر")),
        ],
        variants=[dict(opts=[], price=599)],
        rating=4.5, reviews=18,
        alt=("Red Kookaburra Menace leather cricket ball", "كرة كريكيت كوكابورا مينيس من الجلد الأحمر"),
    ),
    dict(
        id=2107,
        name=("SG Kashmir Willow Cricket Bat — Full Size", "مضرب كريكيت إس جي من خشب الصفصاف الكشميري — مقاس كامل"),
        subtitle=("Short handle, big sweet spot", "يد قصيرة ومنطقة ضرب واسعة"),
        description=(
            "A full-size short-handle bat pressed from seasoned Kashmir willow, with a mid-to-low sweet spot "
            "that rewards drives along the ground. The cane handle and rubber grip soak up vibration, and the bat "
            "arrives pre-knocked and oiled, ready for leather-ball cricket after a short net session.",
            "مضرب بمقاس كامل ويد قصيرة من خشب الصفصاف الكشميري المعالَج، بمنطقة ضرب متوسطة إلى منخفضة مثالية "
            "للضربات الأرضية. اليد من الخيزران بمقبض مطاطي يمتص الاهتزاز، ويصلك المضرب مُجهَّزًا ومدهونًا بالزيت، "
            "جاهزًا للعب بالكرة الجلدية بعد جلسة تدريب قصيرة.",
        ),
        highlights=[
            ("Seasoned Kashmir willow", "خشب صفصاف كشميري معالَج"),
            ("Pre-knocked and oiled", "مُجهَّز ومدهون بالزيت"),
            ("Cane handle with rubber grip", "يد خيزران بمقبض مطاطي"),
        ],
        specs=[
            (("Size", "المقاس"), ("Full size, short handle (SH)", "مقاس كامل بيد قصيرة (SH)")),
            (("Weight", "الوزن"), ("1.15–1.25 kg", "1.15–1.25 كجم")),
            (("Wood", "الخشب"), ("Kashmir willow", "صفصاف كشميري")),
            (("Ball type", "نوع الكرة"), ("Leather ball", "كرة جلدية")),
        ],
        variants=[dict(opts=[], price=2999)],
        rating=4.4, reviews=26,
        alt=("SG Kashmir willow cricket bat with a black grip", "مضرب كريكيت إس جي من خشب الصفصاف الكشميري بمقبض أسود"),
    ),
    dict(
        id=2108,
        name=("Masuri Cricket Helmet with Steel Grille", "خوذة كريكيت ماسوري بشبكة من الصلب"),
        subtitle=("Pro-level protection in sky blue", "حماية بمستوى المحترفين باللون اللبني"),
        description=(
            "A Masuri helmet with a lightweight ABS shell, a shock-absorbing liner and an adjustable steel grille "
            "that keeps a clear view of the ball. Removable neck guards and a quick-dry, washable lining keep it "
            "comfortable through long innings in the summer heat. Certified to the BS 7928 batting-helmet standard.",
            "خوذة ماسوري بقشرة خفيفة من ABS وبطانة ممتصة للصدمات وشبكة صلب قابلة للضبط تمنح رؤية واضحة للكرة. "
            "واقيات الرقبة قابلة للفك والبطانة سريعة الجفاف وقابلة للغسيل لراحة تامة في الأشواط الطويلة تحت حرّ "
            "الصيف. معتمدة وفق مواصفة خوذات الضرب البريطانية BS 7928.",
        ),
        highlights=[
            ("Adjustable steel grille", "شبكة صلب قابلة للضبط"),
            ("Removable neck guards", "واقيات رقبة قابلة للفك"),
            ("Certified to BS 7928", "معتمدة وفق مواصفة BS 7928"),
        ],
        specs=[
            (("Shell", "القشرة"), ("ABS", "بلاستيك ABS")),
            (("Grille", "الشبكة"), ("Steel, adjustable", "صلب قابل للضبط")),
            (("Head size", "مقاس الرأس"), ("S 53–55 · M 55–58 · L 58–61 cm", "S 53–55 · M 55–58 · L 58–61 سم")),
            (("Weight", "الوزن"), ("About 750 g", "نحو 750 جرامًا")),
            (("Colour", "اللون"), ("Sky blue", "لبني")),
        ],
        options=[
            dict(id="size", name=("Size", "المقاس"), kind="chip", values=[
                dict(id="s", label=("S · 53–55 cm", "S · 53–55 سم"), code="S"),
                dict(id="m", label=("M · 55–58 cm", "M · 55–58 سم"), code="M"),
                dict(id="l", label=("L · 58–61 cm", "L · 58–61 سم"), code="L"),
            ]),
        ],
        variants=[
            dict(opts=["s"], price=7999),
            dict(opts=["m"], price=7999),
            dict(opts=["l"], price=7999),
        ],
        rating=4.7, reviews=14,
        alt=("Sky-blue Masuri cricket helmet with a steel grille", "خوذة كريكيت ماسوري لبني بشبكة من الصلب"),
        views=[("side view", "منظر جانبي"), FRONT, ("other side", "الجانب الآخر"), ("front angle", "منظر أمامي مائل")],
    ),
    dict(
        id=2109,
        name=("Slazenger Wooden Cricket Stumps Set", "طقم أوتاد كريكيت خشبية من سلازنجر"),
        subtitle=("Three stumps and two bails — match-ready", "3 أوتاد وعارضتين — جاهز للماتش"),
        description=(
            "Full-size ash stumps with steel-capped pointed tips that push easily into turf, plus a pair of "
            "turned bails. The classic Slazenger panther branding makes it a smart pick for school sports days, "
            "club nets and garden cricket.",
            "أوتاد بمقاس كامل من خشب الدردار بأطراف مدببة مقوّاة بالصلب تنغرس بسهولة في النجيل، ومعها عارضتان "
            "مخروطتان. شعار الفهد الكلاسيكي من سلازنجر يجعله اختيارًا أنيقًا ليوم الرياضة في المدرسة وتدريبات "
            "الأندية والكريكيت في الحديقة.",
        ),
        highlights=[
            ("Full-size 71 cm stumps", "أوتاد بمقاس كامل 71 سم"),
            ("Steel-capped tips for easy setup", "أطراف مقوّاة بالصلب لتثبيت سهل"),
            ("Pair of bails included", "عارضتان مرفقتان"),
        ],
        specs=[
            (("Height", "الارتفاع"), ("71 cm (28 in)", "71 سم (28 بوصة)")),
            (("Wood", "الخشب"), ("Ash", "دردار")),
            (("In the set", "محتويات الطقم"), ("3 stumps + 2 bails", "3 أوتاد وعارضتان")),
            (("Tips", "الأطراف"), ("Steel-capped", "مقوّاة بالصلب")),
        ],
        variants=[dict(opts=[], price=1499)],
        rating=4.3, reviews=11,
        alt=("Three Slazenger wooden cricket stumps with bails", "ثلاثة أوتاد كريكيت خشبية من سلازنجر مع العارضتين"),
    ),
    dict(
        id=2110,
        name=("Yonex Aerosensa 20 Feather Shuttlecocks (Tube of 12)", "ريش بادمنتون يونكس إيروسينسا 20 (علبة 12 ريشة)"),
        subtitle=("Goose-feather shuttles for club play", "ريش أوز طبيعي للعب في النادي"),
        description=(
            "Yonex's popular club-level shuttle with natural goose feathers and a cork base for true flight and "
            "consistent speed. A tube of 12 covers weeks of club sessions, and speed 77 suits warm indoor halls.",
            "ريشة يونكس الشهيرة لمستوى الأندية، بريش أوز طبيعي وقاعدة من الفلين لطيران حقيقي وسرعة ثابتة. "
            "علبة الـ12 ريشة تكفي أسابيع من تدريبات النادي، وسرعة 77 مناسبة للصالات المغلقة في الأجواء الدافئة.",
        ),
        highlights=[
            ("Natural goose feathers", "ريش أوز طبيعي"),
            ("Cork base for stable flight", "قاعدة فلين لطيران ثابت"),
            ("Speed 77 for warm halls", "سرعة 77 للصالات الدافئة"),
        ],
        specs=[
            (("Quantity", "العدد"), ("12 shuttlecocks", "12 ريشة")),
            (("Feathers", "الريش"), ("Goose", "ريش أوز")),
            (("Base", "القاعدة"), ("Cork", "فلين")),
            (("Speed", "السرعة"), ("77 (3)", "77 (الدرجة 3)")),
        ],
        variants=[dict(opts=[], price=1099)],
        rating=4.6, reviews=47,
        alt=("White Yonex feather shuttlecock", "ريشة بادمنتون بيضاء من يونكس"),
    ),
    dict(
        id=2111,
        name=("Classic 32-Panel Football", "كرة قدم كلاسيكية 32 رقعة"),
        subtitle=("The black-and-white ball every match needs", "الكورة الأبيض في أسود بتاعة كل ماتش"),
        description=(
            "The timeless black-and-white football with 32 machine-stitched panels, a butyl bladder that holds "
            "air for weeks and a soft PU cover. Tough enough for asphalt, five-a-side pitches and the beach. "
            "Choose size 5 for adults or size 4 for kids aged 8–12.",
            "كرة القدم الأبيض والأسود الخالدة، بـ32 رقعة مخيطة آليًا ومثانة بيوتيل تحتفظ بالهواء لأسابيع وغلاف "
            "PU ناعم. متينة بما يكفي للأسفلت وملاعب الخماسي والشاطئ. اختر مقاس 5 للكبار أو مقاس 4 للأطفال من 8 "
            "إلى 12 سنة.",
        ),
        highlights=[
            ("32 machine-stitched panels", "32 رقعة مخيطة آليًا"),
            ("Butyl bladder holds air longer", "مثانة بيوتيل تحتفظ بالهواء مدة أطول"),
            ("For asphalt, turf and sand", "للأسفلت والنجيل والرمل"),
        ],
        specs=[
            (("Cover", "الغلاف"), ("PU", "جلد صناعي PU")),
            (("Bladder", "المثانة"), ("Butyl", "بيوتيل")),
            (("Construction", "التصنيع"), ("32 panels, machine-stitched", "32 رقعة مخيطة آليًا")),
            (("Circumference", "المحيط"), ("Size 5: 68–70 cm · Size 4: 63–66 cm", "مقاس 5: 68–70 سم · مقاس 4: 63–66 سم")),
        ],
        options=[
            dict(id="size", name=("Size", "المقاس"), kind="chip", values=[
                dict(id="5", label=("Size 5 · adults", "مقاس 5 · للكبار"), code="S5"),
                dict(id="4", label=("Size 4 · ages 8–12", "مقاس 4 · من 8 إلى 12 سنة"), code="S4"),
            ]),
        ],
        variants=[
            dict(opts=["5"], price=449),
            dict(opts=["4"], price=399),
        ],
        rating=4.5, reviews=1840,
        alt=("Classic black-and-white 32-panel football", "كرة قدم كلاسيكية بالأبيض والأسود من 32 رقعة"),
    ),
    dict(
        id=2112,
        name=("Titleist Pro V1 Golf Balls (Dozen)", "كرات جولف تايتليست Pro V1 (12 كرة)"),
        subtitle=("Tour-level spin with a soft feel", "سبين وإحساس بمستوى المحترفين"),
        description=(
            "Titleist's flagship tour ball combines long, penetrating flight with drop-and-stop spin around the "
            "green and a soft, consistent feel. Ideal for low- to mid-handicap golfers playing the courses of "
            "New Cairo, Sheikh Zayed and El Gouna. Box of 12 white balls.",
            "كرة الجولف الرائدة من تايتليست تجمع بين طيران طويل وقوي وسبين يوقف الكرة قرب الحفرة، مع إحساس ناعم "
            "ومتسق في كل ضربة. مثالية لأصحاب الهانديكاب المنخفض والمتوسط في ملاعب القاهرة الجديدة والشيخ زايد "
            "والجونة. علبة من 12 كرة بيضاء.",
        ),
        highlights=[
            ("Penetrating flight off the tee", "طيران قوي وثابت من نقطة البداية"),
            ("Drop-and-stop control around the green", "تحكم ممتاز في الضربات القريبة من الحفرة"),
            ("Box of 12", "علبة من 12 كرة"),
        ],
        specs=[
            (("Quantity", "العدد"), ("12 balls", "12 كرة")),
            (("Construction", "التركيب"), ("3-piece", "3 طبقات")),
            (("Cover", "الغلاف"), ("Cast urethane elastomer", "يوريثان مصبوب")),
            (("Colour", "اللون"), ("White", "أبيض")),
        ],
        variants=[dict(opts=[], price=3999)],
        rating=4.8, reviews=64,
        alt=("White Titleist Pro V1 golf ball", "كرة جولف تايتليست Pro V1 بيضاء"),
    ),
    dict(
        id=2113,
        name=("Callaway 7-Iron, Steel Shaft", "مضرب جولف كالاواي حديد 7 بعمود صلب"),
        subtitle=("Forgiving cavity-back iron", "مضرب حديد سهل للي لسه بيطوّر لعبه"),
        description=(
            "A single 7-iron with a forgiving cavity-back head and a regular-flex steel shaft, perfect for "
            "filling a gap in your set or for range practice. The wide sole glides through turf and perimeter "
            "weighting keeps off-centre hits straighter. Right-handed, standard length, with a rubber grip.",
            "مضرب حديد 7 منفرد برأس مجوّف الظهر سهل الاستخدام وعمود صلب بمرونة عادية، مثالي لاستكمال طقمك أو "
            "للتدريب في ملعب الضربات. النعل العريض ينزلق بسهولة على العشب وتوزيع الوزن على الأطراف يحافظ على "
            "استقامة الضربات غير المركزية. لليد اليمنى بطول قياسي ومقبض مطاطي.",
        ),
        highlights=[
            ("Cavity-back head for forgiveness", "رأس مجوّف الظهر يسامح أخطاء الضربات"),
            ("Regular-flex steel shaft", "عمود صلب بمرونة عادية"),
            ("Right-handed, standard length", "لليد اليمنى بطول قياسي"),
        ],
        specs=[
            (("Club", "المضرب"), ("7-iron", "حديد 7")),
            (("Loft", "زاوية الميل"), ("30°", "30 درجة")),
            (("Shaft", "العمود"), ("Steel, regular flex", "صلب بمرونة عادية")),
            (("Length", "الطول"), ("37 in (94 cm)", "37 بوصة (94 سم)")),
            (("Hand", "اليد"), ("Right", "يمنى")),
        ],
        variants=[dict(opts=[], price=6999)],
        rating=4.5, reviews=19,
        alt=("Callaway steel-shaft 7-iron golf club", "مضرب جولف حديد 7 من كالاواي بعمود صلب"),
    ),
    dict(
        id=2114,
        name=("Aluminium Baseball Bat, 32-inch", "مضرب بيسبول من الألومنيوم 32 بوصة"),
        subtitle=("Light, balanced and built for practice", "خفيف ومتوازن ومناسب للتمرين"),
        description=(
            "A 32-inch alloy baseball bat with a balanced swing weight, a large sweet spot and a cushioned black "
            "grip that dampens sting on mishits. A durable choice for school clubs, batting practice and league "
            "training.",
            "مضرب بيسبول من سبيكة الألومنيوم بطول 32 بوصة، بوزن متوازن ومنطقة ضرب واسعة ومقبض أسود مبطّن يخفف "
            "الاهتزاز في الضربات غير الدقيقة. اختيار متين لأندية المدارس وتدريبات الضرب وتمارين الدوريات.",
        ),
        highlights=[
            ("Balanced swing weight", "وزن متوازن للضربة"),
            ("Cushioned grip reduces sting", "مقبض مبطّن يقلل الاهتزاز"),
            ("Durable aluminium alloy", "سبيكة ألومنيوم متينة"),
        ],
        specs=[
            (("Length", "الطول"), ("32 in (81 cm)", "32 بوصة (81 سم)")),
            (("Weight", "الوزن"), ("850 g", "850 جرامًا")),
            (("Material", "الخامة"), ("Aluminium alloy", "سبيكة ألومنيوم")),
            (("Grip", "المقبض"), ("Black synthetic, cushioned", "صناعي أسود مبطّن")),
        ],
        variants=[dict(opts=[], price=1199)],
        rating=4.2, reviews=16,
        alt=("Silver aluminium baseball bat with a black grip", "مضرب بيسبول فضي من الألومنيوم بمقبض أسود"),
    ),
    dict(
        id=2115,
        name=("Wilson Championship Tennis Balls", "كرات تنس ويلسون تشامبيونشيب"),
        subtitle=("Extra-duty felt for hard courts", "لباد متين يعيش على الهارد كورت"),
        description=(
            "Wilson Championship balls use an extra-duty felt that resists fluffing on the hard and clay courts "
            "found at most Egyptian clubs. Pressurised for a lively, consistent bounce and approved by the ITF. "
            "Buy a single can, or save with four cans for the season.",
            "كرات تشامبيونشيب من ويلسون بلباد مقوّى يقاوم التآكل على الملاعب الصلبة والترابية المنتشرة في الأندية "
            "المصرية. مضغوطة لارتداد حيوي ومتسق ومعتمدة من الاتحاد الدولي للتنس. اشترِ علبة واحدة أو وفّر مع "
            "أربع علب تكفي الموسم.",
        ),
        highlights=[
            ("Extra-duty felt", "لباد مقوّى للملاعب الصلبة"),
            ("Pressurised for a consistent bounce", "مضغوطة لارتداد ثابت"),
            ("ITF approved", "معتمدة من الاتحاد الدولي للتنس"),
        ],
        specs=[
            (("Balls per can", "عدد الكرات في العلبة"), ("3", "3 كرات")),
            (("Felt", "اللباد"), ("Extra duty", "مقوّى")),
            (("Court", "الملعب"), ("Hard and clay", "صلب وترابي")),
            (("Colour", "اللون"), ("Optic yellow", "أصفر فسفوري")),
        ],
        options=[
            dict(id="pack", name=("Pack", "العبوة"), kind="chip", values=[
                dict(id="1-can", label=("1 can · 3 balls", "علبة واحدة · 3 كرات"), code="1C"),
                dict(id="4-cans", label=("4 cans · 12 balls", "4 علب · 12 كرة"), code="4C"),
            ]),
        ],
        variants=[
            dict(opts=["1-can"], price=449),
            dict(opts=["4-cans"], price=1599),
        ],
        rating=4.6, reviews=233,
        alt=("Yellow Wilson tennis ball", "كرة تنس صفراء من ويلسون"),
    ),
    dict(
        id=2116,
        name=("Wilson Tour Slam Lite Tennis Racket", "مضرب تنس ويلسون تور سلام لايت"),
        subtitle=("Lightweight, oversized and pre-strung", "خفيف ورأسه كبير وجاي مشدود"),
        description=(
            "A lightweight aluminium racket with an oversized 112 sq in head that makes the sweet spot easy to "
            "find — ideal for beginners and recreational players. It arrives pre-strung with a cushioned grip, so "
            "you can head straight to the club courts.",
            "مضرب خفيف من الألومنيوم برأس كبير مساحته 112 بوصة مربعة يسهّل إصابة منطقة الضرب المثالية، مناسب "
            "للمبتدئين ولاعبي الهواية. يصلك مشدودًا ومزوّدًا بمقبض مبطّن، فتبدأ اللعب في النادي فورًا.",
        ),
        highlights=[
            ("Oversized 112 sq in head", "رأس كبير 112 بوصة مربعة"),
            ("Pre-strung and ready to play", "مشدود وجاهز للعب"),
            ("Light 275 g frame", "إطار خفيف 275 جرامًا"),
        ],
        specs=[
            (("Head size", "مساحة الرأس"), ("112 sq in", "112 بوصة مربعة")),
            (("Weight (strung)", "الوزن مشدودًا"), ("275 g", "275 جرامًا")),
            (("Grip size", "مقاس المقبض"), ("L3 (4 3/8)", "L3 (4 3/8 بوصة)")),
            (("Length", "الطول"), ("27.5 in", "27.5 بوصة")),
            (("Frame", "الإطار"), ("Aluminium", "ألومنيوم")),
        ],
        variants=[dict(opts=[], price=2499, compare_at=2999)],
        rating=4.3, reviews=121,
        alt=("Red Wilson tennis racket with a black grip", "مضرب تنس أحمر من ويلسون بمقبض أسود"),
    ),
    dict(
        id=2117,
        name=("Soft-Touch Volleyball, Size 5", "كرة طائرة ناعمة الملمس مقاس 5"),
        subtitle=("Gentle on the forearms, for beach and gym", "مريحة على الدراع، للبحر وللصالة"),
        description=(
            "A size 5 volleyball with a padded soft-touch synthetic leather cover that's kind to forearms during "
            "long rallies. The 18-panel blue, white and grey design is easy to track against the sky, and the "
            "butyl bladder keeps its pressure longer. Suitable for indoor courts, schools and beach play.",
            "كرة طائرة مقاس 5 بغلاف من الجلد الصناعي المبطّن الناعم، مريح للساعدين في التبادلات الطويلة. تصميمها "
            "من 18 رقعة بالأزرق والأبيض والرمادي يسهّل متابعتها في السماء، ومثانة البيوتيل تحافظ على الضغط مدة "
            "أطول. مناسبة للصالات والمدارس وملاعب الشاطئ.",
        ),
        highlights=[
            ("Padded soft-touch cover", "غلاف ناعم مبطّن"),
            ("High-visibility three-colour panels", "رقع بثلاثة ألوان واضحة الرؤية"),
            ("For indoor courts and the beach", "للصالات والشاطئ"),
        ],
        specs=[
            (("Size", "المقاس"), ("5", "مقاس 5")),
            (("Circumference", "المحيط"), ("65–67 cm", "65–67 سم")),
            (("Weight", "الوزن"), ("260–280 g", "260–280 جرامًا")),
            (("Cover", "الغلاف"), ("PU synthetic leather, 18 panels", "جلد صناعي PU من 18 رقعة")),
        ],
        variants=[dict(opts=[], price=599)],
        rating=4.4, reviews=88,
        alt=("Blue, white and grey volleyball", "كرة طائرة بالأزرق والأبيض والرمادي"),
    ),
    dict(
        id=2118,
        name=("Optimum Nutrition Performance Whey", "بروتين أوبتيموم نيوترشن بيرفورمانس واي"),
        subtitle=("24 g protein per scoop", "24 جرام بروتين في كل سكوب"),
        description=(
            "Optimum Nutrition Performance Whey blends whey concentrate, isolate and peptides for 24 g of protein "
            "per scoop, with naturally occurring BCAAs to support recovery after training. It mixes smoothly with "
            "water or milk using just a shaker. Sealed, authentic tubs with batch number and expiry date, stored in "
            "our air-conditioned warehouse.",
            "يجمع بيرفورمانس واي من أوبتيموم نيوترشن بين مركّز ومعزول بروتين الواي والببتيدات ليمنحك 24 جرامًا من "
            "البروتين في كل مغرفة، مع أحماض BCAA طبيعية لدعم الاستشفاء بعد التمرين. يذوب بسهولة في الماء أو الحليب "
            "باستخدام الشيكر فقط. عبوات أصلية مختومة برقم التشغيلة وتاريخ الصلاحية، ومخزّنة في مستودع مكيّف.",
        ),
        highlights=[
            ("24 g protein per scoop", "24 جرامًا من البروتين في كل مغرفة"),
            ("5.5 g naturally occurring BCAAs", "5.5 جرام أحماض BCAA طبيعية"),
            ("Sealed, authentic stock with expiry date", "عبوات أصلية مختومة بتاريخ صلاحية"),
        ],
        specs=[
            (("Protein per serving", "البروتين في الحصة"), ("24 g", "24 جرامًا")),
            (("Serving size", "حجم الحصة"), ("1 scoop (31 g)", "مغرفة واحدة (31 جرامًا)")),
            (("Servings", "عدد الحصص"), ("About 29 (2 lb) · about 73 (5 lb)", "نحو 29 (2 رطل) · نحو 73 (5 رطل)")),
            (("Protein sources", "مصادر البروتين"), ("Whey concentrate, isolate, peptides",
                                                    "مركّز ومعزول الواي والببتيدات")),
        ],
        options=[
            dict(id="size", name=("Size", "الحجم"), kind="chip", values=[
                dict(id="2lb", label=("2 lb · 907 g", "2 رطل · 907 جرامات"), code="2LB"),
                dict(id="5lb", label=("5 lb · 2.27 kg", "5 رطل · 2.27 كجم"), code="5LB"),
            ]),
            dict(id="flavour", name=("Flavour", "النكهة"), kind="chip", values=[
                dict(id="chocolate", label=("Double Rich Chocolate", "شوكولاتة دبل ريتش"), code="CHO"),
                dict(id="vanilla", label=("Vanilla Ice Cream", "آيس كريم فانيليا"), code="VAN"),
            ]),
        ],
        variants=[
            dict(opts=["2lb", "chocolate"], price=3499),
            dict(opts=["2lb", "vanilla"], price=3499),
            dict(opts=["5lb", "chocolate"], price=7999),
            dict(opts=["5lb", "vanilla"], price=7999),
        ],
        rating=4.7, reviews=2316,
        alt=("Black tub of Optimum Nutrition Performance Whey", "عبوة سوداء من بروتين أوبتيموم نيوترشن بيرفورمانس واي"),
    ),
]

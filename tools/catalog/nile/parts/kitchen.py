"""Kitchen & Dining (category 20): products 2001–2030."""

DISHWASHER = (("Dishwasher safe", "آمن في غسالة الأطباق"), ("Yes", "نعم"))

PRODUCTS = [
    dict(
        id=2001,
        name=("Bamboo Cooking Spatula", "ملعقة طهي من البامبو"),
        subtitle=("Won't scratch your non-stick pans", "بامبو طبيعي مش بيخدش الطاسات التيفال"),
        description=(
            "A slim bamboo spatula with an angled edge for stirring, flipping and scraping the pan clean. "
            "Bamboo is naturally hard and light, stays cool in the hand and is gentle on non-stick coatings. "
            "Part of Nile Basics, our own range of sturdy, food-safe kitchen tools at everyday prices.",
            "ملعقة طهي رفيعة من البامبو بطرف مائل للتقليب وقلب الأكل وكشط قاع المقلاة بسهولة. "
            "البامبو خامة طبيعية صلبة وخفيفة لا تسخن في اليد ولا تخدش الطبقات غير اللاصقة. "
            "من نايل بيسكس، تشكيلتنا الخاصة من أدوات المطبخ المتينة والآمنة على الطعام بأسعار يومية.",
        ),
        highlights=[
            ("Safe on non-stick cookware", "آمنة على الأواني غير اللاصقة"),
            ("Naturally hard, lightweight bamboo", "بامبو طبيعي صلب وخفيف الوزن"),
            ("Angled edge scrapes the pan clean", "طرف مائل يكشط قاع المقلاة بسهولة"),
        ],
        specs=[
            (("Material", "الخامة"), ("Natural bamboo", "بامبو طبيعي")),
            (("Length", "الطول"), ("30 cm", "30 سم")),
            (("Use", "الاستخدام"), ("Stirring, flipping, serving", "التقليب والقلب والتقديم")),
            (("Care", "العناية"), ("Hand wash, dry upright", "غسيل يدوي وتجفيف وهي واقفة")),
        ],
        options=[],
        variants=[dict(opts=[], price=89)],
        rating=4.5, reviews=214,
        alt=("Bamboo cooking spatula with an angled edge", "ملعقة طهي من البامبو بطرف مائل"),
    ),
    dict(
        id=2002,
        name=("Black Aluminium Camping Mug, 350 ml", "كوب رحلات من الألومنيوم الأسود 350 مل"),
        subtitle=("Light, tough and along for every trip", "خفيف وجامد ومعاك في كل خروجة"),
        description=(
            "A lightweight 350 ml mug in matte black aluminium for camping trips, the car or the office desk. "
            "The single-wall body shrugs off knocks and the wide handle is comfortable to hold. "
            "It isn't insulated, so give hot drinks a minute before the first sip.",
            "كوب خفيف سعة 350 مل من الألومنيوم الأسود المطفي، مناسب للتخييم والسيارة ومكتب الشغل. "
            "جسمه أحادي الجدار يتحمل الخبطات، ومقبضه العريض مريح في المسك. "
            "الكوب غير معزول حراريًا، فاترك المشروب الساخن يهدأ دقيقة قبل أول رشفة.",
        ),
        highlights=[
            ("Feather-light aluminium, 350 ml", "ألومنيوم خفيف جدًا بسعة 350 مل"),
            ("Matte black finish that hides scuffs", "لون أسود مطفي يخفي الخدوش"),
            ("Made for camping, the car and the office", "مثالي للتخييم والسيارة والمكتب"),
        ],
        specs=[
            (("Material", "الخامة"), ("Aluminium, matte black coating", "ألومنيوم بطلاء أسود مطفي")),
            (("Capacity", "السعة"), ("350 ml", "350 مل")),
            (("Wall", "الجدار"), ("Single wall (not insulated)", "جدار واحد (غير معزول)")),
            (("Care", "العناية"), ("Hand wash", "غسيل يدوي")),
        ],
        options=[],
        variants=[dict(opts=[], price=149)],
        rating=4.2, reviews=96,
        alt=("Black aluminium camping mug", "كوب رحلات من الألومنيوم الأسود"),
        views=[("side view", "منظر جانبي"), ("angled view of the rim", "منظر مائل يُظهر الحافة")],
    ),
    dict(
        id=2003,
        name=("Stainless Steel Balloon Whisk, 25 cm", "مخفقة يدوية من الستانلس ستيل 25 سم"),
        subtitle=("Smooth béchamel, no lumps", "بشاميل ناعم من غير كُتل"),
        description=(
            "Springy stainless-steel wires beat air into eggs and cream and stir the lumps out of béchamel "
            "or custard in seconds. The black soft-touch handle gives a secure grip, and the hanging loop "
            "keeps it within reach of the stove.",
            "مخفقة بأسلاك ستانلس ستيل مرنة تُدخل الهواء في البيض والكريمة، وتفك أي كتل في البشاميل "
            "أو الكاسترد في ثوانٍ. مقبضها الأسود ناعم الملمس وثابت في اليد، وبه حلقة للتعليق لتبقى "
            "بجوار الموقد دائمًا.",
        ),
        highlights=[
            ("Springy stainless-steel balloon head", "رأس منتفخ بأسلاك ستانلس ستيل مرنة"),
            ("Soft-touch grip with hanging loop", "مقبض ناعم الملمس بحلقة للتعليق"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless-steel wires, polypropylene handle",
                                     "أسلاك ستانلس ستيل ومقبض بولي بروبيلين")),
            (("Length", "الطول"), ("25 cm", "25 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=129)],
        rating=4.4, reviews=158,
        alt=("Stainless steel balloon whisk with a black handle", "مخفقة ستانلس ستيل بمقبض أسود"),
    ),
    dict(
        id=2004,
        name=("Tornado Blender with 1.5 L Glass Jar, 600 W", "خلاط تورنيدو بإبريق زجاجي 1.5 لتر، 600 واط"),
        subtitle=("Smoothies, juices and crushed ice", "عصاير وسموزي وتلج مجروش في ثواني"),
        description=(
            "A sturdy 600 W blender with a brushed-stainless base and a thick 1.5-litre glass jar that won't "
            "stain or hold on to smells. The blue dial gives two speeds plus pulse, and the four-blade "
            "stainless cutter handles frozen fruit, ice and Ramadan favourites like karkadeh and tamarind. "
            "Backed by a 2-year Tornado warranty in Egypt.",
            "خلاط قوي بقدرة 600 واط بقاعدة من الستانلس ستيل المصقول وإبريق زجاجي سميك سعة 1.5 لتر "
            "لا يتشرّب الألوان ولا الروائح. يعمل بسرعتين مع خاصية النبض من القرص الأزرق، وسكينته الرباعية "
            "من الستانلس تفرم الفاكهة المجمدة والثلج وتجهّز مشروبات رمضان مثل الكركديه والتمر هندي. "
            "مع ضمان تورنيدو لمدة سنتين في مصر.",
        ),
        highlights=[
            ("600 W motor, 2 speeds + pulse", "موتور 600 واط بسرعتين وخاصية النبض"),
            ("1.5 L glass jar: no stains, no smells", "إبريق زجاج 1.5 لتر لا يحتفظ بالألوان أو الروائح"),
            ("Four-blade stainless cutter crushes ice", "سكينة ستانلس رباعية تجرش الثلج"),
            ("2-year Tornado warranty", "ضمان تورنيدو لمدة سنتين"),
        ],
        specs=[
            (("Power", "القدرة"), ("600 W", "600 واط")),
            (("Jar", "الإبريق"), ("1.5 L glass", "زجاج 1.5 لتر")),
            (("Speeds", "السرعات"), ("2 + pulse", "سرعتان + نبض")),
            (("Blade", "السكين"), ("4-blade stainless steel", "رباعية من الستانلس ستيل")),
            (("Base", "القاعدة"), ("Brushed stainless steel", "ستانلس ستيل مصقول")),
            (("Warranty", "الضمان"), ("2 years", "سنتان")),
        ],
        options=[],
        variants=[dict(opts=[], price=2799, compare_at=3499)],
        rating=4.3, reviews=612,
        alt=("Tornado blender with a glass jar and stainless base", "خلاط تورنيدو بإبريق زجاجي وقاعدة ستانلس"),
        views=[
            ("front view", "من الأمام"),
            ("three-quarter view", "منظر جانبي مائل"),
            ("side view with the power cord", "من الجنب مع سلك الكهرباء"),
            ("angled view from the right", "منظر مائل من اليمين"),
        ],
    ),
    dict(
        id=2005,
        name=("Carbon Steel Wok with Wooden Handle, 32 cm", "مقلاة ووك من الكربون ستيل بمقبض خشبي 32 سم"),
        subtitle=("High-heat stir-fries, crisp every time", "للتشويح على نار عالية والأكل يطلع مقرمش"),
        description=(
            "A 32 cm carbon-steel wok that heats fast and takes real high heat for stir-fries, fried rice "
            "and shrimp. Season it once and it builds a natural non-stick patina that gets better with every "
            "use. The long wooden handle and the helper grip stay cool over the gas flame.",
            "مقلاة ووك قطرها 32 سم من الكربون ستيل تسخن بسرعة وتتحمل النار العالية، مثالية للتشويح والأرز "
            "المقلي والجمبري. بعد تسوية سطحها مرة واحدة تتكوّن عليها طبقة طبيعية غير لاصقة تتحسن مع كل "
            "استخدام. المقبض الخشبي الطويل والمقبض المساعد يظلان باردين فوق نار البوتاجاز.",
        ),
        highlights=[
            ("Heats fast, handles very high heat", "تسخن بسرعة وتتحمل الحرارة العالية"),
            ("Builds a natural non-stick patina", "تكتسب طبقة طبيعية غير لاصقة مع الاستخدام"),
            ("Cool-touch wooden handle + helper grip", "مقبض خشبي لا يسخن ومقبض مساعد"),
        ],
        specs=[
            (("Material", "الخامة"), ("Carbon steel", "كربون ستيل")),
            (("Diameter", "القطر"), ("32 cm", "32 سم")),
            (("Hob", "الموقد"), ("Gas and electric", "غاز وكهرباء")),
            (("Handles", "المقابض"), ("Wood", "خشب")),
            (("Care", "العناية"), ("Hand wash, dry and oil lightly", "غسيل يدوي ثم تجفيف ودهان خفيف بالزيت")),
        ],
        options=[],
        variants=[dict(opts=[], price=899)],
        rating=4.4, reviews=187,
        alt=("Carbon steel wok with wooden handles", "مقلاة ووك من الكربون ستيل بمقابض خشبية"),
    ),
    dict(
        id=2006,
        name=("Beech-Wood Chopping Board, 40 × 25 cm", "لوح تقطيع من خشب الزان 40 × 25 سم"),
        subtitle=("Kind to knives, roomy enough for mahshi prep", "مريح للسكاكين وواسع لتجهيز المحشي"),
        description=(
            "A solid beech board, 40 × 25 cm with one rounded end, roomy enough to chop a whole batch of onions "
            "or prep the vegetables for mahshi. Beech is dense and smooth, so it's gentle on knife edges and "
            "doesn't splinter. Rub it with a little oil now and then to keep it from drying out.",
            "لوح تقطيع من خشب الزان المصمت بمقاس 40 × 25 سم وطرف دائري، واسع بما يكفي لتقطيع كمية بصل كاملة "
            "أو تجهيز خضار المحشي. خشب الزان كثيف وناعم فلا يُتلف حد السكين ولا يتشظى. "
            "ادهنه بقليل من الزيت من وقت لآخر حتى لا يجف.",
        ),
        highlights=[
            ("Solid beech, gentle on knife edges", "خشب زان مصمت يحافظ على حدة السكين"),
            ("Generous 40 × 25 cm surface", "مساحة واسعة 40 × 25 سم"),
            ("Oil occasionally, keep out of the dishwasher", "يُدهن بالزيت أحيانًا ولا يوضع في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Solid beech wood", "خشب زان مصمت")),
            (("Size", "المقاس"), ("40 × 25 cm", "40 × 25 سم")),
            (("Thickness", "السُمك"), ("2 cm", "2 سم")),
            (("Care", "العناية"), ("Hand wash, oil occasionally", "غسيل يدوي ودهان بالزيت أحيانًا")),
        ],
        options=[],
        variants=[dict(opts=[], price=349)],
        rating=4.6, reviews=341,
        alt=("Beech-wood chopping board with a rounded end", "لوح تقطيع من خشب الزان بطرف دائري"),
    ),
    dict(
        id=2007,
        name=("Paşabahçe Glass Citrus Squeezer", "عصّارة حمضيات زجاجية من باشاباتشي"),
        subtitle=("Fresh lemon for every salad and plate of ful", "ليمون فريش لكل سلطة وطبق فول"),
        description=(
            "A clear glass bowl with a pour spout and a bright yellow reamer that holds back the seeds. "
            "Twist half a lemon or orange and the juice runs straight into the bowl, ready for salad dressings, "
            "tahini and ful. The parts come apart for easy washing.",
            "وعاء زجاجي شفاف بفوهة للصب ومخروط عصر أصفر يحجز البذور. "
            "اعصر نصف ليمونة أو برتقالة بلفّة يد واحدة لينزل العصير مباشرة في الوعاء، جاهزًا لصوص السلطة "
            "والطحينة والفول. الأجزاء تنفصل بسهولة للغسيل.",
        ),
        highlights=[
            ("Seed-catching reamer", "مخروط عصر يحجز البذور"),
            ("Glass bowl with pour spout", "وعاء زجاجي بفوهة للصب"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Glass bowl, food-grade plastic reamer",
                                     "وعاء زجاج ومخروط بلاستيك آمن على الطعام")),
            (("Capacity", "السعة"), ("300 ml", "300 مل")),
            (("Diameter", "القطر"), ("14 cm", "14 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=179)],
        rating=4.5, reviews=263,
        alt=("Glass citrus squeezer with a yellow reamer", "عصّارة حمضيات زجاجية بمخروط أصفر"),
    ),
    dict(
        id=2008,
        name=("Egg Slicer", "قطّاعة بيض"),
        subtitle=("Neat slices for salads and sandwiches", "شرايح بيض متساوية للسلطة والساندوتشات"),
        description=(
            "Lay a boiled egg in the cradle, press down, and taut steel wires cut it into even slices in one go. "
            "Turn the egg and press again for cubes, handy for potato salad, sandwiches and school lunch boxes. "
            "It rinses clean in seconds.",
            "ضع البيضة المسلوقة في التجويف واضغط، فتقطعها الأسلاك الستانلس المشدودة إلى شرائح متساوية دفعة "
            "واحدة. لفّ البيضة واضغط مرة ثانية لتحصل على مكعبات، مناسبة لسلطة البطاطس والسندويتشات ولانش بوكس "
            "المدرسة. تُشطف وتنظف في ثوانٍ.",
        ),
        highlights=[
            ("Taut stainless-steel wires", "أسلاك ستانلس ستيل مشدودة"),
            ("Slices or cubes in seconds", "شرائح أو مكعبات في ثوانٍ"),
            ("Dishwasher safe (top rack)", "آمنة في غسالة الأطباق (الرف العلوي)"),
        ],
        specs=[
            (("Material", "الخامة"), ("ABS frame, stainless-steel wires", "إطار ABS وأسلاك ستانلس ستيل")),
            (("Size", "المقاس"), ("13 × 9 cm", "13 × 9 سم")),
            (("Colour", "اللون"), ("Yellow", "أصفر")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=99)],
        rating=4.3, reviews=129,
        alt=("Yellow egg slicer with steel wires", "قطّاعة بيض صفراء بأسلاك ستانلس"),
    ),
    dict(
        id=2009,
        name=("Beko 60 cm Built-in Ceramic Hob, 4 Zones", "سطح طهي سيراميك مدمج من بيكو 60 سم بأربع شعلات"),
        subtitle=("Sleek black glass, one wipe to clean", "زجاج أسود شيك وتنضيفه بمسحة واحدة"),
        description=(
            "A 60 cm built-in ceramic hob with four radiant zones under smooth black glass-ceramic, run by "
            "front knobs with nine power levels. Residual-heat indicators warn you while a zone is still hot, "
            "and the flat surface wipes clean in seconds. It fits a standard 56 × 49 cm worktop cut-out and "
            "needs its own electrical line; add professional installation at checkout.",
            "سطح طهي سيراميك مدمج مقاس 60 سم بأربع شعلات كهربائية تحت سطح زجاجي أسود أملس، يُتحكم فيه "
            "بمفاتيح أمامية بتسع درجات حرارة. مؤشرات الحرارة المتبقية تنبهك طالما الشعلة ما زالت ساخنة، "
            "والسطح المستوي يُنظف بمسحة واحدة. يناسب فتحة رخامة قياسية 56 × 49 سم ويحتاج خط كهرباء مستقلًا، "
            "ويمكنك إضافة التركيب الاحترافي مع الطلب.",
        ),
        highlights=[
            ("4 radiant zones, 9 power levels", "4 شعلات كهربائية و9 درجات حرارة"),
            ("Residual-heat indicators", "مؤشرات للحرارة المتبقية"),
            ("Easy-clean glass-ceramic surface", "سطح زجاج سيراميك سهل التنظيف"),
            ("Optional professional installation", "تركيب احترافي اختياري"),
        ],
        specs=[
            (("Type", "النوع"), ("Built-in radiant ceramic", "سيراميك كهربائي مدمج")),
            (("Width", "العرض"), ("60 cm", "60 سم")),
            (("Zones", "الشعلات"), ("4 (2 × 14.5 cm, 2 × 18 cm)", "4 (2 × 14.5 سم و2 × 18 سم)")),
            (("Total power", "القدرة الكلية"), ("6.0 kW", "6.0 كيلوواط")),
            (("Controls", "التحكم"), ("Front knobs", "مفاتيح أمامية")),
            (("Cut-out", "فتحة التركيب"), ("56 × 49 cm", "56 × 49 سم")),
        ],
        options=[],
        variants=[dict(opts=[], price=13999)],
        modifier_groups=[
            dict(id="install", name=("Installation", "التركيب"), selection="single", min=0, max=1, required=False,
                 options=[dict(id="pro-install", name=("Professional installation", "تركيب احترافي"), price=449)]),
            dict(id="warranty", name=("Extended warranty", "ضمان إضافي"), selection="single", min=0, max=1,
                 required=False,
                 options=[dict(id="plus-2y", name=("+2 years extended warranty", "ضمان إضافي لمدة سنتين"),
                               price=1299)]),
        ],
        rating=4.4, reviews=87,
        alt=("Beko 60 cm black ceramic hob with front knobs", "سطح طهي سيراميك أسود من بيكو 60 سم بمفاتيح أمامية"),
        views=[
            ("front view with the control knobs", "من الأمام مع مفاتيح التحكم"),
            ("angled view", "منظر مائل"),
            ("rear view", "من الخلف"),
            ("top view of the cooking zones", "منظر علوي للشعلات"),
        ],
    ),
    dict(
        id=2010,
        name=("Fine Mesh Strainer, 16 cm", "مصفاة بشبكة ناعمة 16 سم"),
        subtitle=("For tea, sauces and sifting flour", "للشاي والصوصات ونخل الدقيق"),
        description=(
            "A 16 cm fine stainless mesh strainer with a long handle and a hook that rests across pots and "
            "bowls. Use it to strain tea and fresh juice, rinse lentils and rice, or sift flour and icing "
            "sugar for ka'k.",
            "مصفاة ستانلس ستيل بشبكة ناعمة قطرها 16 سم، بمقبض طويل وخطاف يرتكز على حافة القدور والأوعية. "
            "استخدمها لتصفية الشاي والعصير الطازج، وغسل العدس والأرز، ونخل الدقيق والسكر البودرة للكحك.",
        ),
        highlights=[
            ("Fine stainless-steel mesh", "شبكة ستانلس ستيل ناعمة"),
            ("Hook rests across pots and bowls", "خطاف يرتكز على حافة القدور والأوعية"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless steel", "ستانلس ستيل")),
            (("Diameter", "القطر"), ("16 cm", "16 سم")),
            (("Length", "الطول"), ("34 cm", "34 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=119)],
        rating=4.5, reviews=203,
        alt=("Fine mesh stainless steel strainer", "مصفاة ستانلس ستيل بشبكة ناعمة"),
    ),
    dict(
        id=2011,
        name=("Zahran Stainless Steel Dinner Forks", "شوك طعام من الستانلس ستيل من زهران"),
        subtitle=("Everyday cutlery that keeps its shine", "شوك للاستعمال اليومي بيفضل لامع سنين"),
        description=(
            "Classic dinner forks in heavy-gauge 18/10 stainless steel with a mirror polish and a balanced, "
            "rounded handle. Made in Egypt by Zahran, a long-standing name in Egyptian kitchens, they go "
            "straight into the dishwasher and won't rust or bend.",
            "شوك طعام كلاسيكية من الستانلس ستيل 18/10 السميك بلمعة المرآة ومقبض مستدير متوازن. "
            "صناعة مصرية من زهران، الاسم العريق في المطبخ المصري، تدخل غسالة الأطباق مباشرة "
            "ولا تصدأ ولا تنثني.",
        ),
        highlights=[
            ("18/10 stainless steel, mirror finish", "ستانلس ستيل 18/10 بلمعة المرآة"),
            ("Rust-resistant and dishwasher safe", "مقاومة للصدأ وآمنة في غسالة الأطباق"),
            ("Sets of 6 or 12", "طقم 6 أو 12 قطعة"),
        ],
        specs=[
            (("Material", "الخامة"), ("18/10 stainless steel", "ستانلس ستيل 18/10")),
            (("Length", "الطول"), ("20 cm", "20 سم")),
            (("Finish", "التشطيب"), ("Mirror polish", "تلميع مرآة")),
            DISHWASHER,
            (("Made in", "بلد الصنع"), ("Egypt", "مصر")),
        ],
        options=[
            dict(id="set", name=("Set", "الطقم"), kind="chip", values=[
                dict(id="6", label=("Set of 6", "6 قطع"), code="6PC"),
                dict(id="12", label=("Set of 12", "12 قطعة"), code="12PC"),
            ]),
        ],
        variants=[dict(opts=["6"], price=249), dict(opts=["12"], price=449)],
        rating=4.6, reviews=418,
        alt=("Zahran stainless steel dinner fork", "شوكة طعام ستانلس ستيل من زهران"),
    ),
    dict(
        id=2012,
        name=("Paşabahçe Tumblers, 300 ml, Set of 6", "طقم أكواب باشاباتشي 300 مل، 6 قطع"),
        subtitle=("Clear, sturdy glasses for every day", "كوبايات شفافة وتقيلة للاستعمال اليومي"),
        description=(
            "Six clear 300 ml tumblers with a weighted base that feels solid in the hand and stays steady on "
            "the table. Made by Paşabahçe in crystal-clear glass, they're right for water, juice and Ramadan "
            "qamar el-din, and safe in the dishwasher.",
            "ستة أكواب شفافة سعة 300 مل بقاعدة سميكة تمنحها ثباتًا على السفرة وإحساسًا متينًا في اليد. "
            "مصنوعة من زجاج باشاباتشي النقي، مناسبة للمياه والعصير وقمر الدين في رمضان، "
            "وآمنة في غسالة الأطباق.",
        ),
        highlights=[
            ("Weighted base, steady on the table", "قاعدة سميكة ثابتة على السفرة"),
            ("Crystal-clear glass", "زجاج نقي شفاف"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Capacity", "السعة"), ("300 ml each", "300 مل للكوب")),
            (("Pieces", "عدد القطع"), ("6 tumblers", "6 أكواب")),
            (("Material", "الخامة"), ("Soda-lime glass", "زجاج صودا لايم")),
            (("Height", "الارتفاع"), ("10 cm", "10 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=299)],
        rating=4.7, reviews=2184,
        alt=("Paşabahçe clear glass tumbler", "كوب زجاج شفاف من باشاباتشي"),
    ),
    dict(
        id=2013,
        name=("Four-Sided Box Grater", "مبشرة رباعية الأوجه"),
        subtitle=("Cheese, carrots and zest with one tool", "جبنة وجزر وقشر ليمون بأداة واحدة"),
        description=(
            "Four stainless-steel faces, coarse, medium, fine and slicing, cover everything from rumi cheese "
            "and carrots to garlic and lemon zest. The non-slip black base keeps it steady on the board and the "
            "top handle gives a firm grip.",
            "أربعة أوجه من الستانلس ستيل: خشن ومتوسط وناعم وشرائح، لبشر كل شيء من الجبنة الرومي والجزر "
            "إلى الثوم وقشر الليمون. قاعدتها السوداء المانعة للانزلاق تثبتها على اللوح، ومقبضها العلوي "
            "يمنحك قبضة محكمة.",
        ),
        highlights=[
            ("4 grating and slicing faces", "4 أوجه للبشر والتقطيع"),
            ("Non-slip base", "قاعدة مانعة للانزلاق"),
            ("Stainless steel, dishwasher safe", "ستانلس ستيل وآمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless steel, plastic handle and base", "ستانلس ستيل بمقبض وقاعدة بلاستيك")),
            (("Height", "الارتفاع"), ("24 cm", "24 سم")),
            (("Faces", "الأوجه"), ("4: coarse, medium, fine, slicer", "4: خشن ومتوسط وناعم وشرائح")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=199)],
        rating=4.5, reviews=276,
        alt=("Four-sided stainless box grater with a black handle", "مبشرة ستانلس رباعية الأوجه بمقبض أسود"),
    ),
    dict(
        id=2014,
        name=("Moulinex Hand Blender, 600 W", "خلاط يدوي مولينكس 600 واط"),
        subtitle=("Soups and baby food, right in the pot", "شوربة وأكل أطفال في الحلة على طول"),
        description=(
            "Blend soups smooth right in the pot, whip up baby food or make a quick smoothie in a glass. "
            "The 600 W motor drives a stainless-steel blending foot with sharp four-wing blades, and a "
            "two-speed button with turbo puts control at your fingertip. The foot clicks off for washing.",
            "اخلط الشوربة مباشرة في القدر، وجهّز أكل الأطفال أو سموذي سريعًا في الكوب. "
            "موتور بقدرة 600 واط يدير قدمًا من الستانلس ستيل بسكين رباعية حادة، وزر بسرعتين مع خاصية "
            "التيربو يضع التحكم في أطراف أصابعك. القدم تنفصل بضغطة لسهولة الغسيل.",
        ),
        highlights=[
            ("600 W, 2 speeds + turbo", "600 واط بسرعتين وتيربو"),
            ("Stainless-steel blending foot", "قدم خلط من الستانلس ستيل"),
            ("Detachable foot for easy cleaning", "قدم قابلة للفك لتنظيف سهل"),
        ],
        specs=[
            (("Power", "القدرة"), ("600 W", "600 واط")),
            (("Speeds", "السرعات"), ("2 + turbo", "سرعتان + تيربو")),
            (("Blending foot", "قدم الخلط"), ("Stainless steel, detachable", "ستانلس ستيل قابلة للفك")),
            (("Cable", "السلك"), ("1.2 m", "1.2 متر")),
            (("Colour", "اللون"), ("White and red", "أبيض وأحمر")),
        ],
        options=[],
        variants=[dict(opts=[], price=1799)],
        rating=4.5, reviews=1362,
        alt=("Moulinex hand blender in white and red", "خلاط يدوي مولينكس باللونين الأبيض والأحمر"),
    ),
    dict(
        id=2015,
        name=("Silicone Ice Cube Tray, 18 Cubes", "قالب ثلج من السيليكون، 18 مكعبًا"),
        subtitle=("Pop-out cubes for summer drinks", "مكعبات تلج بتطلع بسهولة لعصاير الصيف"),
        description=(
            "Eighteen cube cells in flexible, food-grade silicone: twist the tray or push from below and the "
            "cubes pop out whole, with no cracking and no running it under the tap. It's freezer-safe down to "
            "−40 °C and just as handy for freezing lemon juice or stock in portions.",
            "18 خانة من السيليكون المرن الآمن على الطعام، يكفي أن تلوي القالب أو تضغط من الأسفل لتخرج "
            "المكعبات كاملة دون كسر ودون تمريره تحت الماء. يتحمل الفريزر حتى −40 درجة مئوية، ومناسب أيضًا "
            "لتجميد عصير الليمون أو المرق على حصص.",
        ),
        highlights=[
            ("18 cubes per tray", "18 مكعبًا في القالب"),
            ("Flexible food-grade silicone", "سيليكون مرن آمن على الطعام"),
            ("Freezer and dishwasher safe", "آمن في الفريزر وغسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Food-grade silicone", "سيليكون آمن على الطعام")),
            (("Cubes", "عدد المكعبات"), ("18", "18 مكعبًا")),
            (("Size", "المقاس"), ("28 × 11 cm", "28 × 11 سم")),
            (("Temperature range", "تحمّل الحرارة"), ("−40 to 230 °C", "من −40 إلى 230 درجة مئوية")),
        ],
        options=[],
        variants=[dict(opts=[], price=89)],
        rating=4.4, reviews=389,
        alt=("Blue silicone ice cube tray with 18 cells", "قالب ثلج سيليكون أزرق بـ18 خانة"),
    ),
    dict(
        id=2016,
        name=("Wooden-Handle Kitchen Sieve, 20 cm", "مصفاة مطبخ بمقبض خشبي 20 سم"),
        subtitle=("Rinse rice, drain pasta, sift flour", "لغسيل الرز وتصفية المكرونة ونخل الدقيق"),
        description=(
            "A 20 cm stainless-steel sieve with a reinforced rim and a turned wooden handle that's warm and "
            "comfortable to hold. Big enough to rinse a full cup of rice or drain a pot of pasta, fine enough "
            "to sift the flour for basbousa.",
            "مصفاة ستانلس ستيل قطرها 20 سم بحافة مقوّاة ومقبض خشبي مخروط مريح في اليد. "
            "واسعة بما يكفي لغسل كوب أرز كامل أو تصفية قدر مكرونة، وناعمة بما يكفي لنخل دقيق البسبوسة.",
        ),
        highlights=[
            ("20 cm stainless mesh", "شبكة ستانلس ستيل بقطر 20 سم"),
            ("Comfortable turned wooden handle", "مقبض خشبي مخروط مريح"),
            ("Reinforced rim", "حافة مقوّاة"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless-steel mesh, wooden handle", "شبكة ستانلس ستيل ومقبض خشب")),
            (("Diameter", "القطر"), ("20 cm", "20 سم")),
            (("Length", "الطول"), ("40 cm", "40 سم")),
            (("Care", "العناية"), ("Hand wash", "غسيل يدوي")),
        ],
        options=[],
        variants=[dict(opts=[], price=139)],
        rating=4.3, reviews=147,
        alt=("Kitchen sieve with a wooden handle", "مصفاة مطبخ بمقبض خشبي"),
    ),
    dict(
        id=2017,
        name=("Chef's Knife, 20 cm", "سكين الشيف 20 سم"),
        subtitle=("Sharp, balanced, made for daily prep", "حادة ومتوازنة للتقطيع اليومي"),
        description=(
            "A 20 cm stainless-steel chef's knife ground to a keen edge for chopping onions, slicing meat and "
            "mincing parsley and coriander. The riveted wooden handle sits comfortably in the hand. "
            "Hand-wash and dry it straight away to keep both the edge and the wood in shape.",
            "سكين شيف بطول 20 سم من الستانلس ستيل بحد قاطع لتقطيع البصل وشرائح اللحم وفرم البقدونس والكزبرة. "
            "مقبضها الخشبي المثبت بالمسامير مريح في اليد. "
            "اغسلها يدويًا وجففها فورًا لتحافظ على الحد والخشب.",
        ),
        highlights=[
            ("20 cm stainless-steel blade", "نصل ستانلس ستيل بطول 20 سم"),
            ("Triple-riveted wooden handle", "مقبض خشبي مثبت بثلاثة مسامير"),
            ("Hand wash to keep the edge", "غسيل يدوي للحفاظ على الحدة"),
        ],
        specs=[
            (("Blade", "النصل"), ("20 cm stainless steel", "ستانلس ستيل 20 سم")),
            (("Overall length", "الطول الكلي"), ("33 cm", "33 سم")),
            (("Handle", "المقبض"), ("Wood, triple-riveted", "خشب مثبت بثلاثة مسامير")),
            (("Care", "العناية"), ("Hand wash only", "غسيل يدوي فقط")),
        ],
        options=[],
        variants=[dict(opts=[], price=499)],
        rating=4.4, reviews=232,
        alt=("Chef's knife with a wooden handle", "سكين شيف بمقبض خشبي"),
    ),
    dict(
        id=2018,
        name=("Zahran 3-Tier Stainless Steel Food Carrier", "حافظة طعام زهران ستانلس ستيل بثلاثة أدوار"),
        subtitle=("Three dishes, one handle, still hot", "غداك سخن بتلات أصناف في إيد واحدة"),
        description=(
            "Three stacking stainless-steel bowls held together by a locking clip and a carry handle, so the "
            "rice, the stew and the salad travel separately and arrive intact. A classic for office lunches and "
            "for taking food to family; it's rust-resistant, holds no smells and washes easily.",
            "ثلاثة أوعية ستانلس ستيل فوق بعضها يثبتها مشبك جانبي ومقبض للحمل، فيسافر الأرز والطبيخ والسلطة "
            "كلٌ في مكانه ويصل سليمًا. اختيار كلاسيكي لغداء الشغل أو لتوصيل الأكل للعائلة، "
            "مقاومة للصدأ ولا تحتفظ بالروائح وسهلة الغسيل.",
        ),
        highlights=[
            ("3 stacking stainless bowls", "3 أوعية ستانلس فوق بعضها"),
            ("Locking clip and carry handle", "مشبك قفل ومقبض للحمل"),
            ("Rust-resistant, no lingering smells", "مقاومة للصدأ ولا تحتفظ بالروائح"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless steel", "ستانلس ستيل")),
            (("Tiers", "الأدوار"), ("3 × 0.9 L", "3 أدوار × 0.9 لتر")),
            (("Diameter", "القطر"), ("14 cm", "14 سم")),
            DISHWASHER,
            (("Made in", "بلد الصنع"), ("Egypt", "مصر")),
        ],
        options=[],
        variants=[dict(opts=[], price=699)],
        rating=4.5, reviews=174,
        alt=("Zahran stainless steel stacking food carrier", "حافظة طعام زهران ستانلس ستيل بأدوار"),
    ),
    dict(
        id=2019,
        name=("Tornado Solo Microwave, 25 L", "ميكروويف تورنيدو سولو 25 لترًا"),
        subtitle=("Reheat, defrost and cook in minutes", "سخّن وفك التلج واطبخ في دقايق"),
        description=(
            "A no-fuss 25-litre solo microwave in white with two simple dials, one for the five power levels "
            "and one for the 30-minute timer. 900 W reheats rice and molokhia evenly, the defrost setting "
            "takes care of frozen chicken, and the 31.5 cm turntable fits a family-size plate. Backed by "
            "Tornado's 2-year warranty; add a third year at checkout.",
            "ميكروويف سولو أبيض سعة 25 لترًا بقرصين بسيطين: واحد لمستويات القوة الخمسة وآخر لمؤقت حتى "
            "30 دقيقة. قوة 900 واط تسخّن الأرز والملوخية بالتساوي، ووضع إذابة التجميد يتعامل مع الدجاج "
            "المجمد، والطبق الدوّار بقطر 31.5 سم يسع طبقًا عائليًا. مع ضمان تورنيدو لمدة سنتين، "
            "ويمكنك إضافة سنة ثالثة مع الطلب.",
        ),
        highlights=[
            ("25 L capacity, 900 W", "سعة 25 لترًا وقوة 900 واط"),
            ("5 power levels + defrost", "5 مستويات قوة وإذابة التجميد"),
            ("Simple mechanical dials", "قرصان ميكانيكيان سهلان"),
            ("2-year Tornado warranty", "ضمان تورنيدو لمدة سنتين"),
        ],
        specs=[
            (("Capacity", "السعة"), ("25 L", "25 لترًا")),
            (("Microwave power", "قوة الميكروويف"), ("900 W", "900 واط")),
            (("Power levels", "مستويات القوة"), ("5", "5 مستويات")),
            (("Timer", "المؤقت"), ("Up to 30 minutes", "حتى 30 دقيقة")),
            (("Turntable", "الطبق الدوّار"), ("31.5 cm glass", "زجاج 31.5 سم")),
            (("Colour", "اللون"), ("White", "أبيض")),
        ],
        options=[],
        variants=[dict(opts=[], price=4999, compare_at=5799)],
        modifier_groups=[
            dict(id="warranty", name=("Extended warranty", "ضمان إضافي"), selection="single", min=0, max=1,
                 required=False,
                 options=[dict(id="plus-1y", name=("+1 year extended warranty", "ضمان إضافي لمدة سنة"),
                               price=499)]),
        ],
        rating=4.4, reviews=1847,
        alt=("Tornado 25-litre solo microwave in white", "ميكروويف تورنيدو سولو أبيض سعة 25 لترًا"),
        views=[
            ("front view", "من الأمام"),
            ("angled view", "منظر مائل"),
            ("back panel and power cord", "اللوحة الخلفية وسلك الكهرباء"),
            ("angled view from the right", "منظر مائل من اليمين"),
        ],
    ),
    dict(
        id=2020,
        name=("Mug Tree Stand", "حامل أكواب على شكل شجرة"),
        subtitle=("Six hooks on a small footprint", "6 خطافات في مساحة صغيرة على الرخامة"),
        description=(
            "A black steel mug tree with six curled hooks that frees up the shelf and keeps your favourite "
            "mugs next to the kettle. The weighted scroll base stays steady on the counter. "
            "Take the stand on its own or as a set with four glossy red ceramic mugs.",
            "حامل أكواب من الحديد الأسود بستة خطافات ملتفّة، يحرر الرفوف ويُبقي أكوابك المفضلة بجوار الغلاية. "
            "قاعدته المزخرفة الثقيلة ثابتة على الرخامة. "
            "اختر الحامل وحده أو مع طقم من أربعة أكواب سيراميك حمراء لامعة.",
        ),
        highlights=[
            ("6 hooks for mugs and cups", "6 خطافات للأكواب والمجات"),
            ("Weighted, stable base", "قاعدة ثقيلة وثابتة"),
            ("Optional set of 4 red ceramic mugs", "اختيار مع طقم 4 أكواب سيراميك حمراء"),
        ],
        specs=[
            (("Material", "الخامة"), ("Powder-coated steel", "حديد مطلي بالبودرة")),
            (("Height", "الارتفاع"), ("38 cm", "38 سم")),
            (("Hooks", "الخطافات"), ("6", "6 خطافات")),
            (("Mugs (set option)", "الأكواب (مع الطقم)"), ("4 × 300 ml ceramic", "4 أكواب سيراميك × 300 مل")),
        ],
        options=[
            dict(id="set", name=("Set", "المجموعة"), kind="chip", values=[
                dict(id="stand", label=("Stand only", "الحامل فقط"), code="STD"),
                dict(id="with-mugs", label=("Stand + 4 red mugs", "الحامل + 4 أكواب حمراء"), code="4MUG"),
            ]),
        ],
        variants=[
            dict(opts=["stand"], price=299, image=0),
            dict(opts=["with-mugs"], price=699, image=1),
        ],
        rating=4.3, reviews=118,
        alt=("Black metal mug tree stand", "حامل أكواب معدني أسود على شكل شجرة"),
        views=[("stand only", "الحامل فقط"), ("with four red mugs", "مع أربعة أكواب حمراء")],
    ),
    dict(
        id=2021,
        name=("Tefal Non-Stick Frying Pan", "مقلاة تيفال غير لاصقة"),
        subtitle=("The red spot tells you when it's hot", "النقطة الحمرا بتقولك إمتى الطاسة سخنت"),
        description=(
            "Tefal's titanium-reinforced non-stick coating lets eggs, fish and pancakes slide straight out with "
            "barely a drop of oil. The Thermo-Spot in the centre turns solid red when the pan has reached the "
            "right temperature, and the riveted handle stays cool. Suitable for every hob, induction included.",
            "طبقة تيفال غير اللاصقة المدعّمة بالتيتانيوم تجعل البيض والسمك والبان كيك ينزلق بسهولة بأقل "
            "قدر من الزيت. مؤشر Thermo-Spot في المنتصف يتحول للأحمر الكامل حين تصل المقلاة للحرارة المناسبة، "
            "والمقبض المثبت بالمسامير لا يسخن. تناسب جميع المواقد بما فيها مواقد الحث.",
        ),
        highlights=[
            ("Titanium-reinforced non-stick", "طبقة غير لاصقة مدعّمة بالتيتانيوم"),
            ("Thermo-Spot heat indicator", "مؤشر Thermo-Spot للحرارة"),
            ("All hobs, including induction", "تناسب كل المواقد بما فيها الحث"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Coating", "الطبقة"), ("Titanium non-stick", "تيتانيوم غير لاصقة")),
            (("Body", "الجسم"), ("Aluminium with induction base", "ألومنيوم بقاعدة للحث")),
            (("Sizes", "المقاسات"), ("24 cm or 28 cm", "24 سم أو 28 سم")),
            (("Hob", "الموقد"), ("Gas, electric, ceramic, induction", "غاز وكهرباء وسيراميك وحث")),
            DISHWASHER,
        ],
        options=[
            dict(id="size", name=("Size", "المقاس"), kind="chip", values=[
                dict(id="24", label=("24 cm", "24 سم"), code="24CM"),
                dict(id="28", label=("28 cm", "28 سم"), code="28CM"),
            ]),
        ],
        variants=[
            dict(opts=["24"], price=899, compare_at=1099),
            dict(opts=["28"], price=1099, compare_at=1349),
        ],
        rating=4.7, reviews=3412,
        alt=("Tefal black non-stick frying pan", "مقلاة تيفال سوداء غير لاصقة"),
    ),
    dict(
        id=2022,
        name=("Luminarc Dinner Plates, 25 cm, Set of 6", "طقم أطباق عشاء لومينارك 25 سم، 6 قطع"),
        subtitle=("An olive-sprig rim for everyday dinners", "نقشة غصن زيتون رقيقة لسفرة كل يوم"),
        description=(
            "Six 25 cm dinner plates in Luminarc's tempered opal glass, finished with a delicate olive-sprig "
            "border. Lighter and more chip-resistant than porcelain, they stack neatly and go into the microwave "
            "and dishwasher without a second thought.",
            "ستة أطباق عشاء قطرها 25 سم من زجاج لومينارك الأوبال المقسّى، مزينة بإطار رقيق من أغصان الزيتون. "
            "أخف من البورسلين وأكثر مقاومة للكسر والشرخ، تُرص فوق بعضها بسهولة وتدخل الميكروويف وغسالة "
            "الأطباق بأمان.",
        ),
        highlights=[
            ("Tempered opal glass, chip-resistant", "زجاج أوبال مقسّى مقاوم للكسر"),
            ("Microwave and dishwasher safe", "آمنة في الميكروويف وغسالة الأطباق"),
            ("Set of 6, 25 cm", "طقم 6 أطباق بقطر 25 سم"),
        ],
        specs=[
            (("Material", "الخامة"), ("Tempered opal glass", "زجاج أوبال مقسّى")),
            (("Diameter", "القطر"), ("25 cm", "25 سم")),
            (("Pieces", "عدد القطع"), ("6 plates", "6 أطباق")),
            (("Microwave & dishwasher", "الميكروويف وغسالة الأطباق"), ("Safe", "آمنة")),
        ],
        options=[],
        variants=[dict(opts=[], price=649)],
        rating=4.6, reviews=297,
        alt=("Luminarc white dinner plate with an olive-sprig rim", "طبق عشاء لومينارك أبيض بإطار غصن زيتون"),
    ),
    dict(
        id=2023,
        name=("Silicone-Tipped Kitchen Tongs, 23 cm", "ملقط مطبخ بأطراف سيليكون 23 سم"),
        subtitle=("Grip, flip and serve without scratches", "امسك وقلّب وقدّم من غير ما تخدش الحلل"),
        description=(
            "Stainless-steel tongs with heat-resistant red silicone tips that grip kofta, corn and pasta firmly "
            "without scratching non-stick pans. A pull-tab lock keeps them closed so they take up little room "
            "in the drawer.",
            "ملقط من الستانلس ستيل بأطراف سيليكون حمراء تتحمل الحرارة، يمسك الكفتة والذرة والمكرونة بإحكام "
            "دون خدش المقالي غير اللاصقة. قفل بسيط يُبقيه مغلقًا ليأخذ مساحة صغيرة في الدرج.",
        ),
        highlights=[
            ("Heat-resistant silicone tips", "أطراف سيليكون تتحمل الحرارة"),
            ("Safe on non-stick cookware", "آمن على الأواني غير اللاصقة"),
            ("Locks closed for storage", "يُقفل للتخزين"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless steel and silicone", "ستانلس ستيل وسيليكون")),
            (("Length", "الطول"), ("23 cm", "23 سم")),
            (("Heat resistance", "تحمّل الحرارة"), ("Up to 230 °C", "حتى 230 درجة مئوية")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=129)],
        rating=4.4, reviews=165,
        alt=("Kitchen tongs with red silicone tips", "ملقط مطبخ بأطراف سيليكون حمراء"),
    ),
    dict(
        id=2024,
        name=("Zahran Stainless Steel Pot with Glass Lid", "قدر زهران ستانلس ستيل بغطاء زجاجي"),
        subtitle=("The everyday pot for rice, soups and stews", "حلة كل يوم للرز والشوربة والطبيخ"),
        description=(
            "A heavy stainless-steel pot with an encapsulated sandwich base that spreads heat evenly, so rice "
            "cooks without catching and stews simmer gently. The tempered-glass lid has a steam vent, so you can "
            "keep an eye on things without lifting it. Made in Egypt by Zahran; works on gas, electric and "
            "induction.",
            "قدر من الستانلس ستيل السميك بقاعدة ساندويتش توزّع الحرارة بالتساوي، فيستوي الأرز دون أن يلتصق "
            "ويُطهى الطبيخ على مهل. الغطاء من الزجاج المقسّى وبه فتحة لخروج البخار لتتابع الطهي دون رفعه. "
            "صناعة مصرية من زهران، ويعمل على الغاز والكهرباء والحث.",
        ),
        highlights=[
            ("Sandwich base for even heat", "قاعدة ساندويتش لتوزيع متساوٍ للحرارة"),
            ("Tempered-glass lid with steam vent", "غطاء زجاج مقسّى بفتحة بخار"),
            ("Gas, electric and induction", "يعمل على الغاز والكهرباء والحث"),
            ("Made in Egypt", "صناعة مصرية"),
        ],
        specs=[
            (("Material", "الخامة"), ("18/10 stainless steel", "ستانلس ستيل 18/10")),
            (("Base", "القاعدة"), ("Encapsulated sandwich base", "قاعدة ساندويتش")),
            (("Lid", "الغطاء"), ("Tempered glass with vent", "زجاج مقسّى بفتحة بخار")),
            (("Capacity", "السعة"), ("22 cm 4.5 L · 24 cm 6 L · 26 cm 8 L",
                                    "22 سم 4.5 لتر · 24 سم 6 لتر · 26 سم 8 لتر")),
            DISHWASHER,
        ],
        options=[
            dict(id="size", name=("Size", "المقاس"), kind="chip", values=[
                dict(id="22", label=("22 cm · 4.5 L", "22 سم · 4.5 لتر"), code="22CM"),
                dict(id="24", label=("24 cm · 6 L", "24 سم · 6 لتر"), code="24CM"),
                dict(id="26", label=("26 cm · 8 L", "26 سم · 8 لتر"), code="26CM"),
            ]),
        ],
        variants=[
            dict(opts=["22"], price=1299),
            dict(opts=["24"], price=1499),
            dict(opts=["26"], price=1699),
        ],
        rating=4.6, reviews=528,
        alt=("Zahran stainless steel pot with a glass lid", "قدر زهران ستانلس ستيل بغطاء زجاجي"),
    ),
    dict(
        id=2025,
        name=("Stainless Steel Slotted Turner", "ملعقة تقليب مثقوبة من الستانلس ستيل"),
        subtitle=("Flip the ta'meya, drain the oil", "قلّب الطعمية وصفّي الزيت"),
        description=(
            "A thin, slightly flexible stainless-steel turner that slides under ta'meya, fish fillets and "
            "burgers while the slots let the oil drain back into the pan. The hollow handle stays cooler and has "
            "a hole for hanging. Best kept for stainless and cast-iron pans rather than non-stick.",
            "ملعقة تقليب رفيعة ومرنة قليلًا من الستانلس ستيل، تنزلق تحت أقراص الطعمية وشرائح السمك والبرجر، "
            "وفتحاتها تُرجع الزيت إلى المقلاة. مقبضها المجوّف يظل أبرد وبه فتحة للتعليق. "
            "يُفضّل استخدامها مع أواني الستانلس والحديد الزهر بدلًا من غير اللاصقة.",
        ),
        highlights=[
            ("Slots drain oil back into the pan", "فتحات تُصفّي الزيت في المقلاة"),
            ("Thin, flexible edge", "حافة رفيعة ومرنة"),
            ("Dishwasher safe", "آمنة في غسالة الأطباق"),
        ],
        specs=[
            (("Material", "الخامة"), ("Stainless steel", "ستانلس ستيل")),
            (("Length", "الطول"), ("33 cm", "33 سم")),
            (("Head", "الرأس"), ("9 × 7 cm", "9 × 7 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=149)],
        rating=4.3, reviews=121,
        alt=("Stainless steel slotted turner", "ملعقة تقليب مثقوبة من الستانلس ستيل"),
    ),
    dict(
        id=2026,
        name=("Rotating Spice Rack with 4 Glass Jars", "حامل بهارات دوّار بأربعة برطمانات زجاجية"),
        subtitle=("Cumin, coriander and pepper, one spin away", "الكمون والكسبرة والفلفل في لفّة واحدة"),
        description=(
            "A compact countertop carousel with four glass jars and stainless shaker caps that open for "
            "sprinkling or pouring. Fill them with the spices you reach for every day, cumin, coriander, "
            "black pepper and chilli, and give it a spin to grab the one you need. The jars lift out for "
            "refilling and washing.",
            "حامل دوّار صغير للرخامة بأربعة برطمانات زجاجية وأغطية ستانلس بفتحتين للرش والصب. "
            "املأها بالبهارات التي تستخدمها كل يوم مثل الكمون والكزبرة والفلفل الأسود والشطة، "
            "ولفّ الحامل لتصل لما تحتاجه. البرطمانات تُرفع بسهولة لإعادة الملء والغسيل.",
        ),
        highlights=[
            ("4 glass jars, 120 ml each", "4 برطمانات زجاج سعة كل منها 120 مل"),
            ("Rotating base", "قاعدة دوّارة"),
            ("Shaker caps with two openings", "أغطية بفتحتين للرش والصب"),
        ],
        specs=[
            (("Jars", "البرطمانات"), ("4 × 120 ml glass", "4 برطمانات زجاج × 120 مل")),
            (("Caps", "الأغطية"), ("Stainless steel", "ستانلس ستيل")),
            (("Base", "القاعدة"), ("Rotating ABS", "دوّارة من البلاستيك ABS")),
            (("Size", "المقاس"), ("13 × 13 × 19 cm", "13 × 13 × 19 سم")),
        ],
        options=[],
        variants=[dict(opts=[], price=449)],
        rating=4.6, reviews=23,
        alt=("Rotating spice rack with four glass jars", "حامل بهارات دوّار بأربعة برطمانات زجاجية"),
    ),
    dict(
        id=2027,
        name=("Zahran Stainless Steel Dinner Spoons", "ملاعق طعام من الستانلس ستيل من زهران"),
        subtitle=("A match for the Zahran forks", "على نفس طقم شوك زهران"),
        description=(
            "Generous dinner spoons in 18/10 stainless steel with a mirror polish and a rounded handle that "
            "matches Zahran's dinner forks. Deep enough for soup and roz bel laban, sturdy enough for daily "
            "family use, and fully dishwasher safe.",
            "ملاعق طعام بحجم سخي من الستانلس ستيل 18/10 بلمعة المرآة ومقبض مستدير يطابق شوك زهران. "
            "عميقة بما يكفي للشوربة والأرز باللبن، ومتينة للاستعمال اليومي للعائلة، وآمنة تمامًا في غسالة "
            "الأطباق.",
        ),
        highlights=[
            ("18/10 stainless steel, mirror finish", "ستانلس ستيل 18/10 بلمعة المرآة"),
            ("Matches Zahran dinner forks", "تطابق شوك زهران"),
            ("Sets of 6 or 12", "طقم 6 أو 12 قطعة"),
        ],
        specs=[
            (("Material", "الخامة"), ("18/10 stainless steel", "ستانلس ستيل 18/10")),
            (("Length", "الطول"), ("20 cm", "20 سم")),
            (("Finish", "التشطيب"), ("Mirror polish", "تلميع مرآة")),
            DISHWASHER,
            (("Made in", "بلد الصنع"), ("Egypt", "مصر")),
        ],
        options=[
            dict(id="set", name=("Set", "الطقم"), kind="chip", values=[
                dict(id="6", label=("Set of 6", "6 قطع"), code="6PC"),
                dict(id="12", label=("Set of 12", "12 قطعة"), code="12PC"),
            ]),
        ],
        variants=[dict(opts=["6"], price=249), dict(opts=["12"], price=449)],
        rating=4.6, reviews=392,
        alt=("Zahran stainless steel dinner spoon", "ملعقة طعام ستانلس ستيل من زهران"),
    ),
    dict(
        id=2028,
        name=("Wood-Effect Serving Tray, 45 × 30 cm", "صينية تقديم بلون الخشب 45 × 30 سم"),
        subtitle=("Tea for the guests, served in style", "شاي الضيوف يتقدّم بشياكة"),
        description=(
            "A 45 × 30 cm serving tray with a warm wood-effect finish and raised, curved ends with cut-out "
            "handles for a steady grip. Big enough for six tea glasses and a plate of biscuits, light enough to "
            "carry with one hand. Wipe clean with a damp cloth.",
            "صينية تقديم مقاس 45 × 30 سم بلمسة خشبية دافئة، بأطراف مرفوعة ومنحنية فيها مقابض مفرغة لمسكة ثابتة. "
            "تتسع لستة أكواب شاي وطبق بسكويت، وخفيفة تحملها بيد واحدة. تُنظف بقطعة قماش مبللة.",
        ),
        highlights=[
            ("Raised ends with cut-out handles", "أطراف مرفوعة بمقابض مفرغة"),
            ("Roomy 45 × 30 cm", "مساحة واسعة 45 × 30 سم"),
            ("Light and easy to wipe clean", "خفيفة وسهلة التنظيف"),
        ],
        specs=[
            (("Material", "الخامة"), ("MDF with wood-effect laminate", "خشب MDF بقشرة بلون الخشب")),
            (("Size", "المقاس"), ("45 × 30 cm", "45 × 30 سم")),
            (("Height", "الارتفاع"), ("6 cm", "6 سم")),
            (("Care", "العناية"), ("Wipe with a damp cloth", "تُمسح بقطعة قماش مبللة")),
        ],
        options=[],
        variants=[dict(opts=[], price=399)],
        rating=4.4, reviews=156,
        alt=("Wood-effect serving tray with handles", "صينية تقديم بلون الخشب بمقابض"),
    ),
    dict(
        id=2029,
        name=("Beech-Wood Rolling Pin & Pastry Board", "نشّابة ولوح عجين من خشب الزان"),
        subtitle=("For ka'k, pizza and feteer", "لكحك العيد والبيتزا والفطير"),
        description=(
            "A smooth beech rolling pin with turned handles and a round 30 cm pastry board, ready for rolling out "
            "dough for ka'k, pizza, sambousek and feteer. The dense, fine-grained wood resists sticking, so a "
            "light dusting of flour is all you need.",
            "نشّابة ملساء من خشب الزان بمقابض مخروطة مع لوح عجين دائري قطره 30 سم، جاهزة لفرد عجين كحك العيد "
            "والبيتزا والسمبوسك والفطير. الخشب الكثيف ناعم الملمس لا يلتصق به العجين بسهولة، "
            "ويكفيه رش خفيف من الدقيق.",
        ),
        highlights=[
            ("Rolling pin with turned handles", "نشّابة بمقابض مخروطة"),
            ("Round 30 cm pastry board", "لوح عجين دائري بقطر 30 سم"),
            ("Smooth, fine-grained beech", "خشب زان ناعم الملمس"),
        ],
        specs=[
            (("Material", "الخامة"), ("Beech wood", "خشب زان")),
            (("Rolling pin", "النشّابة"), ("45 cm", "45 سم")),
            (("Board", "اللوح"), ("Ø 30 cm", "قطر 30 سم")),
            (("Care", "العناية"), ("Hand wash, dry well", "غسيل يدوي وتجفيف جيد")),
        ],
        options=[],
        variants=[dict(opts=[], price=299)],
        rating=4.5, reviews=209,
        alt=("Beech-wood rolling pin with a round pastry board", "نشّابة من خشب الزان مع لوح عجين دائري"),
    ),
    dict(
        id=2030,
        name=("Y-Peeler", "مقشّرة خضار على شكل Y"),
        subtitle=("Potatoes and carrots peeled in seconds", "قشّر البطاطس والجزر في ثواني"),
        description=(
            "A Y-shaped peeler with a swivelling stainless blade that follows the curve of potatoes, carrots and "
            "cucumbers and takes off just the skin. The wide yellow grip suits left- and right-handers, and the "
            "side tip digs out potato eyes.",
            "مقشّرة على شكل حرف Y بنصل ستانلس متحرك يتبع انحناء البطاطس والجزر والخيار ويزيل القشرة فقط. "
            "مقبضها الأصفر العريض مريح لليد اليمنى واليسرى، وطرفها الجانبي يزيل عيون البطاطس.",
        ),
        highlights=[
            ("Swivelling stainless-steel blade", "نصل ستانلس ستيل متحرك"),
            ("Comfortable for left and right hands", "مريحة لليد اليمنى واليسرى"),
            ("Built-in potato-eye remover", "طرف لإزالة عيون البطاطس"),
        ],
        specs=[
            (("Blade", "النصل"), ("Swivelling stainless steel", "ستانلس ستيل متحرك")),
            (("Handle", "المقبض"), ("ABS plastic", "بلاستيك ABS")),
            (("Length", "الطول"), ("15 cm", "15 سم")),
            DISHWASHER,
        ],
        options=[],
        variants=[dict(opts=[], price=69)],
        rating=4.5, reviews=312,
        alt=("Yellow Y-shaped vegetable peeler", "مقشّرة خضار صفراء على شكل Y"),
    ),
]

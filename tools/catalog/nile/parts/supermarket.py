"""Supermarket (category 22): products 2201–2225. All express (dark-store stock, delivered within hours)."""

W = ("Weight", "الوزن")
ORIGIN = ("Origin", "المنشأ")
STORAGE = ("Storage", "التخزين")
EGYPT = ("Egypt", "مصر")

PRODUCTS = [
    dict(
        id=2201,
        name=("Red Apples — Imported", "تفاح أحمر مستورد"),
        subtitle=("Crisp & juicy, hand-picked daily", "مقرمش وعصيره كتير، بننقّيه بإيدينا كل يوم"),
        description=(
            "Crisp, juicy red apples, imported and kept chilled all the way to our dark stores. We hand-pick "
            "every order so you get firm, unbruised fruit — perfect for lunch boxes, salads and baking.",
            "تفاح أحمر مستورد مقرمش وغني بالعصارة، يُحفظ مبرّدًا طوال الطريق حتى مخازننا. ننتقي ثمار كل طلب "
            "يدويًا لتصلك متماسكة وخالية من الكدمات، مثالية لعلبة المدرسة والسلطات والمخبوزات.",
        ),
        highlights=[
            ("Hand-picked for every order", "انتقاء يدوي لكل طلب"),
            ("Cold chain from orchard to door", "سلسلة تبريد من البستان حتى بابك"),
            ("Great for snacking, salads and baking", "مثالي للأكل والسلطات والمخبوزات"),
        ],
        specs=[
            (ORIGIN, ("Imported (Chile or South Africa, by season)", "مستورد (تشيلي أو جنوب أفريقيا حسب الموسم)")),
            (W, ("1 kg or 2 kg — about 5–6 apples per kg", "1 كجم أو 2 كجم، نحو 5–6 تفاحات في الكيلو")),
            (STORAGE, ("Refrigerate at 0–4 °C", "يُحفظ في الثلاجة على 0–4 درجات مئوية")),
            (("Best within", "يُفضَّل استهلاكه خلال"), ("2 weeks in the fridge", "أسبوعين في الثلاجة")),
        ],
        options=[
            dict(id="weight", name=W, kind="chip", values=[
                dict(id="1kg", label=("1 kg", "1 كجم"), code="1KG"),
                dict(id="2kg", label=("2 kg", "2 كجم"), code="2KG"),
            ]),
        ],
        variants=[
            dict(opts=["1kg"], price=89),
            dict(opts=["2kg"], price=169),
        ],
        rating=4.4, reviews=640,
        alt=("Red apples, one cut in half", "تفاح أحمر مع تفاحة مقطوعة نصفين"),
    ),
    dict(
        id=2202,
        name=("Beef Ribeye Steak", "ستيك ريب آي بقري"),
        subtitle=("Marbled, tender and grill-ready", "طري ومعرّق ومستني الشواية"),
        description=(
            "Well-marbled ribeye from local grain-fed beef, trimmed and vacuum-packed in our butchery the same day. "
            "Halal and delivered chilled, never frozen — sear hot for 3–4 minutes a side and rest before slicing.",
            "ستيك ريب آي غني بالدهن المتداخل من لحم بقري محلي مُسمَّن على الحبوب، يُشفّى ويُغلَّف بالتفريغ في قسم "
            "الجزارة لدينا في اليوم نفسه. لحم حلال يصلك مبرّدًا دون تجميد؛ اشوِه على نار عالية 3–4 دقائق لكل جانب "
            "واتركه يرتاح قبل التقطيع.",
        ),
        highlights=[
            ("Halal, from local slaughter", "لحم حلال من ذبح محلي"),
            ("Delivered chilled, never frozen", "يصل مبرّدًا دون تجميد"),
            ("Vacuum-packed steaks, 2–3 cm thick", "شرائح بسمك 2–3 سم مغلّفة بالتفريغ"),
        ],
        specs=[
            (("Cut", "القطعية"), ("Ribeye", "ريب آي (عين الضلع)")),
            (ORIGIN, ("Egypt, grain-fed beef", "مصر، أبقار مُسمَّنة على الحبوب")),
            (("Pack", "العبوة"), ("500 g or 1 kg, vacuum-packed", "500 جرام أو 1 كجم، مغلّف بالتفريغ")),
            (STORAGE, ("0–4 °C, use within 4 days", "0–4 درجات مئوية، ويُستهلك خلال 4 أيام")),
            (("Certification", "الشهادة"), ("Halal", "حلال")),
        ],
        options=[
            dict(id="weight", name=W, kind="chip", values=[
                dict(id="500g", label=("500 g", "500 جرام"), code="500G"),
                dict(id="1kg", label=("1 kg", "1 كجم"), code="1KG"),
            ]),
        ],
        variants=[
            dict(opts=["500g"], price=389),
            dict(opts=["1kg"], price=759),
        ],
        rating=4.5, reviews=210,
        alt=("Raw beef ribeye steak", "شريحة ستيك ريب آي نيئة"),
    ),
    dict(
        id=2203,
        name=("Whiskas 1+ Wet Cat Food — Chicken in Jelly, 85 g", "طعام قطط رطب ويسكاس 1+ بالدجاج في الجيلي — 85 جرامًا"),
        subtitle=("A complete meal for grown-up cats", "وجبة كاملة لقطتك الكبيرة"),
        description=(
            "Tender chunks of chicken in jelly, made for adult cats aged one year and up. Each 85 g pouch is a "
            "complete, balanced meal with the vitamins and minerals your cat needs — stock up with the box of 12 and save.",
            "قطع دجاج طرية في الجيلي، مخصّصة للقطط البالغة من عمر سنة فأكثر. كل كيس 85 جرامًا وجبة كاملة ومتوازنة "
            "بالفيتامينات والمعادن التي تحتاجها قطتك، ووفّر أكثر مع علبة الـ12 كيسًا.",
        ),
        highlights=[
            ("Complete & balanced for cats 1+ years", "غذاء كامل ومتوازن للقطط من عمر سنة"),
            ("Chicken chunks in jelly", "قطع دجاج في الجيلي"),
            ("Save with the box of 12", "وفّر مع علبة 12 كيسًا"),
        ],
        specs=[
            (("Life stage", "المرحلة العمرية"), ("Adult cats, 1+ years", "قطط بالغة من عمر سنة فأكثر")),
            (("Flavour", "النكهة"), ("Chicken in jelly", "دجاج في الجيلي")),
            (("Pouch size", "حجم الكيس"), ("85 g", "85 جرامًا")),
            (("Feeding", "الكمية اليومية"), ("About 4 pouches a day for a 4 kg cat", "نحو 4 أكياس يوميًا لقطة وزنها 4 كجم")),
        ],
        options=[
            dict(id="pack", name=("Pack", "العبوة"), kind="chip", values=[
                dict(id="1-pouch", label=("1 pouch", "كيس واحد"), code="1P"),
                dict(id="box-12", label=("Box of 12", "علبة 12 كيسًا"), code="12P"),
            ]),
        ],
        variants=[
            dict(opts=["1-pouch"], price=39),
            dict(opts=["box-12"], price=419, compare_at=468),
        ],
        rating=4.7, reviews=980,
        alt=("Whiskas chicken in jelly cat food pouch", "كيس طعام قطط ويسكاس بالدجاج في الجيلي"),
    ),
    dict(
        id=2204,
        name=("Koki Fresh Chicken", "دجاج كوكي طازج"),
        subtitle=("Fresh every day, never frozen", "طازة كل يوم ومش متلّجة"),
        description=(
            "Fresh, halal Egyptian chicken from Koki, delivered chilled in an insulated bag within hours. Choose a "
            "whole bird of about 1.3 kg for roasting, or a kilo of meaty drumsticks for the oven or the grill.",
            "دجاج مصري طازج وحلال من كوكي، يصلك مبرّدًا في حقيبة معزولة خلال ساعات. اختر دجاجة كاملة بوزن نحو "
            "1.3 كجم للتحمير في الفرن، أو كيلو دبابيس دجاج للشوي أو الفرن.",
        ),
        highlights=[
            ("Halal, from Egyptian farms", "حلال ومن مزارع مصرية"),
            ("Chilled at 0–4 °C, never frozen", "مبرّد على 0–4 درجات ولم يُجمَّد"),
            ("Whole bird or drumsticks", "دجاجة كاملة أو دبابيس"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (W, ("Whole: about 1.3 kg · Drumsticks: 1 kg", "كاملة: نحو 1.3 كجم · دبابيس: 1 كجم")),
            (STORAGE, ("0–4 °C, cook within 2 days", "0–4 درجات مئوية، ويُطهى خلال يومين")),
            (("Certification", "الشهادة"), ("Halal", "حلال")),
        ],
        options=[
            dict(id="cut", name=("Cut", "القطعية"), kind="chip", values=[
                dict(id="whole", label=("Whole chicken (~1.3 kg)", "دجاجة كاملة (نحو 1.3 كجم)"), code="WHL"),
                dict(id="drumsticks", label=("Drumsticks (1 kg)", "دبابيس (1 كجم)"), code="DRM"),
            ]),
        ],
        variants=[
            dict(opts=["whole"], price=239, image=0),
            dict(opts=["drumsticks"], price=189, image=1),
        ],
        rating=4.6, reviews=870,
        alt=("Koki fresh chicken", "دجاج كوكي طازج"),
        views=[("whole chicken", "دجاجة كاملة"), ("drumsticks", "دبابيس دجاج")],
    ),
    dict(
        id=2205,
        name=("Crystal Sunflower Oil", "زيت عباد الشمس كريستال"),
        subtitle=("Light oil for frying and everyday cooking", "زيت خفيف للتحمير والطبخ اليومي"),
        description=(
            "Pure sunflower oil from Crystal, light in taste and colour, for everyday frying, sautéing and baking. "
            "Naturally cholesterol-free and rich in vitamin E, in an easy-pour bottle with a resealable cap.",
            "زيت عباد شمس نقي من كريستال، خفيف الطعم واللون، مناسب للقلي والتشويح والخَبز كل يوم. خالٍ طبيعيًا من "
            "الكوليسترول وغني بفيتامين E، في عبوة سهلة الصبّ بغطاء محكم.",
        ),
        highlights=[
            ("100% sunflower oil", "زيت عباد شمس 100%"),
            ("Cholesterol-free, rich in vitamin E", "خالٍ من الكوليسترول وغني بفيتامين E"),
            ("Handles high-heat frying", "يتحمّل حرارة القلي العالية"),
        ],
        specs=[
            (("Type", "النوع"), ("Refined sunflower oil", "زيت عباد شمس مكرر")),
            (("Size", "الحجم"), ("800 ml or 1.6 L", "800 مل أو 1.6 لتر")),
            (("Made in", "بلد الصنع"), EGYPT),
            (STORAGE, ("Cool, dark place; use within 2 months of opening",
                       "مكان بارد بعيد عن الضوء، ويُستهلك خلال شهرين من الفتح")),
        ],
        options=[
            dict(id="size", name=("Size", "الحجم"), kind="chip", values=[
                dict(id="800ml", label=("800 ml", "800 مل"), code="800"),
                dict(id="1-6l", label=("1.6 L", "1.6 لتر"), code="1600"),
            ]),
        ],
        variants=[
            dict(opts=["800ml"], price=109),
            dict(opts=["1-6l"], price=209),
        ],
        rating=4.6, reviews=4200,
        alt=("Crystal sunflower oil bottle", "زجاجة زيت عباد الشمس كريستال"),
    ),
    dict(
        id=2206,
        name=("Cucumbers", "خيار"),
        subtitle=("Crunchy and fresh from the farm", "مقرمش وطازة من الغيط"),
        description=(
            "Crisp Egyptian cucumbers, picked young for thin skins and few seeds. Ideal for salads, pickles and a "
            "cool snack on hot days.",
            "خيار مصري مقرمش يُقطف صغيرًا ليكون قشره رقيقًا وبذوره قليلة. مثالي للسلطات والتخليل، وكوجبة خفيفة "
            "منعشة في الأيام الحارة.",
        ),
        highlights=[
            ("Thin-skinned baladi cucumbers", "خيار بلدي رقيق القشر"),
            ("Picked and delivered within 48 hours", "يُقطف ويصلك خلال 48 ساعة"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (W, ("1 kg — about 8–10 cucumbers", "1 كجم، نحو 8–10 حبات")),
            (STORAGE, ("Refrigerate, best within 5 days", "يُحفظ في الثلاجة، ويُفضَّل استهلاكه خلال 5 أيام")),
        ],
        variants=[dict(opts=[], price=29)],
        rating=4.3, reviews=520,
        alt=("Fresh cucumbers, whole and sliced", "خيار طازج كامل ومقطّع شرائح"),
    ),
    dict(
        id=2207,
        name=("Blitz Sensitive Puppy Food — Lamb & Rice", "طعام جراء بليتز سنسيتيف — لحم ضأن وأرز"),
        subtitle=("Gentle on sensitive little tummies", "خفيف على المعدة الحساسة للجرو الصغير"),
        description=(
            "Super-premium dry food for puppies up to 12 months with sensitive digestion, made with lamb and rice "
            "and enriched with live probiotics. Suits all breeds — the 15 kg bag is the better value for large-breed puppies.",
            "طعام جاف فائق الجودة للجراء حتى عمر 12 شهرًا ذات الهضم الحساس، مصنوع من لحم الضأن والأرز ومدعّم "
            "ببروبيوتيك حي. مناسب لكل السلالات، وكيس الـ15 كجم أوفر لجراء السلالات الكبيرة.",
        ),
        highlights=[
            ("Lamb as the main protein", "لحم الضأن مصدر البروتين الأساسي"),
            ("Live probiotics, omega-3 and omega-6", "بروبيوتيك حي وأوميغا 3 وأوميغا 6"),
            ("Hypoallergenic grains", "حبوب قليلة التسبّب في الحساسية"),
        ],
        specs=[
            (("Life stage", "المرحلة العمرية"), ("Puppies up to 12 months, all breeds", "جراء حتى 12 شهرًا، كل السلالات")),
            (("Main ingredients", "المكونات الأساسية"), ("Lamb, rice, probiotics, kelp", "لحم ضأن، أرز، بروبيوتيك، أعشاب بحرية")),
            (("Bag size", "حجم الكيس"), ("2 kg or 15 kg", "2 كجم أو 15 كجم")),
            (ORIGIN, ("Imported", "مستورد")),
        ],
        options=[
            dict(id="bag", name=("Bag size", "حجم الكيس"), kind="chip", values=[
                dict(id="2kg", label=("2 kg", "2 كجم"), code="2KG"),
                dict(id="15kg", label=("15 kg", "15 كجم"), code="15KG"),
            ]),
        ],
        variants=[
            dict(opts=["2kg"], price=1099),
            dict(opts=["15kg"], price=6999),
        ],
        rating=4.6, reviews=310,
        alt=("Blitz Sensitive puppy food bag, lamb and rice", "كيس طعام جراء بليتز سنسيتيف بلحم الضأن والأرز"),
    ),
    dict(
        id=2208,
        name=("Farm Brown Eggs", "بيض أحمر طازج"),
        subtitle=("Fresh from the farm, every morning", "طازة من المزرعة كل صبح"),
        description=(
            "Fresh brown eggs from Egyptian farms, graded, date-stamped and packed in protective trays. Order 10 for "
            "the week or the classic tray of 30 for the whole family.",
            "بيض أحمر طازج من مزارع مصرية، مُصنَّف حسب الحجم ومختوم بتاريخ الإنتاج ومعبأ في أطباق تحميه من الكسر. "
            "اطلب 10 بيضات للأسبوع أو الطبق الكلاسيكي بـ30 بيضة للعائلة كلها.",
        ),
        highlights=[
            ("Date-stamped for freshness", "مختوم بتاريخ الإنتاج"),
            ("Medium to large, graded by size", "حجم متوسط إلى كبير ومُصنَّف"),
            ("Broken egg? We replace it free", "بيضة مكسورة؟ نستبدلها مجانًا"),
        ],
        specs=[
            (ORIGIN, ("Egyptian farms", "مزارع مصرية")),
            (("Size", "الحجم"), ("Medium to large (55–65 g)", "متوسط إلى كبير (55–65 جرامًا)")),
            (("Pack", "العبوة"), ("10 eggs or tray of 30", "10 بيضات أو طبق 30 بيضة")),
            (STORAGE, ("Refrigerate; best within 3 weeks of the stamped date",
                       "يُحفظ في الثلاجة، ويُفضَّل استهلاكه خلال 3 أسابيع من التاريخ المختوم")),
        ],
        options=[
            dict(id="pack", name=("Pack", "العبوة"), kind="chip", values=[
                dict(id="10", label=("10 eggs", "10 بيضات"), code="10"),
                dict(id="tray-30", label=("Tray of 30", "طبق 30 بيضة"), code="30"),
            ]),
        ],
        variants=[
            dict(opts=["10"], price=69),
            dict(opts=["tray-30"], price=189),
        ],
        rating=4.7, reviews=6800,
        alt=("Fresh brown eggs", "بيض أحمر طازج"),
    ),
    dict(
        id=2209,
        name=("Norwegian Salmon Steaks", "ستيك سلمون نرويجي"),
        subtitle=("Rich in omega-3, ready for the pan", "غني بالأوميجا 3 وجاهز للطاسة"),
        description=(
            "Bone-in steaks cut from fresh Atlantic salmon flown in from Norway, then portioned and vacuum-packed here. "
            "Bake, grill or pan-sear in about 10 minutes — rich in omega-3 and protein.",
            "شرائح سلمون بالعظم مقطوعة من سلمون أطلسي طازج يصل جوًا من النرويج، ثم تُجزّأ وتُغلَّف بالتفريغ هنا. "
            "اخبزه أو اشوِه أو حمّره في المقلاة في نحو 10 دقائق، وهو غني بالأوميجا 3 والبروتين.",
        ),
        highlights=[
            ("Fresh Atlantic salmon from Norway", "سلمون أطلسي طازج من النرويج"),
            ("2–3 steaks per 500 g pack", "2–3 شرائح في عبوة 500 جرام"),
            ("Delivered chilled in an insulated bag", "يصل مبرّدًا في حقيبة معزولة"),
        ],
        specs=[
            (("Species", "النوع"), ("Atlantic salmon, farmed", "سلمون أطلسي مستزرع")),
            (ORIGIN, ("Norway", "النرويج")),
            (W, ("500 g — 2–3 bone-in steaks", "500 جرام، 2–3 شرائح بالعظم")),
            (STORAGE, ("0–2 °C, cook within 2 days", "0–2 درجة مئوية، ويُطهى خلال يومين")),
        ],
        variants=[dict(opts=[], price=699)],
        rating=4.5, reviews=190,
        alt=("Raw salmon steak", "شريحة سلمون نيئة"),
    ),
    dict(
        id=2210,
        name=("Green Bell Peppers", "فلفل رومي أخضر"),
        subtitle=("Crunchy, glossy and full of flavour", "مقرمش ولامع وطعمه حلو"),
        description=(
            "Firm, glossy green bell peppers from Egyptian farms. Stuff them for mahshi, slice them into salads or "
            "add crunch to stir-fries and pizza.",
            "فلفل رومي أخضر متماسك ولامع من مزارع مصرية. احشُه للمحشي، أو قطّعه للسلطات، أو أضفه إلى الأطباق "
            "السريعة والبيتزا.",
        ),
        highlights=[
            ("Just the right size for mahshi", "حجم مثالي للمحشي"),
            ("Hand-sorted for firmness", "منتقى يدويًا ومتماسك"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (W, ("1 kg — about 6–8 peppers", "1 كجم، نحو 6–8 حبات")),
            (STORAGE, ("Refrigerate, best within a week", "يُحفظ في الثلاجة، ويُفضَّل استهلاكه خلال أسبوع")),
        ],
        variants=[dict(opts=[], price=39)],
        rating=4.3, reviews=260,
        alt=("Two green bell peppers", "حبتان من الفلفل الرومي الأخضر"),
    ),
    dict(
        id=2211,
        name=("Green Chilli Peppers", "فلفل حار أخضر"),
        subtitle=("For a proper kick of heat", "للي بيحب الأكل حرّاق"),
        description=(
            "Slim, fiery green chillies from Upper Egypt — the heat behind a good shatta, salsa or pickle. Sold in a "
            "250 g bag, enough for a week of cooking.",
            "فلفل أخضر حار ورفيع من صعيد مصر، سرّ الشطة الحلوة والصلصات والمخللات. يُباع في كيس 250 جرامًا يكفي "
            "طبخ أسبوع كامل.",
        ),
        highlights=[
            ("Hot and aromatic", "حار وعطري"),
            ("Great for shatta and pickles", "ممتاز للشطة والمخللات"),
        ],
        specs=[
            (ORIGIN, ("Egypt (Upper Egypt)", "مصر (الصعيد)")),
            (W, ("250 g", "250 جرامًا")),
            (("Spiciness", "درجة الحِدّة"), ("Hot", "حار")),
            (STORAGE, ("Refrigerate in a paper bag, up to 10 days", "يُحفظ في الثلاجة داخل كيس ورقي حتى 10 أيام")),
        ],
        variants=[dict(opts=[], price=9)],
        rating=4.2, reviews=95,
        alt=("Green chilli peppers", "فلفل أخضر حار"),
    ),
    dict(
        id=2212,
        name=("Henry's 100% Organic Honey, 350 g", "عسل هنريز العضوي 100% — 350 جرامًا"),
        subtitle=("Pure organic honey, nothing added", "عسل عضوي صافي من غير أي إضافات"),
        description=(
            "Certified-organic wildflower honey from Henry's, gently strained and never blended with syrups. Its "
            "light floral taste suits tea, yoghurt and pancakes — or a spoonful straight from the jar.",
            "عسل زهور برية عضوي معتمد من هنريز، مُصفّى برفق ودون خلطه بأي شراب سكري. طعمه الزهري الخفيف يناسب "
            "الشاي والزبادي والبان كيك، أو ملعقة مباشرة من البرطمان.",
        ),
        highlights=[
            ("Certified organic", "عضوي معتمد"),
            ("No added sugar or syrups", "بدون سكر أو شراب مضاف"),
            ("May crystallise naturally — warm gently", "قد يتبلور طبيعيًا، دفّئه برفق ليعود سائلًا"),
        ],
        specs=[
            (("Net weight", "الوزن الصافي"), ("350 g (250 ml jar)", "350 جرامًا (برطمان 250 مل)")),
            (("Type", "النوع"), ("Organic wildflower honey", "عسل زهور برية عضوي")),
            (ORIGIN, ("Imported", "مستورد")),
            (STORAGE, ("Room temperature, away from moisture", "في درجة حرارة الغرفة بعيدًا عن الرطوبة")),
            (("Note", "ملاحظة"), ("Not suitable for babies under 1 year", "غير مناسب للرضّع أقل من عام")),
        ],
        variants=[dict(opts=[], price=449)],
        rating=4.6, reviews=140,
        alt=("Henry's organic honey jar", "برطمان عسل هنريز العضوي"),
    ),
    dict(
        id=2213,
        name=("Magnum Almond Ice Cream Tub, 440 ml", "آيس كريم ماجنم باللوز — علبة 440 مل"),
        subtitle=("Vanilla, cracking chocolate & almonds", "فانيليا وشوكولاتة بتقرمش ولوز"),
        description=(
            "Magnum's take-home tub: creamy vanilla ice cream with cracking milk-chocolate pieces and roasted "
            "almonds. Delivered frozen in an insulated bag — share it with the family, or don't.",
            "علبة ماجنم العائلية: آيس كريم فانيليا كريمي مع قطع شوكولاتة الحليب المقرمشة واللوز المحمّص. يصلك "
            "مجمّدًا في حقيبة معزولة؛ شاركه مع العائلة… أو احتفظ به لنفسك.",
        ),
        highlights=[
            ("Belgian chocolate and roasted almonds", "شوكولاتة بلجيكية ولوز محمّص"),
            ("Delivered frozen, cold chain all the way", "يصل مجمّدًا بسلسلة تبريد كاملة"),
            ("440 ml tub — 3 to 4 servings", "علبة 440 مل تكفي 3–4 أفراد"),
        ],
        specs=[
            (("Volume", "الحجم"), ("440 ml", "440 مل")),
            (("Flavour", "النكهة"), ("Vanilla, milk chocolate & almonds", "فانيليا وشوكولاتة بالحليب ولوز")),
            (("Allergens", "مسببات الحساسية"), ("Milk, almonds, soy; may contain other nuts",
                                                "حليب، لوز، صويا؛ قد يحتوي على مكسرات أخرى")),
            (STORAGE, ("−18 °C; do not refreeze once thawed", "−18 درجة مئوية، ولا يُعاد تجميده بعد ذوبانه")),
        ],
        variants=[dict(opts=[], price=229, compare_at=279)],
        rating=4.8, reviews=720,
        alt=("Magnum Almond ice cream tub", "علبة آيس كريم ماجنم باللوز"),
        views=[
            ("front", "من الأمام"),
            ("side", "من الجانب"),
            ("lid and side", "الغطاء والجانب"),
            ("front, low angle", "من الأمام بزاوية منخفضة"),
        ],
    ),
    dict(
        id=2214,
        name=("Fresh Orange Juice, 1 L", "عصير برتقال طازج — 1 لتر"),
        subtitle=("Squeezed every morning, no sugar added", "معصور كل صبح ومن غير سكر"),
        description=(
            "Juice from Egyptian navel and Valencia oranges, cold-pressed every morning and bottled in glass with "
            "nothing added — no sugar, water or preservatives. Keep it cold and enjoy within three days.",
            "عصير من برتقال أبو سُرّة وفالنسيا المصري، يُعصر على البارد كل صباح ويُعبأ في زجاج دون أي إضافات: "
            "لا سكر ولا ماء ولا مواد حافظة. احفظه باردًا واستمتع به خلال ثلاثة أيام.",
        ),
        highlights=[
            ("100% orange, nothing added", "برتقال 100% بدون إضافات"),
            ("Cold-pressed daily", "معصور على البارد يوميًا"),
            ("Reusable glass bottle", "زجاجة قابلة لإعادة الاستخدام"),
        ],
        specs=[
            (("Volume", "الحجم"), ("1 L glass bottle", "زجاجة 1 لتر")),
            (("Ingredients", "المكونات"), ("Fresh orange juice", "عصير برتقال طازج")),
            (ORIGIN, ("Egyptian oranges", "برتقال مصري")),
            (("Shelf life", "مدة الصلاحية"), ("3 days, refrigerated", "3 أيام في الثلاجة")),
        ],
        variants=[dict(opts=[], price=89)],
        rating=4.5, reviews=430,
        alt=("Bottle of fresh orange juice", "زجاجة عصير برتقال طازج"),
    ),
    dict(
        id=2215,
        name=("Kiwi — Imported", "كيوي مستورد"),
        subtitle=("Sweet-tart and packed with vitamin C", "مِزّ وحلو ومليان فيتامين سي"),
        description=(
            "Green kiwis imported from New Zealand or Italy depending on the season, delivered ready to ripen. Leave "
            "them on the counter for a day or two until they give slightly, then scoop and enjoy.",
            "كيوي أخضر مستورد من نيوزيلندا أو إيطاليا حسب الموسم، يصلك قابلًا للنضج. اتركه يومًا أو يومين خارج "
            "الثلاجة حتى يلين قليلًا، ثم قشّره أو كُله بالملعقة.",
        ),
        highlights=[
            ("High in vitamin C and fibre", "غني بفيتامين C والألياف"),
            ("Ripens at home in 1–2 days", "ينضج في البيت خلال 1–2 يوم"),
        ],
        specs=[
            (ORIGIN, ("New Zealand or Italy, by season", "نيوزيلندا أو إيطاليا حسب الموسم")),
            (W, ("500 g — about 5–6 kiwis", "500 جرام، نحو 5–6 حبات")),
            (STORAGE, ("Ripen at room temperature, then refrigerate",
                       "يُترك لينضج في درجة حرارة الغرفة ثم يُحفظ في الثلاجة")),
        ],
        variants=[dict(opts=[], price=89)],
        rating=4.4, reviews=180,
        alt=("Kiwis with one cut in half", "حبات كيوي مع حبة مقطوعة نصفين"),
    ),
    dict(
        id=2216,
        name=("Yellow Lemons (Adalia)", "ليمون أضاليا أصفر"),
        subtitle=("Juicy and zesty, for every dish", "معصّر ومنعش لكل أكلة"),
        description=(
            "Large, thin-skinned Adalia lemons grown in Egypt — juicier and milder than baladi limes. Perfect for "
            "lemonade, salad dressings, fish and a squeeze over koshari.",
            "ليمون أضاليا مصري كبير ورقيق القشر، أكثر عصارة وأخف حموضة من الليمون البلدي. مثالي للعصير وتتبيل "
            "السلطات والسمك، وعصرة على الكشري.",
        ),
        highlights=[
            ("Thin skin, lots of juice", "قشر رقيق وعصارة كثيرة"),
            ("Great for lemonade and marinades", "ممتاز للعصير والتتبيلات"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (("Variety", "الصنف"), ("Adalia", "أضاليا")),
            (W, ("1 kg — about 7–9 lemons", "1 كجم، نحو 7–9 حبات")),
            (STORAGE, ("Refrigerate, up to 3 weeks", "يُحفظ في الثلاجة حتى 3 أسابيع")),
        ],
        variants=[dict(opts=[], price=49)],
        rating=4.4, reviews=310,
        alt=("Yellow lemons, whole and halved", "ليمون أصفر كامل ومقطوع نصفين"),
    ),
    dict(
        id=2217,
        name=("Dina Farms Fresh Full-Cream Milk, 1 L", "حليب مزارع دينا الطازج كامل الدسم — 1 لتر"),
        subtitle=("Fresh farm milk, delivered cold", "لبن مزرعة طازة يوصلك ساقع"),
        description=(
            "Fresh pasteurised full-cream cow's milk from Dina Farms' own herds, bottled daily and kept cold from "
            "farm to door. Creamy and naturally rich — for tea, coffee, cereal and baking.",
            "حليب بقري طازج كامل الدسم ومبستر من قطعان مزارع دينا، يُعبأ يوميًا ويبقى مبرّدًا من المزرعة حتى بابك. "
            "كريمي وغني بطبيعته، للشاي والقهوة والكورن فليكس والمخبوزات.",
        ),
        highlights=[
            ("Fresh pasteurised milk, not powdered", "حليب طازج مبستر وليس مجففًا"),
            ("3% fat, no additives", "دسم 3% بدون إضافات"),
            ("Delivered chilled", "يصل مبرّدًا"),
        ],
        specs=[
            (("Volume", "الحجم"), ("1 L", "1 لتر")),
            (("Fat", "نسبة الدسم"), ("3%", "3% تقريبًا")),
            (("Type", "النوع"), ("Fresh pasteurised cow's milk", "حليب بقري طازج مبستر")),
            (("Shelf life", "مدة الصلاحية"), ("7 days at 2–6 °C; 3 days after opening",
                                             "7 أيام على 2–6 درجات مئوية، و3 أيام بعد الفتح")),
        ],
        variants=[dict(opts=[], price=69)],
        rating=4.7, reviews=1150,
        alt=("Bottle of fresh milk", "زجاجة حليب طازج"),
    ),
    dict(
        id=2218,
        name=("Black Mulberries", "توت أسود"),
        subtitle=("Seasonal — back in spring", "موسمي، وراجع في الربيع"),
        description=(
            "Sweet, juicy black mulberries from Egyptian orchards, picked ripe and packed gently in 250 g punnets. "
            "Mulberries have a short spring season, so they're sold out until the next harvest.",
            "توت أسود حلو وغني بالعصارة من بساتين مصرية، يُقطف ناضجًا ويُعبأ برفق في علب 250 جرامًا. موسم التوت "
            "قصير في الربيع، لذلك نفد حتى الحصاد القادم.",
        ),
        highlights=[
            ("Picked ripe, packed by hand", "يُقطف ناضجًا ويُعبأ يدويًا"),
            ("Short spring season", "موسم ربيعي قصير"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (W, ("250 g punnet", "علبة 250 جرامًا")),
            (("Season", "الموسم"), ("March to May", "من مارس إلى مايو")),
            (STORAGE, ("Refrigerate unwashed, eat within 2 days", "يُحفظ في الثلاجة دون غسل، ويُؤكل خلال يومين")),
        ],
        variants=[dict(opts=[], price=79)],
        rating=4.6, reviews=85,
        alt=("Black mulberries with leaves", "توت أسود مع أوراقه"),
    ),
    dict(
        id=2219,
        name=("Nescafé Clásico Instant Coffee", "نسكافيه كلاسيك قهوة سريعة التحضير"),
        subtitle=("Dark roast, rich and full-bodied", "تحميص غامق وطعم تقيل"),
        description=(
            "Nescafé Clásico is a dark-roast instant coffee made from 100% pure coffee for a bold, full-bodied cup. "
            "Just add hot water — or hot milk for a creamy morning coffee.",
            "نسكافيه كلاسيك قهوة سريعة التحضير بتحميص غامق، مصنوعة من بن نقي 100% لكوب قوي وغني. أضف الماء الساخن "
            "فقط، أو الحليب الساخن لقهوة صباحية كريمية.",
        ),
        highlights=[
            ("100% pure coffee", "قهوة نقية 100%"),
            ("Dark roast, bold taste", "تحميص غامق وطعم قوي"),
            ("About 100 cups per 200 g jar", "نحو 100 كوب من برطمان 200 جرام"),
        ],
        specs=[
            (("Type", "النوع"), ("Instant coffee, dark roast", "قهوة سريعة التحضير، تحميص غامق")),
            (("Size", "الحجم"), ("100 g or 200 g glass jar", "برطمان زجاج 100 جرام أو 200 جرام")),
            (("Serving", "الحصة"), ("1 teaspoon (2 g) per cup", "ملعقة صغيرة (2 جرام) لكل كوب")),
            (STORAGE, ("Close tightly and keep dry", "يُغلق جيدًا ويُحفظ في مكان جاف")),
        ],
        options=[
            dict(id="size", name=("Size", "الحجم"), kind="chip", values=[
                dict(id="100g", label=("100 g", "100 جرام"), code="100G"),
                dict(id="200g", label=("200 g", "200 جرام"), code="200G"),
            ]),
        ],
        variants=[
            dict(opts=["100g"], price=199),
            dict(opts=["200g"], price=359, compare_at=419),
        ],
        rating=4.7, reviews=5200,
        alt=("Nescafé Clásico instant coffee jar", "برطمان نسكافيه كلاسيك"),
    ),
    dict(
        id=2220,
        name=("Potatoes", "بطاطس"),
        subtitle=("For fries, oven trays and everything in between", "للبطاطس المحمّرة والصواني وكل حاجة"),
        description=(
            "Fresh Egyptian Spunta potatoes with smooth skins and firm, golden flesh. They fry up crisp, roast "
            "beautifully and make a fluffy mash.",
            "بطاطس مصرية طازجة من صنف سبونتا بقشرة ناعمة ولُبّ ذهبي متماسك. تُقلى مقرمشة، وتُشوى في الفرن "
            "بشكل رائع، وتصلح لبطاطس مهروسة ناعمة.",
        ),
        highlights=[
            ("Firm, all-purpose potatoes", "بطاطس متماسكة لكل الاستخدامات"),
            ("Save with the 3 kg bag", "وفّر مع كيس 3 كجم"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (("Variety", "الصنف"), ("Spunta", "سبونتا")),
            (STORAGE, ("Cool, dark, dry place — not the fridge", "مكان بارد ومظلم وجاف، وليس الثلاجة")),
        ],
        options=[
            dict(id="weight", name=W, kind="chip", values=[
                dict(id="1kg", label=("1 kg", "1 كجم"), code="1KG"),
                dict(id="3kg", label=("3 kg", "3 كجم"), code="3KG"),
            ]),
        ],
        variants=[
            dict(opts=["1kg"], price=29),
            dict(opts=["3kg"], price=79),
        ],
        rating=4.4, reviews=690,
        alt=("Fresh potatoes", "بطاطس طازجة"),
    ),
    dict(
        id=2221,
        name=("Red Onions", "بصل أحمر"),
        subtitle=("The base of every Egyptian pot", "أساس كل طبخة مصري"),
        description=(
            "Firm Egyptian red onions with a sharp bite raw and a sweet finish when cooked. The base of mahshi, "
            "molokhia and every tagine — and great sliced into salads.",
            "بصل أحمر مصري متماسك، لاذع وهو نيء وحلو المذاق بعد الطهي. أساس المحشي والملوخية وكل طاجن، ورائع "
            "شرائح في السلطات.",
        ),
        highlights=[
            ("Dry, firm and long-keeping", "جاف ومتماسك ويُخزَّن طويلًا"),
            ("Sorted by hand, no sprouting", "منتقى يدويًا وخالٍ من الإنبات"),
        ],
        specs=[
            (ORIGIN, EGYPT),
            (W, ("1 kg", "1 كجم")),
            (STORAGE, ("Cool, dry, ventilated place, up to 4 weeks", "مكان بارد وجاف وجيد التهوية حتى 4 أسابيع")),
        ],
        variants=[dict(opts=[], price=29)],
        rating=4.3, reviews=410,
        alt=("Red onions, whole and sliced", "بصل أحمر كامل ومقطّع"),
    ),
    dict(
        id=2222,
        name=("Isis Organic Egyptian Rice, 1 kg", "أرز مصري عضوي من إيزيس — 1 كجم"),
        subtitle=("Egyptian short grain, certified organic", "رز مصري حبة قصيرة وعضوي"),
        description=(
            "Certified-organic Egyptian short-grain rice from Isis, grown without synthetic pesticides and packed in "
            "a paper bag with a window so you can see the grain. Cooks soft and fluffy — perfect for mahshi, roz bel "
            "laban and everyday rice.",
            "أرز مصري قصير الحبة عضوي معتمد من إيزيس، مزروع دون مبيدات صناعية ومعبأ في كيس ورقي بنافذة لترى الحبوب "
            "بنفسك. يُطهى طريًا ومفلفلًا، مثالي للمحشي والأرز باللبن والأرز اليومي.",
        ),
        highlights=[
            ("Certified organic", "عضوي معتمد"),
            ("Egyptian short grain", "حبة مصرية قصيرة"),
            ("Recyclable paper pack", "عبوة ورقية قابلة لإعادة التدوير"),
        ],
        specs=[
            (("Net weight", "الوزن الصافي"), ("1 kg", "1 كجم")),
            (("Type", "النوع"), ("Short-grain white rice", "أرز أبيض قصير الحبة")),
            (ORIGIN, EGYPT),
            (STORAGE, ("Cool, dry place", "مكان بارد وجاف")),
        ],
        variants=[dict(opts=[], price=89)],
        rating=4.5, reviews=260,
        alt=("Isis organic rice in a kraft paper bag", "أرز إيزيس العضوي في كيس ورقي"),
    ),
    dict(
        id=2223,
        name=("Coca-Cola Original", "كوكاكولا الأصلية"),
        subtitle=("Ice-cold, at your door in minutes", "ساقعة وتوصلك في دقايق"),
        description=(
            "The original Coca-Cola taste in a 1 litre bottle — made for sharing at lunch or with friends. Grab a "
            "single bottle or the 6-bottle pack for the week.",
            "طعم كوكاكولا الأصلي في زجاجة 1 لتر، مثالية للمشاركة على الغداء أو مع الأصدقاء. اطلب زجاجة واحدة أو "
            "عبوة الـ6 زجاجات للأسبوع كله.",
        ),
        highlights=[
            ("1 litre bottle", "زجاجة 1 لتر"),
            ("Save with the 6-pack", "وفّر مع عبوة الـ6"),
        ],
        specs=[
            (("Volume", "الحجم"), ("1 L per bottle", "1 لتر للزجاجة")),
            (("Pack", "العبوة"), ("1 bottle or 6 bottles", "زجاجة واحدة أو 6 زجاجات")),
            (("Made in", "بلد الصنع"), EGYPT),
            (STORAGE, ("Best served chilled", "يُقدَّم باردًا")),
        ],
        options=[
            dict(id="pack", name=("Pack", "العبوة"), kind="chip", values=[
                dict(id="1x1l", label=("1 × 1 L", "1 × 1 لتر"), code="1L"),
                dict(id="6x1l", label=("6 × 1 L", "6 × 1 لتر"), code="6X1L"),
            ]),
        ],
        variants=[
            dict(opts=["1x1l"], price=29),
            dict(opts=["6x1l"], price=149),
        ],
        rating=4.8, reviews=8400,
        alt=("Coca-Cola 1 litre bottle", "زجاجة كوكاكولا 1 لتر"),
    ),
    dict(
        id=2224,
        name=("Fresh Strawberries", "فراولة طازجة"),
        subtitle=("Sweet, red and just picked", "حمرا وحلوة ولسه مقطوفة"),
        description=(
            "Sweet Egyptian strawberries from the season's first harvest, picked ripe and packed in 500 g punnets. "
            "Rinse just before eating — perfect with cream, in smoothies or straight from the box.",
            "فراولة مصرية حلوة من أول حصاد في الموسم، تُقطف ناضجة وتُعبأ في علب 500 جرام. اغسلها قبل الأكل مباشرة، "
            "ورائعة مع القشطة أو في السموذي أو مباشرة من العلبة.",
        ),
        highlights=[
            ("Egyptian, early-season harvest", "فراولة مصرية من أول الموسم"),
            ("Packed in protective punnets", "معبأة في علب تحميها من الهرس"),
        ],
        specs=[
            (ORIGIN, ("Egypt (Ismailia and Beheira)", "مصر (الإسماعيلية والبحيرة)")),
            (W, ("500 g punnet", "علبة 500 جرام")),
            (STORAGE, ("Refrigerate, best within 3 days", "يُحفظ في الثلاجة، ويُفضَّل استهلاكه خلال 3 أيام")),
        ],
        variants=[dict(opts=[], price=59)],
        rating=4.5, reviews=520,
        alt=("Fresh strawberries", "فراولة طازجة"),
    ),
    dict(
        id=2225,
        name=("Siwa Natural Mineral Water, 1.5 L", "مياه سيوة الطبيعية المعدنية — 1.5 لتر"),
        subtitle=("Pure spring water from Siwa Oasis", "مياه نقية من واحة سيوة"),
        description=(
            "Natural mineral water bottled at the source in Siwa Oasis, light and low in sodium. Order a single "
            "bottle or a 6-pack and we'll carry it right to your door.",
            "مياه معدنية طبيعية معبأة من المنبع في واحة سيوة، خفيفة وقليلة الصوديوم. اطلب زجاجة واحدة أو عبوة "
            "6 زجاجات، وسنوصلها حتى باب بيتك.",
        ),
        highlights=[
            ("Bottled at source in Siwa Oasis", "معبأة من المنبع في واحة سيوة"),
            ("Low in sodium", "قليلة الصوديوم"),
            ("Carried to your door", "نوصلها حتى باب بيتك"),
        ],
        specs=[
            (("Volume", "الحجم"), ("1.5 L per bottle", "1.5 لتر للزجاجة")),
            (("Pack", "العبوة"), ("1 bottle or 6 bottles", "زجاجة واحدة أو 6 زجاجات")),
            (("Source", "المصدر"), ("Siwa Oasis, Egypt", "واحة سيوة، مصر")),
            (STORAGE, ("Away from direct sunlight", "بعيدًا عن أشعة الشمس المباشرة")),
        ],
        options=[
            dict(id="pack", name=("Pack", "العبوة"), kind="chip", values=[
                dict(id="1x1-5l", label=("1 × 1.5 L", "1 × 1.5 لتر"), code="1X15"),
                dict(id="6x1-5l", label=("6 × 1.5 L", "6 × 1.5 لتر"), code="6X15"),
            ]),
        ],
        variants=[
            dict(opts=["1x1-5l"], price=9),
            dict(opts=["6x1-5l"], price=49),
        ],
        rating=4.7, reviews=3900,
        alt=("Siwa natural mineral water bottle", "زجاجة مياه سيوة الطبيعية"),
    ),
]

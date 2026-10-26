"""Fashion (category 15: products 1501–1514) and Shoes (category 16: products 1601–1610).

Prices are whole EGP (the build converts to piastres). Brand, tags, images and slugs come from
skeleton.py.
"""


def sizes(*ids):
    return dict(id="size", name=("Size", "المقاس"), kind="chip",
                values=[dict(id=i.lower(), label=(i, i)) for i in ids])


def eu(lo, hi):
    return dict(id="size", name=("EU size", "المقاس الأوروبي"), kind="chip",
                values=[dict(id=str(n), label=(str(n), str(n))) for n in range(lo, hi + 1)])


KIDS = dict(id="age", name=("Age", "السن"), kind="chip", values=[
    dict(id="4-5y", label=("4–5 yrs", "4–5 سنوات")),
    dict(id="6-7y", label=("6–7 yrs", "6–7 سنوات")),
    dict(id="8-9y", label=("8–9 yrs", "8–9 سنوات")),
    dict(id="10-11y", label=("10–11 yrs", "10–11 سنة")),
])


def each(option, price, compare_at=None):
    """One variant per value of a single axis, all at the same price."""
    out = []
    for v in option["values"]:
        var = dict(opts=[v["id"]], price=price)
        if compare_at:
            var["compare_at"] = compare_at
        out.append(var)
    return out


# Shared view captions (front / angled / back / side), as photographed.
GARMENT_VIEWS = [("front", "من الأمام"), ("angled view", "من زاوية جانبية"),
                 ("back", "من الخلف"), ("side", "من الجانب")]
DRESS_SIDE_FIRST = [("front", "من الأمام"), ("side", "من الجانب"),
                    ("back", "من الخلف"), ("angled view", "من زاوية جانبية")]
SHOE_VIEWS = [("pair from the front", "الزوج من الأمام"), ("side", "من الجانب"),
              ("pair from the back", "الزوج من الخلف"), ("pair at an angle", "الزوج من زاوية جانبية")]

CARE_30 = (("Care", "العناية"), ("Machine wash 30°C", "غسيل آلي على 30 درجة"))

S_XXL = sizes("S", "M", "L", "XL", "XXL")
XS_XL = sizes("XS", "S", "M", "L", "XL")
XS_L = sizes("XS", "S", "M", "L")
MEN_EU = eu(40, 45)
WOMEN_EU = eu(36, 41)

PRODUCTS = [
    # ------------------------------------------------------------------ Fashion
    dict(
        id=1501,
        name=("Town Team Gingham Oxford Shirt", "قميص أكسفورد بمربعات جينجهام من تاون تيم"),
        subtitle=("Button-down collar · 100% cotton · Regular fit", "ياقة بزراير · قطن 100% · قصة مريحة"),
        description=(
            "A blue-and-black gingham Oxford in soft, breathable cotton with a button-down collar. Wear it "
            "buttoned up with chinos for the office, or open over a white tee at the weekend. The regular fit "
            "and curved hem work tucked in or left out.",
            "قميص أكسفورد بمربعات جينجهام باللونين الأزرق والأسود، من قطن ناعم يسمح بمرور الهواء وبياقة "
            "بأزرار. ارتدِه مغلقًا مع بنطلون تشينو في العمل، أو مفتوحًا فوق تيشيرت أبيض في عطلة نهاية الأسبوع. "
            "قصته المريحة وحافته المستديرة تناسبان إدخاله في البنطلون أو تركه خارجه.",
        ),
        highlights=[
            ("100% cotton Oxford weave", "نسيج أكسفورد من القطن 100%"),
            ("Button-down collar that keeps its shape", "ياقة بأزرار تحافظ على شكلها"),
            ("Works for the office and the weekend", "يناسب العمل والخروجات"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% cotton Oxford", "قطن 100% بنسيج أكسفورد")),
            (("Fit", "القصة"), ("Regular", "عادية مريحة")),
            (("Collar", "الياقة"), ("Button-down", "بأزرار")),
            (("Sleeves", "الأكمام"), ("Long, buttoned cuffs", "طويلة بأساور بأزرار")),
            (("Care", "العناية"), ("Machine wash 30°C, warm iron", "غسيل آلي على 30 درجة، كي بحرارة متوسطة")),
        ],
        options=[S_XXL],
        variants=each(S_XXL, 899),
        rating=4.5, reviews=1846,
        alt=("Town Team gingham Oxford shirt in blue and black",
             "قميص أكسفورد من تاون تيم بمربعات جينجهام زرقاء وسوداء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1502,
        name=("AORUS Gaming T-shirt", "تيشيرت أوروس للألعاب"),
        subtitle=("Eagle graphic · Combed cotton · Unisex fit", "طبعة النسر الشهيرة · قطن ممشط · للجنسين"),
        description=(
            "Official AORUS merch for PC gamers: a white crew-neck tee with the AORUS eagle sketched across the "
            "chest, logos on the sleeves and the Gigabyte 'G' at the top of the back. Soft combed-cotton jersey "
            "that stays comfortable through long sessions.",
            "تيشيرت رسمي من أوروس لعشاق ألعاب الكمبيوتر: لون أبيض بياقة دائرية، عليه رسمة نسر أوروس على "
            "الصدر وشعارات على الأكمام وحرف G الخاص بجيجابايت أعلى الظهر. مصنوع من قطن ممشط ناعم يظل مريحًا "
            "طوال جلسات اللعب الطويلة.",
        ),
        highlights=[
            ("Official AORUS / Gigabyte design", "تصميم رسمي من أوروس وجيجابايت"),
            ("Combed-cotton jersey, 180 gsm", "قطن ممشط بوزن 180 جرامًا للمتر المربع"),
            ("Print made to survive wash after wash", "طباعة تتحمل الغسيل المتكرر"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% combed cotton jersey", "قطن ممشط 100%")),
            (("Fit", "القصة"), ("Regular, unisex", "عادية، للجنسين")),
            (("Neckline", "الياقة"), ("Crew neck", "دائرية")),
            (("Print", "الطباعة"), ("Front eagle, sleeve logos, back-neck G", "نسر على الصدر وشعارات على الأكمام وحرف G أعلى الظهر")),
            (("Care", "العناية"), ("Wash inside out at 30°C", "يُغسل مقلوبًا على 30 درجة")),
        ],
        options=[S_XXL],
        variants=each(S_XXL, 799),
        rating=4.6, reviews=37,
        alt=("White AORUS gaming T-shirt with eagle print", "تيشيرت أوروس أبيض بطبعة النسر"),
        views=[("front", "من الأمام"), ("angled view", "من زاوية جانبية"),
               ("back with G logo", "من الخلف مع شعار G"), ("angled from the other side", "من الزاوية الأخرى")],
    ),
    dict(
        id=1503,
        name=("DeFacto Flannel Check Shirt", "قميص فلانيل بمربعات من ديفاكتو"),
        subtitle=("Brushed cotton flannel · Red & black check", "فلانيل قطن دافي · كاروهات أحمر في أسود"),
        description=(
            "A winter staple in red-and-black buffalo check, cut from brushed cotton flannel that feels soft "
            "from the first wear. Regular fit with a classic point collar and buttoned cuffs; layer it over a tee "
            "or under a puffer jacket.",
            "قطعة أساسية للشتاء بمربعات حمراء وسوداء كبيرة، من قماش فلانيل قطني مُصنفر ناعم من أول ارتداء. "
            "قصة عادية بياقة كلاسيكية وأساور بأزرار، ويمكن ارتداؤه فوق تيشيرت أو تحت جاكيت منفوخ.",
        ),
        highlights=[
            ("Brushed flannel, warm but breathable", "فلانيل مُصنفر دافئ ويسمح بمرور الهواء"),
            ("Doubles as a light overshirt", "يصلح أيضًا كجاكيت خفيف مفتوح"),
            ("Colours that hold up in the wash", "ألوان ثابتة مع الغسيل"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% cotton flannel", "فلانيل قطن 100%")),
            (("Fit", "القصة"), ("Regular", "عادية")),
            (("Collar", "الياقة"), ("Classic point collar", "ياقة كلاسيكية")),
            (("Pattern", "النقشة"), ("Buffalo check", "مربعات بافلو كبيرة")),
            (("Care", "العناية"), ("Machine wash 30°C, no tumble dry", "غسيل آلي على 30 درجة، بدون مجفف")),
        ],
        options=[S_XXL],
        variants=each(S_XXL, 699, 899),
        rating=4.4, reviews=2310,
        alt=("DeFacto red and black flannel check shirt", "قميص فلانيل من ديفاكتو بمربعات حمراء وسوداء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1504,
        name=("Concrete Hibiscus Print Resort Shirt", "قميص صيفي بطبعة زهور الهيبسكس من كونكريت"),
        subtitle=("Camp collar · Flowing viscose · Short sleeves", "ياقة كامب · فسكوز خفيف · نص كم"),
        description=(
            "Made for Sahel nights and Red Sea getaways: a short-sleeve resort shirt in fluid viscose, printed with "
            "white hibiscus on a deep blue ground. The relaxed camp collar and straight hem are meant to be worn "
            "untucked.",
            "صُمّم لسهرات الساحل الشمالي وإجازات البحر الأحمر: قميص بأكمام قصيرة من الفسكوز الانسيابي، بطبعة "
            "زهور هيبسكس بيضاء على أرضية زرقاء داكنة. ياقة كامب مريحة وحافة مستقيمة ليُرتدى خارج البنطلون.",
        ),
        highlights=[
            ("Light, breezy viscose for summer heat", "فسكوز خفيف ومنعش لحر الصيف"),
            ("All-over hibiscus print", "طبعة هيبسكس على القميص بالكامل"),
            ("Straight hem, designed to wear untucked", "حافة مستقيمة تُرتدى خارج البنطلون"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% viscose", "فسكوز 100%")),
            (("Fit", "القصة"), ("Relaxed", "واسعة مريحة")),
            (("Collar", "الياقة"), ("Camp (Cuban) collar", "ياقة كامب (كوبية)")),
            (("Sleeves", "الأكمام"), ("Short", "قصيرة")),
            (("Care", "العناية"), ("Hand wash or gentle cycle 30°C", "غسيل يدوي أو برنامج رقيق على 30 درجة")),
        ],
        options=[S_XXL],
        variants=each(S_XXL, 999),
        rating=4.3, reviews=214,
        alt=("Concrete short-sleeve shirt with blue hibiscus print", "قميص كونكريت بأكمام قصيرة وطبعة هيبسكس زرقاء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1505,
        name=("LC Waikiki Slim-fit Check Shirt", "قميص بمربعات بقصة ضيقة من إل سي وايكيكي"),
        subtitle=("Green & blue tartan · Slim fit · Easy iron", "كاروهات أخضر في أزرق · قصة سليم · سهل الكي"),
        description=(
            "An everyday long-sleeve shirt in a green, blue and cream tartan check with a button-down collar. The "
            "slim cut sits close without pulling, and the cotton-rich fabric irons in seconds on busy mornings.",
            "قميص يومي بأكمام طويلة بمربعات تارتان بالأخضر والأزرق والكريمي، وبياقة بأزرار. قصته الضيقة "
            "تحدد الجسم دون أن تضايقك، وقماشه الغني بالقطن سهل الكي في الصباحات المزدحمة.",
        ),
        highlights=[
            ("Slim through the chest and waist", "قصة ضيقة عند الصدر والوسط"),
            ("Cotton-rich, easy-iron fabric", "قماش غني بالقطن وسهل الكي"),
            ("Great value for daily wear", "سعر ممتاز للبس اليومي"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("70% cotton, 30% polyester", "قطن 70%، بوليستر 30%")),
            (("Fit", "القصة"), ("Slim", "ضيقة (سليم)")),
            (("Collar", "الياقة"), ("Button-down", "بأزرار")),
            (("Sleeves", "الأكمام"), ("Long", "طويلة")),
            (("Care", "العناية"), ("Machine wash 40°C", "غسيل آلي على 40 درجة")),
        ],
        options=[S_XXL],
        variants=each(S_XXL, 599),
        rating=4.2, reviews=389,
        alt=("LC Waikiki green and blue check shirt", "قميص إل سي وايكيكي بمربعات خضراء وزرقاء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1506,
        name=("Girls' Polka-Dot Skater Dress", "فستان بناتي منقّط بتنورة واسعة"),
        subtitle=("Cap sleeves · Twirly skirt · Soft cotton", "أكمام كاب · جيبة واسعة بتلف · قطن ناعم"),
        description=(
            "A light-blue skater dress from LC Waikiki Kids, scattered with blue and lilac dots, with little cap "
            "sleeves and a full skirt that loves to twirl. Soft cotton jersey makes it comfortable for birthdays, "
            "Eid visits and school trips.",
            "فستان سكيتر باللون الأزرق الفاتح من إل سي وايكيكي للأطفال، بنقاط زرقاء وليلكية، وأكمام كاب "
            "قصيرة وتنورة واسعة تدور مع كل حركة. قماشه القطني الناعم يجعله مريحًا في أعياد الميلاد وزيارات "
            "العيد والرحلات المدرسية.",
        ),
        highlights=[
            ("Soft, breathable cotton for active kids", "قطن ناعم يسمح بمرور الهواء للأطفال كثيري الحركة"),
            ("Full skirt with plenty of twirl", "تنورة واسعة تدور بسهولة"),
            ("Easy pull-on design", "سهل اللبس والخلع"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("95% cotton, 5% elastane", "قطن 95%، إيلاستين 5%")),
            (("Sleeves", "الأكمام"), ("Cap sleeves", "أكمام كاب قصيرة")),
            (("Length", "الطول"), ("Knee length", "حتى الركبة")),
            (("Ages", "الأعمار"), ("4 to 11 years", "من 4 إلى 11 سنة")),
            CARE_30,
        ],
        options=[KIDS],
        variants=each(KIDS, 549),
        rating=4.7, reviews=128,
        alt=("Girls' light-blue polka-dot skater dress", "فستان بناتي أزرق فاتح منقّط بتنورة واسعة"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1507,
        name=("Andora Leaf-Print Off-Shoulder Maxi Dress", "فستان ماكسي مكشوف الكتفين بطبعة أوراق من أندورا"),
        subtitle=("Palm-leaf print · Ruffle neckline · Floor length", "طبعة ورق شجر · كشكشة عند الصدر · طويل لحد الأرض"),
        description=(
            "A breezy maxi in a bold black-and-white palm-leaf print, with a ruffled off-shoulder neckline and thin "
            "adjustable straps. Airy viscose falls straight to the floor, made for summer weddings or dinner in the "
            "Sahel; add a cropped denim jacket or a light cardigan to cover the shoulders.",
            "فستان ماكسي خفيف بطبعة أوراق نخيل جريئة بالأبيض والأسود، بفتحة صدر مكشوفة الكتفين مزينة بكشكشة "
            "وحمالات رفيعة قابلة للضبط. مصنوع من الفسكوز الخفيف وينسدل حتى الأرض، ومناسب لأفراح الصيف أو العشاء "
            "في الساحل، ويمكن ارتداء جاكيت جينز قصير أو كارديجان خفيف فوقه لتغطية الكتفين.",
        ),
        highlights=[
            ("Statement palm-leaf print", "طبعة أوراق نخيل لافتة"),
            ("Ruffled off-shoulder neckline with adjustable straps", "صدر مكشوف الكتفين بكشكشة وحمالات قابلة للضبط"),
            ("Fully lined, floor-length skirt", "تنورة طويلة حتى الأرض ببطانة كاملة"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% viscose, polyester lining", "فسكوز 100% ببطانة بوليستر")),
            (("Neckline", "فتحة الصدر"), ("Off-shoulder ruffle", "مكشوفة الكتفين بكشكشة")),
            (("Length", "الطول"), ("Maxi, about 140 cm", "ماكسي، حوالي 140 سم")),
            (("Closure", "الإغلاق"), ("Concealed side zip", "سحّاب جانبي مخفي")),
            (("Care", "العناية"), ("Gentle wash 30°C", "غسيل رقيق على 30 درجة")),
        ],
        options=[XS_XL],
        variants=each(XS_XL, 1499),
        rating=4.6, reviews=342,
        alt=("Andora black and white leaf-print maxi dress", "فستان ماكسي من أندورا بطبعة أوراق بالأبيض والأسود"),
        views=DRESS_SIDE_FIRST,
    ),
    dict(
        id=1508,
        name=("Mobaco Linen-Blend Button-Front Midi Dress", "فستان ميدي من الكتان بأزرار أمامية من موباكو"),
        subtitle=("Linen & Egyptian cotton · Tie waist · Sleeveless", "كتان وقطن مصري · حزام بيتربط · من غير أكمام"),
        description=(
            "Mobaco's easy summer dress: a sleeveless midi in a stone-coloured blend of linen and Egyptian cotton, "
            "buttoned all the way down the front with a self-tie waist. Wear it on its own on hot days, or over a "
            "fitted long-sleeve top for a modest layered look.",
            "فستان موباكو الصيفي السهل: فستان ميدي بدون أكمام بلون الحجر، من مزيج الكتان والقطن المصري، بأزرار "
            "على طول الأمام وحزام من نفس القماش يُربط عند الخصر. ارتديه وحده في الأيام الحارة، أو فوق بلوزة "
            "بأكمام طويلة لإطلالة محتشمة بطبقات.",
        ),
        highlights=[
            ("Linen and Egyptian cotton that breathes", "مزيج كتان وقطن مصري يسمح بمرور الهواء"),
            ("Full button front and self-tie belt", "أزرار أمامية كاملة وحزام من نفس القماش"),
            ("Made in Egypt", "صناعة مصرية"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("55% linen, 45% Egyptian cotton", "كتان 55%، قطن مصري 45%")),
            (("Length", "الطول"), ("Midi, about 120 cm", "ميدي، حوالي 120 سم")),
            (("Fit", "القصة"), ("Regular with tie waist", "عادية بحزام عند الخصر")),
            (("Closure", "الإغلاق"), ("Front buttons", "أزرار أمامية")),
            (("Care", "العناية"), ("Machine wash 30°C, iron while damp", "غسيل آلي على 30 درجة، يُكوى وهو رطب")),
        ],
        options=[XS_XL],
        variants=each(XS_XL, 1299),
        rating=4.5, reviews=41,
        alt=("Mobaco stone-coloured linen-blend button-front midi dress", "فستان ميدي من موباكو بلون الحجر بأزرار أمامية"),
        views=DRESS_SIDE_FIRST,
    ),
    dict(
        id=1509,
        name=("Koton Strapless Tiered Dress with Tie Belt", "فستان بدون حمالات بطبقات وحزام معقود من كوتون"),
        subtitle=("Sage green · Two-tier skirt · Bow belt", "أخضر هادي · جيبة طبقتين · حزام بفيونكة"),
        description=(
            "A sage-green party dress with a structured strapless bodice, a two-tier skater skirt and a fabric belt "
            "that ties into a soft bow at the side. Pair it with a cropped bolero or a fitted cardigan for "
            "engagements and evening gatherings.",
            "فستان سهرة بلون أخضر هادئ، بصدر بدون حمالات محدد الشكل وتنورة سكيتر بطبقتين وحزام من القماش يُربط "
            "بفيونكة ناعمة على الجانب. نسّقيه مع بوليرو قصير أو كارديجان ضيق في الخطوبات والسهرات.",
        ),
        highlights=[
            ("Boned strapless bodice that stays in place", "صدر بدون حمالات مدعّم يثبت في مكانه"),
            ("Two-tier skirt with easy movement", "تنورة بطبقتين تتحرك بخفة"),
            ("Detachable tie belt", "حزام معقود قابل للفك"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("100% polyester crepe, lined", "كريب بوليستر 100% ببطانة")),
            (("Length", "الطول"), ("Above the knee", "فوق الركبة")),
            (("Neckline", "فتحة الصدر"), ("Strapless", "بدون حمالات")),
            (("Closure", "الإغلاق"), ("Concealed back zip", "سحّاب خلفي مخفي")),
            (("Care", "العناية"), ("Hand wash cold", "غسيل يدوي بماء بارد")),
        ],
        options=[XS_L],
        variants=each(XS_L, 999, 1399),
        rating=4.2, reviews=176,
        alt=("Koton sage-green strapless tiered dress with tie belt", "فستان كوتون أخضر هادئ بدون حمالات بطبقات وحزام"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1510,
        name=("Girls' Tartan Party Dress with Bow", "فستان بناتي للحفلات بمربعات التارتان وفيونكة"),
        subtitle=("Black & white tartan · Satin bow · Full skirt", "كاروهات أبيض وأسود · فيونكة ستان · جيبة منفوشة"),
        description=(
            "A little party dress from LC Waikiki Kids in black-and-white tartan, with a sleeveless fitted top, a "
            "full twirl skirt and a black satin sash that ties into a big bow. A soft lining keeps it comfortable "
            "for Eid, birthdays and family photos.",
            "فستان حفلات للبنات من إل سي وايكيكي بمربعات تارتان بالأبيض والأسود، بجزء علوي بدون أكمام وتنورة "
            "واسعة منفوشة وحزام ستان أسود يُربط بفيونكة كبيرة. البطانة الناعمة تجعله مريحًا في العيد وأعياد "
            "الميلاد وصور العائلة.",
        ),
        highlights=[
            ("Big black satin bow", "فيونكة ستان سوداء كبيرة"),
            ("Soft cotton lining, no itch", "بطانة قطنية ناعمة لا تسبب حكة"),
            ("Full skirt that holds its shape", "تنورة منفوشة تحافظ على شكلها"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("Poly-cotton tartan, cotton lining", "تارتان قطن وبوليستر ببطانة قطنية")),
            (("Sleeves", "الأكمام"), ("Sleeveless", "بدون أكمام")),
            (("Length", "الطول"), ("Knee length", "حتى الركبة")),
            (("Ages", "الأعمار"), ("4 to 11 years", "من 4 إلى 11 سنة")),
            (("Care", "العناية"), ("Gentle machine wash 30°C", "غسيل آلي رقيق على 30 درجة")),
        ],
        options=[KIDS],
        variants=each(KIDS, 649),
        rating=4.8, reviews=95,
        alt=("Girls' black and white tartan party dress with black bow",
             "فستان بناتي بمربعات تارتان بالأبيض والأسود وفيونكة سوداء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1511,
        name=("Andora Floral Jacquard Evening Gown", "فستان سهرة جاكار مزهّر من أندورا"),
        subtitle=("Off-shoulder · Floor length · Pink & blue blooms on black",
                  "أوف شولدر · طويل لحد الأرض · ورد بينك وأزرق على أسود"),
        description=(
            "An evening gown in black jacquard woven with pink and blue blooms, with a velvet-trimmed off-shoulder "
            "neckline and a full floor-length skirt. Made for weddings and formal dinners; a matching shawl or "
            "bolero turns it into a modest look in seconds.",
            "فستان سهرة من قماش الجاكار الأسود المنسوج بورود وردية وزرقاء، بفتحة صدر مكشوفة الكتفين بحافة "
            "مخملية وتنورة واسعة طويلة حتى الأرض. مصمم للأفراح وحفلات العشاء الرسمية، ويتحول إلى إطلالة محتشمة "
            "في ثوانٍ مع شال أو بوليرو مناسب.",
        ),
        highlights=[
            ("Woven floral jacquard with a rich sheen", "جاكار منسوج بورود ولمعة فاخرة"),
            ("Velvet-trimmed off-shoulder neckline", "صدر مكشوف الكتفين بحافة مخملية"),
            ("Fully lined with built-in support", "مبطن بالكامل بدعامة داخلية"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("Polyester jacquard, satin lining", "جاكار بوليستر ببطانة ساتان")),
            (("Length", "الطول"), ("Floor length", "طويل حتى الأرض")),
            (("Neckline", "فتحة الصدر"), ("Off-shoulder, velvet trim", "مكشوفة الكتفين بحافة مخملية")),
            (("Closure", "الإغلاق"), ("Concealed back zip", "سحّاب خلفي مخفي")),
            (("Care", "العناية"), ("Dry clean only", "تنظيف جاف فقط")),
        ],
        options=[XS_XL],
        variants=each(XS_XL, 3299),
        rating=4.7, reviews=88,
        alt=("Andora black floral jacquard off-shoulder evening gown",
             "فستان سهرة أسود من أندورا بجاكار مزهّر ومكشوف الكتفين"),
        views=DRESS_SIDE_FIRST,
    ),
    dict(
        id=1512,
        name=("Faux-Leather Corset & Maxi Skirt Set", "طقم كورسيه من الجلد الصناعي مع تنورة ماكسي"),
        subtitle=("Two-piece set · Corset top + satin maxi skirt", "طقم قطعتين · كورسيه وجيبة ساتان ماكسي"),
        description=(
            "A two-piece evening set from DeFacto: a structured faux-leather corset top with a fluid satin maxi skirt "
            "in the same shade, in fiery red or classic black. Wear them together for a bold party look, or style "
            "the skirt with a long-sleeve blouse and the corset over a shirt for a more covered outfit.",
            "طقم سهرة من قطعتين من ديفاكتو: كورسيه من الجلد الصناعي محدد الشكل مع تنورة ماكسي من الساتان "
            "الانسيابي بنفس اللون، متوفر بالأحمر الناري أو الأسود الكلاسيكي. ارتديهما معًا لإطلالة سهرة جريئة، "
            "أو نسّقي التنورة مع بلوزة بأكمام طويلة والكورسيه فوق قميص لإطلالة أكثر احتشامًا.",
        ),
        highlights=[
            ("Two pieces to wear together or apart", "قطعتان ترتديهما معًا أو كلٌّ على حدة"),
            ("Boned faux-leather corset that defines the waist", "كورسيه جلد صناعي مدعّم يحدد الخصر"),
            ("Satin maxi skirt with elastic waistband", "تنورة ماكسي ساتان بخصر مطاطي"),
        ],
        specs=[
            (("Corset", "الكورسيه"), ("Boned faux leather (PU)", "جلد صناعي (PU) مدعّم")),
            (("Skirt", "التنورة"), ("Polyester satin, maxi length", "ساتان بوليستر، طول ماكسي")),
            (("Closure", "الإغلاق"), ("Corset back zip, elastic skirt waist", "سحّاب خلفي للكورسيه وخصر مطاطي للتنورة")),
            (("Care", "العناية"), ("Wipe the corset clean; hand wash the skirt",
                                 "يُمسح الكورسيه بقطعة قماش مبللة، وتُغسل التنورة يدويًا")),
        ],
        options=[
            dict(id="colour", name=("Colour", "اللون"), kind="swatch", values=[
                dict(id="red", label=("Red", "أحمر"), swatch="#C4161C"),
                dict(id="black", label=("Black", "أسود"), swatch="#151515"),
            ]),
            XS_L,
        ],
        variants=[dict(opts=[c, s["id"]], price=2199, image=img)
                  for c, img in (("red", 0), ("black", 4)) for s in XS_L["values"]],
        rating=4.1, reviews=63,
        alt=("Faux-leather corset and satin maxi skirt set", "طقم كورسيه جلد صناعي وتنورة ماكسي ساتان"),
        views=[
            ("in red, front", "باللون الأحمر من الأمام"), ("in red, side", "باللون الأحمر من الجانب"),
            ("in red, back", "باللون الأحمر من الخلف"), ("in red, angled view", "باللون الأحمر من زاوية جانبية"),
            ("in black, front", "باللون الأسود من الأمام"), ("in black, side", "باللون الأسود من الجانب"),
            ("in black, back", "باللون الأسود من الخلف"), ("in black, angled view", "باللون الأسود من زاوية جانبية"),
        ],
    ),
    dict(
        id=1513,
        name=("Polka-Dot Strapless Midi Dress", "فستان ميدي منقّط بدون حمالات"),
        subtitle=("White with red dots · Side bow · Full skirt", "أبيض بنقط حمرا · فيونكة على الجنب · جيبة واسعة"),
        description=(
            "A retro-inspired midi from Max with tiny red dots on crisp white, a strapless bodice, a soft bow at the "
            "waist and a full A-line skirt. Throw a red cardigan or a denim shirt over it for daytime.",
            "فستان ميدي من ماكس بلمسة ريترو، بنقاط حمراء صغيرة على أرضية بيضاء ناصعة، وصدر بدون حمالات وفيونكة "
            "ناعمة عند الخصر وتنورة واسعة على شكل حرف A. أضيفي فوقه كارديجان أحمر أو قميص جينز في النهار.",
        ),
        highlights=[
            ("Fifties-style full skirt", "تنورة واسعة بطابع الخمسينيات"),
            ("Bow detail at the waist", "فيونكة عند الخصر"),
            ("Cotton blend, fully lined", "مزيج قطني ببطانة كاملة"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("65% cotton, 35% polyester, lined", "قطن 65%، بوليستر 35%، ببطانة")),
            (("Length", "الطول"), ("Midi", "ميدي")),
            (("Neckline", "فتحة الصدر"), ("Strapless", "بدون حمالات")),
            (("Closure", "الإغلاق"), ("Back zip", "سحّاب خلفي")),
            CARE_30,
        ],
        options=[XS_L],
        variants=each(XS_L, 1199),
        rating=4.4, reviews=22,
        alt=("White strapless midi dress with red polka dots", "فستان ميدي أبيض بدون حمالات بنقاط حمراء"),
        views=GARMENT_VIEWS,
    ),
    dict(
        id=1514,
        name=("Colour-Block Long-Sleeve Maxi Dress", "فستان ماكسي بأكمام طويلة وألوان متباينة"),
        subtitle=("Burgundy, fuchsia, cream & black · Long sleeves", "نبيتي وفوشيا وكريمي وأسود · كم طويل"),
        description=(
            "An art-gallery dress from Koton: a straight-cut burgundy maxi with one fuchsia and one cream sleeve, a "
            "black back panel and a contrast neckline. Long sleeves and an ankle-grazing length make it an easy, "
            "covered choice for work events and evenings out.",
            "فستان بروح فنية من كوتون: ماكسي بقصة مستقيمة باللون النبيتي، بكم فوشيا وآخر كريمي، ولوح خلفي أسود "
            "وياقة بلون متباين. الأكمام الطويلة والطول حتى الكاحل يجعلانه اختيارًا محتشمًا وسهلًا لمناسبات العمل "
            "والسهرات.",
        ),
        highlights=[
            ("Four-colour block design", "تصميم بأربعة ألوان متباينة"),
            ("Long sleeves, ankle length", "أكمام طويلة وطول حتى الكاحل"),
            ("Soft stretch crepe", "كريب مرن وناعم"),
        ],
        specs=[
            (("Fabric", "الخامة"), ("Crepe: 95% polyester, 5% elastane", "كريب: بوليستر 95%، إيلاستين 5%")),
            (("Fit", "القصة"), ("Straight", "مستقيمة")),
            (("Length", "الطول"), ("Maxi", "ماكسي")),
            (("Sleeves", "الأكمام"), ("Long", "طويلة")),
            CARE_30,
        ],
        options=[XS_XL],
        variants=each(XS_XL, 1799),
        rating=4.0, reviews=47,
        alt=("Burgundy colour-block maxi dress with fuchsia and cream sleeves",
             "فستان ماكسي نبيتي بألوان متباينة وأكمام فوشيا وكريمية"),
        views=GARMENT_VIEWS,
    ),

    # ------------------------------------------------------------------ Shoes
    dict(
        id=1601,
        name=("Nike Air Jordan 1 Mid", "حذاء نايكي إير جوردان 1 ميد"),
        subtitle=("White / Gym Red / Black · Leather · Air cushioning", "أبيض وأحمر وأسود · جلد · نعل Air مريح"),
        description=(
            "The mid-top that started it all, in white, Gym Red and black with quilted red overlays. A leather and "
            "synthetic upper, an Air-Sole unit in the heel and a grippy rubber cupsole make it as at home on the "
            "street as on the court.",
            "الحذاء نصف الطويل الذي بدأ الأسطورة، بألوان الأبيض والأحمر والأسود مع أجزاء حمراء مبطنة بالخياطة. "
            "جزء علوي من الجلد الطبيعي والصناعي، ووحدة Air في الكعب، ونعل مطاطي متين يمسك الأرض، فيناسب الشارع "
            "بقدر ما يناسب الملعب.",
        ),
        highlights=[
            ("Jumpman logo on the tongue and heel", "شعار جامبمان على اللسان والكعب"),
            ("Air-Sole unit for all-day cushioning", "وحدة Air لراحة طوال اليوم"),
            ("Padded collar with mid-top support", "ياقة مبطنة ودعم عند الكاحل"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Leather and synthetic", "جلد طبيعي وصناعي")),
            (("Sole", "النعل"), ("Rubber cupsole with Air unit", "نعل مطاطي مع وحدة Air")),
            (("Closure", "الإغلاق"), ("Laces", "أربطة")),
            (("Colour", "اللون"), ("White / Gym Red / Black", "أبيض / أحمر / أسود")),
            (("Fit", "المقاس"), ("True to size", "مطابق للمقاس المعتاد")),
        ],
        options=[MEN_EU],
        variants=each(MEN_EU, 7499),
        rating=4.8, reviews=1290,
        alt=("Nike Air Jordan 1 Mid in white, red and black", "حذاء نايكي إير جوردان 1 ميد بالأبيض والأحمر والأسود"),
        views=SHOE_VIEWS,
    ),
    dict(
        id=1602,
        name=("Nike Alpha Huarache Elite 4 Mid Baseball Cleats", "حذاء بيسبول نايكي ألفا هواراتشي إيليت 4 ميد"),
        subtitle=("Metal cleats · White / Gorge Green", "نعل بمسامير معدنية · أبيض وأخضر غامق"),
        description=(
            "Built for game day: a mid-cut baseball cleat with a supportive Huarache-style heel cage, a cushioned "
            "midsole and a metal cleat plate that bites into dirt and grass. White with deep Gorge Green overlays.",
            "مصمم ليوم المباراة: حذاء بيسبول نصف طويل بكعب داعم على طراز هواراتشي ونعل أوسط مبطن، مع مسامير "
            "معدنية تثبت في التراب والعشب. باللون الأبيض مع أجزاء خضراء غامقة.",
        ),
        highlights=[
            ("Metal cleats for dirt and grass", "مسامير معدنية للتراب والعشب"),
            ("Mid-cut collar for ankle support", "رقبة متوسطة لدعم الكاحل"),
            ("Lightweight cushioned midsole", "نعل أوسط خفيف ومبطن"),
        ],
        specs=[
            (("Sport", "الرياضة"), ("Baseball and softball", "البيسبول والسوفتبول")),
            (("Upper", "الجزء العلوي"), ("Synthetic leather and mesh", "جلد صناعي وشبك")),
            (("Outsole", "النعل الخارجي"), ("Metal cleat plate", "نعل بمسامير معدنية")),
            (("Cut", "الارتفاع"), ("Mid", "متوسط")),
            (("Colour", "اللون"), ("White / Gorge Green", "أبيض / أخضر غامق")),
        ],
        options=[MEN_EU],
        variants=each(MEN_EU, 6499),
        rating=4.5, reviews=34,
        alt=("Nike Alpha Huarache baseball cleats in white and green", "حذاء بيسبول نايكي ألفا هواراتشي بالأبيض والأخضر"),
        views=SHOE_VIEWS,
    ),
    dict(
        id=1603,
        name=("PUMA Future Rider Play On", "حذاء بوما فيوتشر رايدر بلاي أون"),
        subtitle=("Retro runner · Suede & nylon · Soft foam", "ستايل ريترو · شمواه ونايلون · مريح جدًا"),
        description=(
            "PUMA's '80s runner brought back with modern comfort: a nylon and suede upper in white with blue, red "
            "and yellow panels, a lightweight foam midsole and a grippy rubber outsole. Easy to pair with jeans or "
            "joggers.",
            "حذاء الجري القادم من الثمانينيات بلمسة راحة عصرية: جزء علوي من النايلون والشمواه بالأبيض مع لوحات "
            "زرقاء وحمراء وصفراء، ونعل أوسط من الفوم الخفيف ونعل مطاطي يمسك الأرض. سهل التنسيق مع الجينز أو "
            "بنطلون الجوجر.",
        ),
        highlights=[
            ("Lightweight foam midsole", "نعل أوسط من فوم خفيف"),
            ("Bold retro colour blocking", "ألوان ريترو جريئة"),
            ("Padded collar and tongue", "ياقة ولسان مبطنان"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Nylon and suede", "نايلون وشمواه")),
            (("Midsole", "النعل الأوسط"), ("EVA foam", "فوم EVA")),
            (("Outsole", "النعل الخارجي"), ("Rubber", "مطاط")),
            (("Closure", "الإغلاق"), ("Laces", "أربطة")),
            (("Colour", "اللون"), ("White / blue / red / yellow", "أبيض / أزرق / أحمر / أصفر")),
        ],
        options=[MEN_EU],
        variants=each(MEN_EU, 3999, 5299),
        rating=4.4, reviews=518,
        alt=("PUMA Future Rider sneakers in white, blue, red and yellow",
             "حذاء بوما فيوتشر رايدر بالأبيض والأزرق والأحمر والأصفر"),
        views=SHOE_VIEWS,
    ),
    dict(
        id=1604,
        name=("Off-White Out Of Office Sneakers — Men's", "حذاء أوف وايت أوت أوف أوفيس — رجالي"),
        subtitle=("Signature Arrow · Calf leather · White / Red", "علامة السهم الشهيرة · جلد طبيعي · أبيض وأحمر"),
        description=(
            "Off-White's everyday low-top, inspired by '80s basketball shoes: white calf leather with the oversized "
            "red Arrow on each side, a red heel tab with the label's hand and arrow graphics, and a chunky rubber "
            "sole. Delivered in the original box with a dust bag.",
            "حذاء أوف وايت اليومي المنخفض المستوحى من أحذية كرة السلة في الثمانينيات: جلد عجل أبيض مع سهم أحمر "
            "كبير على كل جانب، ولسان خلفي أحمر عليه رسومات اليد والسهم، ونعل مطاطي سميك. يصلك في علبته الأصلية "
            "مع كيس الحفظ.",
        ),
        highlights=[
            ("100% authentic, from authorised retailers", "أصلي 100% من موزعين معتمدين"),
            ("Calf leather upper", "جزء علوي من جلد العجل"),
            ("Original box and dust bag", "العلبة الأصلية وكيس الحفظ"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Calf leather", "جلد عجل")),
            (("Sole", "النعل"), ("Rubber", "مطاط")),
            (("Heel tab", "اللسان الخلفي"), ("Red leather with Off-White graphics", "جلد أحمر برسومات أوف وايت")),
            (("Closure", "الإغلاق"), ("Laces", "أربطة")),
            (("Colour", "اللون"), ("White / Red", "أبيض / أحمر")),
        ],
        options=[MEN_EU],
        variants=each(MEN_EU, 32999),
        rating=4.7, reviews=26,
        alt=("Off-White Out Of Office sneakers in white with red arrow", "حذاء أوف وايت أوت أوف أوفيس أبيض بسهم أحمر"),
        views=[("side with the red Arrow", "من الجانب مع السهم الأحمر"), ("pair at an angle", "الزوج من زاوية جانبية"),
               ("pair from the front", "الزوج من الأمام"), ("pair from the back", "الزوج من الخلف")],
    ),
    dict(
        id=1605,
        name=("Off-White Out Of Office Sneakers — Women's", "حذاء أوف وايت أوت أوف أوفيس — نسائي"),
        subtitle=("Women's sizing · Signature Arrow · White / Red", "مقاسات حريمي · علامة السهم · أبيض وأحمر"),
        description=(
            "The white-and-red Out Of Office low-top in women's sizing, EU 36 to 41. White calf leather, the "
            "oversized red Arrow on each side and a red heel tab with the label's hand and arrow graphics, on a "
            "chunky rubber sole.",
            "نفس حذاء أوت أوف أوفيس المنخفض بالأبيض والأحمر بمقاسات نسائية من 36 إلى 41. جلد عجل أبيض، وسهم أحمر "
            "كبير على كل جانب، ولسان خلفي أحمر برسومات اليد والسهم، فوق نعل مطاطي سميك.",
        ),
        highlights=[
            ("Women's sizing, EU 36–41", "مقاسات نسائية من 36 إلى 41"),
            ("100% authentic, original box and dust bag", "أصلي 100% بالعلبة الأصلية وكيس الحفظ"),
            ("Sizes sell out fast: follow the product for restocks", "المقاسات تنفد بسرعة، تابعي المنتج لمعرفة موعد توفره"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Calf leather", "جلد عجل")),
            (("Sole", "النعل"), ("Rubber", "مطاط")),
            (("Heel tab", "اللسان الخلفي"), ("Red leather with Off-White graphics", "جلد أحمر برسومات أوف وايت")),
            (("Closure", "الإغلاق"), ("Laces", "أربطة")),
            (("Colour", "اللون"), ("White / Red", "أبيض / أحمر")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 31999),
        rating=4.9, reviews=9,
        alt=("Off-White Out Of Office women's sneakers in white and red", "حذاء أوف وايت أوت أوف أوفيس النسائي بالأبيض والأحمر"),
        views=SHOE_VIEWS,
    ),
    dict(
        id=1606,
        name=("Charles & Keith Pointed-Toe Stiletto Pumps", "حذاء تشارلز آند كيث بكعب رفيع ومقدمة مدببة"),
        subtitle=("Black · 9.5 cm heel · Cognac lining", "أسود · كعب 9.5 سم · بطانة كونياك"),
        description=(
            "A wardrobe essential: sleek black pumps with a sharp pointed toe, a slim 9.5 cm stiletto and a contrast "
            "cognac lining. A cushioned insole takes them from the office to the evening.",
            "قطعة أساسية في كل خزانة: حذاء أسود أنيق بمقدمة مدببة وكعب رفيع بارتفاع 9.5 سم وبطانة بلون الكونياك. "
            "النعل الداخلي المبطن يجعله مناسبًا من المكتب حتى السهرة.",
        ),
        highlights=[
            ("Cushioned insole for longer wear", "نعل داخلي مبطن لارتداء أطول"),
            ("Timeless pointed-toe silhouette", "تصميم كلاسيكي بمقدمة مدببة"),
            ("Non-slip rubber heel tip", "طرف كعب مطاطي مانع للانزلاق"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Faux leather", "جلد صناعي")),
            (("Heel", "الكعب"), ("9.5 cm stiletto", "رفيع بارتفاع 9.5 سم")),
            (("Lining", "البطانة"), ("Cognac faux leather", "جلد صناعي بلون الكونياك")),
            (("Toe", "المقدمة"), ("Pointed", "مدببة")),
            (("Colour", "اللون"), ("Black", "أسود")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 3299),
        rating=4.5, reviews=233,
        alt=("Charles & Keith black pointed-toe stiletto pumps", "حذاء تشارلز آند كيث أسود بمقدمة مدببة وكعب رفيع"),
        views=[("pair from the front, showing the cognac lining", "الزوج من الأمام مع البطانة بلون الكونياك"),
               ("side", "من الجانب"), ("pair from the back", "الزوج من الخلف"),
               ("pair at an angle", "الزوج من زاوية جانبية")],
    ),
    dict(
        id=1607,
        name=("Calvin Klein Logo-Strap Heeled Mules", "صندل كالفن كلاين بكعب وشريط بالشعار"),
        subtitle=("Glossy black · 9 cm heel · Open back", "أسود لامع · كعب 9 سم · مفتوح من ورا"),
        description=(
            "Minimal, glossy and easy to slip on: black heeled mules with a wide Calvin Klein logo band across the "
            "instep, a slim toe strap and a 9 cm stiletto. The open back and padded footbed keep them comfortable "
            "from dinner to dancing.",
            "تصميم بسيط ولامع وسهل اللبس: صندل أسود بكعب، بشريط عريض فوق مشط القدم عليه شعار كالفن كلاين، "
            "وشريط رفيع عند الأصابع، وكعب رفيع بارتفاع 9 سم. الظهر المفتوح والنعل المبطن يحافظان على راحتك من "
            "العشاء حتى الرقص.",
        ),
        highlights=[
            ("Wide band with Calvin Klein lettering", "شريط عريض بحروف كالفن كلاين"),
            ("Slip-on, open-back design", "تصميم مفتوح من الخلف سهل اللبس"),
            ("Padded footbed", "نعل داخلي مبطن"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Glossy synthetic", "جلد صناعي لامع")),
            (("Heel", "الكعب"), ("9 cm stiletto", "رفيع بارتفاع 9 سم")),
            (("Style", "التصميم"), ("Open-back mule", "صندل مفتوح من الخلف")),
            (("Sole", "النعل"), ("Synthetic with rubber heel tip", "صناعي بطرف كعب مطاطي")),
            (("Colour", "اللون"), ("Black", "أسود")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 5999),
        rating=4.3, reviews=71,
        alt=("Calvin Klein black heeled mules with logo strap", "صندل كالفن كلاين أسود بكعب وشريط بالشعار"),
        views=SHOE_VIEWS,
    ),
    dict(
        id=1608,
        name=("Metallic Pointed-Toe Pumps", "حذاء نسائي معدني اللون بمقدمة مدببة"),
        subtitle=("Rose gold · 10 cm heel · Red sole", "روز جولد · كعب 10 سم · نعل أحمر"),
        description=(
            "Party-ready pumps in a mirror-shine rose-gold finish with a pointed toe, a 10 cm stiletto and a "
            "red-lacquered sole that flashes with every step. Perfect with a little black dress or a pastel soirée "
            "gown.",
            "حذاء جاهز للسهرات بلمعة معدنية بلون الذهب الوردي، بمقدمة مدببة وكعب رفيع بارتفاع 10 سم ونعل أحمر "
            "لامع يظهر مع كل خطوة. يتألق مع الفستان الأسود القصير أو فستان السواريه بالألوان الهادئة.",
        ),
        highlights=[
            ("Mirror-shine metallic finish", "لمعة معدنية كالمرآة"),
            ("Red-lacquered sole", "نعل أحمر لامع"),
            ("Cushioned footbed", "نعل داخلي مبطن"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Metallic synthetic", "جلد صناعي معدني")),
            (("Heel", "الكعب"), ("10 cm stiletto", "رفيع بارتفاع 10 سم")),
            (("Toe", "المقدمة"), ("Pointed", "مدببة")),
            (("Sole", "النعل"), ("Red lacquered", "أحمر لامع")),
            (("Colour", "اللون"), ("Rose gold", "ذهبي وردي")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 1999),
        rating=4.1, reviews=146,
        alt=("Rose-gold metallic pointed-toe pumps", "حذاء بكعب رفيع بلون الذهب الوردي ومقدمة مدببة"),
        views=[("pair from the front", "الزوج من الأمام"), ("side", "من الجانب"),
               ("pair from the back, showing the red soles", "الزوج من الخلف مع النعل الأحمر"),
               ("pair at an angle", "الزوج من زاوية جانبية")],
    ),
    dict(
        id=1609,
        name=("Patent Stiletto Pumps", "حذاء نسائي لامع بكعب رفيع"),
        subtitle=("Black patent · 11 cm heel · Nude lining", "أسود لامع · كعب 11 سم · بطانة بيج"),
        description=(
            "A sky-high classic: black patent pumps with a deep pointed toe, an 11 cm needle heel, a nude lining and "
            "a red sole. The low-cut vamp lengthens the leg, making them a go-to for weddings and gala nights.",
            "حذاء كلاسيكي عالٍ: أسود لامع بمقدمة مدببة وكعب رفيع جدًا بارتفاع 11 سم، وبطانة بلون البيج ونعل أحمر. "
            "فتحته الأمامية المنخفضة تُطيل مظهر الساق، فهو اختيار مضمون للأفراح وحفلات السهرة.",
        ),
        highlights=[
            ("High-shine patent finish", "لمعة باتنت عالية"),
            ("Low-cut vamp that elongates the leg", "فتحة أمامية منخفضة تطيل مظهر الساق"),
            ("Nude lining and red sole", "بطانة بيج ونعل أحمر"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Patent faux leather", "جلد صناعي لامع (باتنت)")),
            (("Heel", "الكعب"), ("11 cm stiletto", "رفيع بارتفاع 11 سم")),
            (("Lining", "البطانة"), ("Nude", "بيج")),
            (("Sole", "النعل"), ("Red", "أحمر")),
            (("Colour", "اللون"), ("Black", "أسود")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 2299),
        rating=4.4, reviews=302,
        alt=("Black patent stiletto pumps with red soles", "حذاء أسود لامع بكعب رفيع ونعل أحمر"),
        views=[("pair from the front, showing the nude lining", "الزوج من الأمام مع البطانة البيج"),
               ("side", "من الجانب"), ("pair from the back, showing the red soles", "الزوج من الخلف مع النعل الأحمر"),
               ("pair at an angle", "الزوج من زاوية جانبية")],
    ),
    dict(
        id=1610,
        name=("Bata Patent Mid-Heel Pumps", "حذاء باتا لامع بكعب متوسط"),
        subtitle=("Coral red · 7 cm heel · All-day comfort", "أحمر مرجاني · كعب 7 سم · مريح طول اليوم"),
        description=(
            "A pop of colour you can actually walk in: coral-red patent pumps with a soft pointed toe, a manageable "
            "7 cm heel and Bata's cushioned insole. Great for work days, graduations and family occasions.",
            "لمسة لون يمكنك المشي بها فعلًا: حذاء لامع باللون الأحمر المرجاني بمقدمة مدببة ناعمة وكعب مريح "
            "بارتفاع 7 سم ونعل داخلي مبطن من باتا. مناسب لأيام العمل وحفلات التخرج والمناسبات العائلية.",
        ),
        highlights=[
            ("Comfortable 7 cm heel", "كعب مريح بارتفاع 7 سم"),
            ("Cushioned insole", "نعل داخلي مبطن"),
            ("Easy-clean patent finish", "سطح لامع سهل التنظيف"),
        ],
        specs=[
            (("Upper", "الجزء العلوي"), ("Patent synthetic", "جلد صناعي لامع")),
            (("Heel", "الكعب"), ("7 cm", "7 سم")),
            (("Toe", "المقدمة"), ("Soft pointed", "مدببة ناعمة")),
            (("Insole", "النعل الداخلي"), ("Cushioned", "مبطن")),
            (("Colour", "اللون"), ("Coral red", "أحمر مرجاني")),
        ],
        options=[WOMEN_EU],
        variants=each(WOMEN_EU, 1499, 1999),
        rating=4.2, reviews=467,
        alt=("Bata coral-red patent mid-heel pumps", "حذاء باتا لامع بلون أحمر مرجاني وكعب متوسط"),
        views=SHOE_VIEWS,
    ),
]

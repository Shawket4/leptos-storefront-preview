"""Breakfast & pantry (category 140, ids 2651–2699)."""

from common import DRY, FREEZER, L, P, S, V, axis, weights

CATEGORY = 140

PRODUCTS = [
    P(id=2651, slug="frozen-taameya", brand="souq",
      name=("Ready-to-fry ta'ameya", "طعمية فريزر جاهزة"),
      sub=("Green, herby and made with fava beans", "خضرا ومليانة خضرة... من الفول المدشوش"),
      desc=("Egyptian-style ta'ameya made from crushed fava beans with plenty of coriander, dill and leek, "
            "shaped and frozen. Fry straight from the freezer for four minutes until deep golden and crisp.",
            "طعمية على الطريقة المصرية من الفول المدشوش بكمية كبيرة من الكزبرة والشبت والكرات، "
            "متشكلة ومتجمدة. اقليها على طول من الفريزر 4 دقايق لحد ما تدهّب وتقرمش."),
      hl=[("Fry from frozen", "تتقلي من الفريزر على طول"), ("About 25 pieces per 500 g", "حوالي 25 قرص في النص كيلو")],
      specs=[S("ingredients", "Fava beans, leek, coriander, dill, garlic, spices",
                             "فول مدشوش، كرات، كزبرة، شبت، توم، توابل"), S("storage", FREEZER)],
      img=["m:Falafel"],
      r=(4.6, 1210), tags="bestseller express",
      options=[weights("500g", "1kg")],
      v=[V("500g", 44.99), V("1kg", 84.99)]),

    P(id=2652, slug="heinz-tomato-ketchup", brand="heinz",
      name=("Heinz tomato ketchup", "كاتشب هاينز"),
      sub=("Thick, rich and squeezable", "تقيل وغني وسهل العصر"),
      desc=("Heinz's classic thick tomato ketchup, made with ripe tomatoes. The go-to for fries, burgers and "
            "kofta sandwiches.",
            "كاتشب هاينز الكلاسيكي التقيل، معمول من طماطم مستوية. الأساسي مع البطاطس والبرجر "
            "وساندوتشات الكفتة."),
      hl=[("Squeezy bottle", "إزازة سهلة العصر"), ("Refrigerate after opening", "في التلاجة بعد الفتح")],
      specs=[S("storage", "Cupboard; fridge after opening", "في الدولاب، وبعد الفتح في التلاجة")],
      img=["m:Tomato Ketchup"],
      r=(4.6, 640), tags="express",
      v=[V(None, 59.99, sku="460G")]),

    P(id=2653, slug="maggi-chicken-stock-cubes", brand="maggi",
      name=("Maggi chicken stock cubes", "مرقة فراخ ماجي"),
      sub=("Quick broth for rice and soups", "مرقة سريعة للرز والشوربة"),
      desc=("Maggi chicken stock cubes — crumble one into rice, soups or vegetable stews for an instant "
            "savoury boost.",
            "مكعبات مرقة فراخ ماجي... فتّت مكعب في الرز أو الشوربة أو الخضار لطعم أغنى في ثانية."),
      hl=[("1 cube = 500 ml stock", "المكعب = نص لتر مرقة"), ("Big box saves more", "العلبة الكبيرة أوفر")],
      specs=[S("storage", DRY)],
      img=["m:Chicken Stock Cube"],
      r=(4.5, 870), tags="express",
      options=[axis("pack", [("8", L("8 cubes", "8 مكعبات")), ("24", L("24 cubes", "24 مكعب"))])],
      v=[V("8", 27.99), V("24", 79.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import AIRTIGHT, FRIDGE  # noqa: E402

AFTER_OPEN = S("storage", "Cupboard; fridge after opening", "في الدولاب، وبعد الفتح في التلاجة")

PRODUCTS += [
    P(id=2654, slug="heinz-tomato-paste", brand="heinz",
      name=("Heinz tomato paste", "صلصة طماطم هاينز"),
      sub=("Thick paste for every Egyptian stew", "صلصة تقيلة لكل طبيخ مصري"),
      desc=("Rich, concentrated tomato paste from Heinz — the base of bamia, fasolia, bisella and every red "
            "Egyptian stew. Fry a spoonful in oil before adding water for the deepest flavour.",
            "صلصة طماطم مركّزة وغنية من هاينز... أساس البامية والفاصوليا والبسلة وكل طبيخ أحمر. "
            "شوّح معلقة في الزيت قبل ما تضيف المية علشان الطعم يطلع."),
      hl=[("Concentrated", "مركّزة"), ("Two can sizes", "مقاسين للعلبة")],
      specs=[AFTER_OPEN],
      img=["m:Tomato Puree"],
      r=(4.6, 1050), tags="bestseller express",
      options=[weights("400g", "800g")],
      v=[V("400g", 34.99), V("800g", 64.99)]),

    P(id=2655, slug="hellmanns-mayonnaise", brand="hellmanns",
      name=("Hellmann's Real mayonnaise", "مايونيز هيلمانز"),
      sub=("Creamy, made with real eggs", "كريمي ومعمول ببيض حقيقي"),
      desc=("Hellmann's Real mayonnaise, rich and creamy. Spread it in sandwiches, mix it into potato salad, "
            "or stir it with garlic for a quick toumeya-style dip.",
            "مايونيز هيلمانز ريال، غني وكريمي. ادهنه في الساندوتشات، أو اخلطه في سلطة البطاطس، أو "
            "قلّبه مع التوم لصوص سريع زي التومية."),
      hl=[("Made with eggs", "معمول بالبيض"), ("Fridge after opening", "في التلاجة بعد الفتح")],
      specs=[S("weight", "400 g", "400 جم"), AFTER_OPEN],
      img=["m:Mayonnaise"],
      r=(4.6, 410), tags="",
      v=[V(None, 119.99, sku="400G")]),

    P(id=2656, slug="knorr-beef-stock-cubes", brand="knorr",
      name=("Knorr beef stock cubes", "مرقة لحمة كنور"),
      sub=("Rich beef flavour in a cube", "طعم لحمة غني في مكعب"),
      desc=("Knorr beef stock cubes for a quick, rich broth. Crumble one into fatta rice, stews or a pot of "
            "orzo soup.",
            "مكعبات مرقة لحمة كنور لشوربة سريعة وغنية. فتّت مكعب في رز الفتة أو الطبيخ أو حلة لسان "
            "العصفور."),
      hl=[("8 cubes", "8 مكعبات"), ("1 cube = 500 ml stock", "المكعب = نص لتر مرقة")],
      specs=[S("storage", DRY)],
      img=["m:Beef Stock Cubes"],
      r=(4.5, 520), tags="",
      v=[V(None, 29.99, sku="8CUBES")]),

    P(id=2657, slug="knorr-chicken-bouillon-powder", brand="knorr",
      name=("Knorr chicken bouillon powder", "مرقة فراخ بودرة كنور"),
      sub=("Big tub for big families and kitchens", "علبة كبيرة للعيلة الكبيرة"),
      desc=("Knorr's chicken bouillon powder in a 1 kg tub — measure exactly as much as you need for rice, "
            "soups, sauces and marinades.",
            "مرقة فراخ بودرة من كنور في علبة كيلو... خد بالظبط اللي محتاجه للرز والشوربة والصوصات "
            "والتتبيلات."),
      hl=[("1 kg tub", "علبة كيلو"), ("Easy to measure", "سهلة في القياس")],
      specs=[S("weight", "1 kg", "1 كجم"), S("storage", DRY)],
      img=["m:Chicken Bouillon Powder"],
      r=(4.6, 188), tags="",
      v=[V(None, 299.99, sku="1KG")]),

    P(id=2658, slug="maggi-vegetable-stock-cubes", brand="maggi",
      name=("Maggi vegetable stock cubes", "مرقة خضار ماجي"),
      sub=("For vegetable soups and rice", "لشوربة الخضار والرز"),
      desc=("Maggi vegetable stock cubes for meat-free soups, rice and vegetable stews — handy during the "
            "Coptic fasting seasons too.",
            "مكعبات مرقة خضار ماجي لشوربة من غير لحمة والرز وطبيخ الخضار... مفيدة كمان في الصيام."),
      hl=[("8 cubes", "8 مكعبات"), ("Meat-free", "من غير لحمة")],
      specs=[S("storage", DRY)],
      img=["m:Vegetable Stock Cube"],
      r=(4.3, 176), tags="",
      v=[V(None, 25.99, sku="8CUBES")]),

    P(id=2659, slug="dr-oetker-baking-powder", brand="dr-oetker",
      name=("Dr. Oetker baking powder", "بيكنج باودر دكتور أوتكر"),
      sub=("Reliable rise for cakes and basbousa", "لكيك وبسبوسة بتعلى مظبوط"),
      desc=("Dr. Oetker's double-acting baking powder for light, even cakes, basbousa and ghorayeba. Mix it "
            "with the flour before adding wet ingredients.",
            "بيكنج باودر دكتور أوتكر مزدوج المفعول للكيك والبسبوسة والغريبة الخفيفة. اخلطه مع الدقيق "
            "قبل ما تضيف السوايل."),
      hl=[("Resealable tub", "علبة بتتقفل"), ("Double-acting", "مزدوج المفعول")],
      specs=[S("storage", DRY)],
      img=["m:Baking Powder"],
      r=(4.7, 233), tags="",
      v=[V(None, 64.99, sku="170G")]),

    P(id=2660, slug="dr-oetker-bicarbonate-of-soda", brand="dr-oetker",
      name=("Dr. Oetker bicarbonate of soda", "بيكربونات صوديوم دكتور أوتكر"),
      sub=("For baking — and the fridge", "للخبيز... وللتلاجة"),
      desc=("Pure bicarbonate of soda for kahk, biscuits and gingerbread. A spoonful in an open jar also "
            "keeps the fridge smelling fresh.",
            "بيكربونات صوديوم نقية للكحك والبسكويت. ومعلقة في كوباية مفتوحة بتخلّي ريحة التلاجة حلوة."),
      hl=[("Pure bicarbonate", "بيكربونات نقية"), ("Many household uses", "استعمالات كتير في البيت")],
      specs=[S("storage", DRY)],
      img=["m:Bicarbonate Of Soda"],
      r=(4.6, 120), tags="",
      v=[V(None, 49.99, sku="200G")]),

    P(id=2661, slug="vanilla-pods",
      name=("Vanilla pods", "أعواد فانيليا"),
      sub=("Real Madagascar vanilla", "فانيليا مدغشقر الأصلية"),
      desc=("Plump, oily Madagascar vanilla pods. Split one and scrape the seeds into roz bel laban, "
            "custard or a cake batter, then drop the pod into a jar of sugar for vanilla sugar.",
            "أعواد فانيليا مدغشقر مليانة وطرية. افتح عود واكشط البذور في الرز باللبن أو الكاسترد أو "
            "خلطة الكيك، وبعدين حط العود في برطمان سكر يبقى سكر بالفانيليا."),
      hl=[("2 pods", "عودين"), ("Grade A Madagascar", "مدغشقر درجة أولى")],
      specs=[S("origin", "Madagascar", "مدغشقر"), S("storage", AIRTIGHT)],
      img=["m:Vanilla"],
      r=(4.8, 66), tags="new",
      v=[V(None, 89.99, sku="2PODS")]),

    P(id=2662, slug="kelloggs-rice-krispies", brand="kelloggs",
      name=("Kellogg's Rice Krispies", "كيلوجز رايس كريسبيز"),
      sub=("Snap, crackle and pop", "طقطق وفرقع"),
      desc=("Crispy toasted rice cereal that snaps, crackles and pops in cold milk. Also the base for "
            "marshmallow treats.",
            "سيريال رز محمص مقرمش بيطقطق في اللبن الساقع. وكمان أساس حلويات المارشميلو."),
      hl=[("Fortified with vitamins", "مدعّم بالفيتامينات"), ("Kids' favourite", "مفضل الولاد")],
      specs=[S("weight", "375 g", "375 جم"), S("storage", DRY)],
      img=["m:Rice Krispies"],
      r=(4.5, 176), tags="deal",
      v=[V(None, 179.99, was=199.99, sku="375G")]),

    P(id=2663, slug="heinz-sweet-chilli-sauce", brand="heinz",
      name=("Heinz sweet chilli sauce", "صوص سويت تشيلي هاينز"),
      sub=("Sweet, sticky, gently hot", "مسكّر وتقيل وحراق خفيف"),
      desc=("Thai-style sweet chilli sauce from Heinz. Dip spring rolls and shrimp in it, glaze chicken "
            "wings, or drizzle it over a stir-fry.",
            "صوص سويت تشيلي على الطريقة التايلاندي من هاينز. غمّس فيه السبرينج رولز والجمبري، أو "
            "ادهن بيه أجنحة الفراخ، أو رشّه على التشويحة."),
      hl=[("Squeezy bottle", "إزازة سهلة العصر"), ("Mild heat", "حراق خفيف")],
      specs=[AFTER_OPEN],
      img=["m:Sweet chilli sauce"],
      r=(4.4, 132), tags="",
      v=[V(None, 79.99, sku="260G")]),

    P(id=2664, slug="frozen-french-fries", brand="souq",
      name=("Frozen French fries", "بطاطس نص سوا مجمدة"),
      sub=("Straight-cut, fry or air-fry from frozen", "صوابع مستقيمة... تتقلي أو في الإير فراير"),
      desc=("Straight-cut potato fries, par-fried and frozen. Fry them for three minutes or air-fry for "
            "fifteen for golden, crispy chips with no peeling.",
            "صوابع بطاطس نص سوا ومجمدة. اقليها 3 دقايق أو في الإير فراير ربع ساعة وتطلع دهبي ومقرمشة "
            "من غير تقشير."),
      hl=[("Air-fryer friendly", "تنفع في الإير فراير"), ("No peeling", "من غير تقشير")],
      specs=[S("storage", FREEZER)],
      img=["m:Fries"],
      r=(4.5, 690), tags="express",
      options=[weights("1kg", "2kg")],
      v=[V("1kg", 89.99), V("2kg", 169.99)]),

    P(id=2665, slug="hero-apricot-jam", brand="hero",
      name=("Hero apricot jam", "مربى مشمش هيرو"),
      sub=("Fruity apricot jam for breakfast", "مربى مشمش بطعم الفاكهة للفطار"),
      desc=("Hero's apricot jam, packed with fruit pieces. Spread it on toast with ishta, fill ghorayeba "
            "and pastries, or glaze a fruit tart.",
            "مربى مشمش هيرو مليانة قطع فاكهة. ادهنها على التوست بالقشطة، أو احشي بيها الغريبة "
            "والمعجنات، أو لمّع بيها تارت الفاكهة."),
      hl=[("Real fruit pieces", "قطع فاكهة حقيقية"), ("Glass jar", "برطمان إزاز")],
      specs=[S("weight", "340 g", "340 جم"), AFTER_OPEN],
      img=["m:Apricot Jam"],
      r=(4.5, 288), tags="",
      v=[V(None, 109.99, sku="340G")]),

    P(id=2666, slug="dr-oetker-cocoa-powder", brand="dr-oetker",
      name=("Dr. Oetker fine dark cocoa", "كاكاو خام دكتور أوتكر"),
      sub=("For cakes, brownies and frosting", "للكيك والبراونيز والتغطية"),
      desc=("Fine, dark cocoa powder made for baking. Sift it into cakes, brownies and chocolate frosting for "
            "a deep colour and rich flavour.",
            "كاكاو خام ناعم وغامق مخصوص للخبيز. انخله في الكيك والبراونيز وتغطية الشوكولاتة علشان لون "
            "غامق وطعم غني."),
      hl=[("For baking", "للخبيز"), ("Deep colour", "لون غامق")],
      specs=[S("weight", "190 g", "190 جم"), S("storage", DRY)],
      img=["m:Cocoa Powder"],
      r=(4.6, 144), tags="",
      v=[V(None, 129.99, sku="190G")]),

    P(id=2667, slug="maille-dijon-mustard", brand="maille",
      name=("Maille Dijon mustard", "مستردة ديجون مايّ"),
      sub=("The original Dijon from France", "الديجون الأصلي من فرنسا"),
      desc=("Maille's smooth, sharp Dijon mustard. Whisk it into vinaigrettes, brush it on chicken before "
            "roasting, or spread it in a steak sandwich.",
            "مستردة ديجون مايّ ناعمة وحادة. اخفقها في تتبيلة السلطة، أو ادهن بيها الفراخ قبل الفرن، أو "
            "حطها في ساندوتش الستيك."),
      hl=[("Made in France", "صناعة فرنسية"), ("Sharp and smooth", "حادة وناعمة")],
      specs=[S("weight", "215 g", "215 جم"), AFTER_OPEN],
      img=["m:Dijon Mustard"],
      r=(4.7, 88), tags="",
      v=[V(None, 149.99, sku="215G")]),

    P(id=2668, slug="tabasco-pepper-sauce", brand="tabasco",
      name=("Tabasco pepper sauce", "صوص تاباسكو"),
      sub=("A few drops of fire", "كام نقطة نار"),
      desc=("The original Tabasco red pepper sauce, aged in oak barrels. A few drops wake up eggs, pizza, "
            "soups and Bloody Mary-style tomato juice.",
            "صوص تاباسكو الأصلي من الفلفل الأحمر، متعتق في براميل بلوط. كام نقطة بتصحّي البيض والبيتزا "
            "والشوربة وعصير الطماطم."),
      hl=[("60 ml bottle", "إزازة 60 مل"), ("Very hot", "حراق جدًا")],
      specs=[S("origin", "USA", "أمريكا"), S("storage", DRY)],
      img=["m:Tabasco Sauce"],
      r=(4.6, 77), tags="",
      v=[V(None, 139.99, sku="60ML")]),

    P(id=2669, slug="lea-perrins-worcestershire", brand="lea-perrins",
      name=("Lea & Perrins Worcestershire sauce", "صوص ورشستر لي آند بيرينز"),
      sub=("The secret ingredient in marinades", "السر في التتبيلات"),
      desc=("The original Worcestershire sauce. A splash deepens steak and burger marinades, gravies, stews "
            "and Caesar dressing.",
            "صوص ورشستر الأصلي. شوية منه بيعمّقوا طعم تتبيلة الستيك والبرجر والصوصات والطبيخ وتتبيلة "
            "السيزر."),
      hl=[("150 ml bottle", "إزازة 150 مل"), ("Since 1837", "من سنة 1837")],
      specs=[S("origin", "United Kingdom", "المملكة المتحدة"), S("storage", DRY)],
      img=["m:Worcestershire Sauce"],
      r=(4.6, 66), tags="",
      v=[V(None, 159.99, sku="150ML")]),
]

"""Dates & sweets (category 100, ids 2451–2499)."""

from common import AIRTIGHT, DRY, L, P, S, V, mg, mo, weights

CATEGORY = 100

PRODUCTS = [
    P(id=2451, slug="medjool-dates",
      name=("Medjool dates", "تمر مجدول"),
      sub=("Big, soft and caramel-sweet", "حبة كبيرة وطرية وطعمها كراميل"),
      desc=("Large, soft Medjool dates with a rich caramel sweetness — the king of dates for iftar, coffee "
            "breaks and gifting. Order the gift box for visits and Eid.",
            "تمر مجدول حبته كبيرة وطرية وطعمه زي الكراميل... ملك التمور على الفطار ومع القهوة وفي "
            "الهدايا. اطلب علبة الهدية للزيارات والعيد."),
      hl=[("Jumbo size", "مقاس جامبو"), ("Gift box available", "متاح في علبة هدية")],
      specs=[S("origin", "Jordan Valley", "غور الأردن"), S("storage", "Cool place or fridge", "مكان بارد أو التلاجة")],
      img=["m:Medjool Dates"],
      r=(4.8, 512), tags="featured",
      options=[weights("500g", "1kg")],
      v=[V("500g", 249.99), V("1kg", 479.99)],
      mods=[mg("gift", "Gift packing", "تغليف هدية", [
          mo("box", "Gift box with ribbon", "علبة هدية بشريطة", 45)])]),

    P(id=2452, slug="malban",
      name=("Malban, rose and plain", "ملبن مشكل"),
      sub=("Soft lokum cubes dusted in sugar", "مكعبات ملبن طرية متغطية سكر"),
      desc=("Chewy malban cubes in rose and plain flavours, dusted with powdered sugar — a Mawlid and "
            "Ramadan classic, and lovely alongside Turkish coffee.",
            "ملبن مطاطي بطعم الورد والسادة، متغطي بسكر بودرة... كلاسيكي في المولد ورمضان، وحلو جدًا "
            "مع فنجان القهوة."),
      hl=[("Rose and plain mix", "مشكل ورد وسادة"), ("Made fresh weekly", "بيتعمل طازة كل أسبوع")],
      specs=[S("ingredients", "Sugar, starch, rose water, natural colour", "سكر، نشا، ماء ورد، لون طبيعي"),
             S("storage", AIRTIGHT)],
      img=["m:Turkish Delight"],
      r=(4.5, 288), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 39.99), V("500g", 74.99)]),

    P(id=2453, slug="peanut-fuliya",
      name=("Peanut fuliya", "فولية"),
      sub=("Crunchy peanut brittle, Mawlid style", "فولية مقرمشة زي حلاوة المولد"),
      desc=("Crunchy slabs of roasted peanuts set in golden caramel — the much-loved Mawlid sweet, made "
            "fresh in small batches.",
            "ألواح فول سوداني محمص في كراميل دهبي مقرمش... حلاوة المولد اللي كلنا بنحبها، معمولة "
            "طازة بكميات صغيرة."),
      hl=[("Made in small batches", "بتتعمل بكميات صغيرة"), ("Extra crunchy", "مقرمشة جدًا")],
      specs=[S("ingredients", "Peanuts, sugar, glucose", "فول سوداني، سكر، جلوكوز"), S("storage", AIRTIGHT)],
      img=["m:Peanut Brittle"],
      r=(4.6, 340), tags="new",
      v=[V(None, 44.99, sku="250G")]),

    P(id=2454, slug="nutella", brand="nutella",
      name=("Nutella hazelnut spread", "نوتيلا شوكولاتة بالبندق"),
      sub=("The breakfast favourite", "المفضلة على الفطار"),
      desc=("The original hazelnut cocoa spread. Spread it on warm toast or feteer, fold it into crêpes, or "
            "sneak a spoonful — we won't tell.",
            "كريمة البندق بالكاكاو الأصلية. ادهنها على التوست السخن أو الفطير، أو حطها في الكريب، أو "
            "خد معلقة على السريع... مش هنقول لحد."),
      hl=[("The original", "الأصلية"), ("Family jar available", "متاح البرطمان الكبير")],
      specs=[S("storage", "Room temperature, not the fridge", "برة التلاجة")],
      img=["m:Nutella"],
      r=(4.8, 1440), tags="bestseller",
      options=[weights("350g", "700g")],
      v=[V("350g", 199.99), V("700g", 379.99)]),

    P(id=2455, slug="dark-cooking-chocolate",
      name=("Dark cooking chocolate", "شوكولاتة خام داكنة"),
      sub=("For ganache, brownies and molten cake", "للجاناش والبراونيز والمولتن كيك"),
      desc=("Smooth dark chocolate with 54% cocoa, broken into easy-melt pieces. Ideal for ganache, brownies, "
            "molten cake and chocolate-dipped strawberries.",
            "شوكولاتة داكنة ناعمة 54% كاكاو، متكسرة قطع سهلة تسيح. مثالية للجاناش والبراونيز والمولتن "
            "كيك والفراولة بالشوكولاتة."),
      hl=[("54% cocoa", "54% كاكاو"), ("Melts smoothly", "بتسيح ناعم")],
      specs=[S("storage", DRY)],
      img=["m:Chocolate"],
      r=(4.6, 210), tags="",
      options=[weights("200g", "500g")],
      v=[V("200g", 99.99), V("500g", 239.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import FREEZER, axis  # noqa: E402

PRODUCTS += [
    P(id=2456, slug="magnum-almond-tub", brand="magnum",
      name=("Magnum Almond ice cream tub", "آيس كريم ماجنوم لوز"),
      sub=("Vanilla, cracked chocolate and almonds", "فانيليا وشوكولاتة مقرمشة ولوز"),
      desc=("Creamy vanilla ice cream with crackly Belgian-chocolate pieces and roasted almonds, in a "
            "sharing tub. Delivered frozen in an insulated bag.",
            "آيس كريم فانيليا كريمي بقطع شوكولاتة بلجيكي مقرمشة ولوز محمص، في علبة للمشاركة. بيوصل "
            "متجمد في شنطة حافظة."),
      hl=[("Sharing tub", "علبة للمشاركة"), ("Delivered frozen", "بيوصل متجمد")],
      specs=[S("volume", "440 ml", "440 مل"), S("storage", FREEZER)],
      img=["d:ice-cream-1", "d:ice-cream-3"],
      r=(4.7, 388), tags="",
      v=[V(None, 169.99, sku="440ML")]),

    P(id=2457, slug="mars-bar", brand="mars",
      name=("Mars chocolate bar", "شوكولاتة مارس"),
      sub=("Nougat, caramel and milk chocolate", "نوجا وكراميل وشوكولاتة باللبن"),
      desc=("The classic Mars bar: soft nougat and caramel covered in milk chocolate. Buy one for the road "
            "or a five-pack for the house.",
            "مارس الكلاسيكي: نوجا طرية وكراميل متغطيين بشوكولاتة باللبن. خد واحدة للطريق أو باكت 5 "
            "للبيت."),
      hl=[("51 g bar", "قطعة 51 جم"), ("Five-pack saves", "باكت الخمسة أوفر")],
      specs=[S("storage", DRY)],
      img=["m:Mars Bar"],
      r=(4.6, 410), tags="deal",
      options=[axis("pack", [("1", L("1 bar", "قطعة")), ("5", L("5 bars", "5 قطع"))])],
      v=[V("1", 24.99), V("5", 109.99, was=124.95)]),

    P(id=2458, slug="soft-caramels",
      name=("Soft caramels", "كراميل طري"),
      sub=("Buttery chews, wrapped one by one", "كراميل بالزبدة ملفوف حباية حباية"),
      desc=("Soft, buttery caramel chews, individually wrapped. A sweet jar favourite for guests and a "
            "pocket treat for the kids.",
            "كراميل طري بالزبدة ملفوف حباية حباية. المفضل في برطمان الضيوف وفي جيب الولاد."),
      hl=[("Individually wrapped", "ملفوف حباية حباية"), ("Made with butter", "معمول بالزبدة")],
      specs=[S("storage", DRY)],
      img=["m:Caramel"],
      r=(4.4, 144), tags="",
      v=[V(None, 59.99, sku="250G")]),

    P(id=2459, slug="cadbury-royal-dark", brand="cadbury",
      name=("Cadbury Royal Dark", "كادبوري رويال دارك"),
      sub=("Smooth dark chocolate bar", "لوح شوكولاتة داكنة ناعمة"),
      desc=("Cadbury's Royal Dark — smooth, rich dark chocolate that's not too bitter. Break off a square "
            "with your coffee.",
            "كادبوري رويال دارك... شوكولاتة داكنة ناعمة وغنية ومش مُرّة أوي. خد مربع مع فنجان القهوة."),
      hl=[("Smooth, not bitter", "ناعمة ومش مُرّة"), ("Pairs well with coffee", "حلوة مع القهوة")],
      specs=[S("weight", "90 g", "90 جم"), S("storage", DRY)],
      img=["m:Dark Chocolate"],
      r=(4.5, 133), tags="",
      v=[V(None, 79.99, sku="90G")]),
]

"""Spices & herbs (category 80, ids 2351–2399)."""

from common import AIRTIGHT, L, P, S, V, axis, mg, mo, weights

CATEGORY = 80

KEEP = S("storage", AIRTIGHT)


def grind_fresh():
    return mg("grind", "Grinding", "الطحن", [
        mo("whole", "Leave whole", "حب زي ما هو"),
        mo("ground", "Grind fresh for me", "اطحنه طازة", 5)], required=True)


PRODUCTS = [
    P(id=2351, slug="ground-cumin", brand="abu-auf",
      name=("Ground cumin", "كمون مطحون"),
      sub=("Warm and earthy — ful's best friend", "دافي وريحته حلوة... صاحب الفول"),
      desc=("Freshly ground Egyptian cumin with a warm, earthy aroma. A pinch transforms ful medames, lentil "
            "soup, fried fish and ta'ameya.",
            "كمون مصري مطحون طازة وريحته دافية. رشة واحدة بتغير طعم الفول المدمس وشوربة العدس والسمك "
            "المقلي والطعمية."),
      hl=[("Egyptian cumin", "كمون مصري"), ("Ground in small batches", "بيتطحن بكميات صغيرة")],
      specs=[S("origin", "Egypt", "مصر"), KEEP],
      img=["m:Cumin"],
      r=(4.7, 1340), tags="bestseller",
      options=[weights("100g", "250g")],
      v=[V("100g", 49.99), V("250g", 114.99)]),

    P(id=2352, slug="sweet-paprika", brand="souq",
      name=("Sweet paprika", "بابريكا"),
      sub=("Mild, red and fragrant", "خفيفة وحمرا وريحتها حلوة"),
      desc=("Sweet ground paprika for colour and gentle warmth. Dust it over hummus and eggs, or rub it into "
            "chicken with garlic and lemon before roasting.",
            "بابريكا حلوة مطحونة بتدي لون ودفا خفيف... رشّها على الحمص والبيض، أو تبّل بيها الفراخ مع "
            "التوم والليمون قبل الفرن."),
      hl=[("Not spicy", "مش حرّاقة"), ("Deep red colour", "لون أحمر غامق")],
      specs=[KEEP],
      img=["m:Paprika"],
      r=(4.5, 512), tags="",
      options=[weights("100g", "250g")],
      v=[V("100g", 39.99), V("250g", 89.99)]),

    P(id=2353, slug="cinnamon", brand="souq",
      name=("Cinnamon", "قرفة"),
      sub=("Sticks for drinks, ground for baking", "عيدان للمشروبات ومطحونة للحلويات"),
      desc=("Fragrant cinnamon two ways. Simmer the sticks into a warming winter cinnamon drink or a pot of "
            "rice, and dust the ground cinnamon over roz bel laban, sahlab and basbousa.",
            "قرفة ريحتها فايحة بطريقتين. اغلي العيدان واعمل قرفة سخنة في الشتا أو حطها على حلة الرز، "
            "ورش المطحونة على الرز باللبن والسحلب والبسبوسة."),
      hl=[("Sticks or ground", "عيدان أو مطحونة"), ("100 g pack", "عبوة 100 جم")],
      specs=[S("origin", "Imported", "مستورد"), KEEP],
      img=[("m:Cinnamon Stick", L("sticks", "عيدان")), ("m:Cinnamon", L("ground", "مطحونة"))],
      r=(4.6, 640), tags="",
      options=[axis("form", [("sticks", L("Sticks", "عيدان")), ("ground", L("Ground", "مطحونة"))])],
      v=[V("sticks", 59.99, img=0, sku="STK-100G"), V("ground", 54.99, img=1, sku="GRD-100G")]),

    P(id=2354, slug="green-cardamom", brand="abu-auf",
      name=("Green cardamom", "حبهان (هيل)"),
      sub=("For coffee, rice and sweets", "للقهوة والرز والحلويات"),
      desc=("Plump green cardamom pods with an intense, floral aroma. Crack a few into your Turkish coffee "
            "pot, add them to rice and meat broths, or grind them for kahk and ghorayeba.",
            "حبهان أخضر مليان وريحته قوية. اكسر كام حباية في كنكة القهوة، أو حطه على الرز وشوربة "
            "اللحمة، أو اطحنه للكحك والغريبة."),
      hl=[("Large green pods", "حبة خضرا كبيرة"), ("Intense, floral aroma", "ريحة قوية وفوّاحة")],
      specs=[S("origin", "Guatemala", "جواتيمالا"), KEEP],
      img=["m:Cardamom"],
      r=(4.8, 388), tags="featured",
      options=[weights("50g", "100g")],
      v=[V("50g", 159.99), V("100g", 309.99)],
      mods=[grind_fresh()]),

    P(id=2355, slug="whole-cloves", brand="souq",
      name=("Whole cloves", "قرنفل"),
      sub=("A few go a long way", "شوية بيعملوا فرق كبير"),
      desc=("Whole cloves for meat and chicken broths, spiced tea and pickles. Stud an onion with a few and "
            "drop it into the pot for a fragrant stock.",
            "قرنفل حب لشوربة اللحمة والفراخ والشاي بالتوابل والمخلل. اغرس كام حباية في بصلة وحطها في "
            "الحلة لشوربة ريحتها تجنن."),
      hl=[("Whole buds", "حب كامل"), ("Strong aroma", "ريحته قوية")],
      specs=[S("origin", "Indonesia", "إندونيسيا"), KEEP],
      img=["m:Cloves"],
      r=(4.5, 166), tags="",
      options=[weights("50g", "100g")],
      v=[V("50g", 69.99), V("100g", 134.99)],
      mods=[grind_fresh()]),

    P(id=2356, slug="bay-leaves", brand="souq",
      name=("Bay leaves", "ورق لورا"),
      sub=("For every pot of stock", "لكل حلة شوربة"),
      desc=("Dried bay leaves to perfume chicken and meat stock, rice, lentils and béchamel. Two leaves per "
            "pot are plenty.",
            "ورق لورا ناشف بيدي ريحة حلوة لشوربة الفراخ واللحمة والرز والعدس والبشاميل. ورقتين في "
            "الحلة كفاية."),
      hl=[("Whole, unbroken leaves", "ورق كامل مش مكسّر"), ("25 g goes a long way", "الـ25 جم بتقعد كتير")],
      specs=[KEEP],
      img=["m:Bay Leaves"],
      r=(4.4, 240), tags="",
      v=[V(None, 19.99, sku="25G")]),

    P(id=2357, slug="star-anise", brand="souq",
      name=("Star anise", "ينسون نجمي"),
      sub=("Liquorice-sweet, for drinks and broths", "طعمه زي العرقسوس... للمشروبات والشوربة"),
      desc=("Whole star anise to simmer into hot drinks and syrups, or to add a gentle sweetness to beef "
            "broth and stewed fruit.",
            "ينسون نجمي حب، يتغلي في المشروبات السخنة والشربات، أو يدي حلاوة خفيفة لشوربة اللحمة "
            "والفواكه المطبوخة."),
      hl=[("Whole stars", "نجوم كاملة"), ("Warming winter drink", "مشروب دافي للشتا")],
      specs=[S("origin", "Imported", "مستورد"), KEEP],
      img=["m:Star Anise"],
      r=(4.5, 98), tags="new",
      v=[V(None, 49.99, sku="50G")]),

    P(id=2358, slug="saffron-threads", brand="abu-auf",
      name=("Saffron threads", "زعفران"),
      sub=("Pure threads for rice and sweets", "شعر زعفران أصلي للرز والحلويات"),
      desc=("Deep-red saffron threads with a honeyed aroma. Steep a pinch in warm water before adding it to "
            "rice, chicken or milk puddings.",
            "شعر زعفران أحمر غامق وريحته زي العسل. انقع رشة في مية دافية قبل ما تحطها على الرز أو "
            "الفراخ أو حلويات اللبن."),
      hl=[("Pure threads, no powder", "شعر أصلي مش بودرة"), ("A pinch is enough", "رشة صغيرة كفاية")],
      specs=[S("origin", "Iran", "إيران"), KEEP],
      img=["m:Saffron"],
      r=(4.6, 75), tags="",
      options=[axis("weight", [("1g", L("1 g", "1 جم")), ("2g", L("2 g", "2 جم"))])],
      v=[V("1g", 159.99), V("2g", 309.99)]),

    P(id=2359, slug="hot-chilli-powder", brand="souq",
      name=("Hot chilli powder", "شطة مطحونة"),
      sub=("Proper heat for shatta and stews", "حراقة مظبوطة للشطة والطبيخ"),
      desc=("Finely ground red chilli with real heat. Stir it into tomato sauce, sprinkle it on koshari, or "
            "mix it with oil and garlic for a quick shatta.",
            "شطة حمرا مطحونة ناعم وحراقها حقيقي. حطها في الصلصة، أو رشّها على الكشري، أو اخلطها "
            "بالزيت والتوم لشطة سريعة."),
      hl=[("Hot", "حرّاقة"), ("Finely ground", "مطحونة ناعم")],
      specs=[KEEP],
      img=["m:Chili Powder"],
      r=(4.4, 302), tags="",
      options=[weights("100g", "250g")],
      v=[V("100g", 34.99), V("250g", 79.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import EGYPT  # noqa: E402

PRODUCTS += [
    P(id=2360, slug="ground-nutmeg", brand="souq",
      name=("Ground nutmeg", "جوزة الطيب مطحونة"),
      sub=("Warm spice for béchamel and sweets", "بهار دافي للبشاميل والحلويات"),
      desc=("Ground nutmeg with a warm, sweet aroma. A pinch belongs in every béchamel — and in "
            "mahalabiya, spiced milk and kahk.",
            "جوزة الطيب مطحونة وريحتها دافية ومسكّرة. رشة منها لازم تبقى في كل بشاميل... وفي المهلبية "
            "واللبن بالتوابل والكحك."),
      hl=[("A béchamel must", "أساسية في البشاميل"), ("Use sparingly", "رشة صغيرة كفاية")],
      specs=[S("origin", "Indonesia", "إندونيسيا"), KEEP],
      img=["m:Nutmeg"],
      r=(4.6, 210), tags="",
      v=[V(None, 49.99, sku="50G")]),

    P(id=2361, slug="fresh-rosemary",
      name=("Fresh rosemary", "روزماري فريش"),
      sub=("Woody sprigs for lamb and potatoes", "عيدان للضاني والبطاطس"),
      desc=("A fragrant bunch of fresh rosemary. Tuck sprigs around a leg of lamb or a tray of potatoes, or "
            "infuse it in olive oil.",
            "حزمة روزماري فريش ريحتها فايحة. حطها حوالين فخدة الضاني أو صينية البطاطس، أو انقعها في "
            "زيت الزيتون."),
      hl=[("Cut fresh", "مقطوف فريش"), ("Keeps a week in the fridge", "يعيش أسبوع في التلاجة")],
      specs=[S("origin", EGYPT), S("sold", "Per bunch (about 50 g)", "بالحزمة (حوالي 50 جم)")],
      img=["m:Rosemary"],
      r=(4.5, 66), tags="",
      v=[V(None, 14.99, sku="BUNCH")]),

    P(id=2362, slug="coarse-sea-salt", brand="souq",
      name=("Coarse sea salt", "ملح بحري خشن"),
      sub=("For pickling, grilling and the grinder", "للتخليل والشوي والمطحنة"),
      desc=("Natural coarse sea salt from the lakes of the north coast. Use it for home pickles, crust it on "
            "grilled fish, or fill your grinder.",
            "ملح بحري خشن طبيعي من بحيرات الساحل الشمالي. استعمله في المخلل البيتي، أو غطّي بيه السمك "
            "المشوي، أو املا بيه المطحنة."),
      hl=[("Natural, unrefined", "طبيعي من غير تكرير"), ("Ideal for pickles", "مثالي للمخلل")],
      specs=[S("origin", EGYPT), KEEP],
      img=["m:Coarse Salt"],
      r=(4.4, 88), tags="",
      v=[V(None, 19.99, sku="1KG")]),

    P(id=2363, slug="sesame-seeds", brand="souq",
      name=("White sesame seeds", "سمسم أبيض"),
      sub=("For simit, ka'ak and homemade tahini", "للسميط والكعك والطحينة البيتي"),
      desc=("Hulled white sesame seeds. Toast them golden for simit and ka'ak, sprinkle them over salads, or "
            "grind them with a little oil for fresh tahini at home.",
            "سمسم أبيض مقشور. حمّصه لحد ما يدهّب للسميط والكعك، أو رشّه على السلطة، أو اطحنه بشوية زيت "
            "لطحينة طازة في البيت."),
      hl=[("Hulled", "مقشور"), ("Toast for extra flavour", "حمّصه علشان الطعم")],
      specs=[S("origin", "Sudan", "السودان"), KEEP],
      img=["m:Sesame Seed"],
      r=(4.5, 121), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 49.99), V("500g", 94.99)]),
]

"""Bakery (category 20, ids 2051–2099)."""

from common import L, P, S, V, axis

CATEGORY = 20

DAILY = S("baked", "Daily, before 7 am", "كل يوم قبل 7 الصبح")
KEEP = S("storage", "Room temperature 2 days, or freeze", "يومين برة التلاجة، أو في الفريزر")


def loaves(*vals):
    return axis("pack", [(v[0], L(*v[1])) for v in vals])


PRODUCTS = [
    P(id=2051, slug="shami-bread", brand="souq",
      name=("Shami bread", "عيش شامي"),
      sub=("Soft white pita, baked this morning", "عيش شامي أبيض طري من فرن النهارده"),
      desc=("Soft, pocketed white pita baked before dawn and bagged warm. Split it for ful and ta'ameya "
            "sandwiches, tear it over fatta, or toast it into crunchy chips.",
            "عيش شامي أبيض طري وبجيب، بيتخبز قبل الفجر ويتعبّى سخن. افتحه لسندوتشات الفول "
            "والطعمية، أو قطّعه على الفتة، أو حمّصه عيش مقرمش."),
      hl=[("Baked every morning", "بيتخبز كل يوم الصبح"), ("Freezes and reheats well", "يتجمد ويتسخن كويس")],
      specs=[DAILY, KEEP, S("ingredients", "Wheat flour, water, yeast, salt, sugar",
                            "دقيق قمح، مية، خميرة، ملح، سكر")],
      img=["m:Pita Bread"],
      r=(4.6, 1650), tags="bestseller express",
      options=[loaves(("5pcs", ("5 loaves", "5 أرغفة")), ("10pcs", ("10 loaves", "10 أرغفة")))],
      v=[V("5pcs", 17.99), V("10pcs", 33.99)]),

    P(id=2052, slug="french-baguette", brand="souq",
      name=("French baguette", "باجيت فرنساوي"),
      sub=("Crackly crust, airy inside", "قشرتها مقرمشة وجوها هش"),
      desc=("A French-style baguette with a crackling crust and open crumb. Slice it for garlic bread or a "
            "cheese board, or make a proper romy-and-tomato sandwich.",
            "باجيت على الطريقة الفرنساوي، قشرتها بتطقطق وجوها هش. قطّعها للعيش بالتوم أو طبق "
            "الأجبان، أو اعمل ساندوتش رومي وطماطم يستاهل."),
      hl=[("About 250 g", "حوالي 250 جم"), ("Best eaten the same day", "أحلى في نفس اليوم")],
      specs=[DAILY, KEEP],
      img=["m:Baguette"],
      r=(4.5, 402), tags="express",
      v=[V(None, 24.99, sku="250G")]),

    P(id=2053, slug="country-sourdough", brand="souq",
      name=("Country sourdough loaf", "عيش ساوردو"),
      sub=("48-hour ferment, crusty and tangy", "تخمير 48 ساعة... قشرة مقرمشة وطعم مزّ"),
      desc=("A rustic round loaf made with natural starter and a slow 48-hour ferment, for a deep crust and "
            "a chewy, tangy crumb. Toast it with olive oil and za'atar or serve it with shakshuka.",
            "رغيف ساوردو مدوّر معمول بخميرة طبيعية وتخمير بطيء 48 ساعة، قشرته غامقة وقلبه مطاطي وطعمه "
            "مزّ. حمّصه بزيت الزيتون والزعتر أو قدّمه مع الشكشوكة."),
      hl=[("Natural starter, no commercial yeast", "خميرة طبيعية من غير خميرة صناعي"),
          ("About 600 g", "حوالي 600 جم")],
      specs=[DAILY, S("storage", "Paper bag 3 days, or slice and freeze", "كيس ورق 3 أيام، أو قطّعه وجمّده"),
             S("ingredients", "Wheat flour, whole-wheat flour, water, salt", "دقيق أبيض، دقيق بلدي، مية، ملح")],
      img=["m:Crusty Bread"],
      r=(4.7, 233), tags="new featured",
      v=[V(None, 89.99, sku="600G")]),

    P(id=2054, slug="brioche-buns", brand="souq",
      name=("Brioche buns", "بريوش"),
      sub=("Buttery, soft and a little sweet", "بالزبدة وطري ومسكّر خفيف"),
      desc=("Golden brioche buns enriched with butter and eggs. Lovely at breakfast with cheese and jam, or "
            "as a soft bun for burgers and chicken sandwiches.",
            "بريوش دهبي معمول بالزبدة والبيض. حلو على الفطار بالجبنة والمربى، أو كعيش طري للبرجر "
            "وساندوتشات الفراخ."),
      hl=[("Made with real butter", "بزبدة حقيقية"), ("Toast for 1 minute", "حمّصه دقيقة")],
      specs=[DAILY, KEEP],
      img=["m:Bun"],
      r=(4.5, 178), tags="",
      options=[loaves(("4pcs", ("4 buns", "4 قطع")), ("8pcs", ("8 buns", "8 قطع")))],
      v=[V("4pcs", 44.99), V("8pcs", 84.99)]),

    P(id=2055, slug="sesame-burger-buns", brand="souq",
      name=("Sesame burger buns", "عيش برجر بالسمسم"),
      sub=("Soft buns that hold the juices", "طري ومش بيفرط من صوص البرجر"),
      desc=("Pillowy burger buns topped with toasted sesame, sturdy enough for a juicy homemade burger. "
            "Warm them in a pan for a minute before serving.",
            "عيش برجر طري متغطي سمسم محمص، ماسك نفسه مع البرجر البيتي المليان عصارة. سخّنه على "
            "الطاسة دقيقة قبل التقديم."),
      hl=[("Pack of 6", "6 قطع في الكيس"), ("Toasted sesame top", "متغطي سمسم محمص")],
      specs=[DAILY, KEEP],
      img=["m:Sesame Seed Burger Buns"],
      r=(4.4, 156), tags="",
      v=[V(None, 39.99, sku="6PCS")]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import FREEZER, mg, mo  # noqa: E402

PRODUCTS += [
    P(id=2056, slug="fino-bread", brand="souq",
      name=("Fino bread", "عيش فينو"),
      sub=("Soft rolls for school sandwiches", "عيش طري لسندوتشات المدرسة"),
      desc=("Soft, slightly sweet fino rolls — the classic for school sandwiches with cheese, halawa or "
            "luncheon. Baked fresh every morning.",
            "عيش فينو طري ومسكّر خفيف... الكلاسيكي لسندوتشات المدرسة بالجبنة أو الحلاوة أو اللانشون. "
            "بيتخبز طازة كل يوم الصبح."),
      hl=[("Baked every morning", "بيتخبز كل يوم الصبح"), ("Soft crust", "قشرته طرية")],
      specs=[DAILY, KEEP],
      img=["m:Cuban Bread"],
      r=(4.5, 980), tags="express",
      options=[loaves(("5pcs", ("5 rolls", "5 أرغفة")), ("10pcs", ("10 rolls", "10 أرغفة")))],
      v=[V("5pcs", 14.99), V("10pcs", 27.99)]),

    P(id=2057, slug="ciabatta", brand="souq",
      name=("Ciabatta", "شاباتا"),
      sub=("Open crumb, olive-oil crust", "قلبها هش وقشرتها بزيت الزيتون"),
      desc=("Rustic Italian-style ciabatta with an airy, open crumb. Split it for panini, toast it with "
            "garlic butter, or serve it alongside a bowl of soup.",
            "شاباتا على الطريقة الإيطالي، قلبها هش ومليان فراغات. افتحها للبانيني، أو حمّصها بزبدة "
            "التوم، أو قدّمها جنب طبق الشوربة."),
      hl=[("Pack of 2", "قطعتين في الكيس"), ("Perfect for panini", "مثالية للبانيني")],
      specs=[DAILY, KEEP],
      img=["m:Ciabatta"],
      r=(4.4, 143), tags="",
      v=[V(None, 39.99, sku="2PCS")]),

    P(id=2058, slug="white-toast-bread", brand="souq",
      name=("White toast bread", "عيش توست أبيض"),
      sub=("Soft sliced loaf for the toaster", "رغيف متقطع شرايح للتوستر"),
      desc=("A soft, pre-sliced white loaf that toasts evenly golden. Breakfast with butter and jam, or "
            "grilled cheese sandwiches for the kids.",
            "توست أبيض طري متقطع شرايح ويتحمص دهبي بالتساوي. للفطار بالزبدة والمربى، أو ساندوتشات "
            "جبنة سايحة للولاد."),
      hl=[("Pre-sliced", "متقطع شرايح"), ("About 600 g", "حوالي 600 جم")],
      specs=[DAILY, S("storage", "Room temperature 4 days, or freeze", "4 أيام برة التلاجة، أو في الفريزر")],
      img=["m:Toast"],
      r=(4.4, 620), tags="express",
      v=[V(None, 44.99, sku="600G")]),

    P(id=2059, slug="soft-milk-rolls", brand="souq",
      name=("Soft milk rolls", "ميني رولز باللبن"),
      sub=("Pull-apart rolls, fluffy and light", "رولز طرية بتتقطع بالإيد"),
      desc=("Pull-apart rolls made with milk for a fluffy, cloud-soft crumb. Warm them for breakfast or "
            "fill them with cheese and cucumber for the lunch box.",
            "رولز معمولة باللبن، طرية زي السحاب وبتتقطع بالإيد. سخّنها على الفطار أو احشيها جبنة "
            "وخيار للانش بوكس."),
      hl=[("Made with milk", "معمولة باللبن"), ("6 rolls", "6 قطع")],
      specs=[DAILY, KEEP],
      img=["m:Bread Rolls"],
      r=(4.6, 312), tags="new",
      v=[V(None, 29.99, sku="6PCS")]),

    P(id=2060, slug="tortilla-wraps", brand="souq",
      name=("Tortilla wraps", "خبز تورتيلا"),
      sub=("Soft wraps for shawarma and fajitas", "تورتيلا طرية للشاورما والفاهيتا"),
      desc=("Soft 25 cm wheat tortillas that fold without cracking. Wrap them around shawarma, fajitas or "
            "a quick chicken pané roll.",
            "تورتيلا قمح طرية مقاس 25 سم بتتلف من غير ما تتشقق. لفّ فيها الشاورما أو الفاهيتا أو "
            "رول بانيه سريع."),
      hl=[("6 wraps, 25 cm", "6 أرغفة مقاس 25 سم"), ("Warm for 10 seconds per side", "سخّنها 10 ثواني كل ناحية")],
      specs=[S("storage", "Cool place; fridge after opening", "مكان بارد، وبعد الفتح في التلاجة")],
      img=["m:Tortillas"],
      r=(4.3, 188), tags="",
      v=[V(None, 49.99, sku="6PCS")]),

    P(id=2061, slug="puff-pastry-sheets", brand="souq",
      name=("Puff pastry sheets", "عجينة باف"),
      sub=("Frozen, ready-rolled layers", "عجينة متجمدة جاهزة للفرد"),
      desc=("All-butter-style puff pastry, frozen in ready-rolled sheets. Thaw for 20 minutes, then fill for "
            "patties, mille-feuille or a quick sausage roll.",
            "عجينة باف بطبقات، متجمدة في شيتات جاهزة. سيبها تفك 20 دقيقة واحشيها للباتيه أو الميل "
            "فوي أو رول سجق سريع."),
      hl=[("Ready-rolled", "مفرودة وجاهزة"), ("Bake at 200 °C", "تتخبز على 200 درجة")],
      specs=[S("weight", "500 g", "500 جم"), S("storage", FREEZER)],
      img=["m:Puff Pastry"],
      r=(4.5, 266), tags="",
      v=[V(None, 69.99, sku="500G")]),

    P(id=2062, slug="breadcrumbs", brand="souq",
      name=("Breadcrumbs", "بقسماط مطحون"),
      sub=("From our own bread, for pané and kofta", "من عيشنا... للبانيه والكفتة"),
      desc=("Golden breadcrumbs made from our own day-old bread. Coat chicken pané, fish fillets and "
            "potato croquettes, or bind kofta and stuffed vegetables.",
            "بقسماط دهبي معمول من عيشنا بتاع امبارح. غطّي بيه بانيه الفراخ وفيليه السمك وكفتة "
            "البطاطس، أو استعمله يمسك الكفتة."),
      hl=[("Crunchy coating", "تغطية مقرمشة"), ("No additives", "من غير إضافات")],
      specs=[S("weight", "500 g", "500 جم"), S("storage", "Airtight, cool and dry", "علبة محكمة في مكان جاف")],
      img=["m:Stale bread"],
      r=(4.5, 240), tags="",
      v=[V(None, 34.99, sku="500G")]),

    P(id=2063, slug="farmhouse-loaf", brand="souq",
      name=("Farmhouse loaf", "عيش لوف بلدي"),
      sub=("Baked today, golden crust", "مخبوز النهارده... وقشرته دهبي"),
      desc=("A tall white farmhouse loaf with a golden crust and soft crumb. Slice it thick for toast or "
            "thin for sandwiches.",
            "رغيف لوف أبيض عالي، قشرته دهبي وقلبه طري. قطّعه شرايح تخينة للتوست أو رفيعة "
            "للساندوتشات."),
      hl=[("Baked today", "مخبوز النهارده"), ("About 800 g", "حوالي 800 جم")],
      specs=[DAILY, KEEP],
      img=["m:Bread"],
      r=(4.5, 154), tags="",
      v=[V(None, 54.99, sku="800G")],
      mods=[mg("slice", "Slicing", "التقطيع", [
          mo("whole", "Whole loaf", "رغيف صحيح"),
          mo("thick", "Thick slices", "شرايح تخينة"),
          mo("thin", "Thin slices", "شرايح رفيعة")], required=True)]),
]

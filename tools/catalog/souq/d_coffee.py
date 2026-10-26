"""Coffee & tea (category 110, ids 2501–2549)."""

from common import AIRTIGHT, DRY, L, P, S, V, axis, mg, mo, weights

CATEGORY = 110


def roast():
    return mg("roast", "Roast", "التحميص", [
        mo("light", "Light", "فاتح"),
        mo("medium", "Medium", "وسط"),
        mo("dark", "Dark", "غامق")], required=True)


PRODUCTS = [
    P(id=2501, slug="abu-auf-turkish-coffee", brand="abu-auf",
      name=("Abu Auf Turkish coffee", "بن تركي أبو عوف"),
      sub=("Roasted and ground fresh — plain or with cardamom", "متحمص ومطحون طازة... سادة أو محوّج"),
      desc=("Abu Auf's classic Turkish coffee, roasted and ground fresh in the shop. Pick your roast, keep "
            "it extra fine for the kanaka pot, and add cardamom if you like it mahwag — the Egyptian way.",
            "بن تركي أبو عوف الكلاسيكي، متحمص ومطحون طازة في المحل. اختار التحميصة، وخليه ناعم جدًا "
            "للكنكة، وزوّد الحبهان لو بتحبه محوّج... على الطريقة المصرية."),
      hl=[("Ground on the day you order", "بيتطحن يوم طلبك"), ("Cardamom on request", "محوّج بالحبهان لو تحب"),
          ("Choose light, medium or dark", "فاتح أو وسط أو غامق")],
      specs=[S("brew", "1 heaped teaspoon per cup, in a kanaka", "معلقة صغيرة مليانة للفنجان في الكنكة"),
             S("storage", AIRTIGHT)],
      img=["m:Coffee"],
      r=(4.8, 1890), tags="bestseller featured",
      options=[weights("125g", "250g", "500g")],
      v=[V("125g", 114.99), V("250g", 219.99), V("500g", 429.99)],
      mods=[roast(),
            mg("grind", "Grind", "الطحنة", [
                mo("turkish", "Extra fine (Turkish)", "ناعم جدًا (تركي)"),
                mo("filter", "Medium (filter)", "وسط (فلتر)"),
                mo("beans", "Whole beans", "حبوب")], required=True),
            mg("extras", "Extras", "إضافات", [
                mo("cardamom", "Cardamom (mahwag)", "محوّج بالحبهان", 10),
                mo("mastic", "Mastic", "مستكة", 15)], selection="multi", max_=2)]),

    P(id=2502, slug="espresso-roast-beans", brand="souq",
      name=("Espresso roast beans", "بن إسبريسو"),
      sub=("Roasted this week in our roastery", "متحمص الأسبوع ده في محمصتنا"),
      desc=("A chocolatey espresso blend of Brazilian and Ethiopian beans, roasted weekly in small batches. "
            "Buy it as whole beans or let us grind it for your espresso machine, moka pot or filter.",
            "خلطة إسبريسو طعمها شوكولاتة من بن برازيلي وإثيوبي، بتتحمص كل أسبوع بكميات صغيرة. خده "
            "حبوب أو خلّينا نطحنه لمكنة الإسبريسو أو الموكا أو الفلتر."),
      hl=[("Roast date on every bag", "تاريخ التحميص على كل كيس"), ("Notes of cocoa and caramel", "نكهة كاكاو وكراميل")],
      specs=[S("origin", "Brazil and Ethiopia", "البرازيل وإثيوبيا"), S("roast", "Medium-dark", "وسط مايل للغامق"),
             S("storage", AIRTIGHT)],
      img=["m:Espresso"],
      r=(4.7, 260), tags="new",
      options=[weights("250g", "1kg")],
      v=[V("250g", 279.99), V("1kg", 1049.99)],
      mods=[mg("grind", "Grind", "الطحنة", [
          mo("beans", "Whole beans", "حبوب"),
          mo("espresso", "Espresso", "إسبريسو"),
          mo("moka", "Moka pot", "موكا"),
          mo("filter", "Filter", "فلتر")], required=True)]),

    P(id=2503, slug="el-arosa-black-tea", brand="el-arosa",
      name=("El Arosa black tea", "شاي العروسة"),
      sub=("Strong loose tea, Egyptian style", "شاي ناعم تقيل على المزاج المصري"),
      desc=("El Arosa's fine loose black tea — dark, strong and brisk, made for the Egyptian way: brewed "
            "strong with sugar, with fresh mint, or as shai bel laban.",
            "شاي العروسة ناعم أسود، تقيل ولونه غامق وطعمه مظبوط... معمول للمزاج المصري: تقيل بالسكر، أو "
            "بالنعناع، أو شاي بلبن."),
      hl=[("Fine loose leaf", "شاي ناعم"), ("Strong colour and taste", "لون وطعم تقيل")],
      specs=[S("brew", "1 teaspoon per glass, steep 3 minutes", "معلقة صغيرة للكوباية، وسيبه 3 دقايق"),
             S("storage", DRY)],
      img=["m:Tea"],
      r=(4.7, 2050), tags="bestseller express",
      options=[weights("250g", "500g")],
      v=[V("250g", 74.99), V("500g", 144.99)]),

    P(id=2504, slug="nescafe-classic", brand="nescafe",
      name=("Nescafé Classic", "نسكافيه كلاسيك"),
      sub=("Rich instant coffee in a jar", "قهوة سريعة الذوبان في برطمان"),
      desc=("Nescafé Classic's rich, full-bodied instant coffee. Stir a spoonful into hot water or milk, or "
            "whip it with sugar for a frothy beaten Nescafé.",
            "نسكافيه كلاسيك طعمه غني وتقيل. معلقة في مية أو لبن سخن، أو اضربه بالسكر لنسكافيه مضروب "
            "بالرغوة."),
      hl=[("100% coffee", "قهوة 100%"), ("Big jar on offer", "البرطمان الكبير عليه عرض")],
      specs=[S("storage", DRY)],
      img=["d:nescafe-coffee-1"],
      r=(4.6, 980), tags="deal",
      options=[weights("50g", "100g", "200g")],
      v=[V("50g", 124.99), V("100g", 239.99), V("200g", 459.99, was=489.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2505, slug="baladi-anise", brand="isis",
      name=("Isis baladi anise", "ينسون بلدي إيزيس"),
      sub=("Soothing anise seeds for a warm cup", "ينسون بلدي لكوباية دافية تريّح"),
      desc=("Whole Egyptian anise seeds from Isis. Simmer a spoonful for five minutes for the soothing "
            "after-dinner drink every Egyptian grandmother swears by — with honey and a slice of lemon.",
            "ينسون بلدي حب من إيزيس. اغلي معلقة 5 دقايق واعمل المشروب الدافي اللي كل تيتة مصرية بتحلف "
            "بيه بعد الأكل... بالعسل وشريحة ليمون."),
      hl=[("Egyptian anise", "ينسون مصري"), ("Caffeine-free", "من غير كافيين")],
      specs=[S("origin", "Egypt", "مصر"), S("brew", "1 teaspoon per cup, simmer 5 minutes",
                                                 "معلقة صغيرة للكوباية، تغلي 5 دقايق"), S("storage", AIRTIGHT)],
      img=["m:Anise"],
      r=(4.6, 320), tags="",
      options=[weights("100g", "250g")],
      v=[V("100g", 29.99), V("250g", 69.99)]),

    P(id=2506, slug="dried-mint", brand="isis",
      name=("Isis dried mint", "نعناع ناشف إيزيس"),
      sub=("For mint tea, salads and yogurt", "لشاي النعناع والسلطة والزبادي"),
      desc=("Dried Egyptian mint leaves from Isis, rubbed fine. Brew it alone or with black tea, crumble it "
            "over cucumber yogurt salad, or stir it into lentil soup.",
            "نعناع مصري ناشف من إيزيس، مفروك ناعم. اعمله شاي لوحده أو مع الشاي الأسود، أو رشّه على "
            "سلطة الزبادي بالخيار، أو حطه في شوربة العدس."),
      hl=[("Egyptian mint", "نعناع مصري"), ("Strong aroma", "ريحته قوية")],
      specs=[S("origin", "Egypt", "مصر"), S("storage", AIRTIGHT)],
      img=["m:Chopped Parsley"],
      r=(4.5, 176), tags="",
      v=[V(None, 24.99, sku="50G")]),

    P(id=2507, slug="hersheys-cocoa", brand="hersheys",
      name=("Hershey's cocoa powder", "كاكاو هيرشيز"),
      sub=("100% cocoa for drinks and baking", "كاكاو 100% للمشروبات والخبيز"),
      desc=("Hershey's unsweetened 100% cocoa. Whisk it with hot milk and sugar for a proper winter cocoa, "
            "or use it in brownies, cakes and chocolate sauce.",
            "كاكاو هيرشيز 100% من غير سكر. اخفقه مع اللبن السخن والسكر لكوباية كاكاو شتوية، أو استعمله "
            "في البراونيز والكيك وصوص الشوكولاتة."),
      hl=[("100% cocoa", "كاكاو 100%"), ("Unsweetened", "من غير سكر")],
      specs=[S("weight", "226 g", "226 جم"), S("origin", "USA", "أمريكا"), S("storage", DRY)],
      img=["m:Cocoa"],
      r=(4.7, 154), tags="",
      v=[V(None, 249.99, sku="226G")]),

    P(id=2508, slug="hot-chocolate-mix", brand="souq",
      name=("Hot chocolate mix", "شوكولاتة سخنة"),
      sub=("Just add hot milk", "ضيف لبن سخن وبس"),
      desc=("A rich, ready-sweetened hot chocolate powder. Stir three spoonfuls into hot milk and top with "
            "whipped cream on a cold evening.",
            "بودرة شوكولاتة سخنة غنية ومتحلية جاهزة. قلّب 3 معالق في لبن سخن وزيّنها بكريمة مخفوقة "
            "في ليلة برد."),
      hl=[("Ready-sweetened", "متحلية جاهزة"), ("Winter favourite", "مفضلة الشتا")],
      specs=[S("weight", "400 g", "400 جم"), S("storage", DRY)],
      img=["m:Hot Chocolate"],
      r=(4.4, 98), tags="new",
      v=[V(None, 129.99, sku="400G")]),
]

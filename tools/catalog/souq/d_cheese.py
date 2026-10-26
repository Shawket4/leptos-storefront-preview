"""Cheese (category 40, ids 2151–2199)."""

from common import EGYPT, FRIDGE, L, P, S, V, mg, mo, weights

CATEGORY = 40

WRAPPED = L("Fridge, wrapped in paper", "في التلاجة ملفوفة في ورق")


def slicing():
    return mg("cut", "Cut", "التقطيع", [
        mo("piece", "One piece", "حتة واحدة"),
        mo("thin", "Thin slices", "شرايح رفيعة"),
        mo("thick", "Thick slices", "شرايح تخينة"),
        mo("grated", "Grated", "مبشورة", 5)], required=True)


PRODUCTS = [
    P(id=2151, slug="aged-romy-cheese",
      name=("Aged romy cheese", "جبنة رومي قديمة"),
      sub=("Sharp and salty, cut at the counter", "حادقة ومملحة... بتتقطع على الكاونتر"),
      desc=("Egypt's favourite hard cheese, aged for at least eight months until it's sharp, salty and "
            "slightly crumbly. We cut it to order: one piece for the fridge, thin slices for sandwiches, or "
            "grated for macarona béchamel.",
            "الجبنة الرومي اللي مصر كلها بتحبها، متعتقة 8 شهور على الأقل لحد ما تبقى حادقة ومملحة "
            "وبتتفتت شوية. بنقطعها على طلبك: حتة واحدة، أو شرايح رفيعة للساندوتشات، أو مبشورة "
            "للمكرونة بالبشاميل."),
      hl=[("Aged 8+ months", "متعتقة 8 شهور وأكتر"), ("Sliced or grated to order", "بنشرّحها أو نبشرها على الطلب")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("type", "Hard, aged", "جافة متعتقة"),
             S("origin", EGYPT), S("storage", WRAPPED)],
      img=["m:Cheese"],
      r=(4.7, 1290), tags="bestseller",
      options=[weights("250g", "500g", "1kg", loose=True)],
      v=[V("250g", 89.99), V("500g", 174.99), V("1kg", 339.99)],
      mods=[slicing()]),

    P(id=2152, slug="emmental",
      name=("Emmental", "جبنة إيمنتال"),
      sub=("Mild and nutty, melts beautifully", "طعمها خفيف وفيها نَفَس البندق"),
      desc=("A mild, nutty Emmental that melts beautifully. Slice it into toasties, grate it over a gratin, "
            "or cube it for the kids' lunch box.",
            "جبنة إيمنتال طعمها خفيف وفيها نَفَس البندق، وبتسيح حلو جدًا. شرّحها في التوست، أو "
            "ابشرها على الجراتان، أو قطّعها مكعبات للانش بوكس."),
      hl=[("Great melting cheese", "بتسيح حلو"), ("Cut fresh from the wheel", "متقطعة طازة من العجلة")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", "Imported", "مستورد"), S("storage", WRAPPED)],
      img=["m:Emmentaler Cheese"],
      r=(4.5, 214), tags="",
      options=[weights("200g", "400g")],
      v=[V("200g", 169.99), V("400g", 329.99)],
      mods=[slicing()]),

    P(id=2153, slug="maasdam",
      name=("Maasdam", "جبنة ماسدام"),
      sub=("Sweet and holey — the breakfast cheese", "أم خروم... وطعمها مسكّر"),
      desc=("Dutch-style Maasdam with big holes and a sweet, buttery flavour. Lovely on a breakfast platter "
            "with olives and cucumbers, and it melts smoothly into omelettes.",
            "جبنة ماسدام هولندي بخروم كبيرة وطعم مسكّر بالزبدة. حلوة في طبق الفطار مع الزيتون "
            "والخيار، وبتسيح ناعم في الأومليت."),
      hl=[("Sweet, buttery flavour", "طعم مسكّر بالزبدة"), ("This week's offer", "عرض الأسبوع")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", "Netherlands", "هولندا"), S("storage", WRAPPED)],
      img=["m:Swiss Cheese"],
      r=(4.6, 302), tags="deal",
      options=[weights("200g", "400g")],
      v=[V("200g", 149.99, was=159.99), V("400g", 289.99, was=309.99)],
      mods=[slicing()]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import axis  # noqa: E402

PRODUCTS += [
    P(id=2154, slug="gruyere",
      name=("Gruyère", "جبنة جرويير"),
      sub=("Rich, nutty Swiss cheese", "جبنة سويسري غنية وطعمها بندقي"),
      desc=("Aged Swiss-style Gruyère with a rich, nutty depth. Grate it over onion soup, melt it into a "
            "croque-monsieur with smoked turkey, or slice it for the cheese board.",
            "جبنة جرويير متعتقة على الطريقة السويسري، طعمها غني وبندقي. ابشرها على شوربة البصل، أو "
            "سيّحها في ساندوتش بالتيركي المدخن، أو قطّعها لطبق الأجبان."),
      hl=[("Aged and nutty", "متعتقة وطعمها بندقي"), ("Excellent melter", "بتسيح ممتاز")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", "Switzerland", "سويسرا"), S("storage", WRAPPED)],
      img=["m:Gruyère"],
      r=(4.6, 88), tags="new",
      options=[weights("200g", "400g")],
      v=[V("200g", 219.99), V("400g", 429.99)],
      mods=[slicing()]),

    P(id=2155, slug="blue-cheese",
      name=("Blue cheese", "جبنة ريكفور"),
      sub=("Creamy, tangy blue", "كريمية وطعمها حاد"),
      desc=("A creamy blue cheese with bold, tangy veins — what Egyptian delis call 'roquefort'. Crumble it "
            "into salads, melt it into a steak sauce, or pair it with walnuts and honey.",
            "جبنة زرقا كريمية بعروق طعمها قوي... اللي محلات الجبن في مصر بتسميها ريكفور. فتّتها في "
            "السلطة، أو سيّحها في صوص الستيك، أو قدّمها مع عين الجمل والعسل."),
      hl=[("Bold and tangy", "طعم قوي وحاد"), ("Great with honey and walnuts", "تحفة مع العسل وعين الجمل")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", "Denmark", "الدنمارك"), S("storage", WRAPPED)],
      img=["m:Stilton Cheese"],
      r=(4.5, 132), tags="",
      v=[V(None, 149.99, sku="200G")]),

    P(id=2156, slug="mozzarella-pearls",
      name=("Fresh mozzarella pearls", "موتزاريلا فريش كرات"),
      sub=("Bite-size balls in brine", "كرات صغيرة في مية مالحة"),
      desc=("Soft, milky mozzarella pearls packed in brine. Toss them with cherry tomatoes and basil, scatter "
            "them over pizza, or add them to pasta salad.",
            "كرات موتزاريلا طرية وطعمها لبني في مية مالحة. قلّبها مع الطماطم الشيري والريحان، أو "
            "وزّعها على البيتزا، أو حطها في سلطة المكرونة."),
      hl=[("Made with fresh cow's milk", "من لبن بقري طازة"), ("Eat within 3 days of opening", "تتاكل خلال 3 أيام من الفتح")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", EGYPT), S("storage", FRIDGE)],
      img=["m:Mozzarella Balls"],
      r=(4.4, 120), tags="",
      v=[V(None, 109.99, sku="200G")]),

    P(id=2157, slug="gouda",
      name=("Gouda", "جبنة جودة"),
      sub=("Mild and creamy Dutch cheese", "جبنة هولندي خفيفة وكريمية"),
      desc=("A mild, creamy young Gouda that everyone in the family likes. Slice it for sandwiches, cube it "
            "for snacks, or melt it over burgers.",
            "جبنة جودة صغيرة طعمها خفيف وكريمي، كل العيلة بتحبها. قطّعها للساندوتشات، أو مكعبات "
            "للتسالي، أو سيّحها على البرجر."),
      hl=[("Kid-friendly", "الولاد بيحبوها"), ("Sliced at the counter", "بتتقطع على الكاونتر")],
      specs=[S("milk", "Cow's milk", "لبن بقري"), S("origin", "Netherlands", "هولندا"), S("storage", WRAPPED)],
      img=["m:Gouda Cheese"],
      r=(4.5, 260), tags="",
      options=[weights("200g", "400g")],
      v=[V("200g", 129.99), V("400g", 249.99)],
      mods=[slicing()]),

    P(id=2158, slug="olives",
      name=("Olives, green or black", "زيتون أخضر وأسود"),
      sub=("From the deli counter, by the half kilo", "من ركن المخللات... بالنص كيلو"),
      desc=("Cured olives from the deli counter, sold by the half kilo. Green ones are crunchy and lemony; "
            "black ones are soft and mellow; or take a mix for the breakfast table.",
            "زيتون مخلل من ركن المخللات بالنص كيلو. الأخضر مقرمش وفيه نَفَس ليمون، والأسود طري وطعمه "
            "هادي، أو خد مشكل لسفرة الفطار."),
      hl=[("Sold by the half kilo", "بالنص كيلو"), ("Green, black or mixed", "أخضر أو أسود أو مشكل")],
      specs=[S("origin", "Siwa and Fayoum, Egypt", "سيوة والفيوم، مصر"), S("storage", FRIDGE)],
      img=["m:Olive"],
      r=(4.6, 690), tags="bestseller",
      options=[axis("type", [("green", L("Green", "أخضر")), ("black", L("Black", "أسود")),
                             ("mixed", L("Mixed", "مشكل"))])],
      v=[V("green", 49.99, sku="GRN-500G"), V("black", 59.99, sku="BLK-500G"), V("mixed", 54.99, sku="MIX-500G")]),
]

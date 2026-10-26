"""Oils & honey — ids 2401–2449, listed under breakfast & pantry (category 90 was folded into 140)."""

from common import DRY, EGYPT, L, P, S, V, axis, weights

CATEGORY = 140

PRODUCTS = [
    P(id=2401, slug="crystal-sunflower-oil", brand="crystal",
      name=("Crystal sunflower oil", "زيت كريستال عباد الشمس"),
      sub=("Light oil for frying and cooking", "زيت خفيف للقلي والطبيخ"),
      desc=("Crystal's pure sunflower oil — light in colour and taste, and happy at frying temperatures for "
            "ta'ameya, potatoes and fish. A staple in Egyptian kitchens.",
            "زيت كريستال عباد شمس نقي، لونه وطعمه خفاف، ويستحمل حرارة القلي للطعمية والبطاطس والسمك. "
            "أساسي في كل مطبخ مصري."),
      hl=[("100% sunflower", "عباد شمس 100%"), ("1.6 L bottle on offer", "إزازة الـ1.6 لتر عليها عرض")],
      specs=[S("origin", EGYPT), S("storage", DRY)],
      img=["d:cooking-oil-1"],
      r=(4.6, 1210), tags="express deal",
      options=[axis("size", [("800ml", L("800 ml", "800 مل")), ("1600ml", L("1.6 L", "1.6 لتر"))])],
      v=[V("800ml", 79.99), V("1600ml", 154.99, was=164.99)]),

    P(id=2402, slug="wadi-food-extra-virgin-olive-oil", brand="wadi-food",
      name=("Wadi Food extra virgin olive oil", "زيت زيتون بكر ممتاز وادي فود"),
      sub=("Egyptian olives, cold-pressed", "زيتون مصري معصور على البارد"),
      desc=("Cold-pressed extra virgin olive oil from Egyptian groves. Fruity and peppery — drizzle it over "
            "ful, hummus, salads and labneh.",
            "زيت زيتون بكر ممتاز معصور على البارد من مزارع زيتون مصرية. طعمه فاكهي وفيه لسعة فلفل... "
            "رشّه على الفول والحمص والسلطة واللبنة."),
      hl=[("Cold-pressed", "معصور على البارد"), ("Acidity below 0.8%", "حموضة أقل من 0.8%")],
      specs=[S("origin", EGYPT), S("type", "Extra virgin", "بكر ممتاز"), S("storage", DRY)],
      img=["m:Olive Oil"],
      r=(4.7, 420), tags="featured",
      options=[axis("size", [("500ml", L("500 ml", "500 مل")), ("1l", L("1 L", "1 لتر"))])],
      v=[V("500ml", 249.99), V("1l", 469.99)]),

    P(id=2403, slug="imtenan-clover-honey", brand="imtenan",
      name=("Imtenan clover honey", "عسل نحل برسيم امتنان"),
      sub=("Pure Egyptian bee honey", "عسل نحل مصري طبيعي"),
      desc=("Light, floral clover honey from Egyptian apiaries. Spread it on hot feteer, stir it into a cup "
            "of anise, or swap it for sugar in your morning yogurt.",
            "عسل برسيم فاتح وريحته ورد، من مناحل مصرية. ادهنه على الفطير السخن، أو حطه في كوباية "
            "الينسون، أو بدّل بيه السكر في الزبادي الصبح."),
      hl=[("100% natural honey", "عسل طبيعي 100%"), ("Clover blossom", "زهر البرسيم")],
      specs=[S("origin", EGYPT), S("storage", "Room temperature; crystallising is natural",
                                   "برة التلاجة، والتسكر طبيعي")],
      img=["m:Honey"],
      r=(4.7, 690), tags="bestseller",
      options=[weights("400g", "800g")],
      v=[V("400g", 189.99), V("800g", 359.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2404, slug="siwa-olive-oil",
      name=("Siwa extra virgin olive oil", "زيت زيتون سيوي بكر"),
      sub=("Single-oasis oil, grassy and bold", "زيت من واحة سيوة... طعمه أخضر وقوي"),
      desc=("Extra virgin olive oil pressed from olives grown in the Siwa oasis. Grassy, bold and a little "
            "peppery — the one to drizzle over ful, white cheese and tomatoes.",
            "زيت زيتون بكر معصور من زيتون واحة سيوة. طعمه أخضر وقوي وفيه لسعة... ده اللي ترشه على "
            "الفول والجبنة البيضا والطماطم."),
      hl=[("From the Siwa oasis", "من واحة سيوة"), ("Bold and peppery", "طعم قوي ولاسع")],
      specs=[S("origin", "Siwa, Egypt", "سيوة، مصر"), S("type", "Extra virgin", "بكر ممتاز"), S("storage", DRY)],
      img=["m:Extra Virgin Olive Oil"],
      r=(4.6, 133), tags="new",
      v=[V(None, 229.99, sku="500ML")]),
]

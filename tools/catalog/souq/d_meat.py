"""Meat & poultry (category 50, ids 2201–2249)."""

from common import HALAL, L, P, S, V, axis, mg, mo, weights

CATEGORY = 50

CHILLED = S("storage", "Fridge 2 days, freezer 3 months", "التلاجة يومين، والفريزر 3 شهور")


def packing():
    return mg("packing", "Packing", "التغليف", [
        mo("vacuum", "Vacuum-packed", "تغليف فاكيوم", 10),
        mo("portions", "Split into 250 g bags", "تقسيم أكياس ربع كيلو", 5)], selection="multi", max_=2)


PRODUCTS = [
    P(id=2201, slug="baladi-beef-kandouz",
      name=("Baladi beef (kandouz)", "لحمة كندوز بلدي"),
      sub=("Fresh daily — cubed, sliced or minced", "دبيحة يومية... مكعبات أو شرايح أو مفرومة"),
      desc=("Fresh, locally slaughtered young beef from our halal butcher, trimmed of excess fat. Choose "
            "cubes for stews and fatta, thin slices for escalope, mince, or a ready kofta mix with onion "
            "and parsley.",
            "لحمة كندوز بلدي دبيحة يومية من جزارنا الحلال، متنضفة من الدهن الزيادة. اختار مكعبات للطبيخ "
            "والفتة، أو شرايح رفيعة للبانيه، أو مفرومة، أو خلطة كفتة جاهزة بالبصل والبقدونس."),
      hl=[("Slaughtered daily", "دبيحة كل يوم"), ("Cut and minced to order", "بتتقطع وتتفرم على الطلب"),
          ("Vacuum packing on request", "تغليف فاكيوم لو تحب")],
      specs=[S("halal", HALAL), S("cut", "Leg (kandouz)", "فخدة كندوز"), CHILLED],
      img=["m:Goat Meat"],
      r=(4.6, 1460), tags="bestseller express",
      options=[weights("500g", "1kg", "2kg", loose=True)],
      v=[V("500g", 249.99), V("1kg", 489.99), V("2kg", 969.99)],
      mods=[mg("cut", "Cut", "التقطيع", [
          mo("cubes", "Cubes for stews and fatta", "مكعبات للطبيخ والفتة"),
          mo("escalope", "Thin slices for escalope", "شرايح رفيعة للبانيه", 10),
          mo("minced", "Minced", "مفرومة", 15),
          mo("kofta", "Kofta mix (onion, parsley, spices)", "خلطة كفتة بالبصل والبقدونس والتوابل", 35)],
          required=True), packing()]),

    P(id=2202, slug="minced-beef",
      name=("Minced beef", "لحمة مفرومة"),
      sub=("Minced fresh, around 20% fat", "مفرومة طازة... دهن حوالي 20%"),
      desc=("Beef minced to order with around 20% fat for juicy kofta, hawawshi and stuffed vegetables. "
            "Want it leaner? Pick lean and we'll use trimmed cuts only.",
            "لحمة مفرومة على الطلب فيها حوالي 20% دهن، علشان الكفتة والحواوشي والمحشي يطلعوا مليانين "
            "عصارة. عايزها أخف؟ اختار قليلة الدهن وهنفرملك من قطعيات متنضفة بس."),
      hl=[("Minced on the spot", "بتتفرم ساعتها"), ("Hawawshi seasoning on request", "تتبيلة حواوشي لو تحب")],
      specs=[S("halal", HALAL), S("fat", "About 20% (lean: 10%)", "حوالي 20% (قليلة الدهن: 10%)"), CHILLED],
      img=["m:Minced Beef"],
      r=(4.5, 1112), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 239.99), V("1kg", 469.99)],
      mods=[mg("fat", "Fat level", "نسبة الدهن", [
          mo("regular", "Regular (20%)", "عادي (20%)"),
          mo("lean", "Lean (10%)", "قليلة الدهن (10%)", 30)], required=True),
          mg("season", "Seasoning", "التتبيلة", [
              mo("hawawshi", "Hawawshi spice mix", "تتبيلة حواوشي", 20)])]),

    P(id=2203, slug="sirloin-steak",
      name=("Sirloin steak", "ستيك سيرلوين"),
      sub=("Thick-cut, for the grill or pan", "تقطيع تخين للشواية أو الطاسة"),
      desc=("Well-marbled sirloin cut into 2.5 cm steaks. Sear hard in a hot pan with butter and garlic, "
            "let it rest, then slice. Two steaks per 500 g pack.",
            "سيرلوين متعرّق دهن كويس ومتقطع ستيكات سُمكها 2.5 سم. شوّحه على طاسة نار عالية بالزبدة "
            "والتوم، سيبه يرتاح وبعدين قطّعه. قطعتين في عبوة النص كيلو."),
      hl=[("2.5 cm thick", "سُمك 2.5 سم"), ("Well marbled", "متعرّق دهن كويس")],
      specs=[S("halal", HALAL), S("cut", "Sirloin, 2.5 cm steaks", "سيرلوين، ستيكات 2.5 سم"), CHILLED],
      img=["m:Sirloin steak", "d:beef-steak-1"],
      r=(4.6, 302), tags="featured",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 379.99), V("1kg", 749.99)]),

    P(id=2204, slug="lamb-rib-chops",
      name=("Lamb rib chops", "ريش ضاني"),
      sub=("Tender chops for the grill", "ريش طرية للشوي"),
      desc=("Tender lamb rib chops from Barki sheep, cut with a little fat for flavour. Marinate with onion "
            "juice, rosemary and black pepper, then grill or bake until just pink.",
            "ريش ضاني طرية من خرفان برقي، متقطعة ومعاها شوية دهن للطعم. تبّلها بعصير البصل "
            "والروزماري والفلفل الإسود، واشويها أو ادخلها الفرن لحد ما تستوي على قدها."),
      hl=[("Barki lamb", "ضاني برقي"), ("Cut with a little fat", "متقطعة بشوية دهن")],
      specs=[S("halal", HALAL), S("cut", "Rib chops", "ريش"), CHILLED],
      img=["m:Lamb Loin Chops"],
      r=(4.7, 418), tags="featured",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 349.99), V("1kg", 689.99)],
      mods=[mg("marinade", "Marinade", "التتبيلة", [
          mo("kebab", "Kebab marinade (onion and spices)", "تتبيلة كباب (بصل وتوابل)", 25)])]),

    P(id=2205, slug="whole-fresh-chicken", brand="koki",
      name=("Whole fresh chicken", "فرخة كاملة طازة"),
      sub=("Chilled, never frozen — cut how you like", "مبردة مش مجمدة... بنقطعها زي ما تحب"),
      desc=("A whole fresh chicken from Koki, chilled and never frozen. Choose the size, then tell us how to "
            "cut it — whole for roasting, quarters, eight pieces for molokhia stock, or fully deboned.",
            "فرخة كاملة طازة من كوكي، مبردة مش مجمدة. اختار الوزن وقولّنا نقطعها إزاي... كاملة للفرن، "
            "أو 4 قطع، أو 8 قطع لشوربة الملوخية، أو مخلية من العضم خالص."),
      hl=[("Chilled, never frozen", "مبردة مش مجمدة"), ("Cutting and deboning service", "تقطيع وتخلية على الطلب")],
      specs=[S("halal", HALAL), S("storage", "Fridge 2 days", "التلاجة يومين"), S("origin", "Egypt", "مصر")],
      img=["d:chicken-meat-1"],
      r=(4.6, 1980), tags="bestseller express",
      options=[axis("weight", [("1200g", L("1.1–1.3 kg", "1.1–1.3 كيلو")),
                               ("1500g", L("1.4–1.6 kg", "1.4–1.6 كيلو"))])],
      v=[V("1200g", 179.99), V("1500g", 219.99)],
      mods=[mg("cut", "Cutting", "التقطيع", [
          mo("whole", "Whole, cleaned", "كاملة منضفة"),
          mo("quarters", "Quarters", "4 قطع"),
          mo("eighths", "8 pieces", "8 قطع", 5),
          mo("deboned", "Deboned", "مخلية من العضم", 35)], required=True),
          mg("skin", "Skin", "الجلد", [mo("skinless", "Skin removed", "من غير جلد", 10)])]),

    P(id=2206, slug="chicken-breast-fillets", brand="koki",
      name=("Chicken breast fillets", "صدور فراخ فيليه"),
      sub=("Boneless and skinless — pané ready on request", "من غير عضم ولا جلد... ونجهزها بانيه"),
      desc=("Plump boneless, skinless chicken breasts trimmed by hand. Leave them whole, have them butterflied "
            "and flattened for pané, cubed for shish, or marinated shish-tawook style.",
            "صدور فراخ مليانة من غير عضم ولا جلد، متنضفة بالإيد. سيبها زي ما هي، أو خليها تتفتح "
            "وتتخبط للبانيه، أو مكعبات للشيش، أو متبلة شيش طاووق."),
      hl=[("Hand-trimmed", "متنضفة بالإيد"), ("Shish tawook marinade on request", "تتبيلة شيش طاووق على الطلب")],
      specs=[S("halal", HALAL), CHILLED],
      img=["m:Chicken Thighs"],
      r=(4.6, 1655), tags="bestseller express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 129.99), V("1kg", 249.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("plain", "As they are", "زي ما هي"),
          mo("pane", "Butterflied for pané", "مفتوحة ومخبوطة للبانيه", 10),
          mo("cubes", "Cubed for shish", "مكعبات للشيش", 10),
          mo("tawook", "Shish tawook marinade", "متبلة شيش طاووق", 30)], required=True)]),

    P(id=2207, slug="chicken-drumsticks", brand="koki",
      name=("Chicken drumsticks", "دبابيس فراخ"),
      sub=("Juicy dark meat for the oven tray", "لحم غامق مليان عصارة لصينية الفرن"),
      desc=("Meaty chicken drumsticks, ideal for a potato tray bake, the air fryer or a spicy grilled "
            "marinade. The kids' favourite piece.",
            "دبابيس فراخ مليانة لحمة، مثالية لصينية البطاطس أو الإير فراير أو تتبيلة مشوية حرّاقة. "
            "الحتة المفضلة عند الولاد."),
      hl=[("Great in the air fryer", "تحفة في الإير فراير"), ("Family 2 kg bag", "كيس العيلة 2 كيلو")],
      specs=[S("halal", HALAL), CHILLED],
      img=["d:chicken-meat-2"],
      r=(4.4, 520), tags="",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 149.99), V("2kg", 289.99)]),

    P(id=2208, slug="chicken-livers",
      name=("Chicken livers", "كبدة فراخ"),
      sub=("Fresh livers for a quick sauté", "كبدة طازة لتشويحة سريعة"),
      desc=("Fresh, cleaned chicken livers. Sauté them hot with garlic, green pepper and a squeeze of baladi "
            "lemon for the classic Alexandrian sandwich filling.",
            "كبدة فراخ طازة ومتنضفة. شوّحها على نار عالية بالتوم والفلفل الأخضر وعصرة ليمون بلدي، "
            "لحشوة ساندوتش الكبدة الإسكندراني."),
      hl=[("Cleaned and trimmed", "متنضفة ومتشفية"), ("Ready in 10 minutes", "جاهزة في 10 دقايق")],
      specs=[S("halal", HALAL), S("storage", "Fridge 1 day, freezer 2 months", "التلاجة يوم، والفريزر شهرين")],
      img=["m:Chicken Liver"],
      r=(4.4, 388), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 69.99), V("1kg", 134.99)]),

    P(id=2209, slug="marinated-beef-shawarma", brand="souq",
      name=("Marinated beef shawarma", "شاورما لحمة متبلة"),
      sub=("Thin-sliced and marinated, ready for the pan", "شرايح رفيعة متبلة وجاهزة للطاسة"),
      desc=("Thinly sliced beef marinated overnight in our shawarma spice blend, vinegar and garlic. Cook it "
            "hot and fast in a pan, then pile it into shami bread with tahini and pickles.",
            "لحمة متقطعة شرايح رفيعة ومتبلة من الليلة اللي فاتت ببهارات الشاورما والخل والتوم. "
            "سوّيها على نار عالية بسرعة، وحطها في عيش شامي بالطحينة والمخلل."),
      hl=[("Marinated overnight", "متبلة من الليلة اللي فاتت"), ("Cooks in 8 minutes", "تستوي في 8 دقايق")],
      specs=[S("halal", HALAL), S("ingredients", "Beef, vinegar, garlic, shawarma spices, oil",
                                  "لحمة، خل، توم، بهارات شاورما، زيت"), CHILLED],
      img=["m:Doner Meat"],
      r=(4.5, 610), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 289.99), V("1kg", 569.99)]),

    P(id=2210, slug="oxtail",
      name=("Oxtail", "ديل عجالي"),
      sub=("For slow-cooked stews and soup", "للطواجن والشوربة على نار هادية"),
      desc=("Meaty oxtail cut into rounds. Braise it low and slow with onions and tomatoes until it falls "
            "off the bone, or simmer it into a rich, gelatinous soup.",
            "ديل عجالي متقطع حلقات مليانة لحمة... اطبخه على نار هادية مع البصل والطماطم لحد ما يسيب "
            "العضم، أو اعمل منه شوربة تقيلة."),
      hl=[("Cut into rounds", "متقطع حلقات"), ("Best slow-cooked", "أحلى على نار هادية")],
      specs=[S("halal", HALAL), CHILLED],
      img=["m:Oxtail"],
      r=(4.5, 144), tags="",
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 389.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2211, slug="lamb-shoulder",
      name=("Lamb shoulder", "كتف ضاني"),
      sub=("For the oven or a pot of fatta", "للفرن أو لحلة الفتة"),
      desc=("Bone-in lamb shoulder from Barki sheep, rich and tender after a slow roast. Roast it whole, "
            "or cut it into chunks for fatta and stews.",
            "كتف ضاني برقي بالعضم، طري وغني بعد طبخ هادي في الفرن. اطبخه صحيح في الفرن، أو قطّعه حتت "
            "للفتة والطواجن."),
      hl=[("Bone-in for flavour", "بالعضم علشان الطعم"), ("Barki lamb", "ضاني برقي")],
      specs=[S("halal", HALAL), S("cut", "Shoulder, bone-in", "كتف بالعضم"), CHILLED],
      img=["m:Lamb Shoulder"],
      r=(4.6, 233), tags="",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 599.99), V("2kg", 1189.99)],
      mods=[mg("cut", "Cut", "التقطيع", [
          mo("whole", "Whole, for roasting", "صحيح للفرن"),
          mo("chunks", "Chunks for fatta and stews", "حتت للفتة والطبيخ", 10)], required=True), packing()]),

    P(id=2212, slug="beef-shank",
      name=("Beef shank (mouza)", "موزة بقري"),
      sub=("The cut for fatta and slow roasts", "قطعية الفتة والفرن على نار هادية"),
      desc=("Meaty beef shank with its marrow bone — slow-cook it until it falls apart for fatta, or roast "
            "it whole with onions and spices for a family feast.",
            "موزة بقري مليانة لحمة وبالنخاع... اطبخها على نار هادية لحد ما تدوب للفتة، أو اشويها صحيحة "
            "بالبصل والتوابل لعزومة العيلة."),
      hl=[("With marrow bone", "بالنخاع"), ("Best slow-cooked", "أحلى على نار هادية")],
      specs=[S("halal", HALAL), S("cut", "Shank (mouza)", "موزة"), CHILLED],
      img=["m:Beef Shin"],
      r=(4.7, 488), tags="featured",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 469.99), V("2kg", 929.99)],
      mods=[packing()]),

    P(id=2213, slug="veal-escalope",
      name=("Veal escalope", "بتلو بانيه"),
      sub=("Thin, tender slices for pané", "شرايح رفيعة وطرية للبانيه"),
      desc=("Tender milk-fed veal sliced thin and flattened for escalope pané. Bread and fry for three "
            "minutes a side, or cook it alla Milanese with a squeeze of lemon.",
            "بتلو طري متقطع شرايح رفيعة ومخبوط للبانيه. غطّيه بالبقسماط واقليه 3 دقايق كل ناحية، أو "
            "اعمله على الطريقة الميلانيز بعصرة ليمون."),
      hl=[("Sliced and flattened", "متقطع ومخبوط"), ("Pan-ready in 6 minutes", "يستوي في 6 دقايق")],
      specs=[S("halal", HALAL), S("cut", "Leg, sliced thin", "فخدة شرايح رفيعة"), CHILLED],
      img=["m:Veal"],
      r=(4.6, 302), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 329.99), V("1kg", 649.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("plain", "Plain slices", "شرايح سادة"),
          mo("breaded", "Seasoned and breaded", "متبّلة ومتغطية بقسماط", 25)], required=True)]),

    P(id=2214, slug="minced-lamb",
      name=("Minced lamb", "ضاني مفروم"),
      sub=("For kofta and kebab halla", "للكفتة الضاني وكباب الحلة"),
      desc=("Lamb minced to order with a little fat for juicy kofta skewers, stuffed vine leaves or a pot of "
            "kebab halla.",
            "ضاني مفروم على الطلب ومعاه شوية دهن، لأسياخ كفتة مليانة عصارة أو محشي ورق العنب أو حلة "
            "كباب حلة."),
      hl=[("Minced fresh", "بيتفرم طازة"), ("About 15% fat", "دهن حوالي 15%")],
      specs=[S("halal", HALAL), CHILLED],
      img=["m:Lamb Mince"],
      r=(4.5, 176), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 309.99), V("1kg", 609.99)],
      mods=[mg("season", "Seasoning", "التتبيلة", [
          mo("kofta", "Kofta mix (onion, parsley, spices)", "خلطة كفتة (بصل وبقدونس وتوابل)", 25)])]),

    P(id=2215, slug="muscovy-duck",
      name=("Muscovy duck", "بط مسكوفي"),
      sub=("Farm duck, cleaned and ready", "بط مزارع متنضف وجاهز"),
      desc=("A whole Muscovy duck from a Sharqia farm, cleaned and singed. Boil it with mastic and cardamom, "
            "then roast until crisp — the classic Friday duck with rice and orzo.",
            "بطة مسكوفي كاملة من مزرعة في الشرقية، متنضفة ومتشوّطة. اسلقها بالمستكة والحبهان وبعدين "
            "حمّرها في الفرن... بطة الجمعة الكلاسيكية مع الرز بالشعرية."),
      hl=[("Cleaned and singed", "متنضفة ومتشوّطة"), ("From a Sharqia farm", "من مزرعة في الشرقية")],
      specs=[S("halal", HALAL), S("origin", "Sharqia, Egypt", "الشرقية، مصر"), CHILLED],
      img=["m:Duck"],
      r=(4.5, 154), tags="",
      options=[axis("weight", [("2000g", L("2–2.5 kg", "2–2.5 كيلو")), ("2500g", L("2.5–3 kg", "2.5–3 كيلو"))])],
      v=[V("2000g", 499.99), V("2500g", 609.99, stock=0)],
      mods=[mg("cut", "Cutting", "التقطيع", [
          mo("whole", "Whole", "كاملة"),
          mo("quarters", "Quarters", "4 قطع")], required=True)]),

    P(id=2216, slug="rotisserie-chicken", brand="souq",
      name=("Rotisserie chicken", "فرخة مشوية"),
      sub=("Hot from the grill, with sides if you like", "سخنة من الشواية... ومعاها اللي تحبه"),
      desc=("A whole chicken marinated in garlic, lemon and spices, then slow-roasted on the spit. Add rice, "
            "toumeya and pickles for a complete family dinner.",
            "فرخة كاملة متبلة بالتوم والليمون والتوابل ومشوية على السيخ على نار هادية. زوّد رز وتومية "
            "ومخلل وخلّي عشا العيلة كامل."),
      hl=[("Ready to eat", "جاهزة للأكل"), ("Delivered hot", "بتوصل سخنة"), ("Add sides", "زوّد أطباق جانبية")],
      specs=[S("halal", HALAL), S("storage", "Eat within 2 hours or refrigerate", "تتاكل خلال ساعتين أو في التلاجة")],
      img=["m:Chicken"],
      r=(4.7, 1320), tags="bestseller express",
      options=[axis("size", [("whole", L("Whole chicken", "فرخة كاملة")), ("half", L("Half chicken", "نص فرخة"))])],
      v=[V("whole", 249.99), V("half", 129.99)],
      mods=[mg("sides", "Sides", "إضافات", [
          mo("rice", "Rice with vermicelli", "رز بالشعرية", 35),
          mo("toum", "Toumeya garlic sauce", "تومية", 10),
          mo("pickles", "Pickles", "مخلل", 10),
          mo("bread", "Shami bread, 3 loaves", "عيش شامي 3 أرغفة", 12)], selection="multi", max_=4)]),

    P(id=2217, slug="beef-fillet",
      name=("Beef fillet", "فلتو بقري"),
      sub=("The tenderest cut, trimmed", "أطرى قطعية... متنضفة"),
      desc=("Whole beef tenderloin, trimmed of silverskin. Roast it in one piece, or slice it into "
            "medallions or thin strips for stroganoff.",
            "فلتو بقري متنضف من الغشا. اشويه حتة واحدة، أو قطّعه ميداليون أو شرايح رفيعة "
            "للاستروجانوف."),
      hl=[("Trimmed by hand", "متنضف بالإيد"), ("Whole tenderloin", "فلتو كامل")],
      specs=[S("halal", HALAL), S("cut", "Tenderloin", "فلتو"), CHILLED],
      img=["m:Beef Fillet"],
      r=(4.8, 211), tags="featured",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 459.99), V("1kg", 899.99)],
      mods=[mg("cut", "Cut", "التقطيع", [
          mo("whole", "Whole piece", "حتة واحدة"),
          mo("medallions", "Medallions", "ميداليون"),
          mo("strips", "Strips for stroganoff", "شرايح للاستروجانوف")], required=True)]),

    P(id=2218, slug="leg-of-lamb",
      name=("Leg of lamb", "فخدة ضاني"),
      sub=("For Eid and Friday roasts", "لعزومات العيد والجمعة"),
      desc=("A whole leg of Barki lamb, tied for even roasting. Rub it with garlic and rosemary and roast "
            "it low and slow.",
            "فخدة ضاني برقي كاملة ومربوطة علشان تستوي بالتساوي. ادعكها بالتوم والروزماري وادخلها "
            "الفرن على نار هادية."),
      hl=[("Tied for roasting", "مربوطة للفرن"), ("Bone-in for flavour", "بالعضم علشان الطعم")],
      specs=[S("halal", HALAL), S("cut", "Leg, bone-in", "فخدة بالعضم"), CHILLED],
      img=["m:Lamb"],
      r=(4.7, 129), tags="",
      options=[axis("weight", [("2000g", L("About 2 kg", "حوالي 2 كيلو")), ("3000g", L("About 3 kg", "حوالي 3 كيلو"))])],
      v=[V("2000g", 1179.99), V("3000g", 1749.99)],
      mods=[mg("marinade", "Marinade", "التتبيلة", [
          mo("garlic", "Garlic and rosemary, roast-ready", "توم وروزماري جاهزة للفرن", 40)])]),
]

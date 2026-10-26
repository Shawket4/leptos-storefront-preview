"""Fish & seafood (category 60, ids 2251–2299)."""

from common import L, P, S, V, axis, mg, mo, weights

CATEGORY = 60

COOK_SOON = S("storage", "Fridge; cook within 24 hours", "في التلاجة، ويتطبخ خلال 24 ساعة")


def cleaning():
    return mg("cleaning", "Cleaning", "التنضيف", [
        mo("whole", "Whole, as caught", "صحيح من غير تنضيف"),
        mo("cleaned", "Scaled and gutted", "منضف ومقشّر"),
        mo("butterfly", "Butterflied for the grill", "مفتوح فراشة للشوي", 15),
        mo("fillet", "Filleted", "فيليه", 30)], required=True)


def cooking():
    return mg("cooking", "Ready to cook", "جاهز للطبخ", [
        mo("bran", "Seasoned and bran-coated for the grill", "متبّل ومتغطي ردّة للشوي", 20),
        mo("fry", "Seasoned and floured for frying", "متبّل ومدقّق للقلي", 15),
        mo("sangari", "Sangari marinade (tomato, pepper, garlic)", "تتبيلة سنجاري (طماطم وفلفل وتوم)", 25)])


PRODUCTS = [
    P(id=2251, slug="sea-bass",
      name=("Sea bass", "قاروص"),
      sub=("Whole Mediterranean sea bass", "قاروص صحيح من البحر المتوسط"),
      desc=("Firm, white-fleshed sea bass from the Mediterranean coast, two to three fish per kilo. Have it "
            "scaled and gutted, butterflied for the grill or filleted — and add our bran coating for "
            "proper Alexandrian-style grilled fish.",
            "قاروص لحمه أبيض ومتماسك من ساحل البحر المتوسط، من 2 لـ 3 سمكات في الكيلو. اطلبه منضف "
            "ومقشّر، أو مفتوح فراشة للشوي، أو فيليه... وزوّد الردّة علشان تشويه على الطريقة "
            "الإسكندراني."),
      hl=[("2–3 fish per kg", "2–3 سمكات في الكيلو"), ("Cleaned free of charge", "التنضيف مجانًا"),
          ("Grill-ready options", "متاح متبّل وجاهز للشوي")],
      specs=[S("catch", "Mediterranean, Egypt", "البحر المتوسط، مصر"), COOK_SOON],
      img=["m:barramundi"],
      r=(4.6, 488), tags="featured",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 379.99), V("2kg", 749.99)],
      mods=[cleaning(), cooking()]),

    P(id=2252, slug="sea-bream",
      name=("Sea bream", "دنيس"),
      sub=("Sweet, delicate and perfect grilled", "طعمه حلو ورقيق... ملك الشوي"),
      desc=("Silver-pink sea bream with sweet, delicate flesh — Egypt's favourite fish for the grill. Choose "
            "the cleaning and prep, and it arrives ready for the pan, the oven or the charcoal.",
            "دنيس فضي مايل للوردي، لحمه حلو ورقيق... السمك المفضل للشوي في مصر. اختار التنضيف "
            "والتتبيلة، ويوصلك جاهز للطاسة أو الفرن أو الفحم."),
      hl=[("3–4 fish per kg", "3–4 سمكات في الكيلو"), ("Our best-selling fish", "أكتر سمك بيتطلب عندنا")],
      specs=[S("catch", "Mediterranean, Egypt", "البحر المتوسط، مصر"), COOK_SOON],
      img=["m:Snapper Fish"],
      r=(4.7, 702), tags="bestseller",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 329.99), V("2kg", 649.99)],
      mods=[cleaning(), cooking()]),

    P(id=2253, slug="red-snapper",
      name=("Red snapper", "مرجان أحمر"),
      sub=("Red Sea catch, firm and sweet", "صيد البحر الأحمر... لحمه متماسك وحلو"),
      desc=("Bright-red snapper from the Red Sea with firm, sweet flesh that stands up to a sangari tray "
            "with tomatoes, peppers and garlic. We clean and scale it for free.",
            "مرجان أحمر من البحر الأحمر، لحمه متماسك وطعمه حلو، يستحمل يتعمل صينية سنجاري بالطماطم "
            "والفلفل والتوم. وبننضفه ونقشّره ببلاش."),
      hl=[("Red Sea catch", "صيد البحر الأحمر"), ("Great for sangari trays", "ممتاز لصينية السنجاري")],
      specs=[S("catch", "Red Sea, Egypt", "البحر الأحمر، مصر"), COOK_SOON],
      img=["m:Red Snapper"],
      r=(4.5, 233), tags="",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 359.99), V("2kg", 709.99)],
      mods=[cleaning(), cooking()]),

    P(id=2254, slug="norwegian-salmon-fillet",
      name=("Norwegian salmon fillet", "فيليه سلمون نرويجي"),
      sub=("Skin-on and pin-boned", "بالجلد ومن غير شوك"),
      desc=("Skin-on Norwegian salmon fillet, pin-boned and portioned. Pan-sear it skin-side down until "
            "crisp, bake it with lemon and dill, or cube it for a poke bowl.",
            "فيليه سلمون نرويجي بالجلد ومن غير شوك ومتقسم. حمّره على الطاسة من ناحية الجلد لحد ما "
            "يقرمش، أو ادخله الفرن بالليمون والشبت، أو قطّعه مكعبات للبوكي."),
      hl=[("Pin-boned", "من غير شوك"), ("Flown in chilled", "بيوصل مبرّد بالطيارة")],
      specs=[S("origin", "Norway", "النرويج"), COOK_SOON],
      img=["m:Salmon"],
      r=(4.7, 298), tags="featured",
      options=[weights("250g", "500g")],
      v=[V("250g", 359.99), V("500g", 699.99)]),

    P(id=2255, slug="fresh-shrimp",
      name=("Fresh shrimp", "جمبري طازة"),
      sub=("Choose your size; we peel and devein", "اختار المقاس... وإحنا نقشّر وننضف"),
      desc=("Fresh shrimp sold by size — medium for rice and soups, large for grilling and jumbo for the "
            "showstopper platter. Order them peeled and deveined to skip the fiddly work.",
            "جمبري طازة بالمقاس: وسط للرز والشوربة، كبير للشوي، وجامبو لصينية تفرّح. اطلبه مقشّر "
            "ومنضف وريّح نفسك من الشغل الرخم."),
      hl=[("Priced per kg", "السعر للكيلو"), ("Peeling and deveining service", "تقشير وتنضيف على الطلب")],
      specs=[S("catch", "Mediterranean and Red Sea", "البحر المتوسط والأحمر"), S("sold", "Per kg", "بالكيلو"),
             COOK_SOON],
      img=["m:Shrimp"],
      r=(4.6, 556), tags="featured",
      options=[axis("size", [("medium", L("Medium", "وسط")), ("large", L("Large", "كبير")),
                             ("jumbo", L("Jumbo", "جامبو"))])],
      v=[V("medium", 479.99, sku="MED-1KG"), V("large", 649.99, sku="LRG-1KG"),
         V("jumbo", 849.99, sku="JMB-1KG", stock=0)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "Whole", "صحيح"),
          mo("peeled", "Peeled and deveined", "مقشّر ومنضف", 60),
          mo("butterfly", "Butterflied for the grill", "مفتوح فراشة للشوي", 40)], required=True)]),

    P(id=2256, slug="sardines",
      name=("Sardines", "سردين"),
      sub=("Fresh sardines for frying", "سردين طازة للقلي"),
      desc=("Small fresh sardines, a Mediterranean favourite: gut them, dust with cumin-spiced flour and fry "
            "until crisp, then serve with tahini and baladi bread.",
            "سردين طازة صغير من البحر المتوسط... ينضف ويتدقق بدقيق متبل بالكمون ويتقلي لحد ما "
            "يقرمش، ويتقدم بالطحينة والعيش البلدي."),
      hl=[("Budget-friendly", "سعره في المتناول"), ("Rich in omega-3", "غني بأوميجا 3")],
      specs=[S("catch", "Mediterranean, Egypt", "البحر المتوسط، مصر"), COOK_SOON],
      img=["m:Sardines"],
      r=(4.3, 211), tags="",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 89.99), V("2kg", 174.99)],
      mods=[mg("cleaning", "Cleaning", "التنضيف", [
          mo("whole", "Whole", "صحيح"),
          mo("cleaned", "Gutted and cleaned", "منضف", 10)], required=True)]),

    P(id=2257, slug="tilapia-fillet",
      name=("Tilapia fillet", "فيليه بلطي"),
      sub=("Boneless Nile tilapia, ready to cook", "بلطي من غير شوك... جاهز للطبخ"),
      desc=("Boneless, skinless tilapia fillets — the mild everyday fish of the Nile. Coat in breadcrumbs for "
            "crispy fish fingers, bake in a sangari tray or simmer in a tomato and cumin sauce.",
            "فيليه بلطي من غير شوك ولا جلد... سمك النيل اللي طعمه خفيف وبيتاكل كل يوم. غطّيه بالبقسماط "
            "لأصابع سمك مقرمشة، أو اعمله صينية سنجاري، أو اطبخه في صلصة طماطم بالكمون."),
      hl=[("No bones, no skin", "من غير شوك ولا جلد"), ("Kid-friendly", "الولاد بيحبوه")],
      specs=[S("origin", "Kafr El Sheikh, Egypt", "كفر الشيخ، مصر"),
             S("storage", "Fridge 1 day, freezer 3 months", "التلاجة يوم، والفريزر 3 شهور")],
      img=["m:White Fish Fillets"],
      r=(4.4, 830), tags="express deal",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 114.99), V("1kg", 219.99, was=239.99)]),

    P(id=2258, slug="mackerel",
      name=("Mackerel", "ماكريل"),
      sub=("Rich, oily fish for the grill", "سمك دهني غني للشوي"),
      desc=("Whole mackerel with rich, omega-3-packed flesh. Grill it over charcoal with a cumin-and-garlic "
            "rub, or bake it with lemon slices.",
            "ماكريل صحيح لحمه غني بأوميجا 3. اشويه على الفحم بتتبيلة كمون وتوم، أو ادخله الفرن "
            "بشرايح الليمون."),
      hl=[("Rich in omega-3", "غني بأوميجا 3"), ("Best grilled", "أحلى مشوي")],
      specs=[S("catch", "Mediterranean", "البحر المتوسط"), COOK_SOON],
      img=["m:Mackerel"],
      r=(4.3, 142), tags="",
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 149.99)],
      mods=[cleaning(), cooking()]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
from common import FREEZER, FRIDGE  # noqa: E402

PRODUCTS += [
    P(id=2259, slug="smoked-salmon",
      name=("Smoked salmon", "سلمون مدخن"),
      sub=("Thin slices, ready to serve", "شرايح رفيعة جاهزة للتقديم"),
      desc=("Cold-smoked salmon in thin, silky slices. Serve it on toast with cream cheese and dill, fold it "
            "into scrambled eggs, or add it to a Friday brunch platter.",
            "سلمون مدخن على البارد في شرايح رفيعة وناعمة. قدّمه على التوست بالجبنة الكريمي والشبت، "
            "أو مع البيض المخفوق، أو في طبق فطار الجمعة."),
      hl=[("Ready to eat", "جاهز للأكل"), ("Sliced thin", "شرايح رفيعة")],
      specs=[S("origin", "Norway", "النرويج"), S("storage", FRIDGE)],
      img=["m:Smoked Salmon"],
      r=(4.6, 210), tags="",
      options=[weights("100g", "200g")],
      v=[V("100g", 189.99), V("200g", 369.99)]),

    P(id=2260, slug="tuna-steak",
      name=("Fresh tuna loin", "تونة فريش"),
      sub=("Red Sea tuna for steaks and tartare", "تونة البحر الأحمر للستيك والتارتار"),
      desc=("A deep-red loin of fresh Red Sea tuna. Cut it into thick steaks and sear for a minute a side, "
            "or slice it raw for tartare.",
            "حتة تونة فريش لونها أحمر غامق من البحر الأحمر. قطّعها ستيكات تخينة وشوّحها دقيقة كل ناحية، "
            "أو قطّعها ني للتارتار."),
      hl=[("Sashimi-grade freshness", "طازة جدًا"), ("Deep-red, firm flesh", "لحمها أحمر غامق ومتماسك")],
      specs=[S("catch", "Red Sea, Egypt", "البحر الأحمر، مصر"), COOK_SOON],
      img=["m:Tuna"],
      r=(4.5, 66), tags="new",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 349.99), V("1kg", 689.99)]),

    P(id=2261, slug="hake-fillet",
      name=("Hake fillet", "فيليه ميرلوزا"),
      sub=("Mild white fillets, frozen at sea", "فيليه أبيض خفيف متجمد في البحر"),
      desc=("Boneless hake fillets, frozen at sea to lock in freshness. Mild and flaky — perfect for fish "
            "fingers, a sangari tray or fish with tahini sauce.",
            "فيليه ميرلوزا من غير شوك، متجمد في البحر علشان يحافظ على طزاجته. طعمه خفيف وبيتفتت... "
            "مثالي لأصابع السمك أو صينية السنجاري أو السمك بالطحينة."),
      hl=[("Boneless", "من غير شوك"), ("Frozen at sea", "متجمد في البحر")],
      specs=[S("origin", "Imported", "مستورد"), S("storage", FREEZER)],
      img=["m:Cod"],
      r=(4.3, 288), tags="express",
      options=[weights("1kg", "2kg")],
      v=[V("1kg", 239.99), V("2kg", 469.99)]),

    P(id=2262, slug="peeled-shrimp-frozen",
      name=("Peeled shrimp, frozen", "جمبري مقشر مجمد"),
      sub=("Cleaned, deveined, ready in minutes", "منضف ومقشر... يستوي في دقايق"),
      desc=("Medium shrimp, peeled, deveined and individually frozen. Take out just what you need for "
            "shrimp rice, pasta or a quick garlic-butter sauté.",
            "جمبري وسط مقشر ومنضف ومتجمد حباية حباية. طلّع اللي محتاجه بس لرز الجمبري أو المكرونة أو "
            "تشويحة زبدة وتوم سريعة."),
      hl=[("Individually frozen", "متجمد حباية حباية"), ("No peeling needed", "من غير تقشير")],
      specs=[S("origin", "Imported", "مستورد"), S("storage", FREEZER)],
      img=["m:Prawns"],
      r=(4.5, 377), tags="deal",
      options=[weights("500g", "1kg")],
      v=[V("500g", 299.99, was=329.99), V("1kg", 579.99, was=639.99)]),

    P(id=2263, slug="sea-bass-fillet",
      name=("Sea bass fillet", "فيليه قاروص"),
      sub=("Skin-on fillets, pin-boned", "فيليه بالجلد ومن غير شوك"),
      desc=("Skin-on sea bass fillets, pin-boned by our fishmonger. Pan-fry them skin-side down until crisp "
            "and finish with lemon butter.",
            "فيليه قاروص بالجلد، والسماك بتاعنا شال الشوك. حمّره على الطاسة من ناحية الجلد لحد ما "
            "يقرمش وكمّله بزبدة الليمون."),
      hl=[("Pin-boned", "من غير شوك"), ("Crisp skin in 5 minutes", "جلد مقرمش في 5 دقايق")],
      specs=[S("catch", "Mediterranean, Egypt", "البحر المتوسط، مصر"), COOK_SOON],
      img=["m:Sea Bass Fillets"],
      r=(4.6, 198), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 279.99), V("1kg", 549.99)]),

    P(id=2264, slug="rainbow-trout",
      name=("Rainbow trout", "سمك تراوت"),
      sub=("Whole trout for the oven", "تراوت صحيح للفرن"),
      desc=("Whole rainbow trout with delicate pink flesh. Stuff it with lemon, garlic and dill and bake it "
            "in foil for 20 minutes.",
            "تراوت صحيح لحمه وردي ورقيق. احشيه ليمون وتوم وشبت ولفّه في ورق فويل وادخله الفرن 20 دقيقة."),
      hl=[("1–2 fish per kg", "1–2 سمكة في الكيلو"), ("Mild and delicate", "طعمه خفيف ورقيق")],
      specs=[S("origin", "Turkey", "تركيا"), COOK_SOON],
      img=["m:Trout"],
      r=(4.3, 54), tags="",
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 399.99)],
      mods=[cleaning()]),
]

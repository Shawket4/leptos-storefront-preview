"""Rice, grains & pulses (category 70, ids 2301–2349)."""

from common import DRY, EGYPT, L, P, S, V, weights

CATEGORY = 70

PRODUCTS = [
    P(id=2301, slug="el-doha-egyptian-rice", brand="el-doha",
      name=("El Doha Egyptian rice", "أرز الضحى مصري"),
      sub=("Medium-grain rice for mahshi and every day", "رز مصري للمحشي ولكل يوم"),
      desc=("Egyptian medium-grain rice that cooks up soft and slightly sticky — exactly what you want for "
            "mahshi, rice with vermicelli and creamy roz bel laban. Rinse, soak for 15 minutes and cook one "
            "cup of rice to one and a quarter cups of water.",
            "رز مصري حبته متوسطة، بيستوي طري ولازق خفيف... بالظبط اللي محتاجه للمحشي والرز بالشعرية "
            "والرز باللبن. اغسله وانقعه ربع ساعة، واطبخ كوباية رز على كوباية وربع مية."),
      hl=[("Sorted and cleaned grain", "حبة متنقية ونضيفة"), ("5 kg bag saves more", "شيكارة الـ5 كيلو أوفر")],
      specs=[S("type", "Egyptian medium grain", "مصري حبة متوسطة"), S("origin", EGYPT),
             S("cooking", "18–20 minutes", "18–20 دقيقة"), S("storage", DRY)],
      img=["d:rice-1"],
      r=(4.7, 2410), tags="bestseller express deal",
      options=[weights("1kg", "5kg")],
      v=[V("1kg", 42.99), V("5kg", 204.99, was=214.95)]),

    P(id=2302, slug="black-eyed-beans", brand="souq",
      name=("Black-eyed beans", "لوبيا ناشفة"),
      sub=("For the classic lobia stew", "لطاجن اللوبيا بالصلصة"),
      desc=("Dried black-eyed beans, cleaned and sorted. Soak overnight and cook with onions, tomato sauce "
            "and a little meat for the Egyptian lobia stew served over white rice.",
            "لوبيا ناشفة متنقية ومتنضفة. انقعها من بالليل واطبخها بالبصل والصلصة وحتة لحمة، لطبق "
            "اللوبيا المصري اللي بيتاكل مع الرز الأبيض."),
      hl=[("Hand-sorted, no stones", "متنقية بالإيد ومن غير زلط"), ("Soak overnight", "تتنقع من بالليل")],
      specs=[S("origin", EGYPT), S("cooking", "45 minutes after soaking", "45 دقيقة بعد النقع"), S("storage", DRY)],
      img=["m:Black Eyed Peas"],
      r=(4.4, 198), tags="",
      options=[weights("500g", "1kg")],
      v=[V("500g", 44.99), V("1kg", 84.99)]),

    P(id=2303, slug="belila-wheat", brand="souq",
      name=("Whole wheat for belila", "قمح بليلة"),
      sub=("Cleaned wheat berries for belila", "قمح منقي للبليلة"),
      desc=("Cleaned whole wheat berries for belila — boil until tender, then serve warm with hot milk, "
            "sugar, raisins and coconut. Also the base for Ashura pudding.",
            "قمح منقي ومغسول للبليلة... يتسلق لحد ما يطرى ويتقدم سخن باللبن والسكر والزبيب وجوز الهند. "
            "وكمان أساس حلويات عاشوراء."),
      hl=[("Winter breakfast favourite", "فطار الشتا المفضل"), ("Cleaned and sorted", "متنقي ومتنضف")],
      specs=[S("origin", EGYPT), S("cooking", "60–90 minutes", "60–90 دقيقة"), S("storage", DRY)],
      img=["m:Whole Wheat"],
      r=(4.5, 157), tags="",
      options=[weights("500g", "1kg")],
      v=[V("500g", 19.99), V("1kg", 36.99)]),

    P(id=2304, slug="regina-spaghetti", brand="regina",
      name=("Regina spaghetti", "مكرونة ريجينا اسباجتي"),
      sub=("Durum semolina, 10 minutes to al dente", "سميد قمح صلب... 10 دقايق وتبقى مظبوطة"),
      desc=("Long spaghetti from Regina, one of Egypt's best-loved pasta makers, made with durum wheat "
            "semolina so it stays firm. Toss with a quick tomato sauce, or with minced beef for a "
            "family-size Bolognese.",
            "اسباجتي ريجينا، من أشهر مصانع المكرونة في مصر، معمولة من سميد القمح الصلب علشان تفضل "
            "ماسكة نفسها. قلّبها مع صلصة طماطم سريعة أو باللحمة المفرومة لطبق بولونيز يكفي العيلة."),
      hl=[("100% durum semolina", "سميد قمح صلب 100%"), ("Cooks in 10 minutes", "تستوي في 10 دقايق")],
      specs=[S("ingredients", "Durum wheat semolina, water", "سميد قمح صلب، مية"),
             S("cooking", "10 minutes", "10 دقايق"), S("origin", EGYPT)],
      img=["m:Spaghetti"],
      r=(4.5, 870), tags="express",
      options=[weights("400g", "1kg")],
      v=[V("400g", 24.99), V("1kg", 54.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2305, slug="regina-farfalle", brand="regina",
      name=("Regina farfalle", "مكرونة فيونكة ريجينا"),
      sub=("Bow ties for pasta salad and béchamel", "فيونكة لسلطة المكرونة والبشاميل"),
      desc=("Regina's bow-tie pasta, made from durum semolina. The ridged wings hold creamy sauces and "
            "dressings, so it's a favourite for cold pasta salads.",
            "مكرونة فيونكة من ريجينا معمولة من سميد القمح الصلب. أجنحتها بتمسك الصوص والتتبيلة، "
            "علشان كده هي المفضلة لسلطة المكرونة الباردة."),
      hl=[("Durum semolina", "سميد قمح صلب"), ("Cooks in 11 minutes", "تستوي في 11 دقيقة")],
      specs=[S("ingredients", "Durum wheat semolina, water", "سميد قمح صلب، مية"),
             S("cooking", "11 minutes", "11 دقيقة"), S("origin", EGYPT)],
      img=["m:Farfalle"],
      r=(4.4, 233), tags="",
      v=[V(None, 26.99, sku="400G")]),

    P(id=2306, slug="barilla-rigatoni", brand="barilla",
      name=("Barilla rigatoni", "مكرونة ريجاتوني باريلا"),
      sub=("Italy's favourite pasta, al dente in 13 minutes", "مكرونة إيطاليا المفضلة... مظبوطة في 13 دقيقة"),
      desc=("Barilla's ridged rigatoni tubes catch every bit of a thick ragù or a baked pasta with cheese. "
            "Made in Italy from durum wheat semolina.",
            "ريجاتوني باريلا المحزّزة بتمسك الصوص التقيل والمكرونة المخبوزة بالجبنة. صناعة إيطالية من "
            "سميد القمح الصلب."),
      hl=[("Made in Italy", "صناعة إيطالية"), ("Great for baked pasta", "ممتازة للمكرونة في الفرن")],
      specs=[S("ingredients", "Durum wheat semolina, water", "سميد قمح صلب، مية"),
             S("cooking", "13 minutes", "13 دقيقة"), S("origin", "Italy", "إيطاليا")],
      img=["m:Rigatoni"],
      r=(4.7, 165), tags="",
      v=[V(None, 99.99, sku="500G")]),

    P(id=2307, slug="bonduelle-red-kidney-beans", brand="bonduelle",
      name=("Bonduelle red kidney beans", "فاصوليا حمرا بونديل"),
      sub=("Cooked and ready — just rinse", "مسلوقة وجاهزة... اشطفها بس"),
      desc=("Tender red kidney beans, cooked and canned. Rinse and add them to chili, salads or a quick "
            "bean stew with tomato and cumin.",
            "فاصوليا حمرا طرية مسلوقة في علبة. اشطفها وحطها على الشيلي أو السلطة أو طبيخ فاصوليا "
            "سريع بالطماطم والكمون."),
      hl=[("Ready to use", "جاهزة على طول"), ("No soaking", "من غير نقع")],
      specs=[S("weight", "400 g (240 g drained)", "400 جم (240 جم صافي)"), S("origin", "France", "فرنسا"),
             S("storage", DRY)],
      img=["m:Haricot Beans"],
      r=(4.5, 143), tags="",
      v=[V(None, 59.99, sku="400G")]),

    P(id=2308, slug="heinz-baked-beans", brand="heinz",
      name=("Heinz baked beans", "فاصوليا بيضا بالصلصة هاينز"),
      sub=("Beans in tomato sauce, ready in 3 minutes", "فاصوليا بالصلصة... جاهزة في 3 دقايق"),
      desc=("Heinz's famous beans in a rich tomato sauce. Heat and serve on toast, next to eggs and "
            "sausages for a full breakfast, or spoon over a jacket potato.",
            "فاصوليا هاينز المشهورة في صلصة طماطم غنية. سخّنها وقدّمها على التوست، أو جنب البيض "
            "والسجق لفطار كامل، أو على بطاطس مشوية."),
      hl=[("Ready in 3 minutes", "جاهزة في 3 دقايق"), ("A breakfast classic", "كلاسيكية على الفطار")],
      specs=[S("weight", "415 g", "415 جم"), S("storage", DRY)],
      img=["m:Baked Beans"],
      r=(4.4, 198), tags="",
      v=[V(None, 69.99, sku="415G")]),
]

"""Fruits & vegetables (category 10, ids 2001–2049)."""

from common import EGYPT, FREEZER, FRIDGE, DRY, L, P, S, V, axis, mg, mo, weights

FREEZER_V = FREEZER

CATEGORY = 10

BY_WEIGHT = S("sold", "By weight", "بالوزن")
BUNCH = S("sold", "Per bunch (about 150 g)", "بالحزمة (حوالي 150 جم)")
ROOM = L("Room temperature, fridge once ripe", "برة التلاجة لحد ما تستوي، وبعدين في التلاجة")
VEG_FRIDGE = L("Vegetable drawer of the fridge", "درج الخضار في التلاجة")
HERB_FRIDGE = L("Wrap in a damp towel, fridge 3–4 days", "ملفوفة في فوطة مبلولة في التلاجة 3–4 أيام")

PRODUCTS = [
    P(id=2001, slug="tomatoes",
      name=("Tomatoes", "طماطم"),
      sub=("Firm and red — for salads, sauce and everything", "حمرا ومشدودة... للسلطة والصلصة وكل طبخة"),
      desc=("Everyday Egyptian tomatoes, hand-sorted at the market each morning so you get firm, evenly "
            "ripe fruit. Great raw in a baladi salad or cooked down into a rich sauce for tagines, "
            "stews and macarona béchamel.",
            "طماطم مصري بنفرزها بإيدينا كل يوم الصبح في السوق، علشان توصلك مشدودة ومستوية على قدها. "
            "تنفع في السلطة البلدي وتتسبك صلصة حلوة للطواجن والخضار والمكرونة بالبشاميل."),
      hl=[("Hand-sorted daily, no soft fruit", "متفرزة كل يوم... مفيش ولا حباية طرية"),
          ("Pick ½, 1 or 2 kg", "خد نص كيلو أو كيلو أو 2 كيلو")],
      specs=[S("origin", EGYPT), S("storage", ROOM), BY_WEIGHT],
      img=["m:Tomatoes", "m:Tomato"],
      r=(4.5, 1874), tags="express bestseller",
      options=[weights("500g", "1kg", "2kg", loose=True)],
      v=[V("500g", 10.99), V("1kg", 19.99), V("2kg", 37.99)]),

    P(id=2002, slug="cherry-tomatoes",
      name=("Cherry tomatoes", "طماطم شيري"),
      sub=("Sweet, bite-size, great for lunch boxes", "حلوة وصغيرة... تنفع في لانش بوكس الولاد"),
      desc=("Sweet greenhouse cherry tomatoes in a handy punnet. Halve them into salads and pasta, roast "
            "them with olive oil and garlic, or just snack on them straight from the box.",
            "طماطم شيري زراعة صوب ومسكّرة في علبة. قطّعها نصين في السلطة والمكرونة، أو اشويها بزيت "
            "الزيتون والتوم، أو كُلها كده على طول من العلبة."),
      hl=[("Greenhouse-grown, extra sweet", "زراعة صوب وطعمها مسكّر"),
          ("Rinse just before eating", "اغسلها قبل الأكل على طول")],
      specs=[S("origin", EGYPT), S("storage", FRIDGE)],
      img=["m:Cherry Tomatoes"],
      r=(4.6, 412), tags="express",
      options=[weights("250g", "500g")],
      v=[V("250g", 29.99), V("500g", 54.99)]),

    P(id=2003, slug="onions",
      name=("Onions", "بصل بلدي"),
      sub=("The base of every Egyptian pot", "أساس أي حلة في مطبخنا"),
      desc=("Firm, dry-skinned onions for everyday cooking — from the tasha of molokhia to the crispy "
            "topping on koshari. Keep them somewhere cool and airy and they last for weeks.",
            "بصل ناشف ومتماسك للطبيخ اليومي... من طشة الملوخية لحد البصل المحمّر بتاع الكشري. احفظه "
            "في مكان بارد ومتهوّي ويقعد معاك أسابيع."),
      hl=[("Dry skins, long keeping", "قشرته ناشفة ويعيش كتير"),
          ("Fries up sweet and crispy", "يتحمّر مقرمش وطعمه مسكّر")],
      specs=[S("origin", EGYPT), S("storage", DRY), BY_WEIGHT],
      img=["m:Onions"],
      r=(4.4, 1320), tags="express deal",
      options=[weights("500g", "1kg", "3kg", loose=True)],
      v=[V("500g", 9.99), V("1kg", 17.99), V("3kg", 49.99, was=53.99)]),

    P(id=2004, slug="red-onions",
      name=("Red onions", "بصل أحمر"),
      sub=("Mild and crunchy — the salad onion", "طعمه خفيف ومقرمش... بصل السلطة"),
      desc=("Purple-red onions with a gentle bite. Slice them raw over grilled kofta, toss them into a "
            "tahini salad or mix with sumac for a quick relish next to the grill.",
            "بصل أحمر طعمه خفيف، يتقطع شرايح على الكفتة المشوية، أو في سلطة الطحينة، أو يتتبل "
            "بالسماق جنب المشويات."),
      hl=[("Sweeter and milder than white onions", "أحلى وأخف من البصل العادي"),
          ("Keeps its colour in pickles", "بيحافظ على لونه في المخلل")],
      specs=[S("origin", EGYPT), S("storage", DRY), BY_WEIGHT],
      img=["m:Red Onions", "d:red-onions-1"],
      r=(4.5, 486), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 12.49), V("1kg", 22.99)]),

    P(id=2005, slug="baladi-garlic",
      name=("Baladi garlic", "توم بلدي"),
      sub=("Small cloves, big flavour — peeled on request", "فصوصه صغيرة وطعمه نفّاذ... ونقشّره لو تحب"),
      desc=("Egyptian baladi garlic with tight, papery heads and a punchy aroma — the one mums swear by "
            "for tasha, toumeya and pickles. Ask us to peel it and it arrives ready to crush.",
            "توم بلدي راسه مقفولة وريحته نفّاذة... اللي كل أم بتحلف بيه للطشة والتومية والمخلل. ولو "
            "مش فاضي، اطلبه مقشّر ويوصلك جاهز للدق."),
      hl=[("Peeled option — no sticky fingers", "متاح مقشّر... من غير ما إيدك تتلزق"),
          ("Dry-cured for long storage", "ناشف ويتخزن مدة طويلة")],
      specs=[S("origin", EGYPT), S("variety", "Baladi (local)", "بلدي"), S("storage", DRY)],
      img=["m:Garlic", "m:Garlic Bulb"],
      r=(4.7, 903), tags="bestseller express",
      options=[weights("250g", "500g", "1kg", loose=True)],
      v=[V("250g", 29.99), V("500g", 56.99), V("1kg", 109.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("as-is", "As is, whole heads", "زي ما هو بالراس"),
          mo("peeled", "Peeled cloves", "فصوص مقشّرة", 12)], required=True)]),

    P(id=2006, slug="potatoes",
      name=("Potatoes", "بطاطس"),
      sub=("For frying, mashing and the oven tray", "للتحمير والبيوريه وصينية الفرن"),
      desc=("All-round Egyptian potatoes with creamy flesh. They fry up golden, mash smooth and hold their "
            "shape in a potato-and-chicken tray bake.",
            "بطاطس مصري تنفع لكل حاجة وقلبها كريمي. تتحمّر دهبي، وتعمل بيوريه ناعم، وتمسك نفسها في "
            "صينية البطاطس بالفراخ."),
      hl=[("Washed and sorted by size", "مغسولة ومتفرزة بالحجم"),
          ("Store in the dark, not the fridge", "خزّنها في مكان ضلمة مش في التلاجة")],
      specs=[S("origin", EGYPT), S("storage", "Cool, dark place", "مكان بارد وضلمة"), BY_WEIGHT],
      img=["m:Potatoes", "d:potatoes-1"],
      r=(4.4, 1102), tags="express",
      options=[weights("1kg", "2kg", "3kg", loose=True)],
      v=[V("1kg", 19.99), V("2kg", 37.99), V("3kg", 54.99)]),

    P(id=2007, slug="carrots",
      name=("Carrots", "جزر"),
      sub=("Sweet and crunchy — juice, salad or stew", "مسكّر ومقرمش... عصير أو سلطة أو طبيخ"),
      desc=("Bright, sweet carrots that are as good grated into a salad as they are simmered with peas "
            "in a classic bisella stew. Juice them with orange for a vitamin-packed breakfast.",
            "جزر لونه زاهي وطعمه مسكّر، يتبشر في السلطة أو يتطبخ مع البسلة. واعصره مع البرتقال لفطار "
            "كله فيتامينات."),
      hl=[("Great for juicing", "ممتاز للعصير"), ("Firm, no woody cores", "متماسك ومن غير قلب خشب")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Carrots"],
      r=(4.5, 640), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 11.99), V("1kg", 21.99)]),

    P(id=2008, slug="cucumbers",
      name=("Cucumbers", "خيار"),
      sub=("Baladi cucumbers for salad and pickles", "خيار بلدي للسلطة والمخلل"),
      desc=("Crisp baladi cucumbers with thin skins and few seeds — the backbone of every Egyptian green "
            "salad, and just the right size for pickling at home.",
            "خيار بلدي مقرمش وقشره رفيع وبذره قليل... أساس السلطة الخضرا في كل بيت، وحجمه مظبوط "
            "للتخليل في البيت."),
      hl=[("Thin skin, no need to peel", "قشره رفيع ومش محتاج يتقشّر"),
          ("Picked small for crunch", "متقطف صغير علشان يفضل مقرمش")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Cucumber", "d:cucumber-1"],
      r=(4.3, 958), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 13.49), V("1kg", 24.99)]),

    P(id=2009, slug="zucchini",
      name=("Zucchini", "كوسة"),
      sub=("Tender and pale — cored for mahshi on request", "طرية ولونها فاتح... ونحفرها للمحشي"),
      desc=("Pale, slender Egyptian zucchini with tender skins and a mild, sweet taste. Order them cored "
            "and they arrive ready to stuff with rice and herbs for kousa mahshi, or slice them for a "
            "béchamel bake.",
            "كوسة مصري رفيعة ولونها فاتح وقشرها طري. اطلبها محفورة وتوصلك جاهزة تتحشي رز وخضرة، أو "
            "قطّعها حلقات لصينية كوسة بالبشاميل."),
      hl=[("Coring service for mahshi", "بنحفرها لك للمحشي"),
          ("Small, even-sized pieces", "حبات صغيرة ومتساوية")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Courgettes", "m:Zucchini"],
      r=(4.5, 377), tags="express",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 29.99), V("2kg", 57.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "Whole", "صحيحة"),
          mo("cored", "Cored for mahshi", "محفورة للمحشي", 12)], required=True)]),

    P(id=2010, slug="aubergines",
      name=("Aubergines", "باذنجان رومي"),
      sub=("For baba ghanoug, moussaka and frying", "للبابا غنوج والمسقعة والتحمير"),
      desc=("Glossy black aubergines with firm flesh. Char them whole for baba ghanoug, slice and fry them "
            "for moussaka, or pickle the small ones with garlic and chilli.",
            "باذنجان رومي أسود لامع ولحمه متماسك. اشويه على النار للبابا غنوج، أو قطّعه شرايح وحمّره "
            "للمسقعة، والصغير منه يتخلل بالتوم والشطة."),
      hl=[("Few seeds, not bitter", "بذره قليل ومش مرّ"), ("Glossy and firm", "لامع ومشدود")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Aubergine"],
      r=(4.4, 512), tags="express",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 24.99), V("2kg", 47.99)]),

    P(id=2011, slug="bell-peppers",
      name=("Bell peppers", "فلفل رومي"),
      sub=("Green, red or yellow — pick your colour", "أخضر ولا أحمر ولا أصفر... اختار لونك"),
      desc=("Thick-walled bell peppers in 500 g bags. Green ones are the classic for mahshi and salads; "
            "red and yellow are sweeter and shine in stir-fries, fajitas and roasted with olive oil.",
            "فلفل رومي لحمه تخين في أكياس نص كيلو. الأخضر هو بتاع المحشي والسلطة، والأحمر والأصفر "
            "أحلى وبيلمعوا في التشويح والفاهيتا والشوي بزيت الزيتون."),
      hl=[("Sold in 500 g bags", "بيتباع في أكياس نص كيلو"),
          ("Red and yellow are greenhouse-grown", "الأحمر والأصفر زراعة صوب")],
      specs=[S("origin", EGYPT), S("pack", "500 g bag", "كيس نص كيلو"), S("storage", VEG_FRIDGE)],
      img=[("m:Green Pepper", L("green", "أخضر")), ("m:Red Pepper", L("red", "أحمر")),
           ("m:Yellow Pepper", L("yellow", "أصفر"))],
      r=(4.5, 288), tags="express",
      options=[axis("colour", [("green", L("Green", "أخضر"), "#3E7D2C"),
                               ("red", L("Red", "أحمر"), "#C0281F"),
                               ("yellow", L("Yellow", "أصفر"), "#F2B705")], kind="swatch")],
      v=[V("green", 14.99, img=0), V("red", 34.99, img=1), V("yellow", 34.99, img=2, stock=0)]),

    P(id=2012, slug="hot-green-chillies",
      name=("Hot green chillies", "فلفل حامي أخضر"),
      sub=("For shatta, pickles and the brave", "للشطة والمخلل وللي بيحبوا الحراق"),
      desc=("Long green chillies with a proper kick. Blitz them with garlic and lemon for a fresh shatta, "
            "slit them into the pickle jar, or fry them whole alongside fish.",
            "فلفل أخضر حامي طويل وحراقه مظبوط. اضربه مع التوم والليمون لشطة فريش، أو شقّه وحطه في "
            "برطمان المخلل، أو اقليه صحيح جنب السمك."),
      hl=[("Medium-hot", "حراق متوسط"), ("Great fried with fish", "حلو متقلي جنب السمك")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Green Chilli", "d:green-chili-pepper-1"],
      r=(4.4, 205), tags="",
      options=[weights("250g", "500g", loose=True)],
      v=[V("250g", 9.99), V("500g", 18.99)]),

    P(id=2013, slug="baladi-lemons",
      name=("Baladi lemons", "ليمون بلدي"),
      sub=("Small, juicy and properly sour", "صغير ومليان عصير وحامض على أصوله"),
      desc=("The small, thin-skinned Egyptian lemon every kitchen runs on — squeezed over ful and liver, "
            "into salads and into fresh lemonade with mint. Picked green-gold for the most juice.",
            "الليمون البلدي الصغير أبو قشرة رفيعة اللي المطبخ المصري ماشي عليه... على الفول والكبدة "
            "وفي السلطة وعصير الليمون بالنعناع. بنختاره أخضر مدهّب علشان يبقى مليان عصير."),
      hl=[("Thin skin, lots of juice", "قشره رفيع وعصيره كتير"),
          ("Also great for pickling", "ينفع كمان للتخليل")],
      specs=[S("origin", EGYPT), S("variety", "Baladi (Mexican lime type)", "بلدي"), S("storage", VEG_FRIDGE)],
      img=["m:Unwaxed Lime", "m:Lime"],
      r=(4.6, 1490), tags="bestseller express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 27.99), V("1kg", 54.99)]),

    P(id=2015, slug="bananas",
      name=("Bananas", "موز بلدي"),
      sub=("Sweet baladi bananas, ripe this week", "موز بلدي مسكّر ومستوي على قده"),
      desc=("Egyptian bananas — shorter and sweeter than the imported kind. Perfect for breakfast, school "
            "snacks or blending with cold milk into a banana shake.",
            "موز مصري أقصر وأحلى من المستورد. ينفع للفطار ولشنطة المدرسة، أو يتضرب مع اللبن الساقع "
            "عصير موز باللبن."),
      hl=[("Ripened naturally", "مستوي طبيعي"), ("Sweeter than imported", "أحلى من المستورد")],
      specs=[S("origin", EGYPT), S("storage", "Room temperature", "برة التلاجة"), BY_WEIGHT],
      img=["m:Banana"],
      r=(4.5, 1033), tags="express bestseller",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 39.99), V("2kg", 77.99)]),

    P(id=2016, slug="baladi-oranges",
      name=("Baladi oranges", "برتقال بلدي"),
      sub=("Juicy oranges for the morning glass", "برتقال عصير لكوباية الصبح"),
      desc=("Thin-skinned baladi oranges bursting with juice — squeeze a glass at breakfast or peel and "
            "segment them for the kids. The new season's fruit is just starting to arrive.",
            "برتقال بلدي قشره رفيع ومليان عصير... اعصر كوباية على الفطار أو قشّره للولاد. أول الموسم "
            "الجديد ابتدا ينزل."),
      hl=[("Best for juicing", "أحسن حاجة للعصير"), ("3 kg family bag on offer", "كيس 3 كيلو بسعر أوفر")],
      specs=[S("origin", EGYPT), S("season", "October – April", "أكتوبر – أبريل"), BY_WEIGHT],
      img=["m:Orange"],
      r=(4.3, 621), tags="deal express",
      options=[weights("1kg", "3kg", loose=True)],
      v=[V("1kg", 24.99), V("3kg", 69.99, was=74.99)]),

    P(id=2017, slug="imported-apples",
      name=("Imported apples", "تفاح مستورد"),
      sub=("Crisp red or tart green", "أحمر مقرمش أو أخضر مزّ"),
      desc=("Crunchy imported apples sold by the kilo. Red for snacking and lunch boxes; green for a "
            "sharper bite, baking and salads.",
            "تفاح مستورد مقرمش بالكيلو. الأحمر للتسالي وشنطة المدرسة، والأخضر طعمه مزّ وينفع "
            "للخبيز والسلطات."),
      hl=[("Sold per kilo", "بالكيلو"), ("Keeps for weeks in the fridge", "يعيش أسابيع في التلاجة")],
      specs=[S("origin", "Imported (varies by season)", "مستورد (حسب الموسم)"), S("storage", FRIDGE)],
      img=[("m:Apples", L("red", "أحمر")), ("m:Cooking Apples", L("green", "أخضر"))],
      r=(4.4, 356), tags="",
      options=[axis("colour", [("red", L("Red", "أحمر"), "#B3202A"),
                               ("green", L("Green", "أخضر"), "#8DB600")], kind="swatch")],
      v=[V("red", 109.99, img=0, sku="RED-1KG"), V("green", 119.99, img=1, sku="GRN-1KG")]),

    P(id=2018, slug="mango",
      name=("Mango", "مانجو"),
      sub=("Last of the season — zebdia & awees", "آخر الموسم... زبدية وعويسي"),
      desc=("Late-season Egyptian mangoes from Ismailia: fragrant, fibre-free flesh that's just right for "
            "eating with a spoon or blending into a thick mango juice. Grab them while they last.",
            "مانجو مصري آخر الموسم من الإسماعيلية، ريحتها تجنن ولحمها من غير ألياف... تتاكل "
            "بالمعلقة أو تتضرب عصير مانجو تقيل. الحق قبل ما تخلص."),
      hl=[("Zebdia: creamy and sweet", "الزبدية: كريمية ومسكّرة"),
          ("Awees: the connoisseur's pick", "العويسي: اختيار العارفين")],
      specs=[S("origin", "Ismailia, Egypt", "الإسماعيلية، مصر"), S("season", "July – October", "يوليو – أكتوبر"),
             S("sold", "Per kg", "بالكيلو")],
      img=["m:Mango"],
      r=(4.8, 744), tags="deal featured",
      options=[axis("type", [("zebdia", L("Zebdia", "زبدية")), ("awees", L("Awees", "عويسي"))])],
      v=[V("zebdia", 74.99, was=84.99, sku="ZEB-1KG"), V("awees", 94.99, stock=0, sku="AWS-1KG")]),

    P(id=2019, slug="strawberries",
      name=("Strawberries", "فراولة"),
      sub=("First of the new season", "أول فراولة في الموسم"),
      desc=("The first strawberries of the Egyptian winter season, hand-packed in 500 g boxes. Sweet "
            "enough to eat as they are, and perfect for juice, milkshakes or a quick fruit salad.",
            "أول فراولة مصري في موسم الشتا، متعبية بالإيد في علب نص كيلو. حلوة كده على طول، وتنفع "
            "عصير أو ميلك شيك أو فروت سلاط."),
      hl=[("New season", "موسم جديد"), ("Hand-packed, 500 g box", "علبة نص كيلو متعبية بالإيد")],
      specs=[S("origin", "Beheira, Egypt", "البحيرة، مصر"), S("pack", "500 g box", "علبة 500 جم"),
             S("storage", FRIDGE)],
      img=["m:Strawberries", "d:strawberry-1"],
      r=(4.6, 189), tags="new featured express",
      v=[V(None, 59.99, sku="500G")]),

    P(id=2020, slug="pomegranates",
      name=("Pomegranates", "رمان"),
      sub=("In season now — order it seeded", "في موسمه دلوقتي... واطلبه مفرّط"),
      desc=("Deep-red pomegranates from Upper Egypt, heavy with juice. Order them whole, or let us seed "
            "them into a tub so you can sprinkle the jewels over salads and desserts — no stained hands.",
            "رمان صعيدي أحمر غامق ومليان عصير. اطلبه صحيح، أو خلّينا نفرّطه لك في علبة علشان ترشه "
            "على السلطة والحلويات... من غير ما إيدك تتصبغ."),
      hl=[("Peak season", "عز الموسم"), ("Seeding service available", "بنفرّطه لك لو تحب")],
      specs=[S("origin", "Asyut, Egypt", "أسيوط، مصر"), S("season", "September – December", "سبتمبر – ديسمبر"),
             BY_WEIGHT],
      img=["m:Pomegranate"],
      r=(4.7, 268), tags="featured express",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 34.99), V("2kg", 66.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "Whole fruit", "صحيح"),
          mo("seeded", "Seeded into a tub", "مفرّط في علبة", 20)], required=True)]),

    P(id=2021, slug="molokhia",
      name=("Fresh molokhia", "ملوخية خضرا"),
      sub=("Picked and minced fresh on request", "بنقطّفها ونخرطها طازة على طلبك"),
      desc=("Fresh molokhia cut this morning. Take it on the stem as it comes, picked, or picked and finely "
            "minced — ready to drop straight into hot stock and finish with a garlic-and-coriander tasha.",
            "ملوخية خضرا مقطوفة النهارده الصبح. خدها بعيدانها، أو مقطّفة، أو مقطّفة ومخروطة ناعم... "
            "جاهزة تنزل على الشوربة السخنة وتتقفل بطشة التوم والكزبرة."),
      hl=[("Minced to order with a makhrata", "بتتخرط على الطلب بالمخرطة"),
          ("Freeze minced molokhia for up to 3 months", "المخروطة تتخزن في الفريزر لحد 3 شهور")],
      specs=[S("origin", EGYPT), S("storage", "Fridge 2 days, or freeze once minced",
                                   "التلاجة يومين، أو الفريزر بعد الخرط")],
      img=["m:Mulukhiyah"],
      r=(4.7, 1211), tags="bestseller express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 16.99), V("1kg", 29.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("stems", "On the stem", "بعيدانها"),
          mo("picked", "Leaves picked", "مقطّفة ورق", 5),
          mo("minced", "Picked and minced", "مقطّفة ومخروطة", 12)], required=True)]),

    P(id=2022, slug="spinach",
      name=("Spinach", "سبانخ"),
      sub=("Tender leaves for stews and fatayer", "ورق طري للطبيخ والفطاير"),
      desc=("Dark, tender spinach for a classic spinach stew with rice, for cheese-and-spinach fatayer, or "
            "a quick sauté with garlic and lemon.",
            "سبانخ ورقها غامق وطري، للسبانخ المطبوخة بالرز، أو فطاير السبانخ بالجبنة، أو تتشوّح "
            "بالتوم والليمون."),
      hl=[("Washed twice", "متغسلة مرتين"), ("Iron-rich winter green", "مليانة حديد")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Spinach"],
      r=(4.3, 233), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 13.99), V("1kg", 24.99)]),

    P(id=2023, slug="rocket",
      name=("Rocket", "جرجير"),
      sub=("The peppery bunch every ful plate needs", "الحزمة اللي طبق الفول مايكملش من غيرها"),
      desc=("A generous bunch of baladi rocket with a peppery bite — the classic side for ful, ta'ameya "
            "and grilled fish.",
            "حزمة جرجير بلدي طعمها حراق شوية... الطبق الجانبي الأساسي مع الفول والطعمية والسمك "
            "المشوي."),
      hl=[("Cut this morning", "مقطوف النهارده الصبح"), ("Washed and bunched", "مغسول ومحزّم")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Rocket"],
      r=(4.5, 802), tags="express",
      v=[V(None, 5.99, sku="BUNCH")]),

    P(id=2024, slug="parsley",
      name=("Parsley", "بقدونس"),
      sub=("Flat-leaf, for tabbouleh and kofta", "بقدونس بلدي للتبولة والكفتة"),
      desc=("A big bunch of flat-leaf parsley, bright and fragrant. Chop it into tabbouleh, kofta mix or "
            "green tahini, or tuck sprigs around a grilled-fish platter.",
            "حزمة بقدونس كبيرة ريحتها حلوة ولونها زاهي. افرمه في التبولة وخلطة الكفتة والطحينة "
            "الخضرا، أو زيّن بيه صينية السمك المشوي."),
      hl=[("Big bunch", "حزمة كبيرة"), ("Flat-leaf baladi parsley", "بقدونس بلدي ورقه عريض")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Parsley"],
      r=(4.6, 690), tags="express",
      v=[V(None, 5.99, sku="BUNCH")]),

    P(id=2025, slug="fresh-coriander",
      name=("Fresh coriander", "كزبرة خضرا"),
      sub=("The secret of a good molokhia", "سر الملوخية الحلوة"),
      desc=("Fresh green coriander for the tasha in molokhia, bissara and soups, or chopped into "
            "salads.",
            "كزبرة خضرا فريش لطشة الملوخية والبصارة والشوربة، أو تتفرم في السلطات."),
      hl=[("Fragrant leaves and stems", "ورق وعيدان ريحتهم فايحة"), ("Cut daily", "بتتقطف كل يوم")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Coriander Leaves"],
      r=(4.5, 455), tags="express",
      v=[V(None, 5.99, sku="BUNCH")]),

    P(id=2026, slug="dill",
      name=("Dill", "شبت"),
      sub=("For fish, sambousek and soups", "للسمك والسمبوسك والشوربة"),
      desc=("Feathery fresh dill to stir into lentil soup, tuck inside fish before grilling, or mix with "
            "white cheese for a sambousek filling.",
            "شبت فريش يتحط في شوربة العدس، أو جوه السمك قبل الشوي، أو مع الجبنة البيضا لحشو "
            "السمبوسك."),
      hl=[("Feathery and fragrant", "ورقه ناعم وريحته حلوة"), ("Lovely with fish", "حلو جدًا مع السمك")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Dill"],
      r=(4.4, 210), tags="",
      v=[V(None, 5.99, sku="BUNCH")]),

    P(id=2027, slug="fresh-mint",
      name=("Fresh mint", "نعناع أخضر"),
      sub=("For tea, lemonade and salads", "للشاي والليمون بالنعناع والسلطة"),
      desc=("A fragrant bunch of fresh mint. Drop a few leaves into a glass of tea, blend with lemon for a "
            "frosty lemonade, or chop into tabbouleh.",
            "حزمة نعناع ريحتها فايحة. حط كام ورقة في كوباية الشاي، أو اضربه مع الليمون عصير ساقع، أو "
            "افرمه في التبولة."),
      hl=[("Strong, sweet aroma", "ريحته قوية وحلوة"), ("Perfect in tea", "مظبوط في الشاي")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Mint"],
      r=(4.7, 980), tags="express bestseller",
      v=[V(None, 6.99, sku="BUNCH")]),

    P(id=2028, slug="fresh-vine-leaves",
      name=("Fresh vine leaves", "ورق عنب فريش"),
      sub=("Tender young leaves for mahshi", "ورق صغير طري للمحشي"),
      desc=("Tender young vine leaves picked for rolling. Blanch them for a minute in salted water, then "
            "roll with rice, herbs and tomato for a pot of warak enab.",
            "ورق عنب صغير وطري متقطف مخصوص للّف. اسلقه دقيقة في مية بملح، ولفّه بالرز والخضرة "
            "والطماطم لحلة محشي ورق عنب."),
      hl=[("Young, thin leaves", "ورق صغير ورفيع"), ("Freezes well after blanching", "يتخزن في الفريزر بعد السلق")],
      specs=[S("origin", EGYPT), S("season", "April – October", "أبريل – أكتوبر"), S("storage", FRIDGE)],
      img=["m:Vine Leaves"],
      r=(4.5, 340), tags="",
      options=[weights("250g", "500g", loose=True)],
      v=[V("250g", 34.99), V("500g", 64.99, stock=0)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2014, slug="adalia-lemons",
      name=("Adalia lemons", "ليمون أضاليا"),
      sub=("Big yellow lemons with plenty of zest", "ليمون أصفر كبير وقشره ريحته حلوة"),
      desc=("Large, thick-skinned yellow lemons with fragrant zest and mild juice. Grate the zest into "
            "basbousa and cakes, slice them into tea, or use the juice for marinades.",
            "ليمون أصفر كبير قشره تخين وريحته فايحة وعصيره خفيف. ابشر القشر في البسبوسة والكيك، أو "
            "قطّعه شرايح في الشاي، أو استعمل عصيره في التتبيلات."),
      hl=[("Great for zest", "قشره ممتاز للبشر"), ("Milder than baladi lemons", "أخف من الليمون البلدي")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Lemon", "d:lemon-1"],
      r=(4.3, 176), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 23.99), V("1kg", 44.99)]),

    P(id=2029, slug="green-onions",
      name=("Green onions", "بصل أخضر"),
      sub=("The classic Sham el-Nessim side", "رفيق الفسيخ في شم النسيم... وكل يوم"),
      desc=("A bunch of fresh green onions, mild and crunchy. Slice them over ful and eggs, into salads, or "
            "serve them whole next to grilled fish.",
            "حزمة بصل أخضر طازة طعمها خفيف ومقرمش. قطّعه على الفول والبيض وفي السلطة، أو قدّمه صحيح "
            "جنب السمك المشوي."),
      hl=[("Cut this morning", "مقطوف النهارده"), ("Mild flavour", "طعمه خفيف")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Spring Onions"],
      r=(4.4, 290), tags="express",
      v=[V(None, 6.99, sku="BUNCH")]),

    P(id=2030, slug="leeks",
      name=("Leeks", "كرات"),
      sub=("Baladi leeks for ta'ameya and soups", "كرات بلدي للطعمية والشوربة"),
      desc=("Tender baladi leeks, the green secret inside homemade ta'ameya. Also lovely sweated down for "
            "soups or chopped raw into a green salad.",
            "كرات بلدي طري، السر الأخضر جوه الطعمية البيتي. وحلو كمان يتشوّح للشوربة أو يتقطع ني في "
            "السلطة الخضرا."),
      hl=[("Essential for ta'ameya", "أساسي في الطعمية"), ("Washed bunches", "حزم مغسولة")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Leek"],
      r=(4.3, 133), tags="",
      v=[V(None, 7.99, sku="BUNCH")]),

    P(id=2031, slug="radishes",
      name=("Radishes", "فجل"),
      sub=("Peppery and crunchy, leaves and all", "حراق ومقرمش... بورقه"),
      desc=("A bunch of baladi radishes with their leaves. Slice them into a green salad, eat them with ful "
            "and ta'ameya, or nibble the tender leaves too.",
            "حزمة فجل بلدي بورقه. قطّعه في السلطة الخضرا، أو كُله مع الفول والطعمية، وورقه الطري "
            "بيتاكل كمان."),
      hl=[("Comes with leaves", "بورقه"), ("Crisp and peppery", "مقرمش وحراق")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Radish"],
      r=(4.2, 98), tags="",
      v=[V(None, 5.99, sku="BUNCH")]),

    P(id=2032, slug="beetroot",
      name=("Beetroot", "بنجر"),
      sub=("For salads, pickles and juice", "للسلطة والمخلل والعصير"),
      desc=("Earthy, sweet beetroot. Boil or roast it for a tahini-beet salad, add a slice to the pickle jar "
            "for that famous pink colour, or juice it with carrot and apple.",
            "بنجر طعمه مسكّر. اسلقه أو اشويه لسلطة بنجر بالطحينة، أو حط شريحة في برطمان المخلل علشان "
            "اللون البمبي المشهور، أو اعصره مع الجزر والتفاح."),
      hl=[("Gives pickles their pink colour", "بيدي المخلل لونه البمبي"), ("Sweet and earthy", "مسكّر وطعمه أرضي")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Beetroot"],
      r=(4.3, 121), tags="",
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 24.99)]),

    P(id=2033, slug="green-beans",
      name=("Green beans", "فاصوليا خضرا"),
      sub=("Slender and stringless", "رفيعة ومن غير خيوط"),
      desc=("Slender green beans for the classic Egyptian fasolia stew with tomato sauce and meat, or "
            "blanched and tossed with garlic and lemon as a side.",
            "فاصوليا خضرا رفيعة لطبيخ الفاصوليا المصري بالصلصة واللحمة، أو مسلوقة ومشوّحة بالتوم "
            "والليمون كطبق جانبي."),
      hl=[("Tender, stringless pods", "قرون طرية من غير خيوط"), ("Picked young", "متقطفة صغيرة")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Green Beans"],
      r=(4.4, 260), tags="express",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 22.99), V("1kg", 42.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "Whole", "صحيحة"),
          mo("cut", "Trimmed and cut", "مقمّعة ومتقطعة", 8)], required=True)]),

    P(id=2034, slug="frozen-green-peas", brand="souq",
      name=("Frozen green peas", "بسلة خضرا مجمدة"),
      sub=("Sweet peas, frozen within hours of picking", "بسلة مسكّرة متجمدة بعد القطف بساعات"),
      desc=("Sweet garden peas, shelled and frozen within hours of picking so they stay bright and tender. "
            "Straight into bisella with carrots, rice, or a quick pasta.",
            "بسلة مسكّرة متفرطة ومتجمدة بعد القطف بساعات علشان تفضل لونها زاهي وطرية. على طول في "
            "البسلة بالجزر أو الرز أو المكرونة."),
      hl=[("No shelling needed", "مفرطة وجاهزة"), ("Cook from frozen", "تتطبخ من الفريزر على طول")],
      specs=[S("origin", EGYPT), S("storage", FREEZER_V)],
      img=["m:Peas"],
      r=(4.5, 402), tags="express",
      options=[weights("400g", "1kg")],
      v=[V("400g", 34.99), V("1kg", 79.99)]),

    P(id=2035, slug="green-fava-beans",
      name=("Green fava beans", "فول أخضر"),
      sub=("Back in winter — notify me", "راجع في الشتا... نبلغك أول ما ينزل"),
      desc=("Fresh green fava beans, a short winter treat. Shell them and cook with rice and dill, or stew "
            "them with onions and coriander. The new crop arrives around January.",
            "فول أخضر طازة، متعة قصيرة في الشتا. فصّصه واطبخه بالرز والشبت، أو بالبصل والكزبرة. "
            "المحصول الجديد بينزل حوالين يناير."),
      hl=[("Seasonal: January – March", "موسمي: يناير – مارس"), ("Sold in the pod", "بيتباع بقشره")],
      specs=[S("origin", EGYPT), S("season", "January – March", "يناير – مارس"), BY_WEIGHT],
      img=["m:Broad Beans"],
      r=(4.6, 87), tags="", soldout=True,
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 29.99)]),

    P(id=2036, slug="corn-on-the-cob",
      name=("Corn on the cob", "درة شامي"),
      sub=("Sweet cobs for grilling", "كيزان مسكّرة للشوي"),
      desc=("Sweet corn cobs with their husks peeled back. Grill them over charcoal Corniche-style with salt "
            "and lemon, or boil them for the kids.",
            "كيزان درة مسكّرة. اشويها على الفحم زي الكورنيش بالملح والليمون، أو اسلقها للولاد."),
      hl=[("Sweet, tender kernels", "حباية مسكّرة وطرية"), ("Grill or boil", "تتشوي أو تتسلق")],
      specs=[S("origin", EGYPT), S("season", "July – November", "يوليو – نوفمبر"), S("storage", VEG_FRIDGE)],
      img=["m:Corn On The Cob"],
      r=(4.5, 214), tags="",
      options=[axis("count", [("3", L("3 cobs", "3 كيزان")), ("6", L("6 cobs", "6 كيزان"))])],
      v=[V("3", 24.99), V("6", 46.99)]),

    P(id=2037, slug="pumpkin",
      name=("Pumpkin", "قرع عسلي"),
      sub=("Sweet orange pumpkin for béchamel and pie", "قرع عسلي مسكّر للبشاميل والبسبوسة"),
      desc=("Dense, sweet pumpkin for pumpkin with béchamel, soup or the beloved pumpkin pie with "
            "raisins and nuts.",
            "قرع عسلي لحمه تقيل ومسكّر، للقرع بالبشاميل أو الشوربة أو بسبوسة القرع بالزبيب والمكسرات."),
      hl=[("Sold in pieces by weight", "بيتباع حتت بالوزن"), ("Naturally sweet", "مسكّر طبيعي")],
      specs=[S("origin", EGYPT), S("season", "September – February", "سبتمبر – فبراير"), BY_WEIGHT],
      img=["m:Pumpkin"],
      r=(4.5, 156), tags="new",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 24.99), V("2kg", 47.99)],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "As is, in pieces", "حتت زي ما هو"),
          mo("cubed", "Peeled and cubed", "مقشّر ومكعبات", 10)], required=True)]),

    P(id=2038, slug="white-mushrooms",
      name=("White mushrooms", "مشروم أبيض"),
      sub=("Firm button mushrooms, locally grown", "مشروم زراعة مصرية متماسك"),
      desc=("Firm white button mushrooms. Sauté them with butter and garlic, add them to a creamy chicken "
            "pasta, or slice them raw into salads.",
            "مشروم أبيض متماسك. شوّحه بالزبدة والتوم، أو حطه على مكرونة الفراخ بالكريمة، أو قطّعه ني "
            "في السلطة."),
      hl=[("Locally grown", "زراعة مصرية"), ("Wipe, don't soak", "امسحه ومتنقعهوش")],
      specs=[S("origin", EGYPT), S("storage", "Fridge, in a paper bag", "في التلاجة في كيس ورق")],
      img=["m:Mushrooms"],
      r=(4.4, 233), tags="",
      options=[weights("200g", "400g")],
      v=[V("200g", 39.99), V("400g", 74.99)]),

    P(id=2039, slug="broccoli",
      name=("Broccoli", "بروكلي"),
      sub=("Tight green heads", "رؤوس خضرا مقفولة"),
      desc=("Fresh broccoli heads with tight florets. Steam them for the lunch box, roast them with olive "
            "oil and lemon, or toss them into a stir-fry.",
            "بروكلي طازة راسه مقفولة. اسلقه على البخار للانش بوكس، أو اشويه بزيت الزيتون والليمون، أو "
            "حطه في التشويحة."),
      hl=[("About 500 g a head", "حوالي نص كيلو للراس"), ("Tight, green florets", "زهرات مقفولة وخضرا")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE)],
      img=["m:Broccoli"],
      r=(4.3, 119), tags="",
      v=[V(None, 39.99, sku="500G")]),

    P(id=2040, slug="fresh-ginger",
      name=("Fresh ginger", "جنزبيل فريش"),
      sub=("Juicy roots for tea and marinades", "جذور مليانة عصارة للشاي والتتبيلات"),
      desc=("Plump, juicy ginger root. Slice it into hot water with lemon and honey for a winter drink, or "
            "grate it into marinades and stir-fries.",
            "جنزبيل فريش مليان عصارة. قطّعه شرايح في مية سخنة بالليمون والعسل لمشروب الشتا، أو ابشره "
            "في التتبيلات والتشويحات."),
      hl=[("Great for winter drinks", "ممتاز لمشروبات الشتا"), ("Thin skin", "قشره رفيع")],
      specs=[S("origin", "China", "الصين"), S("storage", VEG_FRIDGE)],
      img=["m:Ginger"],
      r=(4.4, 188), tags="",
      options=[weights("250g", "500g", loose=True)],
      v=[V("250g", 29.99), V("500g", 56.99)]),

    P(id=2041, slug="kiwi",
      name=("Kiwi", "كيوي"),
      sub=("Tangy-sweet and packed with vitamin C", "مزّ ومسكّر ومليان فيتامين سي"),
      desc=("Juicy green kiwi fruit, tangy and sweet. Halve and scoop with a spoon, slice into fruit salads, "
            "or blend into a green smoothie.",
            "كيوي أخضر مليان عصير، طعمه مزّ ومسكّر. قسّمه نصين وكُله بالمعلقة، أو قطّعه في الفروت "
            "سلاط، أو اضربه سموذي."),
      hl=[("Rich in vitamin C", "غني بفيتامين سي"), ("Ripen at room temperature", "يستوي برة التلاجة")],
      specs=[S("origin", "Imported", "مستورد"), S("storage", ROOM), BY_WEIGHT],
      img=["m:Kiwi", "d:kiwi-1"],
      r=(4.4, 143), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 54.99), V("1kg", 104.99)]),

    P(id=2042, slug="avocado",
      name=("Avocado", "أفوكادو"),
      sub=("Egyptian avocados — new season", "أفوكادو مصري... الموسم الجديد"),
      desc=("Creamy Egyptian avocados from the new season. Tell us when you want to eat them and we'll pick "
            "ripe-today or firm ones that will be ready in a few days.",
            "أفوكادو مصري كريمي من الموسم الجديد. قولّنا هتاكله إمتى وإحنا نختارهولك مستوي للنهارده أو "
            "ناشف يستوي بعد كام يوم."),
      hl=[("Pick your ripeness", "اختار درجة الاستواء"), ("New season", "موسم جديد")],
      specs=[S("origin", "Ismailia, Egypt", "الإسماعيلية، مصر"), S("season", "October – March", "أكتوبر – مارس"),
             BY_WEIGHT],
      img=["m:Avocado"],
      r=(4.5, 167), tags="new",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 59.99), V("1kg", 114.99)],
      mods=[mg("ripeness", "Ripeness", "الاستواء", [
          mo("today", "Ripe, eat today", "مستوي... يتاكل النهارده"),
          mo("later", "Firm, for later this week", "ناشف... يستوي على آخر الأسبوع")], required=True)]),

    P(id=2043, slug="cabbage",
      name=("White cabbage", "كرنب"),
      sub=("Big heads for mahshi kromb", "رؤوس كبيرة لمحشي الكرنب"),
      desc=("A big white cabbage, about 1.5–2 kg, with large outer leaves for rolling mahshi kromb. Separate "
            "the leaves, blanch them for a minute and start rolling.",
            "كرنبة بيضا كبيرة حوالي 1.5–2 كيلو، ورقها الخارجي كبير للف محشي الكرنب. فصّص الورق واسلقه "
            "دقيقة وابدأ لفّ."),
      hl=[("Large leaves for rolling", "ورق كبير للّف"), ("Tight, heavy heads", "رؤوس مقفولة وتقيلة")],
      specs=[S("origin", EGYPT), S("sold", "Per head (1.5–2 kg)", "بالراس (1.5–2 كيلو)"),
             S("storage", VEG_FRIDGE)],
      img=["m:Cabbage"],
      r=(4.4, 205), tags="",
      v=[V(None, 24.99, sku="HEAD")],
      mods=[mg("prep", "Preparation", "التجهيز", [
          mo("whole", "Whole head", "راس كاملة"),
          mo("leaves", "Leaves separated for mahshi", "ورق مفصّص للمحشي", 10)], required=True)]),

    P(id=2044, slug="baladi-lettuce",
      name=("Baladi lettuce", "خس بلدي"),
      sub=("Long, crisp leaves", "ورق طويل ومقرمش"),
      desc=("A crisp head of baladi lettuce with long leaves and a sweet heart. Tear it into salads or stack "
            "the leaves next to grilled kofta.",
            "خساية بلدي ورقها طويل ومقرمش وقلبها مسكّر. قطّعها في السلطة أو رصّ ورقها جنب الكفتة المشوية."),
      hl=[("Crisp and sweet heart", "قلبها مقرمش ومسكّر"), ("Per head", "بالخساية")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE)],
      img=["m:Lettuce"],
      r=(4.3, 176), tags="express",
      v=[V(None, 9.99, sku="HEAD")]),

    P(id=2045, slug="pears",
      name=("Pears", "كمترى"),
      sub=("Juicy and sweet", "مليانة عصير ومسكّرة"),
      desc=("Sweet, juicy pears that soften beautifully as they ripen. Lovely for snacking, poaching or "
            "slicing over a cheese board.",
            "كمترى مسكّرة ومليانة عصير وبتطرى حلو لما تستوي. حلوة كده على طول أو مسلوقة في شربات أو "
            "شرايح جنب الأجبان."),
      hl=[("Ripen at room temperature", "تستوي برة التلاجة"), ("Sweet and juicy", "مسكّرة ومليانة عصير")],
      specs=[S("origin", "Imported", "مستورد"), S("storage", ROOM), BY_WEIGHT],
      img=["m:Pears"],
      r=(4.2, 98), tags="",
      options=[weights("1kg", loose=True)],
      v=[V("1kg", 64.99)]),

    P(id=2046, slug="weekly-veg-box", brand="souq",
      name=("Weekly veg box", "سلة خضار الأسبوع"),
      sub=("5 kg of the week's best — save 12%", "5 كيلو من أحسن خضار الأسبوع... ووفّر 12%"),
      desc=("Our pickers fill a 5 kg box with the week's best: tomatoes, potatoes, onions, cucumbers, "
            "peppers, carrots, zucchini and a bunch of herbs. The mix changes with the season.",
            "اللي بيختاروا لنا بيملوا سلة 5 كيلو بأحسن خضار الأسبوع: طماطم وبطاطس وبصل وخيار وفلفل "
            "وجزر وكوسة وحزمة خضرة. التشكيلة بتتغير مع الموسم."),
      hl=[("About 5 kg of mixed veg", "حوالي 5 كيلو خضار مشكل"), ("Cheaper than buying separately", "أوفر من الشرا بالقطعة"),
          ("Mix changes weekly", "التشكيلة بتتغير كل أسبوع")],
      specs=[S("origin", EGYPT), S("weight", "About 5 kg", "حوالي 5 كيلو")],
      img=["m:Vegetables"],
      r=(4.7, 512), tags="featured deal",
      v=[V(None, 149.99, was=169.99, sku="5KG")]),

    P(id=2047, slug="celery",
      name=("Celery", "كرفس"),
      sub=("Leafy bunch for soups and stocks", "حزمة بورقها للشوربة والمرقة"),
      desc=("A leafy bunch of celery for chicken stock, lentil soup and salads — the leaves carry most of "
            "the flavour, so don't throw them away.",
            "حزمة كرفس بورقها لشوربة الفراخ وشوربة العدس والسلطات... الورق فيه أغلب الطعم فماترموهوش."),
      hl=[("Leaves on", "بورقه"), ("For stocks and soups", "للمرقة والشوربة")],
      specs=[S("origin", EGYPT), BUNCH, S("storage", HERB_FRIDGE)],
      img=["m:Celery"],
      r=(4.2, 77), tags="",
      v=[V(None, 12.99, sku="BUNCH")]),

    P(id=2048, slug="turnips",
      name=("Turnips", "لفت"),
      sub=("For pink turnip pickles", "للفت المخلل البمبي"),
      desc=("Firm white turnips, the star of homemade pink pickles with a slice of beetroot. Also good "
            "roasted or simmered in soups.",
            "لفت أبيض متماسك، نجم المخلل البيتي البمبي مع شريحة بنجر. وحلو كمان مشوي أو في الشوربة."),
      hl=[("Firm and crisp", "متماسك ومقرمش"), ("Pickling season", "موسم التخليل")],
      specs=[S("origin", EGYPT), S("storage", VEG_FRIDGE), BY_WEIGHT],
      img=["m:Turnips"],
      r=(4.3, 64), tags="",
      options=[weights("1kg", "2kg", loose=True)],
      v=[V("1kg", 19.99), V("2kg", 37.99)]),

    P(id=2049, slug="fresh-figs",
      name=("Fresh figs", "تين"),
      sub=("Back next summer", "راجع الصيف الجاي"),
      desc=("Soft, honey-sweet Egyptian figs from the summer harvest. The season has ended — tap notify me "
            "and we'll tell you when the first boxes arrive.",
            "تين مصري طري ومسكّر زي العسل من حصاد الصيف. الموسم خلص... دوس نبّهني وهنقولك أول ما "
            "أول الصناديق توصل."),
      hl=[("Seasonal: July – September", "موسمي: يوليو – سبتمبر"), ("Very delicate", "حساس جدًا")],
      specs=[S("origin", EGYPT), S("season", "July – September", "يوليو – سبتمبر")],
      img=["m:Figs"],
      r=(4.7, 133), tags="", soldout=True,
      options=[weights("500g", loose=True)],
      v=[V("500g", 49.99)]),
]

"""Drinks & juices (category 120, ids 2551–2599)."""

from common import DRY, FRIDGE, L, P, S, V, axis, weights

CATEGORY = 120

PRODUCTS = [
    P(id=2551, slug="baraka-natural-water", brand="baraka",
      name=("Baraka natural water", "مياه بركة طبيعية"),
      sub=("Natural mineral water, by the pack", "مياه طبيعية بالكرتونة"),
      desc=("Baraka natural mineral water with a light, clean taste. Stock up by the pack for the house, the "
            "car and the office.",
            "مياه بركة الطبيعية طعمها خفيف ونضيف. خزّن بالكرتونة للبيت والعربية والشغل."),
      hl=[("Delivered to your door", "بتوصل لحد الباب"), ("Two pack sizes", "مقاسين للكرتونة")],
      specs=[S("storage", DRY)],
      img=["d:water-1"],
      r=(4.6, 1520), tags="bestseller express",
      options=[axis("pack", [("6x1500ml", L("6 × 1.5 L", "6 × 1.5 لتر")),
                             ("12x600ml", L("12 × 600 ml", "12 × 600 مل"))])],
      v=[V("6x1500ml", 69.99), V("12x600ml", 74.99)]),

    P(id=2552, slug="fresh-orange-juice", brand="souq",
      name=("Freshly squeezed orange juice", "عصير برتقال فريش"),
      sub=("Squeezed this morning, no sugar added", "معصور الصبح من غير سكر"),
      desc=("Pure orange juice squeezed every morning from baladi oranges — no sugar, no water, nothing "
            "added. Keep it cold and drink it within 48 hours.",
            "عصير برتقال فريش بيتعصر كل يوم الصبح من برتقال بلدي... من غير سكر ولا مية ولا أي إضافات. "
            "خليه في التلاجة واشربه خلال 48 ساعة."),
      hl=[("100% juice", "عصير 100%"), ("Squeezed daily", "بيتعصر كل يوم")],
      specs=[S("storage", FRIDGE), S("shelf", "48 hours", "48 ساعة")],
      img=["d:juice-1"],
      r=(4.7, 420), tags="express featured",
      options=[axis("size", [("330ml", L("330 ml", "330 مل")), ("1l", L("1 L", "1 لتر"))])],
      v=[V("330ml", 34.99), V("1l", 89.99)]),

    P(id=2553, slug="coca-cola", brand="coca-cola",
      name=("Coca-Cola", "كوكاكولا"),
      sub=("Ice-cold, by the bottle", "ساقعة... بالإزازة"),
      desc=("The classic Coca-Cola taste in a shareable bottle. Keep a few in the fridge for koshari night "
            "and family get-togethers.",
            "طعم كوكاكولا الأصلي في إزازة تكفي الكل. خلّي كام واحدة في التلاجة لليلة الكشري ولمّة "
            "العيلة."),
      hl=[("Delivered chilled", "بتوصل ساقعة"), ("1 L or 2 L", "1 لتر أو 2 لتر")],
      specs=[S("storage", "Best served cold", "أحلى وهي ساقعة")],
      img=["d:soft-drinks-1"],
      r=(4.5, 760), tags="express",
      options=[axis("size", [("1l", L("1 L", "1 لتر")), ("2l", L("2 L", "2 لتر"))])],
      v=[V("1l", 29.99), V("2l", 49.99)]),

    P(id=2554, slug="ice-cubes-bag", brand="souq",
      name=("Ice cubes bag", "كيس تلج"),
      sub=("Clear ice, delivered frozen", "تلج شفاف يوصلك متجمد"),
      desc=("A 2 kg bag of clear ice cubes made from filtered water, delivered in an insulated bag so it "
            "arrives frozen. For juices, cool boxes and summer gatherings.",
            "كيس تلج 2 كيلو مكعبات شفافة من مية متفلترة، بيوصل في شنطة حافظة علشان يفضل متجمد. للعصاير "
            "والآيس بوكس ولمّات الصيف."),
      hl=[("Filtered water", "مية متفلترة"), ("Insulated delivery", "توصيل في شنطة حافظة")],
      specs=[S("storage", "Freezer", "الفريزر")],
      img=["m:Ice Cubes"],
      r=(4.4, 160), tags="express",
      v=[V(None, 19.99, sku="2KG")]),

    P(id=2555, slug="dried-tamarind", brand="souq",
      name=("Dried tamarind", "تمر هندي"),
      sub=("For the Ramadan tamr hindi drink", "لعصير التمر هندي في رمضان"),
      desc=("Sweet-sour dried tamarind. Soak it overnight, strain and sweeten for the deep-brown tamr hindi "
            "drink that fills every Ramadan table.",
            "تمر هندي ناشف طعمه مزّ ومسكّر. انقعه من بالليل وصفّيه وحلّيه، وتطلع عصير التمر هندي اللي "
            "مايغيبش عن سفرة رمضان."),
      hl=[("250 g makes about 3 L", "الربع كيلو بيعمل حوالي 3 لتر"), ("No added sugar", "من غير سكر مضاف")],
      specs=[S("origin", "India", "الهند"), S("storage", DRY)],
      img=["m:Tamarind Ball"],
      r=(4.5, 210), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 34.99), V("500g", 64.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2556, slug="schweppes-lemonade", brand="schweppes",
      name=("Schweppes lemonade", "شويبس ليمون"),
      sub=("Sparkling and lemony", "فوّار وبطعم الليمون"),
      desc=("Crisp, sparkling Schweppes lemonade. Serve it ice-cold with a slice of baladi lemon and a few "
            "mint leaves.",
            "شويبس ليمون فوّار ومنعش. قدّمه ساقع جدًا بشريحة ليمون بلدي وكام ورقة نعناع."),
      hl=[("1 L bottle", "إزازة 1 لتر"), ("Best served ice-cold", "أحلى وهو ساقع جدًا")],
      specs=[S("volume", "1 L", "1 لتر"), S("storage", DRY)],
      img=["m:Lemonade"],
      r=(4.3, 144), tags="",
      v=[V(None, 29.99, sku="1L")]),

    P(id=2557, slug="rose-water", brand="souq",
      name=("Rose water", "ماء ورد"),
      sub=("For sweets, sharbat and mahalabiya", "للحلويات والشربات والمهلبية"),
      desc=("Distilled rose water with a delicate floral aroma. A few drops perfume mahalabiya, konafa "
            "syrup, rice pudding and a glass of cold sharbat.",
            "ماء ورد مقطّر ريحته رقيقة. كام نقطة بتعطّر المهلبية وشربات الكنافة والرز باللبن وكوباية "
            "شربات ساقعة."),
      hl=[("Distilled", "مقطّر"), ("A few drops go far", "كام نقطة كفاية")],
      specs=[S("volume", "250 ml", "250 مل"), S("storage", DRY)],
      img=["m:Rose water"],
      r=(4.5, 132), tags="",
      v=[V(None, 39.99, sku="250ML")]),

    P(id=2558, slug="orange-blossom-water", brand="souq",
      name=("Orange blossom water", "ماء زهر"),
      sub=("For syrups, kahk and qatayef", "للشربات والكحك والقطايف"),
      desc=("Fragrant distilled orange blossom water. Add a splash to sugar syrup for qatayef and basbousa, "
            "or to kahk dough for that unmistakable Eid aroma.",
            "ماء زهر مقطّر ريحته فايحة. حط شوية في الشربات للقطايف والبسبوسة، أو في عجينة الكحك علشان "
            "ريحة العيد اللي متتنسيش."),
      hl=[("Distilled blossoms", "زهر مقطّر"), ("Ramadan and Eid essential", "أساسي في رمضان والعيد")],
      specs=[S("volume", "250 ml", "250 مل"), S("storage", DRY)],
      img=["m:Orange Blossom Water"],
      r=(4.5, 99), tags="",
      v=[V(None, 44.99, sku="250ML")]),
]

"""Nuts & snacks (category 130, ids 2601–2649)."""

from common import AIRTIGHT, DRY, L, P, S, V, axis, weights

CATEGORY = 130

KEEP = S("storage", AIRTIGHT)
ROASTED = S("roast", "Roasted in our roastery this week", "متحمص في محمصتنا الأسبوع ده")

PRODUCTS = [
    P(id=2601, slug="roasted-peanuts-in-shell", brand="souq",
      name=("Roasted peanuts in shell", "سوداني بقشره محمص"),
      sub=("Roasted today — the movie-night classic", "متحمص النهارده... رفيق السهرة"),
      desc=("Egyptian peanuts roasted in their shells in our own roastery, lightly salted. Crack them over a "
            "cup of tea or during a late-night match.",
            "فول سوداني مصري متحمص بقشره في محمصتنا، بملح خفيف. قزّقزه مع كوباية شاي أو في ماتش آخر "
            "الليل."),
      hl=[("Lightly salted", "ملح خفيف"), ("Roasted daily", "بيتحمص كل يوم")],
      specs=[S("origin", "Egypt", "مصر"), ROASTED, KEEP],
      img=["m:Peanuts"],
      r=(4.5, 1140), tags="bestseller express",
      options=[weights("250g", "500g", "1kg", loose=True)],
      v=[V("250g", 29.99), V("500g", 57.99), V("1kg", 112.99)]),

    P(id=2602, slug="roasted-cashews", brand="abu-auf",
      name=("Roasted cashews", "كاجو محمص"),
      sub=("Jumbo and lightly salted", "حبة جامبو بملح خفيف"),
      desc=("Jumbo W240 cashews roasted until golden with a touch of salt. A favourite on guest trays and in "
            "Eid boxes.",
            "كاجو جامبو W240 متحمص لحد ما يدهّب بملح خفيف. نجم صينية الضيوف وعلب العيد."),
      hl=[("W240 jumbo grade", "درجة جامبو W240"), ("Lightly salted", "ملح خفيف")],
      specs=[S("origin", "Vietnam", "فيتنام"), KEEP],
      img=["m:Cashews"],
      r=(4.7, 690), tags="featured",
      options=[weights("250g", "500g")],
      v=[V("250g", 219.99), V("500g", 429.99)]),

    P(id=2603, slug="raw-almonds", brand="abu-auf",
      name=("Raw almonds", "لوز ني"),
      sub=("For baking, snacking and soaking", "للحلويات والتسالي والنقع"),
      desc=("Whole raw almonds with the skin on. Soak them overnight, toast them for om ali and basbousa, "
            "or blitz them into almond milk.",
            "لوز ني بقشره... انقعه من بالليل، أو حمّصه لأم علي والبسبوسة، أو اضربه لبن لوز."),
      hl=[("Raw, unsalted", "ني ومن غير ملح"), ("California almonds", "لوز كاليفورنيا")],
      specs=[S("origin", "USA", "أمريكا"), KEEP],
      img=["m:Almonds"],
      r=(4.6, 340), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 189.99), V("500g", 369.99)]),

    P(id=2604, slug="walnuts-in-shell", brand="abu-auf",
      name=("Walnuts in shell", "عين جمل بقشره"),
      sub=("Crack your own, new crop", "قزّقز بنفسك... محصول جديد"),
      desc=("New-crop walnuts in their shells, with plump, buttery kernels. Crack them for a snack, or add "
            "them to Ramadan khoshaf and om ali.",
            "عين جمل بقشره من المحصول الجديد، قلبه مليان وطعمه زبدي. قزّقزه تسالي، أو حطه في خشاف "
            "رمضان وأم علي."),
      hl=[("New crop", "محصول جديد"), ("Easy-crack shells", "قشره سهل يتكسر")],
      specs=[S("origin", "Chile", "تشيلي"), KEEP],
      img=["m:Walnuts"],
      r=(4.4, 188), tags="",
      options=[weights("500g", "1kg", loose=True)],
      v=[V("500g", 149.99), V("1kg", 289.99)]),

    P(id=2605, slug="roasted-pistachios", brand="abu-auf",
      name=("Roasted pistachios", "فستق حلبي محمص"),
      sub=("Salted, in the shell", "مملح بقشره"),
      desc=("Plump, naturally opened pistachios roasted with sea salt — the star of every Eid nut tray.",
            "فستق حلبي مليان ومفتوح طبيعي، متحمص بملح بحري... نجم صينية مكسرات العيد."),
      hl=[("Naturally opened", "مفتوح طبيعي"), ("Roasted with sea salt", "متحمص بملح بحري")],
      specs=[S("origin", "Iran", "إيران"), KEEP],
      img=["m:Pistachio"],
      r=(4.8, 455), tags="featured",
      options=[weights("250g", "500g")],
      v=[V("250g", 299.99), V("500g", 589.99)]),

    P(id=2606, slug="roasted-hazelnuts", brand="abu-auf",
      name=("Roasted hazelnuts", "بندق محمص"),
      sub=("Toasty and sweet", "محمص وطعمه مسكّر"),
      desc=("Whole hazelnuts roasted until fragrant. Snack on them, chop them over ice cream, or blitz them "
            "into a homemade chocolate spread.",
            "بندق صحيح متحمص لحد ما ريحته تطلع. كُله تسالي، أو قطّعه على الآيس كريم، أو اضربه وأعمل "
            "شوكولاتة دهن في البيت."),
      hl=[("Whole kernels", "حبات كاملة"), ("Unsalted", "من غير ملح")],
      specs=[S("origin", "Turkey", "تركيا"), KEEP],
      img=["m:Hazlenuts"],
      r=(4.6, 166), tags="",
      v=[V(None, 209.99, sku="250G")]),

    P(id=2607, slug="golden-raisins", brand="abu-auf",
      name=("Golden raisins", "زبيب أصفر"),
      sub=("For khoshaf, om ali and rice", "للخشاف وأم علي والرز"),
      desc=("Soft, sweet golden raisins. Soak them with dried apricots and prunes for Ramadan khoshaf, "
            "scatter them over om ali, or stir them into rice with nuts.",
            "زبيب أصفر طري ومسكّر. انقعه مع المشمشية والقراصيا لخشاف رمضان، أو رشّه على أم علي، أو "
            "حطه على الرز بالمكسرات."),
      hl=[("Seedless", "من غير بذر"), ("Soft and plump", "طري ومليان")],
      specs=[S("origin", "Iran", "إيران"), KEEP],
      img=["m:Light Raisins"],
      r=(4.5, 288), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 59.99), V("500g", 114.99)]),

    P(id=2608, slug="dried-apricots", brand="abu-auf",
      name=("Dried apricots", "مشمشية"),
      sub=("Soft and tangy, a Ramadan essential", "طرية ومزّة... أساسية في رمضان"),
      desc=("Soft, tangy dried apricots — a must for khoshaf, and lovely chopped into yogurt or couscous.",
            "مشمشية طرية وطعمها مزّ... أساسية في الخشاف وحلوة متقطعة في الزبادي أو الكسكسي."),
      hl=[("Soft and juicy", "طرية ومليانة"), ("No added sugar", "من غير سكر مضاف")],
      specs=[S("origin", "Turkey", "تركيا"), KEEP],
      img=["m:Dried Apricots"],
      r=(4.6, 210), tags="",
      options=[weights("250g", "500g")],
      v=[V("250g", 99.99), V("500g", 194.99)]),

    P(id=2609, slug="lib-sury-sunflower-seeds", brand="souq",
      name=("Lib sury (sunflower seeds)", "لب سوري"),
      sub=("Roasted and salted — crack and chat", "متحمص ومملح... للقزقزة والرغي"),
      desc=("Striped sunflower seeds roasted with salt in our roastery — the essential crack-and-chat snack "
            "for evenings on the balcony.",
            "لب سوري متحمص بالملح في محمصتنا... أساس القزقزة والرغي في سهرة البلكونة."),
      hl=[("Roasted daily", "بيتحمص كل يوم"), ("Big, full seeds", "حباية كبيرة ومليانة")],
      specs=[ROASTED, KEEP],
      img=["m:Seeds"],
      r=(4.6, 980), tags="bestseller",
      options=[weights("250g", "500g", "1kg", loose=True)],
      v=[V("250g", 44.99), V("500g", 86.99), V("1kg", 169.99)]),

    P(id=2610, slug="oreo-original", brand="oreo",
      name=("Oreo original", "بسكويت أوريو"),
      sub=("Twist, lick, dunk", "لف، الحس، غمّس"),
      desc=("The original chocolate sandwich cookie with vanilla-flavoured cream. Dunk it in cold milk, "
            "crumble it over ice cream, or bake it into an Oreo cheesecake.",
            "بسكويت أوريو الأصلي بالكريمة بطعم الفانيليا. غمّسه في لبن ساقع، أو فتّته على الآيس كريم، "
            "أو اعمل بيه تشيز كيك أوريو."),
      hl=[("Snack packs", "باكتات سناك"), ("12-pack on offer", "الـ12 باكت عليها عرض")],
      specs=[S("storage", DRY)],
      img=["m:Oreo Cream Biscuits"],
      r=(4.7, 760), tags="deal",
      options=[axis("pack", [("6", L("6 packs", "6 باكت")), ("12", L("12 packs", "12 باكت"))])],
      v=[V("6", 49.99), V("12", 94.99, was=99.98)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2611, slug="prunes", brand="abu-auf",
      name=("Prunes", "قراصيا"),
      sub=("Soft pitted prunes for khoshaf", "قراصيا طرية من غير نوى للخشاف"),
      desc=("Soft, sweet pitted prunes — a khoshaf essential alongside apricots and raisins, and a gentle "
            "snack on their own.",
            "قراصيا طرية ومسكّرة من غير نوى... أساسية في الخشاف مع المشمشية والزبيب، وحلوة لوحدها "
            "كتسالي."),
      hl=[("Pitted", "من غير نوى"), ("Soft and juicy", "طرية ومليانة")],
      specs=[S("origin", "Chile", "تشيلي"), KEEP],
      img=["m:Prunes"],
      r=(4.4, 132), tags="",
      v=[V(None, 89.99, sku="250G")]),

    P(id=2612, slug="dark-raisins", brand="abu-auf",
      name=("Dark raisins", "زبيب أسود"),
      sub=("Sweet and chewy", "مسكّر وطري"),
      desc=("Plump, dark seedless raisins. Mix them into couscous with sugar and butter, bake them into "
            "cakes, or add them to belila.",
            "زبيب أسود مليان من غير بذر. قلّبه في الكسكسي بالسكر والزبدة، أو حطه في الكيك، أو على "
            "البليلة."),
      hl=[("Seedless", "من غير بذر"), ("Great in baking", "ممتاز في الخبيز")],
      specs=[S("origin", "Iran", "إيران"), KEEP],
      img=["m:Currants"],
      r=(4.3, 98), tags="",
      v=[V(None, 54.99, sku="250G")]),

    P(id=2613, slug="desiccated-coconut", brand="abu-auf",
      name=("Desiccated coconut", "جوز هند مبشور"),
      sub=("For basbousa, belila and truffles", "للبسبوسة والبليلة والترافل"),
      desc=("Finely shredded unsweetened coconut. Fold it into basbousa batter, sprinkle it over belila or "
            "roll chocolate truffles in it.",
            "جوز هند مبشور ناعم من غير سكر. حطه في خلطة البسبوسة، أو رشّه على البليلة، أو غلّف بيه "
            "كور الشوكولاتة."),
      hl=[("Unsweetened", "من غير سكر"), ("Fine shred", "مبشور ناعم")],
      specs=[S("origin", "Sri Lanka", "سريلانكا"), KEEP],
      img=["m:Desiccated Coconut"],
      r=(4.4, 121), tags="",
      options=[weights("200g", "500g")],
      v=[V("200g", 49.99), V("500g", 114.99)]),
]

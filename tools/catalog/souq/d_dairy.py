"""Dairy & eggs (category 30, ids 2101–2149)."""

from common import EGYPT, FRIDGE, L, P, S, V, axis, weights

CATEGORY = 30

PRODUCTS = [
    P(id=2101, slug="juhayna-full-cream-milk", brand="juhayna",
      name=("Juhayna full-cream milk", "لبن جهينة كامل الدسم"),
      sub=("Long-life milk for the whole family", "لبن طويل الأجل للبيت كله"),
      desc=("Juhayna's full-cream UHT milk — the carton found in nearly every Egyptian fridge door. Rich "
            "enough for tea with milk, rice pudding and béchamel, and it keeps unopened in the cupboard.",
            "لبن جهينة كامل الدسم معقّم... الكرتونة اللي موجودة في باب تلاجة كل بيت تقريبًا. تقيل "
            "كفاية للشاي باللبن والرز باللبن والبشاميل، ويتخزن في الدولاب قبل الفتح."),
      hl=[("3% fat", "دسم 3%"), ("Six-pack saves on every litre", "كرتونة الست أوفر في كل لتر")],
      specs=[S("fat", "3% (full cream)", "3% (كامل الدسم)"), S("storage", "Cupboard; fridge after opening (3 days)",
                                    "في الدولاب، وبعد الفتح في التلاجة 3 أيام"), S("origin", EGYPT)],
      img=["m:Milk"],
      r=(4.7, 2210), tags="bestseller express deal",
      options=[axis("pack", [("1l", L("1 L", "1 لتر")), ("6x1l", L("6 × 1 L", "6 × 1 لتر"))])],
      v=[V("1l", 54.99), V("6x1l", 309.99, was=329.94)]),

    P(id=2102, slug="fresh-buffalo-milk",
      name=("Fresh buffalo milk", "لبن جاموسي طازة"),
      sub=("Thick, creamy, straight from the farm", "تقيل وبالقشطة... من المزرعة على طول"),
      desc=("Fresh pasteurised buffalo milk from a Beheira farm — richer and creamier than cow's milk, with "
            "a layer of ishta on top once boiled. Use it for rice pudding, mahalabiya and a seriously good "
            "cup of tea with milk.",
            "لبن جاموسي طازة مبستر من مزرعة في البحيرة... أتقل وفيه قشطة أكتر من البقري، ولما يتغلي "
            "بيعمل وش قشطة. ينفع للرز باللبن والمهلبية وأحلى كوباية شاي بلبن."),
      hl=[("About 7% fat", "دسم حوالي 7%"), ("Delivered chilled", "بيوصل متلّج")],
      specs=[S("fat", "About 7%", "حوالي 7%"), S("storage", FRIDGE), S("shelf", "4 days", "4 أيام"),
             S("origin", "Beheira, Egypt", "البحيرة، مصر")],
      img=["d:milk-1"],
      r=(4.8, 640), tags="featured express",
      options=[axis("size", [("1l", L("1 L", "1 لتر")), ("2l", L("2 L", "2 لتر"))])],
      v=[V("1l", 59.99), V("2l", 114.99, stock=0)]),

    P(id=2103, slug="red-eggs",
      name=("Red eggs", "بيض أحمر"),
      sub=("Fresh farm eggs, by the box or tray", "بيض مزارع طازة بالعلبة أو الكرتونة"),
      desc=("Brown-shelled eggs from Sharqia farms, collected within the week and candled before packing. "
            "Choose a box of 10 or the 30-egg tray Egyptian homes stock up on.",
            "بيض قشره أحمر من مزارع الشرقية، متجمع في نفس الأسبوع ومتكشف عليه قبل التعبئة. خد علبة 10 "
            "أو كرتونة 30 زي ما البيوت المصرية متعودة."),
      hl=[("Large, 60–65 g each", "حجم كبير، 60–65 جم للبيضة"), ("Tray of 30 on offer", "كرتونة الـ30 بسعر أوفر")],
      specs=[S("origin", "Sharqia, Egypt", "الشرقية، مصر"), S("storage", FRIDGE),
             S("shelf", "21 days from packing", "21 يوم من التعبئة")],
      img=["d:eggs-1"],
      r=(4.6, 1730), tags="bestseller express deal",
      options=[axis("count", [("10", L("10 eggs", "10 بيضات")), ("30", L("Tray of 30", "كرتونة 30"))])],
      v=[V("10", 74.99), V("30", 209.99, was=224.99)]),

    P(id=2104, slug="baladi-eggs",
      name=("Baladi eggs", "بيض بلدي"),
      sub=("Free-range village eggs with deep yolks", "بيض فلاحي صفاره غامق"),
      desc=("Small free-range eggs from village hens, with deep-orange yolks and real flavour. Perfect "
            "soft-boiled, in an omelette with pastirma, or in your Sham el-Nessim basket.",
            "بيض بلدي صغير من فراخ متربية في الريف، صفاره برتقالي غامق وطعمه حقيقي. حلو مسلوق "
            "برشت أو أومليت بالبسطرمة أو في سلة شم النسيم."),
      hl=[("Free-range hens", "فراخ متربية سرح"), ("Deep-orange yolks", "صفار برتقالي غامق")],
      specs=[S("origin", "Fayoum, Egypt", "الفيوم، مصر"), S("storage", FRIDGE)],
      img=["m:Eggs"],
      r=(4.7, 512), tags="",
      options=[axis("count", [("10", L("10 eggs", "10 بيضات")), ("20", L("20 eggs", "20 بيضة"))])],
      v=[V("10", 89.99), V("20", 174.99)]),

    P(id=2105, slug="anchor-unsalted-butter", brand="anchor",
      name=("Anchor unsalted butter", "زبدة أنكور غير مملحة"),
      sub=("New Zealand butter for baking and toast", "زبدة نيوزيلاندي للخبيز والتوست"),
      desc=("Creamy pasture-fed butter from New Zealand, unsalted so you control the seasoning. A favourite "
            "for ghorayeba, kahk dough and a Friday-morning toast.",
            "زبدة كريمية من نيوزيلاندا من أبقار متربية على المراعي، غير مملحة علشان تظبط الملح بنفسك. "
            "المفضلة لعجينة الغريبة والكحك وتوست الجمعة الصبح."),
      hl=[("82% milk fat", "دسم لبن 82%"), ("Ideal for kahk and ghorayeba", "مثالية للكحك والغريبة")],
      specs=[S("fat", "82% milk fat", "دسم لبن 82%"), S("origin", "New Zealand", "نيوزيلاندا"),
             S("storage", FRIDGE)],
      img=["m:Butter"],
      r=(4.8, 860), tags="",
      options=[weights("200g", "400g")],
      v=[V("200g", 154.99), V("400g", 299.99)]),
]

# ---- pass 2 -----------------------------------------------------------------------------------
PRODUCTS += [
    P(id=2106, slug="fresh-ishta",
      name=("Fresh ishta", "قشطة بلدي"),
      sub=("Thick buffalo cream for honey and feteer", "قشطة جاموسي تقيلة للعسل والفطير"),
      desc=("Thick, spoonable ishta skimmed from buffalo milk. Spread it on feteer meshaltet with honey or "
            "black honey, or crown a plate of om ali and konafa.",
            "قشطة بلدي تقيلة من لبن الجاموس. ادهنها على الفطير المشلتت بالعسل أو العسل الأسود، أو "
            "زيّن بيها أم علي والكنافة."),
      hl=[("From buffalo milk", "من لبن جاموسي"), ("Eat within 4 days", "تتاكل خلال 4 أيام")],
      specs=[S("storage", FRIDGE), S("shelf", "4 days", "4 أيام"), S("origin", "Beheira, Egypt", "البحيرة، مصر")],
      img=["m:Clotted Cream"],
      r=(4.8, 455), tags="featured",
      options=[weights("250g", "500g")],
      v=[V("250g", 69.99), V("500g", 134.99)]),

    P(id=2107, slug="alpro-soya-unsweetened", brand="alpro",
      name=("Alpro soya drink, unsweetened", "ألبرو لبن صويا من غير سكر"),
      sub=("Plant-based, no added sugar", "نباتي ومن غير سكر مضاف"),
      desc=("Alpro's unsweetened soya drink, naturally low in sugar and fortified with calcium and vitamins. "
            "Pour it on cereal, into coffee, or use it in baking.",
            "لبن صويا من ألبرو من غير سكر، قليل السكر طبيعيًا ومدعّم بالكالسيوم والفيتامينات. حطه على "
            "السيريال أو في القهوة أو استعمله في الخبيز."),
      hl=[("No added sugar", "من غير سكر مضاف"), ("Calcium-fortified", "مدعّم بالكالسيوم"),
          ("Dairy-free", "خالي من اللبن البقري")],
      specs=[S("origin", "Belgium", "بلجيكا"), S("storage", "Cupboard; fridge after opening (5 days)",
                                                    "في الدولاب، وبعد الفتح في التلاجة 5 أيام")],
      img=["m:Soya Milk"],
      r=(4.4, 98), tags="",
      v=[V(None, 229.99, sku="1L")]),

    P(id=2108, slug="oldenburger-whipping-cream", brand="oldenburger",
      name=("Oldenburger whipping cream", "كريمة خفق أولدنبورجر"),
      sub=("35% fat, whips up firm", "دسم 35%... بتتخفق تقيلة"),
      desc=("German UHT whipping cream with 35% fat that whips into firm, stable peaks. For cakes, "
            "trifles, pasta sauces and a splash in your hot chocolate.",
            "كريمة خفق ألماني دسم 35% بتتخفق وتقف تقيلة وثابتة. للتورت والترايفل وصوصات المكرونة "
            "وشوية في الشوكولاتة السخنة."),
      hl=[("35% fat", "دسم 35%"), ("Whip well chilled", "اخفقها وهي ساقعة")],
      specs=[S("fat", "35% fat", "دسم 35%"), S("origin", "Germany", "ألمانيا"), S("storage", "Fridge", "في التلاجة")],
      img=["m:Whipping Cream"],
      r=(4.6, 177), tags="",
      options=[axis("size", [("200ml", L("200 ml", "200 مل")), ("1l", L("1 L", "1 لتر"))])],
      v=[V("200ml", 74.99), V("1l", 329.99)]),
]

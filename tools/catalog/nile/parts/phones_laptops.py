"""Phones (category 10) and Laptops & Tablets (category 11): products 1001–1016 and 1101–1108.

Prices are whole EGP; L10n values are (en, ar) tuples. Slug, category, brand, tags and images come
from skeleton.py.
"""


def gb(n, code=None):
    d = dict(id=str(n), label=(f"{n} GB", f"{n} جيجابايت"))
    if code:
        d["code"] = code
    return d


STORAGE = ("Storage", "السعة التخزينية")
MEM_STORAGE = ("Memory & storage", "الذاكرة والسعة")
CONFIG = ("Configuration", "المواصفات")
CONNECTIVITY = ("Connectivity", "الاتصال")

W6 = (("Warranty", "الضمان"), ("6 months from Nile + 14-day returns", "6 أشهر من نايل + استرجاع خلال 14 يومًا"))
W12 = (("Warranty", "الضمان"), ("1-year local warranty", "ضمان محلي لمدة عام"))

BACK_FRONT = ("back and front", "من الخلف والأمام")

PRODUCTS = [
    # ------------------------------------------------------------------ Phones
    dict(
        id=1001,
        name=("Apple iPhone 13 Pro (Renewed) — Sierra Blue", "أبل آيفون 13 برو (مُجدَّد) — أزرق سييرا"),
        subtitle=("Pro cameras and a 120 Hz screen, 6-month warranty", "كاميرات برو وشاشة 120 هرتز، وضمان 6 شهور"),
        description=(
            "A certified renewed iPhone 13 Pro in Sierra Blue, with the 120 Hz ProMotion display and a triple "
            "12 MP camera system that still holds its own. Nile technicians inspect, reset and deep-clean every "
            "unit, and the battery is guaranteed at 85% health or better. Choose Excellent for a phone that looks "
            "like new, or Very good to save a little with light signs of use.",
            "آيفون 13 برو مُجدَّد ومعتمد باللون الأزرق سييرا، بشاشة ProMotion بتردد 120 هرتز ونظام كاميرات "
            "ثلاثي بدقة 12 ميجابكسل ما زال ينافس بقوة. يفحص فنيو نايل كل جهاز ويعيدون ضبطه وينظفونه بالكامل، "
            "وصحة بطاريته مضمونة بنسبة 85% أو أكثر. اختر حالة «ممتازة» لجهاز يبدو كالجديد، أو «جيدة جدًا» "
            "لتوفّر أكثر مع آثار استخدام خفيفة.",
        ),
        highlights=[
            ("Passed a 70-point inspection, reset and deep-cleaned", "اجتاز فحصًا من 70 نقطة، ومُعاد ضبطه ومنظَّف بالكامل"),
            ("Battery health 85% or higher, guaranteed", "صحة البطارية 85% أو أكثر بضمان"),
            ("Cinematic mode, macro photos and 3× optical zoom", "وضع التصوير السينمائي وصور الماكرو وتقريب بصري 3×"),
            ("6-month Nile warranty and 14-day returns", "ضمان نايل 6 أشهر واسترجاع خلال 14 يومًا"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.1" Super Retina XDR OLED, ProMotion up to 120 Hz',
                                    "6.1 بوصة Super Retina XDR OLED بتقنية ProMotion حتى 120 هرتز")),
            (("Chip", "الشريحة"), ("A15 Bionic with 5-core GPU", "A15 Bionic مع معالج رسومات خماسي النوى")),
            (("Cameras", "الكاميرات"), ("12 MP wide, ultra-wide and 3× telephoto + LiDAR; 12 MP front",
                                       "ثلاث كاميرات 12 ميجابكسل (واسعة وفائقة الاتساع وتقريب 3×) مع LiDAR، وأمامية 12 ميجابكسل")),
            (("Battery", "البطارية"), ("Health 85%+, up to 22 hours of video playback",
                                      "صحة 85% أو أكثر، وتشغيل فيديو حتى 22 ساعة")),
            (("Condition", "الحالة"), ("Excellent or Very good grade", "حالة ممتازة أو جيدة جدًا")),
            W6,
        ],
        options=[
            dict(id="storage", name=STORAGE, kind="chip", values=[gb(128), gb(256)]),
            dict(id="condition", name=("Condition", "الحالة"), kind="chip", values=[
                dict(id="excellent", code="EX", label=("Excellent — like new", "ممتازة — كالجديد")),
                dict(id="very-good", code="VG", label=("Very good — light marks", "جيدة جدًا — خدوش خفيفة")),
            ]),
        ],
        variants=[
            dict(opts=["128", "excellent"], price=23999, compare_at=26999),
            dict(opts=["128", "very-good"], price=21999, compare_at=24999),
            dict(opts=["256", "excellent"], price=26999, compare_at=29999),
            dict(opts=["256", "very-good"], price=24999, compare_at=27999),
        ],
        modifier_groups=[
            dict(id="protect", name=("Protection", "الحماية"), selection="multi", min=0, max=2, required=False,
                 options=[
                     dict(id="glass", name=("Screen protector, fitted", "واقي شاشة مع التركيب"), price=249),
                     dict(id="warranty6", name=("+6 months warranty", "ضمان إضافي 6 أشهر"), price=999),
                 ]),
        ],
        rating=4.5, reviews=912,
        alt=("Apple iPhone 13 Pro in Sierra Blue", "آيفون 13 برو باللون الأزرق سييرا"),
        views=[
            ("back with the triple camera", "من الخلف مع الكاميرا الثلاثية"),
            ("front, display on", "من الأمام والشاشة مضاءة"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1002,
        name=("Apple iPhone X (Renewed) — Space Grey", "أبل آيفون X (مُجدَّد) — رمادي فلكي"),
        subtitle=("Face ID and an edge-to-edge OLED for less", "Face ID وشاشة OLED من الحافة للحافة بسعر أوفر"),
        description=(
            "The iPhone that introduced Face ID and the all-screen design, renewed to grade A. You get a clean "
            "5.8-inch Super Retina OLED display, a dual 12 MP camera with 2× optical zoom and Portrait Lighting, "
            "and a battery at 85% health or better. It runs up to iOS 16 — a smart first iPhone or a dependable "
            "second phone.",
            "الآيفون الذي قدّم Face ID والتصميم بالشاشة الكاملة، مُجدَّد بدرجة A. ستحصل على شاشة Super Retina "
            "OLED نظيفة مقاس 5.8 بوصة، وكاميرا مزدوجة بدقة 12 ميجابكسل بتقريب بصري 2× وإضاءة البورتريه، "
            "وبطارية بصحة 85% أو أكثر. يعمل حتى نظام iOS 16، فهو اختيار ذكي كأول آيفون أو كهاتف ثانٍ يُعتمد عليه.",
        ),
        highlights=[
            ("Grade A: minimal signs of use", "درجة A: آثار استخدام طفيفة جدًا"),
            ("Face ID and wireless charging", "Face ID وشحن لاسلكي"),
            ("Battery health 85% or higher", "صحة البطارية 85% أو أكثر"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('5.8" Super Retina OLED, 2436 × 1125', "5.8 بوصة Super Retina OLED بدقة 2436 × 1125")),
            (("Chip", "الشريحة"), ("A11 Bionic", "شريحة A11 Bionic")),
            (("Cameras", "الكاميرات"), ("Dual 12 MP (wide + 2× telephoto); 7 MP TrueDepth front",
                                       "مزدوجة 12 ميجابكسل (واسعة + تقريب 2×) وأمامية TrueDepth بدقة 7 ميجابكسل")),
            (("Software", "نظام التشغيل"), ("Supports up to iOS 16", "يدعم حتى نظام iOS 16")),
            (("Condition", "الحالة"), ("Grade A, battery health 85%+", "درجة A، وصحة البطارية 85% أو أكثر")),
            W6,
        ],
        options=[dict(id="storage", name=STORAGE, kind="chip", values=[gb(64), gb(256)])],
        variants=[
            dict(opts=["64"], price=7999),
            dict(opts=["256"], price=9299),
        ],
        rating=4.3, reviews=386,
        alt=("Apple iPhone X in Space Grey", "آيفون X باللون الرمادي الفلكي"),
        views=[
            ("back", "من الخلف"),
            ("front, home screen", "من الأمام مع الشاشة الرئيسية"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1003,
        name=("Apple iPhone 6s (Renewed) — Silver", "أبل آيفون 6s (مُجدَّد) — فضي"),
        subtitle=("Compact iPhone with Touch ID, ideal as a first phone", "آيفون صغير ببصمة Touch ID، ينفع أول موبايل"),
        description=(
            "A compact 4.7-inch iPhone with Touch ID and a 12 MP camera that records 4K video — renewed, tested "
            "and ready for calls, WhatsApp and photos. It supports up to iOS 15, which makes it a sensible first "
            "phone for a teenager or a dependable backup.",
            "آيفون مدمج بشاشة 4.7 بوصة مع بصمة Touch ID وكاميرا 12 ميجابكسل تسجّل فيديو بدقة 4K، مُجدَّد ومفحوص "
            "وجاهز للمكالمات وواتساب والصور. يدعم حتى نظام iOS 15، لذلك يناسب كأول موبايل لابنك أو ابنتك أو كهاتف "
            "احتياطي يُعتمد عليه.",
        ),
        highlights=[
            ("Tested, factory reset and cleaned", "مفحوص ومُعاد لضبط المصنع ومنظَّف"),
            ("Touch ID and 3D Touch", "بصمة Touch ID وخاصية 3D Touch"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('4.7" Retina HD, 1334 × 750', "4.7 بوصة Retina HD بدقة 1334 × 750")),
            (("Chip", "الشريحة"), ("A9", "شريحة A9")),
            (("Cameras", "الكاميرات"), ("12 MP rear with 4K video; 5 MP front", "خلفية 12 ميجابكسل بفيديو 4K، وأمامية 5 ميجابكسل")),
            (("Software", "نظام التشغيل"), ("Supports up to iOS 15", "يدعم حتى نظام iOS 15")),
            W6,
        ],
        options=[dict(id="storage", name=STORAGE, kind="chip", values=[gb(32), gb(128)])],
        variants=[
            dict(opts=["32"], price=3299),
            dict(opts=["128"], price=3799),
        ],
        rating=4.1, reviews=233,
        alt=("Apple iPhone 6s in Silver", "آيفون 6s باللون الفضي"),
        views=[
            ("back", "من الخلف"),
            ("front", "من الأمام"),
            ("front and back, side angle", "من الأمام والخلف بزاوية جانبية"),
        ],
    ),
    dict(
        id=1004,
        name=("Apple iPhone SE 1st Gen (Renewed)", "أبل آيفون SE الجيل الأول (مُجدَّد)"),
        subtitle=("The 4-inch iPhone that fits any pocket", "الآيفون الـ 4 بوصة اللي يدخل أي جيب"),
        description=(
            "The original 4-inch iPhone SE: the classic compact design with the A9 chip and a 12 MP camera that "
            "records 4K video. This renewed unit comes with 32 GB of storage — ideal for kids, light users or "
            "anyone who misses a true one-handed phone. Supports up to iOS 15.",
            "آيفون SE الأصلي بمقاس 4 بوصة: التصميم الصغير الكلاسيكي مع شريحة A9 وكاميرا 12 ميجابكسل تسجّل فيديو "
            "4K. يأتي هذا الجهاز المُجدَّد بسعة 32 جيجابايت، وهو مثالي للأطفال وللاستخدام الخفيف ولكل من يفتقد "
            "هاتفًا يُستخدم بيد واحدة فعلًا. يدعم حتى نظام iOS 15.",
        ),
        highlights=[
            ("Truly one-handed 4-inch design", "تصميم 4 بوصة يُستخدم بيد واحدة فعلًا"),
            ("Touch ID home button", "زر رئيسي ببصمة Touch ID"),
            ("Tested, with a 6-month Nile warranty", "مفحوص وبضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('4" Retina, 1136 × 640', "4 بوصة Retina بدقة 1136 × 640")),
            (("Chip", "الشريحة"), ("A9 with M9 motion coprocessor", "شريحة A9 مع معالج الحركة M9")),
            (("Storage", "السعة التخزينية"), ("32 GB", "32 جيجابايت")),
            (("Cameras", "الكاميرات"), ("12 MP rear with 4K video; 1.2 MP front", "خلفية 12 ميجابكسل بفيديو 4K، وأمامية 1.2 ميجابكسل")),
            W6,
        ],
        options=[dict(id="color", name=("Colour", "اللون"), kind="swatch", values=[
            dict(id="space-grey", label=("Space Grey", "رمادي فلكي"), swatch="#4A4B4F"),
            dict(id="silver", label=("Silver", "فضي"), swatch="#D8D9DB"),
        ])],
        variants=[
            dict(opts=["space-grey"], price=2599, image=0),
            dict(opts=["silver"], price=2599, image=2),
        ],
        rating=4.0, reviews=141,
        alt=("Apple iPhone SE (1st generation)", "آيفون SE الجيل الأول"),
        views=[
            ("Space Grey, back", "الرمادي الفلكي من الخلف"),
            ("front, home screen", "من الأمام مع الشاشة الرئيسية"),
            ("Silver back and Space Grey front", "الفضي من الخلف والرمادي الفلكي من الأمام"),
        ],
    ),
    dict(
        id=1005,
        name=("Samsung Galaxy S10+ (Renewed) — Prism Black", "سامسونج جالاكسي S10+ (مُجدَّد) — أسود بريزم"),
        subtitle=("Triple camera and a 6.4-inch Dynamic AMOLED", "تلات كاميرات وشاشة Dynamic AMOLED مقاس 6.4 بوصة"),
        description=(
            "Samsung's 2019 flagship, renewed: a 6.4-inch QHD+ Dynamic AMOLED with HDR10+, a triple rear camera "
            "with ultra-wide, and two selfie cameras in the pill-shaped cut-out. The 4,100 mAh battery supports "
            "wireless charging and Wireless PowerShare for topping up your earbuds. Every unit passes a full "
            "functional test and ships with a 6-month Nile warranty.",
            "هاتف سامسونج الرائد لعام 2019 بحالة مُجدَّدة: شاشة Dynamic AMOLED بدقة QHD+ مقاس 6.4 بوصة تدعم "
            "HDR10+، وكاميرا خلفية ثلاثية منها عدسة فائقة الاتساع، وكاميرتان للسيلفي في الفتحة البيضاوية. بطارية "
            "4100 مللي أمبير تدعم الشحن اللاسلكي ومشاركة الطاقة لشحن سماعاتك، وكل جهاز يجتاز فحصًا كاملًا ويصلك "
            "بضمان نايل لمدة 6 أشهر.",
        ),
        highlights=[
            ("Ultrasonic in-display fingerprint reader", "قارئ بصمة فوق صوتي داخل الشاشة"),
            ("Wireless charging and Wireless PowerShare", "شحن لاسلكي ومشاركة الطاقة لاسلكيًا"),
            ("Battery health 85%+, 6-month warranty", "صحة البطارية 85% أو أكثر وضمان 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.4" Dynamic AMOLED QHD+, HDR10+', "6.4 بوصة Dynamic AMOLED بدقة QHD+ وتدعم HDR10+")),
            (("Processor", "المعالج"), ("Exynos 9820, 8 GB RAM", "Exynos 9820 مع ذاكرة 8 جيجابايت")),
            (("Rear camera", "الكاميرا الخلفية"), ("12 MP wide + 12 MP telephoto + 16 MP ultra-wide",
                                                  "12 ميجابكسل واسعة + 12 ميجابكسل تقريب + 16 ميجابكسل فائقة الاتساع")),
            (("Front camera", "الكاميرا الأمامية"), ("10 MP + 8 MP depth", "10 ميجابكسل + 8 ميجابكسل للعمق")),
            (("Battery", "البطارية"), ("4,100 mAh, fast wired and wireless charging", "4100 مللي أمبير بشحن سريع سلكي ولاسلكي")),
            (("Software", "نظام التشغيل"), ("Android 12 (One UI 4.1)", "أندرويد 12 بواجهة One UI 4.1")),
        ],
        options=[dict(id="storage", name=STORAGE, kind="chip", values=[gb(128), gb(512)])],
        variants=[
            dict(opts=["128"], price=8999, compare_at=10499),
            dict(opts=["512"], price=10999, compare_at=12499),
        ],
        rating=4.3, reviews=528,
        alt=("Samsung Galaxy S10+ in Prism Black", "سامسونج جالاكسي S10+ باللون الأسود بريزم"),
        views=[
            ("back with the triple camera", "من الخلف مع الكاميرا الثلاثية"),
            ("front, display on", "من الأمام والشاشة مضاءة"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1006,
        name=("Samsung Galaxy S8 (Renewed) — Midnight Black", "سامسونج جالاكسي S8 (مُجدَّد) — أسود ليلي"),
        subtitle=("Curved Infinity Display and a microSD slot", "شاشة Infinity منحنية ومكان لكارت ميموري"),
        description=(
            "The phone that started Samsung's Infinity Display era: a curved 5.8-inch Quad HD+ Super AMOLED that "
            "still looks premium. It has a 12 MP Dual Pixel camera, wireless charging and a microSD slot to add "
            "storage beyond the built-in 64 GB. Renewed, tested and backed by a 6-month Nile warranty.",
            "الهاتف الذي بدأ به عصر شاشات Infinity من سامسونج: شاشة Super AMOLED منحنية بدقة Quad HD+ مقاس "
            "5.8 بوصة ما زالت تبدو فاخرة. يضم كاميرا Dual Pixel بدقة 12 ميجابكسل وشحنًا لاسلكيًا ومنفذًا لبطاقة "
            "microSD لزيادة المساحة فوق الـ 64 جيجابايت المدمجة. مُجدَّد ومفحوص وبضمان نايل لمدة 6 أشهر.",
        ),
        highlights=[
            ("Edge-to-edge curved Infinity Display", "شاشة Infinity منحنية من الحافة للحافة"),
            ("microSD slot up to 256 GB", "منفذ microSD حتى 256 جيجابايت"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('5.8" Quad HD+ Super AMOLED, curved', "5.8 بوصة Super AMOLED منحنية بدقة Quad HD+")),
            (("Processor", "المعالج"), ("Exynos 8895, 4 GB RAM", "Exynos 8895 مع ذاكرة 4 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("64 GB + microSD up to 256 GB", "64 جيجابايت + microSD حتى 256 جيجابايت")),
            (("Cameras", "الكاميرات"), ("12 MP Dual Pixel f/1.7 rear; 8 MP autofocus front",
                                       "خلفية 12 ميجابكسل Dual Pixel بفتحة f/1.7، وأمامية 8 ميجابكسل بتركيز تلقائي")),
            (("Battery", "البطارية"), ("3,000 mAh, fast and wireless charging", "3000 مللي أمبير بشحن سريع ولاسلكي")),
            W6,
        ],
        variants=[dict(opts=[], price=4699)],
        rating=4.2, reviews=344,
        alt=("Samsung Galaxy S8 in Midnight Black", "سامسونج جالاكسي S8 باللون الأسود الليلي"),
        views=[
            ("back", "من الخلف"),
            ("front, Infinity Display", "من الأمام بشاشة Infinity"),
            ("front and back", "من الأمام والخلف"),
        ],
    ),
    dict(
        id=1007,
        name=("Samsung Galaxy S7 edge (Renewed) — Black Onyx", "سامسونج جالاكسي S7 إيدج (مُجدَّد) — أسود أونيكس"),
        subtitle=("Dual-edge screen and a 3,600 mAh battery", "شاشة منحنية من الناحيتين وبطارية 3600 مللي أمبير"),
        description=(
            "A renewed Galaxy S7 edge with the dual-curved 5.5-inch QHD Super AMOLED that made it a design icon. "
            "Its bright f/1.7 Dual Pixel camera still copes well in low light, and the 3,600 mAh battery "
            "comfortably lasts a day of calls and messaging. A budget-friendly second phone with a 6-month Nile "
            "warranty.",
            "جالاكسي S7 إيدج مُجدَّد بشاشة Super AMOLED منحنية من الجانبين بدقة QHD ومقاس 5.5 بوصة جعلته أيقونة "
            "في التصميم. كاميرته Dual Pixel بفتحة f/1.7 ما زالت تتعامل جيدًا مع الإضاءة المنخفضة، وبطاريته 3600 "
            "مللي أمبير تكفي يومًا من المكالمات والرسائل بسهولة. هاتف ثانٍ اقتصادي بضمان نايل لمدة 6 أشهر.",
        ),
        highlights=[
            ("Dual-curved screen with Edge panels", "شاشة منحنية من الجانبين مع لوحات Edge"),
            ("microSD slot for extra storage", "منفذ microSD لمساحة إضافية"),
            ("Tested, with a 6-month Nile warranty", "مفحوص وبضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('5.5" QHD Super AMOLED, dual edge', "5.5 بوصة Super AMOLED بدقة QHD ومنحنية من الجانبين")),
            (("Processor", "المعالج"), ("Exynos 8890, 4 GB RAM", "Exynos 8890 مع ذاكرة 4 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("32 GB + microSD up to 256 GB", "32 جيجابايت + microSD حتى 256 جيجابايت")),
            (("Cameras", "الكاميرات"), ("12 MP Dual Pixel f/1.7 rear; 5 MP front",
                                       "خلفية 12 ميجابكسل Dual Pixel بفتحة f/1.7، وأمامية 5 ميجابكسل")),
            (("Battery", "البطارية"), ("3,600 mAh, wireless charging", "3600 مللي أمبير بشحن لاسلكي")),
            W6,
        ],
        variants=[dict(opts=[], price=3399)],
        rating=4.0, reviews=197,
        alt=("Samsung Galaxy S7 edge in Black Onyx", "سامسونج جالاكسي S7 إيدج باللون الأسود أونيكس"),
        views=[
            ("back", "من الخلف"),
            ("front, curved edge display", "من الأمام بالشاشة المنحنية"),
            ("front and back", "من الأمام والخلف"),
        ],
    ),
    dict(
        id=1008,
        name=("OPPO F19 Pro+ 5G — Space Silver", "أوبو F19 برو+ 5G — فضي"),
        subtitle=("Brand new: 50 W Flash Charge and a 48 MP quad camera",
                  "جديد بكرتونته: شحن 50 واط وكاميرا رباعية 48 ميجابكسل"),
        description=(
            "A brand-new, sealed OPPO F19 Pro+ 5G from the last batch in Egypt, now well below its launch price. "
            "The 6.43-inch AMOLED, 5G-ready Dimensity 800U and 48 MP quad camera cover everyday needs, while "
            "50 W Flash Charge takes the 4,310 mAh battery from empty to full in about 48 minutes.",
            "أوبو F19 برو+ 5G جديد بكرتونته من آخر دفعة في مصر، وبسعر أقل بكثير من سعر طرحه. شاشة AMOLED مقاس "
            "6.43 بوصة ومعالج Dimensity 800U الداعم لشبكات 5G وكاميرا رباعية بدقة 48 ميجابكسل تغطي احتياجاتك "
            "اليومية، بينما يملأ شحن Flash Charge بقدرة 50 واط البطارية 4310 مللي أمبير في نحو 48 دقيقة.",
        ),
        highlights=[
            ("New and sealed, 1-year local warranty", "جديد بكرتونته وضمان محلي لمدة عام"),
            ("50 W Flash Charge: full in about 48 minutes", "شحن Flash Charge بقدرة 50 واط: شحن كامل في نحو 48 دقيقة"),
            ("AI portrait video and night mode", "فيديو بورتريه بالذكاء الاصطناعي ووضع ليلي"),
            ("Slim 7.8 mm body, 173 g", "جسم نحيف بسمك 7.8 مم ووزن 173 جرامًا"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.43" FHD+ AMOLED', "6.43 بوصة AMOLED بدقة FHD+")),
            (("Processor", "المعالج"), ("MediaTek Dimensity 800U 5G", "ميدياتك Dimensity 800U بدعم 5G")),
            (("Memory & storage", "الذاكرة والسعة"), ("8 GB RAM, 128 GB + microSD", "ذاكرة 8 جيجابايت وسعة 128 جيجابايت + microSD")),
            (("Rear camera", "الكاميرا الخلفية"), ("48 MP + 8 MP ultra-wide + 2 MP macro + 2 MP mono",
                                                  "48 ميجابكسل + 8 ميجابكسل فائقة الاتساع + 2 ميجابكسل ماكرو + 2 ميجابكسل أحادية اللون")),
            (("Front camera", "الكاميرا الأمامية"), ("16 MP", "16 ميجابكسل")),
            (("Battery", "البطارية"), ("4,310 mAh, 50 W Flash Charge", "4310 مللي أمبير بشحن 50 واط")),
        ],
        variants=[dict(opts=[], price=10999, compare_at=13999)],
        rating=4.4, reviews=267,
        alt=("OPPO F19 Pro+ 5G in Space Silver", "أوبو F19 برو+ 5G باللون الفضي"),
        views=[
            ("back with the quad camera", "من الخلف مع الكاميرا الرباعية"),
            ("front, home screen", "من الأمام مع الشاشة الرئيسية"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1009,
        name=("OPPO A57 (Renewed) — Gold", "أوبو A57 (مُجدَّد) — ذهبي"),
        subtitle=("A light, pocketable backup phone", "موبايل احتياطي خفيف وصغير"),
        description=(
            "A renewed OPPO A57 in gold: a light, pocketable 5.2-inch phone with a 16 MP selfie camera and a "
            "fingerprint-reader home button. With 32 GB plus a microSD slot, it's a no-fuss backup for calls, "
            "WhatsApp and the occasional photo. Tested, cleaned and covered by a 6-month Nile warranty.",
            "أوبو A57 مُجدَّد باللون الذهبي، هاتف خفيف بشاشة 5.2 بوصة يدخل الجيب بسهولة، مع كاميرا سيلفي "
            "16 ميجابكسل وزر رئيسي بقارئ بصمة. بسعة 32 جيجابايت ومنفذ لبطاقة microSD، فهو هاتف احتياطي بسيط "
            "للمكالمات وواتساب والصور من حين لآخر. مفحوص ومنظَّف وبضمان نايل لمدة 6 أشهر.",
        ),
        highlights=[
            ("16 MP selfie camera", "كاميرا سيلفي 16 ميجابكسل"),
            ("Fingerprint home button", "زر رئيسي بقارئ بصمة"),
            ("Dual SIM with a separate microSD slot", "شريحتان مع منفذ microSD مستقل"),
        ],
        specs=[
            (("Display", "الشاشة"), ('5.2" HD IPS', "5.2 بوصة IPS بدقة HD")),
            (("Processor", "المعالج"), ("Snapdragon 435, 3 GB RAM", "Snapdragon 435 مع ذاكرة 3 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("32 GB + microSD up to 256 GB", "32 جيجابايت + microSD حتى 256 جيجابايت")),
            (("Cameras", "الكاميرات"), ("13 MP rear; 16 MP front", "خلفية 13 ميجابكسل، وأمامية 16 ميجابكسل")),
            (("Battery", "البطارية"), ("2,900 mAh", "2900 مللي أمبير")),
            W6,
        ],
        variants=[dict(opts=[], price=1999)],
        rating=3.9, reviews=88,
        alt=("OPPO A57 in Gold", "أوبو A57 باللون الذهبي"),
        views=[
            ("back", "من الخلف"),
            ("front, home screen", "من الأمام مع الشاشة الرئيسية"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1010,
        name=("OPPO A18 — Glowing Blue", "أوبو A18 — أزرق متوهج"),
        subtitle=("5,000 mAh battery and a smooth 90 Hz screen", "بطارية 5000 مللي أمبير تقضّي اليوم وشاشة 90 هرتز ناعمة"),
        description=(
            "OPPO's dependable everyday phone: a 6.56-inch 90 Hz display, a 5,000 mAh battery that easily lasts a "
            "full day, and an Ultra Volume mode that boosts the speaker up to 300%. The Helio G85 chip with 4 GB of "
            "RAM (plus 4 GB of extended RAM) keeps social apps and YouTube running smoothly, and IP54 protection "
            "shrugs off splashes and dust.",
            "هاتف أوبو اليومي الذي يُعتمد عليه: شاشة 6.56 بوصة بتردد 90 هرتز، وبطارية 5000 مللي أمبير تكفي يومًا "
            "كاملًا بسهولة، ووضع Ultra Volume الذي يرفع صوت السماعة حتى 300%. معالج Helio G85 مع ذاكرة 4 جيجابايت "
            "(و4 جيجابايت إضافية ممتدة) يشغّل تطبيقات التواصل ويوتيوب بسلاسة، وحماية IP54 تقيه من الرذاذ والأتربة.",
        ),
        highlights=[
            ("New with a 1-year local warranty", "جديد بضمان محلي لمدة عام"),
            ("5,000 mAh battery for a full day", "بطارية 5000 مللي أمبير تكفي اليوم كله"),
            ("300% Ultra Volume mode", "وضع Ultra Volume لرفع الصوت حتى 300%"),
            ("IP54 splash and dust resistance", "مقاومة الرذاذ والأتربة بمعيار IP54"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.56" HD+, 90 Hz', "6.56 بوصة بدقة HD+ وتردد 90 هرتز")),
            (("Processor", "المعالج"), ("MediaTek Helio G85", "ميدياتك Helio G85")),
            (("Memory", "الذاكرة"), ("4 GB RAM + 4 GB extended", "ذاكرة 4 جيجابايت + 4 جيجابايت ممتدة")),
            (("Cameras", "الكاميرات"), ("8 MP + 2 MP rear; 5 MP front", "خلفية 8 + 2 ميجابكسل، وأمامية 5 ميجابكسل")),
            (("Battery", "البطارية"), ("5,000 mAh", "5000 مللي أمبير")),
            (("Software", "نظام التشغيل"), ("ColorOS 13.1 (Android 13)", "ColorOS 13.1 بنظام أندرويد 13")),
        ],
        options=[dict(id="storage", name=MEM_STORAGE, kind="chip", values=[
            dict(id="64", label=("4 GB + 64 GB", "4 جيجابايت + 64 جيجابايت")),
            dict(id="128", label=("4 GB + 128 GB", "4 جيجابايت + 128 جيجابايت")),
        ])],
        variants=[
            dict(opts=["64"], price=5999),
            dict(opts=["128"], price=6799),
        ],
        rating=4.2, reviews=37,
        alt=("OPPO A18 in Glowing Blue", "أوبو A18 باللون الأزرق المتوهج"),
        views=[
            ("back, Glowing Blue", "من الخلف باللون الأزرق المتوهج"),
            ("front, waterdrop display", "من الأمام بشاشة النوتش الصغير"),
            ("front, side angle", "من الأمام بزاوية جانبية"),
            ("back, side angle", "من الخلف بزاوية جانبية"),
        ],
    ),
    dict(
        id=1011,
        name=("realme C35 — Glowing Green", "ريلمي C35 — أخضر متوهج"),
        subtitle=("50 MP camera and a sharp Full HD+ screen on a budget", "كاميرا 50 ميجابكسل وشاشة Full HD+ بسعر في المتناول"),
        description=(
            "One of Egypt's best-selling budget phones, now at a sharper price. The realme C35 pairs a 6.6-inch "
            "Full HD+ display with a 50 MP AI triple camera and a 5,000 mAh battery with 18 W fast charging, all in "
            "a slim 8.1 mm body with a glowing green finish.",
            "من أكثر الموبايلات الاقتصادية مبيعًا في مصر، وبسعر أفضل الآن. يجمع ريلمي C35 بين شاشة Full HD+ مقاس "
            "6.6 بوصة وكاميرا ثلاثية بالذكاء الاصطناعي بدقة 50 ميجابكسل وبطارية 5000 مللي أمبير بشحن سريع 18 واط، "
            "في جسم نحيف بسمك 8.1 مم ولون أخضر متوهج.",
        ),
        highlights=[
            ("New with a 1-year local warranty", "جديد بضمان محلي لمدة عام"),
            ("50 MP AI triple camera", "كاميرا ثلاثية بالذكاء الاصطناعي 50 ميجابكسل"),
            ("microSD slot up to 1 TB", "منفذ microSD حتى 1 تيرابايت"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.6" Full HD+ (2408 × 1080)', "6.6 بوصة بدقة Full HD+ (2408 × 1080)")),
            (("Processor", "المعالج"), ("Unisoc T616 octa-core", "معالج Unisoc T616 ثماني النوى")),
            (("Rear camera", "الكاميرا الخلفية"), ("50 MP + 2 MP macro + B&W lens", "50 ميجابكسل + 2 ميجابكسل ماكرو + عدسة أبيض وأسود")),
            (("Front camera", "الكاميرا الأمامية"), ("8 MP", "8 ميجابكسل")),
            (("Battery", "البطارية"), ("5,000 mAh, 18 W fast charging", "5000 مللي أمبير بشحن سريع 18 واط")),
            (("Body", "الهيكل"), ("8.1 mm, 189 g", "سمك 8.1 مم ووزن 189 جرامًا")),
        ],
        options=[dict(id="storage", name=MEM_STORAGE, kind="chip", values=[
            dict(id="64", label=("4 GB + 64 GB", "4 جيجابايت + 64 جيجابايت")),
            dict(id="128", label=("4 GB + 128 GB", "4 جيجابايت + 128 جيجابايت")),
        ])],
        variants=[
            dict(opts=["64"], price=5499, compare_at=6499),
            dict(opts=["128"], price=5999, compare_at=6999),
        ],
        rating=4.4, reviews=3218,
        alt=("realme C35 in Glowing Green", "ريلمي C35 باللون الأخضر المتوهج"),
        views=[
            ("back, Glowing Green", "من الخلف باللون الأخضر المتوهج"),
            ("front", "من الأمام"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1012,
        name=("realme X (Renewed) — Punk Blue", "ريلمي X (مُجدَّد) — أزرق بانك"),
        subtitle=("Notch-free AMOLED with a pop-up selfie camera", "شاشة AMOLED كاملة وكاميرا سيلفي بتطلع لوحدها"),
        description=(
            "A renewed realme X with a full-screen 6.53-inch AMOLED — no notch, because the 16 MP selfie camera "
            "pops up from the top only when you need it. The 48 MP Sony main camera, Snapdragon 710 and 8 GB of RAM "
            "still make it a lively everyday phone, and VOOC 3.0 charging refills the 3,765 mAh battery quickly.",
            "ريلمي X مُجدَّد بشاشة AMOLED كاملة مقاس 6.53 بوصة بلا نوتش، لأن كاميرا السيلفي بدقة 16 ميجابكسل "
            "تخرج من أعلى الهاتف عند الحاجة فقط. كاميرا سوني الرئيسية بدقة 48 ميجابكسل ومعالج Snapdragon 710 "
            "وذاكرة 8 جيجابايت تجعله هاتفًا يوميًا نشيطًا، وشحن VOOC 3.0 يملأ البطارية 3765 مللي أمبير بسرعة.",
        ),
        highlights=[
            ("Notch-free 6.53-inch AMOLED", "شاشة AMOLED كاملة مقاس 6.53 بوصة بلا نوتش"),
            ("48 MP Sony main camera", "كاميرا رئيسية سوني بدقة 48 ميجابكسل"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.53" FHD+ AMOLED, no notch', "6.53 بوصة AMOLED بدقة FHD+ بلا نوتش")),
            (("Processor", "المعالج"), ("Snapdragon 710, 8 GB RAM, 128 GB", "Snapdragon 710 مع ذاكرة 8 جيجابايت وسعة 128 جيجابايت")),
            (("Rear camera", "الكاميرا الخلفية"), ("48 MP Sony IMX586 + 5 MP depth", "48 ميجابكسل سوني IMX586 + 5 ميجابكسل للعمق")),
            (("Front camera", "الكاميرا الأمامية"), ("16 MP pop-up", "16 ميجابكسل منبثقة")),
            (("Battery", "البطارية"), ("3,765 mAh, VOOC 3.0 20 W", "3765 مللي أمبير بشحن VOOC 3.0 بقدرة 20 واط")),
            W6,
        ],
        variants=[dict(opts=[], price=4299)],
        rating=4.2, reviews=154,
        alt=("realme X in Punk Blue", "ريلمي X باللون الأزرق"),
        views=[
            ("back, Punk Blue", "من الخلف باللون الأزرق"),
            ("front with the pop-up camera raised", "من الأمام والكاميرا المنبثقة مرفوعة"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1013,
        name=("realme XT (Renewed) — Pearl Blue", "ريلمي XT (مُجدَّد) — أزرق لؤلؤي"),
        subtitle=("64 MP quad camera with ultra-wide and macro", "كاميرا رباعية 64 ميجابكسل فيها واسعة وماكرو"),
        description=(
            "The realme XT was one of the first phones with a 64 MP camera, and it still captures crisp, detailed "
            "daylight shots — with ultra-wide and macro lenses alongside. This renewed unit has a 6.4-inch Super "
            "AMOLED, Snapdragon 712 with 8 GB of RAM and a 4,000 mAh battery with VOOC 3.0 fast charging.",
            "كان ريلمي XT من أوائل الهواتف بكاميرا 64 ميجابكسل، وما زال يلتقط صورًا نهارية حادة وغنية بالتفاصيل، "
            "مع عدستين فائقة الاتساع وماكرو بجانبها. يأتي هذا الجهاز المُجدَّد بشاشة Super AMOLED مقاس 6.4 بوصة "
            "ومعالج Snapdragon 712 مع ذاكرة 8 جيجابايت وبطارية 4000 مللي أمبير بشحن VOOC 3.0 السريع.",
        ),
        highlights=[
            ("64 MP main camera with ultra-wide and macro", "كاميرا رئيسية 64 ميجابكسل مع عدسة واسعة وماكرو"),
            ("Super AMOLED with in-display fingerprint", "شاشة Super AMOLED ببصمة داخل الشاشة"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.4" FHD+ Super AMOLED', "6.4 بوصة Super AMOLED بدقة FHD+")),
            (("Processor", "المعالج"), ("Snapdragon 712, 8 GB RAM, 128 GB", "Snapdragon 712 مع ذاكرة 8 جيجابايت وسعة 128 جيجابايت")),
            (("Rear camera", "الكاميرا الخلفية"), ("64 MP + 8 MP ultra-wide + 2 MP macro + 2 MP depth",
                                                  "64 ميجابكسل + 8 ميجابكسل فائقة الاتساع + 2 ميجابكسل ماكرو + 2 ميجابكسل للعمق")),
            (("Front camera", "الكاميرا الأمامية"), ("16 MP", "16 ميجابكسل")),
            (("Battery", "البطارية"), ("4,000 mAh, VOOC 3.0 20 W", "4000 مللي أمبير بشحن VOOC 3.0 بقدرة 20 واط")),
            W6,
        ],
        variants=[dict(opts=[], price=4799)],
        rating=4.3, reviews=171,
        alt=("realme XT in Pearl Blue", "ريلمي XT باللون الأزرق اللؤلؤي"),
        views=[
            ("back with the quad camera", "من الخلف مع الكاميرا الرباعية"),
            ("front", "من الأمام"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1014,
        name=("vivo S1 (Renewed) — Skyline Blue", "فيفو S1 (مُجدَّد) — أزرق سماوي"),
        subtitle=("32 MP AI selfie camera and an in-screen fingerprint", "كاميرا سيلفي 32 ميجابكسل وبصمة في الشاشة"),
        description=(
            "Made for selfies: the vivo S1 has a 32 MP AI front camera, a 6.38-inch Super AMOLED with a small "
            "waterdrop notch and a fingerprint reader built into the screen. This renewed unit comes with 6 GB of "
            "RAM, 128 GB of storage and a 4,500 mAh battery with 18 W fast charging.",
            "مصمَّم لعشاق السيلفي: يأتي فيفو S1 بكاميرا أمامية بالذكاء الاصطناعي بدقة 32 ميجابكسل، وشاشة Super "
            "AMOLED مقاس 6.38 بوصة بنوتش صغير على شكل قطرة ماء، وقارئ بصمة مدمج في الشاشة. هذا الجهاز المُجدَّد "
            "بذاكرة 6 جيجابايت وسعة 128 جيجابايت وبطارية 4500 مللي أمبير بشحن سريع 18 واط.",
        ),
        highlights=[
            ("32 MP AI selfie camera", "كاميرا سيلفي 32 ميجابكسل بالذكاء الاصطناعي"),
            ("Fingerprint reader under the screen", "قارئ بصمة تحت الشاشة"),
            ("4,500 mAh battery with 18 W fast charging", "بطارية 4500 مللي أمبير بشحن سريع 18 واط"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.38" FHD+ Super AMOLED', "6.38 بوصة Super AMOLED بدقة FHD+")),
            (("Processor", "المعالج"), ("MediaTek Helio P65, 6 GB RAM, 128 GB", "ميدياتك Helio P65 مع ذاكرة 6 جيجابايت وسعة 128 جيجابايت")),
            (("Rear camera", "الكاميرا الخلفية"), ("16 MP + 8 MP ultra-wide + 2 MP depth",
                                                  "16 ميجابكسل + 8 ميجابكسل فائقة الاتساع + 2 ميجابكسل للعمق")),
            (("Front camera", "الكاميرا الأمامية"), ("32 MP", "32 ميجابكسل")),
            (("Battery", "البطارية"), ("4,500 mAh, 18 W", "4500 مللي أمبير بشحن 18 واط")),
            W6,
        ],
        variants=[dict(opts=[], price=4499)],
        rating=4.1, reviews=96,
        alt=("vivo S1 in Skyline Blue", "فيفو S1 باللون الأزرق السماوي"),
        views=[
            ("back, Skyline Blue", "من الخلف باللون الأزرق السماوي"),
            ("front", "من الأمام"),
            BACK_FRONT,
        ],
    ),
    dict(
        id=1015,
        name=("vivo V9 (Renewed) — Champagne Gold", "فيفو V9 (مُجدَّد) — ذهبي شامبين"),
        subtitle=("24 MP selfie camera and a 6.3-inch FullView screen", "كاميرا سيلفي 24 ميجابكسل وشاشة FullView مقاس 6.3 بوصة"),
        description=(
            "The vivo V9 brought the all-screen notch look to the mid-range, with a 6.3-inch FullView display and a "
            "24 MP selfie camera that still takes flattering portraits. This renewed champagne-gold unit has 4 GB of "
            "RAM, 64 GB of storage with a microSD slot, and a rear fingerprint sensor.",
            "قدّم فيفو V9 شكل الشاشة الكاملة مع النوتش للفئة المتوسطة، بشاشة FullView مقاس 6.3 بوصة وكاميرا "
            "سيلفي 24 ميجابكسل ما زالت تلتقط صورًا جميلة. هذا الجهاز المُجدَّد باللون الذهبي الشامبين يأتي بذاكرة "
            "4 جيجابايت وسعة 64 جيجابايت مع منفذ microSD وقارئ بصمة خلفي.",
        ),
        highlights=[
            ("24 MP selfie camera with AI beauty", "كاميرا سيلفي 24 ميجابكسل مع تجميل بالذكاء الاصطناعي"),
            ("6.3-inch FullView display", "شاشة FullView مقاس 6.3 بوصة"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.3" FHD+ FullView IPS', "6.3 بوصة FullView IPS بدقة FHD+")),
            (("Processor", "المعالج"), ("Snapdragon 626, 4 GB RAM", "Snapdragon 626 مع ذاكرة 4 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("64 GB + microSD", "64 جيجابايت + microSD")),
            (("Cameras", "الكاميرات"), ("16 MP + 5 MP rear; 24 MP front", "خلفية 16 + 5 ميجابكسل، وأمامية 24 ميجابكسل")),
            (("Battery", "البطارية"), ("3,260 mAh", "3260 مللي أمبير")),
            W6,
        ],
        variants=[dict(opts=[], price=3299)],
        rating=4.0, reviews=112,
        alt=("vivo V9 in Champagne Gold", "فيفو V9 باللون الذهبي الشامبين"),
        views=[
            ("back, Champagne Gold", "من الخلف باللون الذهبي الشامبين"),
            ("front, notch display", "من الأمام بشاشة النوتش"),
            ("front and back, lying flat", "من الأمام والخلف بوضع أفقي"),
        ],
    ),
    dict(
        id=1016,
        name=("vivo X21 (Renewed) — Ruby Red", "فيفو X21 (مُجدَّد) — أحمر ياقوتي"),
        subtitle=("Fingerprint under the display, in ruby red", "البصمة تحت الشاشة، وبلون أحمر ياقوتي"),
        description=(
            "One of the first phones to hide the fingerprint reader under the display — and in a striking ruby red. "
            "The renewed vivo X21 has a 6.28-inch Super AMOLED, a dual 12 MP Dual Pixel camera for quick focus, and "
            "6 GB of RAM with 128 GB of storage. Tested, cleaned and covered by a 6-month Nile warranty.",
            "من أوائل الهواتف التي أخفت قارئ البصمة تحت الشاشة، وبلون أحمر ياقوتي لافت. يأتي فيفو X21 المُجدَّد "
            "بشاشة Super AMOLED مقاس 6.28 بوصة وكاميرا مزدوجة بدقة 12 ميجابكسل بتقنية Dual Pixel للتركيز السريع، "
            "وذاكرة 6 جيجابايت مع سعة 128 جيجابايت. مفحوص ومنظَّف وبضمان نايل لمدة 6 أشهر.",
        ),
        highlights=[
            ("In-display fingerprint reader", "قارئ بصمة داخل الشاشة"),
            ("Dual Pixel camera for fast focus", "كاميرا Dual Pixel لتركيز سريع"),
            ("6-month Nile warranty", "ضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('6.28" FHD+ Super AMOLED', "6.28 بوصة Super AMOLED بدقة FHD+")),
            (("Processor", "المعالج"), ("Snapdragon 660, 6 GB RAM, 128 GB", "Snapdragon 660 مع ذاكرة 6 جيجابايت وسعة 128 جيجابايت")),
            (("Rear camera", "الكاميرا الخلفية"), ("12 MP Dual Pixel + 5 MP", "12 ميجابكسل Dual Pixel + 5 ميجابكسل")),
            (("Front camera", "الكاميرا الأمامية"), ("12 MP", "12 ميجابكسل")),
            (("Battery", "البطارية"), ("3,200 mAh, 18 W fast charging", "3200 مللي أمبير بشحن سريع 18 واط")),
            W6,
        ],
        variants=[dict(opts=[], price=4999)],
        rating=4.2, reviews=133,
        alt=("vivo X21 in Ruby Red", "فيفو X21 باللون الأحمر الياقوتي"),
        views=[
            ("back, Ruby Red", "من الخلف باللون الأحمر الياقوتي"),
            ("front with the in-display fingerprint icon", "من الأمام مع أيقونة البصمة على الشاشة"),
            BACK_FRONT,
        ],
    ),
    # ------------------------------------------------------------------ Laptops & Tablets
    dict(
        id=1101,
        name=("Apple MacBook Pro 14-inch (M5) — Space Black", "أبل ماك بوك برو 14 بوصة (M5) — أسود فلكي"),
        subtitle=("M5 chip, Liquid Retina XDR and all-day battery", "شريحة M5 وشاشة Liquid Retina XDR وبطارية تقضّي اليوم كله"),
        description=(
            "The 14-inch MacBook Pro with Apple's M5 chip brings a big jump in graphics and on-device AI, with a "
            "Neural Accelerator built into every GPU core. Its Liquid Retina XDR display runs at up to 120 Hz and "
            "reaches 1,600 nits for HDR, and the battery lasts up to 24 hours. HDMI, an SDXC slot, three "
            "Thunderbolt ports and MagSafe mean fewer dongles.",
            "ماك بوك برو مقاس 14 بوصة بشريحة M5 من أبل يقدّم قفزة كبيرة في الرسومات والذكاء الاصطناعي على الجهاز، "
            "مع مسرّع عصبي مدمج في كل نواة رسومات. شاشة Liquid Retina XDR تعمل بتردد يصل إلى 120 هرتز وسطوع 1600 "
            "شمعة لمحتوى HDR، والبطارية تدوم حتى 24 ساعة. ومع منفذ HDMI وفتحة SDXC وثلاثة منافذ Thunderbolt وشاحن "
            "MagSafe ستحتاج إلى وصلات أقل.",
        ),
        highlights=[
            ("Apple M5: 10-core CPU and 10-core GPU", "شريحة Apple M5 بمعالج 10 نوى ورسومات 10 نوى"),
            ("Up to 24 hours of battery life", "بطارية تدوم حتى 24 ساعة"),
            ("12 MP Center Stage camera, six-speaker sound", "كاميرا Center Stage بدقة 12 ميجابكسل ونظام صوت بست سماعات"),
            ("Arabic–English backlit keyboard", "لوحة مفاتيح مضيئة عربي وإنجليزي"),
        ],
        specs=[
            (("Display", "الشاشة"), ('14.2" Liquid Retina XDR, 3024 × 1964, up to 120 Hz',
                                    "14.2 بوصة Liquid Retina XDR بدقة 3024 × 1964 وتردد حتى 120 هرتز")),
            (("Chip", "الشريحة"), ("Apple M5: 10-core CPU, 10-core GPU, 16-core Neural Engine",
                                  "Apple M5: معالج 10 نوى ورسومات 10 نوى ومحرك عصبي 16 نواة")),
            (("Memory & storage", "الذاكرة والسعة"), ("16 GB + 512 GB SSD or 24 GB + 1 TB SSD",
                                                     "16 جيجابايت + SSD سعة 512 جيجابايت أو 24 جيجابايت + SSD سعة 1 تيرابايت")),
            (("Ports", "المنافذ"), ("3× Thunderbolt 4, HDMI, SDXC, MagSafe 3, headphone jack",
                                   "3 منافذ Thunderbolt 4 وHDMI وفتحة SDXC وMagSafe 3 ومنفذ سماعة")),
            (("Battery", "البطارية"), ("Up to 24 h video streaming, 70 W USB-C adapter",
                                      "حتى 24 ساعة بث فيديو، مع شاحن USB-C بقدرة 70 واط")),
            (("Weight", "الوزن"), ("1.55 kg", "1.55 كجم")),
        ],
        options=[dict(id="config", name=CONFIG, kind="chip", values=[
            dict(id="16-512", code="16G512", label=("16 GB · 512 GB SSD", "16 جيجابايت · 512 جيجابايت SSD")),
            dict(id="24-1tb", code="24G1T", label=("24 GB · 1 TB SSD", "24 جيجابايت · 1 تيرابايت SSD")),
        ])],
        variants=[
            dict(opts=["16-512"], price=104999),
            dict(opts=["24-1tb"], price=129999),
        ],
        modifier_groups=[
            dict(id="warranty", name=("Extended warranty", "ضمان ممتد"), selection="single", min=0, max=1,
                 required=False, options=[
                     dict(id="ext1", name=("+1 year Nile Care", "سنة إضافية من Nile Care"), price=5999),
                     dict(id="ext2", name=("+2 years Nile Care", "سنتان إضافيتان من Nile Care"), price=9999),
                 ]),
            dict(id="software", name=("Software", "البرامج"), selection="multi", min=0, max=1, required=False,
                 options=[
                     dict(id="m365", name=("Microsoft 365 Personal, 12 months", "مايكروسوفت 365 الشخصي لمدة 12 شهرًا"),
                          price=3499),
                 ]),
        ],
        rating=4.8, reviews=46,
        alt=("Apple MacBook Pro 14-inch in Space Black", "ماك بوك برو 14 بوصة باللون الأسود الفلكي"),
        views=[
            ("open, front view", "مفتوح من الأمام"),
            ("open, three-quarter view", "مفتوح بزاوية ثلاثة أرباع"),
            ("open, side view", "مفتوح من الجانب"),
        ],
    ),
    dict(
        id=1102,
        name=("ASUS Zenbook Pro 14 Duo OLED (UX8402) — Tech Black", "أسوس زين بوك برو 14 ديو OLED (UX8402) — أسود"),
        subtitle=("Two OLED touch screens, Core i9 and RTX 4060", "شاشتين تاتش ومعالج Core i9 وكارت RTX 4060"),
        description=(
            "Built for editors and designers: a 14.5-inch 2.8K 120 Hz OLED main display plus a 12.7-inch ScreenPad "
            "Plus that tilts up to hold your timelines, palettes and chats. Inside, an Intel Core i9-13900H, NVIDIA "
            "GeForce RTX 4060, 32 GB of RAM and a 1 TB SSD handle 4K editing and 3D work — now more than EGP 20,000 "
            "below its usual price.",
            "مصمَّم للمونتير والمصمم: شاشة رئيسية OLED بدقة 2.8K مقاس 14.5 بوصة وتردد 120 هرتز، وشاشة ScreenPad "
            "Plus ثانية مقاس 12.7 بوصة ترتفع بزاوية لتعرض الخط الزمني ولوحات الأدوات والمحادثات. وفي الداخل معالج "
            "Intel Core i9-13900H وكارت NVIDIA GeForce RTX 4060 وذاكرة 32 جيجابايت وSSD سعة 1 تيرابايت لمونتاج 4K "
            "والأعمال ثلاثية الأبعاد، والآن بأقل من سعره المعتاد بأكثر من 20,000 جنيه.",
        ),
        highlights=[
            ("12.7-inch ScreenPad Plus tilts up for your tools", "شاشة ScreenPad Plus مقاس 12.7 بوصة ترتفع لأدواتك"),
            ("Pantone-validated 2.8K OLED at 120 Hz", "شاشة OLED بدقة 2.8K وتردد 120 هرتز معتمدة من Pantone"),
            ("Save over EGP 20,000", "وفّر أكثر من 20,000 جنيه"),
        ],
        specs=[
            (("Main display", "الشاشة الرئيسية"), ('14.5" 2.8K OLED touch, 120 Hz', "14.5 بوصة OLED لمسية بدقة 2.8K وتردد 120 هرتز")),
            (("Second display", "الشاشة الثانية"), ('12.7" ScreenPad Plus touch', "شاشة ScreenPad Plus لمسية مقاس 12.7 بوصة")),
            (("Processor", "المعالج"), ("Intel Core i9-13900H", "إنتل Core i9-13900H")),
            (("Graphics", "كارت الشاشة"), ("NVIDIA GeForce RTX 4060, 8 GB", "إنفيديا GeForce RTX 4060 بذاكرة 8 جيجابايت")),
            (("Memory & storage", "الذاكرة والسعة"), ("32 GB LPDDR5, 1 TB PCIe 4.0 SSD", "ذاكرة 32 جيجابايت LPDDR5 وSSD سعة 1 تيرابايت PCIe 4.0")),
            (("Weight & OS", "الوزن والنظام"), ("1.75 kg, Windows 11 Home", "1.75 كجم، بنظام ويندوز 11 هوم")),
        ],
        variants=[dict(opts=[], price=89999, compare_at=109999)],
        rating=4.5, reviews=74,
        alt=("ASUS Zenbook Pro 14 Duo OLED with ScreenPad Plus", "أسوس زين بوك برو 14 ديو OLED مع شاشة ScreenPad Plus"),
        views=[
            ("open, both screens on", "مفتوح والشاشتان مضاءتان"),
            ("angled, ScreenPad Plus raised", "بزاوية مع رفع شاشة ScreenPad Plus"),
            ("side angle", "بزاوية جانبية"),
        ],
    ),
    dict(
        id=1103,
        name=("HUAWEI MateBook X Pro (2024)", "هواوي ميت بوك X برو (2024)"),
        subtitle=("Under 1 kg with a 3.1K OLED touch screen", "أقل من كيلو وشاشة OLED تاتش بدقة 3.1K"),
        description=(
            "Huawei's flagship ultrabook weighs just 980 g yet packs an Intel Core Ultra 9 processor, 32 GB of RAM "
            "and a 1 TB SSD. The 14.2-inch 3.1K OLED touch display runs at up to 120 Hz, and the full-size keyboard "
            "and large touchpad make long writing sessions comfortable.",
            "لابتوب هواوي الرائد يزن 980 جرامًا فقط، ومع ذلك يضم معالج Intel Core Ultra 9 وذاكرة 32 جيجابايت وSSD "
            "سعة 1 تيرابايت. شاشته OLED اللمسية مقاس 14.2 بوصة بدقة 3.1K تعمل بتردد يصل إلى 120 هرتز، ولوحة "
            "المفاتيح الكاملة ولوحة اللمس الواسعة تجعلان الكتابة الطويلة مريحة.",
        ),
        highlights=[
            ("Magnesium-alloy body at just 980 g", "جسم من سبائك المغنيسيوم بوزن 980 جرامًا فقط"),
            ("3.1K OLED touch display up to 120 Hz", "شاشة OLED لمسية بدقة 3.1K حتى 120 هرتز"),
            ("Intel Core Ultra 9 with 32 GB RAM", "معالج Intel Core Ultra 9 مع ذاكرة 32 جيجابايت"),
        ],
        specs=[
            (("Display", "الشاشة"), ('14.2" 3.1K OLED touch, up to 120 Hz', "14.2 بوصة OLED لمسية بدقة 3.1K وحتى 120 هرتز")),
            (("Processor", "المعالج"), ("Intel Core Ultra 9 185H", "إنتل Core Ultra 9 185H")),
            (("Memory & storage", "الذاكرة والسعة"), ("32 GB LPDDR5x, 1 TB SSD", "ذاكرة 32 جيجابايت LPDDR5x وSSD سعة 1 تيرابايت")),
            (("Battery", "البطارية"), ("70 Wh, 90 W charger", "70 واط ساعة مع شاحن 90 واط")),
            (("Weight", "الوزن"), ("980 g", "980 جرامًا")),
            (("Software", "نظام التشغيل"), ("Windows 11 Home", "ويندوز 11 هوم")),
        ],
        variants=[dict(opts=[], price=84999)],
        rating=4.6, reviews=18,
        alt=("HUAWEI MateBook X Pro", "هواوي ميت بوك X برو"),
        views=[
            ("open, front view", "مفتوح من الأمام"),
            ("open, angled view", "مفتوح بزاوية"),
            ("open, side angle", "مفتوح بزاوية جانبية"),
        ],
    ),
    dict(
        id=1104,
        name=('Lenovo Yoga 7i 2-in-1 (14") — Arctic Grey', "لينوفو يوجا 7i قابل للتحويل 2 في 1 (14 بوصة) — رمادي قطبي"),
        subtitle=("Flips into a tablet for notes and films", "يتقلب تابلت للمذاكرة والأفلام"),
        description=(
            "A 14-inch convertible with a 360° hinge: type in laptop mode, stand it up as a tent to watch, or fold "
            "it flat to take notes with a pen. The 1920 × 1200 OLED touch display is bright and colour-rich, Intel "
            "Core Ultra processors with 16 GB of RAM keep multitasking smooth, and the 71 Wh battery covers a full "
            "university day.",
            "لابتوب 14 بوصة قابل للتحويل بمفصلة 360 درجة: اكتب في وضع اللابتوب، أو اجعله كالخيمة لمشاهدة الأفلام، "
            "أو اطوِه بالكامل لتدوين الملاحظات بالقلم. شاشته OLED اللمسية بدقة 1920 × 1200 ساطعة وغنية بالألوان، "
            "ومعالجات Intel Core Ultra مع ذاكرة 16 جيجابايت تضمن تعددًا سلسًا للمهام، وبطاريته 71 واط ساعة تكفي "
            "يومًا جامعيًا كاملًا.",
        ),
        highlights=[
            ("360° hinge: laptop, tent, stand or tablet", "مفصلة 360 درجة: لابتوب أو خيمة أو حامل أو تابلت"),
            ("OLED touch display with pen support", "شاشة OLED لمسية تدعم القلم"),
            ("71 Wh battery for a full day", "بطارية 71 واط ساعة تكفي يومًا كاملًا"),
        ],
        specs=[
            (("Display", "الشاشة"), ('14" WUXGA OLED touch (1920 × 1200)', "14 بوصة OLED لمسية بدقة WUXGA (1920 × 1200)")),
            (("Processor", "المعالج"), ("Intel Core Ultra 5 125U or Core Ultra 7 155U", "إنتل Core Ultra 5 125U أو Core Ultra 7 155U")),
            (("Memory & storage", "الذاكرة والسعة"), ("16 GB LPDDR5x; 512 GB or 1 TB SSD",
                                                     "ذاكرة 16 جيجابايت LPDDR5x وSSD سعة 512 جيجابايت أو 1 تيرابايت")),
            (("Ports", "المنافذ"), ("2× Thunderbolt 4, USB-A, HDMI, microSD", "منفذا Thunderbolt 4 وUSB-A وHDMI وقارئ microSD")),
            (("Battery", "البطارية"), ("71 Wh", "71 واط ساعة")),
            (("Weight", "الوزن"), ("1.49 kg", "1.49 كجم")),
        ],
        options=[dict(id="config", name=CONFIG, kind="chip", values=[
            dict(id="u5-512", code="U5", label=("Core Ultra 5 · 16 GB · 512 GB", "Core Ultra 5 · 16 جيجابايت · 512 جيجابايت")),
            dict(id="u7-1tb", code="U7", label=("Core Ultra 7 · 16 GB · 1 TB", "Core Ultra 7 · 16 جيجابايت · 1 تيرابايت")),
        ])],
        variants=[
            dict(opts=["u5-512"], price=46999),
            dict(opts=["u7-1tb"], price=53999),
        ],
        rating=4.4, reviews=58,
        alt=("Lenovo Yoga 7i 2-in-1 in Arctic Grey", "لينوفو يوجا 7i القابل للتحويل باللون الرمادي"),
        views=[
            ("open, front view", "مفتوح من الأمام"),
            ("open, angled view", "مفتوح بزاوية"),
            ("open, side view", "مفتوح من الجانب"),
        ],
    ),
    dict(
        id=1105,
        name=("Dell XPS 13 (9350) Copilot+ PC — Graphite", "ديل XPS 13 (9350) بتقنية Copilot+ — جرافيت"),
        subtitle=("Copilot+ PC in a 1.19 kg aluminium body", "لابتوب Copilot+ في جسم ألومنيوم وزنه 1.19 كيلو"),
        description=(
            "Dell's most compact XPS, rebuilt around Intel's Lunar Lake Core Ultra processors with a 47 TOPS NPU for "
            "Copilot+ AI features. The edge-to-edge keyboard, seamless glass touchpad and 13.4-inch 120 Hz display "
            "give it a clean, modern feel, and the efficient chip delivers genuinely long battery life from a "
            "1.19 kg machine.",
            "أصغر لابتوب XPS من ديل، أُعيد تصميمه حول معالجات Intel Core Ultra من جيل Lunar Lake مع وحدة NPU بقدرة "
            "47 TOPS لمزايا الذكاء الاصطناعي في Copilot+. لوحة المفاتيح الممتدة من الحافة للحافة ولوحة اللمس "
            "الزجاجية المدمجة والشاشة مقاس 13.4 بوصة بتردد 120 هرتز تمنحه إحساسًا عصريًا نظيفًا، والمعالج الموفّر "
            "يمنح بطارية تدوم طويلًا فعلًا في جهاز وزنه 1.19 كجم.",
        ),
        highlights=[
            ("Copilot+ PC with a 47 TOPS NPU", "لابتوب Copilot+ بوحدة NPU بقدرة 47 TOPS"),
            ("Edge-to-edge keyboard and seamless glass touchpad", "لوحة مفاتيح من الحافة للحافة ولوحة لمس زجاجية مدمجة"),
            ("Long battery life from Lunar Lake efficiency", "بطارية طويلة بفضل كفاءة معالجات Lunar Lake"),
        ],
        specs=[
            (("Display", "الشاشة"), ('13.4" FHD+ (1920 × 1200), 120 Hz', "13.4 بوصة بدقة FHD+ (1920 × 1200) وتردد 120 هرتز")),
            (("Processor", "المعالج"), ("Intel Core Ultra 7 256V (16 GB) or 258V (32 GB)",
                                       "إنتل Core Ultra 7 256V بذاكرة 16 جيجابايت أو 258V بذاكرة 32 جيجابايت")),
            (("Graphics", "كارت الشاشة"), ("Intel Arc 140V", "إنتل Arc 140V")),
            (("Storage", "السعة التخزينية"), ("512 GB or 1 TB SSD", "SSD سعة 512 جيجابايت أو 1 تيرابايت")),
            (("Ports", "المنافذ"), ("2× Thunderbolt 4 (USB-C)", "منفذان Thunderbolt 4 بنوع USB-C")),
            (("Weight", "الوزن"), ("1.19 kg", "1.19 كجم")),
        ],
        options=[dict(id="config", name=CONFIG, kind="chip", values=[
            dict(id="16-512", code="16G512", label=("Core Ultra 7 256V · 16 GB · 512 GB",
                                                    "Core Ultra 7 256V · 16 جيجابايت · 512 جيجابايت")),
            dict(id="32-1tb", code="32G1T", label=("Core Ultra 7 258V · 32 GB · 1 TB",
                                                   "Core Ultra 7 258V · 32 جيجابايت · 1 تيرابايت")),
        ])],
        variants=[
            dict(opts=["16-512"], price=74999),
            dict(opts=["32-1tb"], price=87999),
        ],
        rating=4.5, reviews=22,
        alt=("Dell XPS 13 in Graphite", "ديل XPS 13 باللون الجرافيت"),
        views=[
            ("open, front view", "مفتوح من الأمام"),
            ("open, angled view", "مفتوح بزاوية"),
            ("open, side angle", "مفتوح بزاوية جانبية"),
        ],
    ),
    dict(
        id=1106,
        name=("Apple iPad mini (A17 Pro) — Starlight", "أبل آيباد ميني (A17 Pro) — ضوء النجوم"),
        subtitle=("Pocket-size iPad with the A17 Pro chip", "آيباد صغير يتشال في الشنطة وفيه قوة A17 Pro"),
        description=(
            "The smallest iPad, now with the A17 Pro chip and 8 GB of memory, built for Apple Intelligence. Its "
            "8.3-inch Liquid Retina display is perfect for reading, sketching and gaming on the go, and it works with "
            "Apple Pencil Pro for precise notes and drawing. Touch ID, USB-C and a 12 MP camera complete a tablet "
            "that fits in one hand.",
            "أصغر آيباد من أبل، أصبح الآن بشريحة A17 Pro وذاكرة 8 جيجابايت ومصمَّمًا لمزايا Apple Intelligence. "
            "شاشته Liquid Retina مقاس 8.3 بوصة مثالية للقراءة والرسم والألعاب أثناء التنقل، ويعمل مع قلم Apple "
            "Pencil Pro لتدوين الملاحظات والرسم بدقة. ومع Touch ID ومنفذ USB-C وكاميرا 12 ميجابكسل يكتمل جهاز "
            "لوحي يُمسك بيد واحدة.",
        ),
        highlights=[
            ("A17 Pro chip with 8 GB memory", "شريحة A17 Pro مع ذاكرة 8 جيجابايت"),
            ("Works with Apple Pencil Pro", "يدعم قلم Apple Pencil Pro"),
            ("Touch ID, USB-C and Wi-Fi 6E", "Touch ID ومنفذ USB-C وشبكة Wi-Fi 6E"),
        ],
        specs=[
            (("Display", "الشاشة"), ('8.3" Liquid Retina, 2266 × 1488', "8.3 بوصة Liquid Retina بدقة 2266 × 1488")),
            (("Chip", "الشريحة"), ("A17 Pro, 8 GB RAM", "شريحة A17 Pro مع ذاكرة 8 جيجابايت")),
            (("Cameras", "الكاميرات"), ("12 MP wide back; 12 MP ultra-wide front with Center Stage",
                                       "خلفية 12 ميجابكسل واسعة، وأمامية 12 ميجابكسل فائقة الاتساع مع Center Stage")),
            (("Connectivity", "الاتصال"), ("Wi-Fi 6E; 5G on Cellular models; USB-C",
                                          "Wi-Fi 6E، و5G في طرازات الشبكة الخلوية، ومنفذ USB-C")),
            (("Battery", "البطارية"), ("Up to 10 hours", "حتى 10 ساعات")),
            (("Weight", "الوزن"), ("293 g (Wi-Fi)", "293 جرامًا (واي فاي)")),
        ],
        options=[
            dict(id="storage", name=STORAGE, kind="chip", values=[gb(128), gb(256), gb(512)]),
            dict(id="network", name=CONNECTIVITY, kind="chip", values=[
                dict(id="wifi", code="WF", label=("Wi-Fi", "واي فاي")),
                dict(id="cellular", code="CL", label=("Wi-Fi + Cellular", "واي فاي + شبكة خلوية")),
            ]),
        ],
        variants=[
            dict(opts=["128", "wifi"], price=30999),
            dict(opts=["128", "cellular"], price=38999),
            dict(opts=["256", "wifi"], price=36499),
            dict(opts=["256", "cellular"], price=44499),
            dict(opts=["512", "wifi"], price=47999),
            dict(opts=["512", "cellular"], price=55999),
        ],
        modifier_groups=[
            dict(id="pencil", name=("Add Apple Pencil", "أضف قلم أبل"), selection="single", min=0, max=1,
                 required=False, options=[
                     dict(id="pencil-pro", name=("Apple Pencil Pro", "قلم Apple Pencil Pro"), price=7499),
                 ]),
        ],
        rating=4.8, reviews=41,
        alt=("Apple iPad mini in Starlight", "آيباد ميني بلون ضوء النجوم"),
        views=[
            ("front, display on", "من الأمام والشاشة مضاءة"),
            ("front, angled", "من الأمام بزاوية"),
            ("back in Starlight", "من الخلف بلون ضوء النجوم"),
            ("front, angled from the side", "من الأمام بزاوية جانبية"),
        ],
    ),
    dict(
        id=1107,
        name=("Samsung Galaxy Tab S10+ — Moonstone Gray", "سامسونج جالاكسي تاب S10+ — رمادي حجر القمر"),
        subtitle=("12.4-inch AMOLED with the S Pen in the box", "شاشة AMOLED مقاس 12.4 بوصة والقلم في العلبة"),
        description=(
            "A 12.4-inch Dynamic AMOLED 2X at 120 Hz, quad AKG speakers and a 5.6 mm aluminium body make the "
            "Tab S10+ a superb screen for films and drawing. The S Pen comes in the box and snaps magnetically to "
            "the back, the Dimensity 9300+ with 12 GB of RAM powers Galaxy AI tools like Note Assist, and IP68 "
            "protection means a splash won't ruin the day.",
            "شاشة Dynamic AMOLED 2X مقاس 12.4 بوصة بتردد 120 هرتز وأربع سماعات AKG وجسم ألومنيوم بسمك 5.6 مم "
            "تجعل تاب S10+ شاشة رائعة للأفلام والرسم. قلم S Pen موجود في العلبة ويلتصق مغناطيسيًا بالظهر، ومعالج "
            "Dimensity 9300+ مع ذاكرة 12 جيجابايت يشغّل أدوات Galaxy AI مثل مساعد الملاحظات، ومقاومة IP68 تحميه "
            "من رذاذ الماء.",
        ),
        highlights=[
            ("S Pen included", "قلم S Pen مرفق في العلبة"),
            ("Quad AKG speakers with Dolby Atmos", "أربع سماعات AKG مع Dolby Atmos"),
            ("IP68 water and dust resistance", "مقاومة للماء والأتربة بمعيار IP68"),
            ("Galaxy AI: Note Assist and Circle to Search", "Galaxy AI: مساعد الملاحظات والبحث بالدائرة"),
        ],
        specs=[
            (("Display", "الشاشة"), ('12.4" Dynamic AMOLED 2X, 2800 × 1752, 120 Hz',
                                    "12.4 بوصة Dynamic AMOLED 2X بدقة 2800 × 1752 وتردد 120 هرتز")),
            (("Processor", "المعالج"), ("MediaTek Dimensity 9300+, 12 GB RAM", "ميدياتك Dimensity 9300+ مع ذاكرة 12 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("256 or 512 GB + microSD up to 1.5 TB",
                                             "256 أو 512 جيجابايت + microSD حتى 1.5 تيرابايت")),
            (("Battery", "البطارية"), ("10,090 mAh, 45 W fast charging", "10090 مللي أمبير بشحن سريع 45 واط")),
            (("Cameras", "الكاميرات"), ("13 MP + 8 MP ultra-wide rear; 12 MP ultra-wide front",
                                       "خلفية 13 + 8 ميجابكسل فائقة الاتساع، وأمامية 12 ميجابكسل فائقة الاتساع")),
            (("In the box", "محتويات العلبة"), ("Tablet, S Pen, USB-C cable", "التابلت وقلم S Pen وكابل USB-C")),
        ],
        options=[
            dict(id="storage", name=STORAGE, kind="chip", values=[gb(256), gb(512)]),
            dict(id="network", name=CONNECTIVITY, kind="chip", values=[
                dict(id="wifi", code="WF", label=("Wi-Fi", "واي فاي")),
                dict(id="5g", code="5G", label=("Wi-Fi + 5G", "واي فاي + 5G")),
            ]),
        ],
        variants=[
            dict(opts=["256", "wifi"], price=52999, compare_at=59999),
            dict(opts=["256", "5g"], price=58999, compare_at=65999),
            dict(opts=["512", "wifi"], price=59999),
            dict(opts=["512", "5g"], price=65999),
        ],
        rating=4.7, reviews=29,
        alt=("Samsung Galaxy Tab S10+ with S Pen", "سامسونج جالاكسي تاب S10+ مع قلم S Pen"),
        views=[
            ("front with the S Pen", "من الأمام مع قلم S Pen"),
            ("angled, with the S Pen", "بزاوية مع قلم S Pen"),
            ("back with the magnetic S Pen strip", "من الخلف مع مكان القلم المغناطيسي"),
            ("front, angled from the other side", "من الأمام بزاوية من الجهة الأخرى"),
        ],
    ),
    dict(
        id=1108,
        name=("Samsung Galaxy Tab A 10.1 (Renewed) — White", "سامسونج جالاكسي تاب A 10.1 (مُجدَّد) — أبيض"),
        subtitle=("Big-screen starter tablet for kids and reading", "تابلت شاشته كبيرة للأطفال والقراية والفرجة"),
        description=(
            "A renewed 10.1-inch Galaxy Tab A with a sharp 1920 × 1200 display and a long-lasting 7,300 mAh battery "
            "— great for cartoons, e-books, YouTube and video calls with family. It arrives reset with 32 GB of "
            "storage plus a microSD slot, and runs Android 8.1, so it suits light apps rather than heavy games.",
            "جالاكسي تاب A مُجدَّد بشاشة 10.1 بوصة بدقة 1920 × 1200 وبطارية 7300 مللي أمبير تدوم طويلًا، مناسب "
            "للكارتون والكتب الإلكترونية ويوتيوب ومكالمات الفيديو مع العائلة. يصل مُعاد الضبط بسعة 32 جيجابايت مع "
            "منفذ microSD، ويعمل بنظام أندرويد 8.1، لذا يناسب التطبيقات الخفيفة أكثر من الألعاب الثقيلة.",
        ),
        highlights=[
            ("Big 10.1-inch Full HD screen", "شاشة كبيرة 10.1 بوصة بدقة Full HD"),
            ("7,300 mAh battery for long cartoon sessions", "بطارية 7300 مللي أمبير لساعات طويلة من الكارتون"),
            ("Renewed with a 6-month Nile warranty", "مُجدَّد بضمان نايل لمدة 6 أشهر"),
        ],
        specs=[
            (("Display", "الشاشة"), ('10.1" WUXGA (1920 × 1200)', "10.1 بوصة بدقة WUXGA (1920 × 1200)")),
            (("Processor", "المعالج"), ("Exynos 7870 octa-core, 2 GB RAM", "Exynos 7870 ثماني النوى مع ذاكرة 2 جيجابايت")),
            (("Storage", "السعة التخزينية"), ("32 GB + microSD up to 256 GB", "32 جيجابايت + microSD حتى 256 جيجابايت")),
            (("Battery", "البطارية"), ("7,300 mAh", "7300 مللي أمبير")),
            (("Cameras", "الكاميرات"), ("8 MP rear; 2 MP front", "خلفية 8 ميجابكسل، وأمامية 2 ميجابكسل")),
            (("Software", "نظام التشغيل"), ("Android 8.1", "أندرويد 8.1")),
        ],
        variants=[dict(opts=[], price=3999)],
        rating=4.1, reviews=117,
        alt=("Samsung Galaxy Tab A 10.1 in White", "سامسونج جالاكسي تاب A 10.1 باللون الأبيض"),
        views=[
            ("front, display on", "من الأمام والشاشة مضاءة"),
            ("front, angled", "من الأمام بزاوية"),
            ("back", "من الخلف"),
            ("front, angled from the other side", "من الأمام بزاوية من الجهة الأخرى"),
        ],
    ),
]

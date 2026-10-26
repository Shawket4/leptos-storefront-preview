"""Souq categories. Ids and slugs are stable; each category owns a block of 50 product ids."""

from common import L

ID_BLOCKS = {10: 2001, 20: 2051, 30: 2101, 40: 2151, 50: 2201, 60: 2251, 70: 2301, 80: 2351,
             90: 2401, 100: 2451, 110: 2501, 120: 2551, 130: 2601, 140: 2651}


def C(cid, slug, name, blurb, icon, image_of, order):
    return {"id": cid, "slug": slug, "name": L(*name), "blurb": L(*blurb), "icon": icon,
            "image_of": image_of, "display_order": order}


CATEGORIES = [
    C(10, "fruits-vegetables", ("Fruits & vegetables", "خضار وفاكهة"),
      ("Picked this week, at your door in hours", "طازة من الغيط لحد باب البيت في ساعتين"),
      "carrot", 2001, 1),
    C(20, "bakery", ("Bakery", "مخبوزات"),
      ("Shami, fino and loaves out of the oven every morning", "عيش شامي وفينو وعيش فرنساوي طالع من الفرن كل يوم الصبح"),
      "croissant", 2051, 2),
    C(30, "dairy-eggs", ("Dairy & eggs", "ألبان وبيض"),
      ("Fresh milk, butter, cream and farm eggs", "لبن وزبدة وقشطة وبيض بلدي طازة"),
      "milk", 2101, 3),
    C(40, "cheese", ("Cheese & olives", "جبن وزيتون"),
      ("Romy, Emmental, blue and olives — cut to order", "رومي وإيمنتال وريكفور وزيتون... بنقطعهولك زي ما تحب"),
      "refrigerator", 2151, 4),
    C(50, "meat-poultry", ("Meat & poultry", "لحوم وفراخ"),
      ("Halal cuts, minced and prepped the way you like", "لحمة وفراخ حلال... بنفرم ونقطع ونجهز على مزاجك"),
      "beef", 2201, 5),
    C(60, "fish-seafood", ("Fish & seafood", "أسماك ومأكولات بحرية"),
      ("Cleaned, filleted or grill-ready — your call", "بننضف ونفيليه ونجهز للشوي أو القلي على طلبك"),
      "fish", 2251, 6),
    C(70, "rice-grains-pulses", ("Rice, grains & pulses", "أرز وبقول وحبوب"),
      ("Egyptian rice, pasta, beans and belila wheat", "رز مصري ومكرونة وفاصوليا وقمح بليلة... أساس كل طبخة"),
      "wheat", 2302, 7),
    C(80, "spices-herbs", ("Spices & herbs", "بهارات وعطارة"),
      ("Whole or freshly ground, just like the attar's", "حب أو مطحون طازة زي العطار بالظبط"),
      "leaf", 2351, 8),
    C(100, "dates-sweets", ("Dates & sweets", "بلح وحلويات"),
      ("Medjool, malban, fuliya and little treats", "مجدول وملبن وفولية... حلّي بُقّك"),
      "candy", 2451, 9),
    C(110, "coffee-tea", ("Coffee & tea", "بن وشاي"),
      ("Turkish coffee roasted fresh, tea and herbal brews", "بن محمص طازة وشاي وأعشاب"),
      "coffee", 2501, 10),
    C(120, "drinks-juices", ("Drinks & juices", "مشروبات وعصائر"),
      ("Water, fresh juice, sodas and Ramadan favourites", "مية وعصير فريش وحاجة ساقعة وتمر هندي"),
      "cup-soda", 2552, 11),
    C(130, "nuts-snacks", ("Nuts & snacks", "مكسرات وتسالي"),
      ("Roasted daily: lib, peanuts, cashews and more", "لب وسوداني وكاجو... متحمص كل يوم"),
      "nut", 2602, 12),
    C(140, "breakfast-pantry", ("Breakfast & pantry", "فطار ومونة البيت"),
      ("Oils, honey, sauces and the kitchen basics", "زيت وعسل وصلصة وكل أساسيات المطبخ"),
      "shopping-basket", 2651, 13),
]

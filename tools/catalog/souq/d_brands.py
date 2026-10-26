"""Brands. Only the ones referenced by products are emitted into souq.json."""

from common import L


def B(bid, en, ar):
    return {"id": bid, "name": L(en, ar)}


BRANDS = [
    B("souq", "Souq", "سوق"),
    B("juhayna", "Juhayna", "جهينة"),
    B("domty", "Domty", "دومتي"),
    B("obour-land", "Obour Land", "عبور لاند"),
    B("el-rashidi", "El Rashidi El Mizan", "الرشيدي الميزان"),
    B("abu-auf", "Abu Auf", "أبو عوف"),
    B("isis", "Isis", "إيزيس"),
    B("imtenan", "Imtenan", "امتنان"),
    B("el-arosa", "El Arosa", "العروسة"),
    B("lipton", "Lipton", "ليبتون"),
    B("bisco-misr", "Bisco Misr", "بسكو مصر"),
    B("edita", "Edita", "إيديتا"),
    B("harvest", "Harvest Foods", "هارفست"),
    B("el-doha", "El Doha", "الضحى"),
    B("wadi-food", "Wadi Food", "وادي فود"),
    B("regina", "Regina", "ريجينا"),
    B("barilla", "Barilla", "باريلا"),
    B("bonduelle", "Bonduelle", "بونديل"),
    B("crystal", "Crystal", "كريستال"),
    B("baraka", "Baraka", "بركة"),
    B("anchor", "Anchor", "أنكور"),
    B("nescafe", "Nescafé", "نسكافيه"),
    B("coca-cola", "Coca-Cola", "كوكاكولا"),
    B("nutella", "Nutella", "نوتيلا"),
    B("oreo", "Oreo", "أوريو"),
    B("heinz", "Heinz", "هاينز"),
    B("maggi", "Maggi", "ماجي"),
    B("knorr", "Knorr", "كنور"),
    B("koki", "Koki", "كوكي"),
    B("dr-oetker", "Dr. Oetker", "دكتور أوتكر"),
    B("mars", "Mars", "مارس"),
    B("alpro", "Alpro", "ألبرو"),
    B("oldenburger", "Oldenburger", "أولدنبورجر"),
    B("magnum", "Magnum", "ماجنوم"),
    B("cadbury", "Cadbury", "كادبوري"),
    B("hellmanns", "Hellmann's", "هيلمانز"),
    B("kelloggs", "Kellogg's", "كيلوجز"),
    B("hersheys", "Hershey's", "هيرشيز"),
    B("schweppes", "Schweppes", "شويبس"),
    B("hero", "Hero", "هيرو"),
    B("maille", "Maille", "مايّ"),
    B("tabasco", "Tabasco", "تاباسكو"),
    B("lea-perrins", "Lea & Perrins", "لي آند بيرينز"),
]

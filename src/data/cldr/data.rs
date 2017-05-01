use super::{CldrData,CldrTree};
use ::data::Item;


pub static LOC_ROOT : CldrData = CldrData {
    parents: &[
    ],
    index: &[
        (Item::DecimalDigits, 10),
        (Item::DecimalSeparator, 11),
        (Item::GroupSeparator, 12),
        (Item::PlusSign, 13),
        (Item::MinusSign, 14),
        (Item::PercentSign, 15),
        (Item::PerMilleSign, 18),
        (Item::EngineeringExponent, 19),
        (Item::CommonExponent, 23),
        (Item::InfinitySymbol, 26),
        (Item::NotANumberSymbol, 29),
        (Item::Grouping, 30),
        (Item::FractionalGrouping, 30),
        (Item::MinGroupingDigits, 31),
        (Item::MinIntegralDigits, 32),
    ],
    data: "0123456789.,+-%‰E×10∞NaN311",
};

pub static TAG_ROOT : CldrTree = CldrTree {
    data: &LOC_ROOT,
    children: &[
        ("af", &TAG_AF),
        ("agq", &TAG_AGQ),
        ("ak", &TAG_AK),
        ("am", &TAG_AM),
        ("ar", &TAG_AR),
        ("as", &TAG_AS),
        ("asa", &TAG_ASA),
        ("ast", &TAG_AST),
        ("az", &TAG_AZ),
        ("bas", &TAG_BAS),
        ("be", &TAG_BE),
        ("bem", &TAG_BEM),
        ("bez", &TAG_BEZ),
        ("bg", &TAG_BG),
        ("bm", &TAG_BM),
        ("bn", &TAG_BN),
        ("bo", &TAG_BO),
        ("br", &TAG_BR),
        ("brx", &TAG_BRX),
        ("bs", &TAG_BS),
        ("ca", &TAG_CA),
        ("ce", &TAG_CE),
        ("cgg", &TAG_CGG),
        ("chr", &TAG_CHR),
        ("ckb", &TAG_CKB),
        ("cs", &TAG_CS),
        ("cu", &TAG_CU),
        ("cy", &TAG_CY),
        ("da", &TAG_DA),
        ("dav", &TAG_DAV),
        ("de", &TAG_DE),
        ("dje", &TAG_DJE),
        ("dsb", &TAG_DSB),
        ("dua", &TAG_DUA),
        ("dyo", &TAG_DYO),
        ("dz", &TAG_DZ),
        ("ebu", &TAG_EBU),
        ("ee", &TAG_EE),
        ("el", &TAG_EL),
        ("en", &TAG_EN),
        ("eo", &TAG_EO),
        ("es", &TAG_ES),
        ("et", &TAG_ET),
        ("eu", &TAG_EU),
        ("ewo", &TAG_EWO),
        ("fa", &TAG_FA),
        ("ff", &TAG_FF),
        ("fi", &TAG_FI),
        ("fil", &TAG_FIL),
        ("fo", &TAG_FO),
        ("fr", &TAG_FR),
        ("fur", &TAG_FUR),
        ("fy", &TAG_FY),
        ("ga", &TAG_GA),
        ("gd", &TAG_GD),
        ("gl", &TAG_GL),
        ("gsw", &TAG_GSW),
        ("gu", &TAG_GU),
        ("guz", &TAG_GUZ),
        ("gv", &TAG_GV),
        ("ha", &TAG_HA),
        ("haw", &TAG_HAW),
        ("he", &TAG_HE),
        ("hi", &TAG_HI),
        ("hr", &TAG_HR),
        ("hsb", &TAG_HSB),
        ("hu", &TAG_HU),
        ("hy", &TAG_HY),
        ("id", &TAG_ID),
        ("ig", &TAG_IG),
        ("ii", &TAG_II),
        ("is", &TAG_IS),
        ("it", &TAG_IT),
        ("ja", &TAG_JA),
        ("jgo", &TAG_JGO),
        ("jmc", &TAG_JMC),
        ("ka", &TAG_KA),
        ("kab", &TAG_KAB),
        ("kam", &TAG_KAM),
        ("kde", &TAG_KDE),
        ("kea", &TAG_KEA),
        ("khq", &TAG_KHQ),
        ("ki", &TAG_KI),
        ("kk", &TAG_KK),
        ("kkj", &TAG_KKJ),
        ("kl", &TAG_KL),
        ("kln", &TAG_KLN),
        ("km", &TAG_KM),
        ("kn", &TAG_KN),
        ("ko", &TAG_KO),
        ("kok", &TAG_KOK),
        ("ks", &TAG_KS),
        ("ksb", &TAG_KSB),
        ("ksf", &TAG_KSF),
        ("ksh", &TAG_KSH),
        ("kw", &TAG_KW),
        ("ky", &TAG_KY),
        ("lag", &TAG_LAG),
        ("lb", &TAG_LB),
        ("lg", &TAG_LG),
        ("lkt", &TAG_LKT),
        ("ln", &TAG_LN),
        ("lo", &TAG_LO),
        ("lrc", &TAG_LRC),
        ("lt", &TAG_LT),
        ("lu", &TAG_LU),
        ("luo", &TAG_LUO),
        ("luy", &TAG_LUY),
        ("lv", &TAG_LV),
        ("mas", &TAG_MAS),
        ("mer", &TAG_MER),
        ("mfe", &TAG_MFE),
        ("mg", &TAG_MG),
        ("mgh", &TAG_MGH),
        ("mgo", &TAG_MGO),
        ("mk", &TAG_MK),
        ("ml", &TAG_ML),
        ("mn", &TAG_MN),
        ("mr", &TAG_MR),
        ("ms", &TAG_MS),
        ("mt", &TAG_MT),
        ("mua", &TAG_MUA),
        ("my", &TAG_MY),
        ("mzn", &TAG_MZN),
        ("naq", &TAG_NAQ),
        ("nb", &TAG_NB),
        ("nd", &TAG_ND),
        ("nds", &TAG_NDS),
        ("ne", &TAG_NE),
        ("nl", &TAG_NL),
        ("nmg", &TAG_NMG),
        ("nn", &TAG_NN),
        ("nnh", &TAG_NNH),
        ("nus", &TAG_NUS),
        ("nyn", &TAG_NYN),
        ("om", &TAG_OM),
        ("or", &TAG_OR),
        ("os", &TAG_OS),
        ("pa", &TAG_PA),
        ("pl", &TAG_PL),
        ("prg", &TAG_PRG),
        ("ps", &TAG_PS),
        ("pt", &TAG_PT),
        ("qu", &TAG_QU),
        ("rm", &TAG_RM),
        ("rn", &TAG_RN),
        ("ro", &TAG_RO),
        ("rof", &TAG_ROF),
        ("ru", &TAG_RU),
        ("rw", &TAG_RW),
        ("rwk", &TAG_RWK),
        ("sah", &TAG_SAH),
        ("saq", &TAG_SAQ),
        ("sbp", &TAG_SBP),
        ("se", &TAG_SE),
        ("seh", &TAG_SEH),
        ("ses", &TAG_SES),
        ("sg", &TAG_SG),
        ("shi", &TAG_SHI),
        ("si", &TAG_SI),
        ("sk", &TAG_SK),
        ("sl", &TAG_SL),
        ("smn", &TAG_SMN),
        ("sn", &TAG_SN),
        ("so", &TAG_SO),
        ("sq", &TAG_SQ),
        ("sr", &TAG_SR),
        ("sv", &TAG_SV),
        ("sw", &TAG_SW),
        ("ta", &TAG_TA),
        ("te", &TAG_TE),
        ("teo", &TAG_TEO),
        ("th", &TAG_TH),
        ("ti", &TAG_TI),
        ("tk", &TAG_TK),
        ("to", &TAG_TO),
        ("tr", &TAG_TR),
        ("twq", &TAG_TWQ),
        ("tzm", &TAG_TZM),
        ("ug", &TAG_UG),
        ("uk", &TAG_UK),
        ("ur", &TAG_UR),
        ("uz", &TAG_UZ),
        ("vai", &TAG_VAI),
        ("vi", &TAG_VI),
        ("vo", &TAG_VO),
        ("vun", &TAG_VUN),
        ("wae", &TAG_WAE),
        ("xog", &TAG_XOG),
        ("yav", &TAG_YAV),
        ("yi", &TAG_YI),
        ("yo", &TAG_YO),
        ("yue", &TAG_YUE),
        ("zgh", &TAG_ZGH),
        ("zh", &TAG_ZH),
        ("zu", &TAG_ZU),
    ],
};

pub static LOC_AF : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_AF : CldrTree = CldrTree {
    data: &LOC_AF,
    children: &[
        ("NA", &TAG_AF_NA),
        ("ZA", &TAG_AF_ZA),
    ],
};

pub static LOC_AF_NA : CldrData = CldrData {
    parents: &[
        &LOC_AF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AF_NA : CldrTree = CldrTree {
    data: &LOC_AF_NA,
    children: &[
    ],
};

pub static LOC_AF_ZA : CldrData = CldrData {
    parents: &[
        &LOC_AF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AF_ZA : CldrTree = CldrTree {
    data: &LOC_AF_ZA,
    children: &[
    ],
};

pub static LOC_AGQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_AGQ : CldrTree = CldrTree {
    data: &LOC_AGQ,
    children: &[
        ("CM", &TAG_AGQ_CM),
    ],
};

pub static LOC_AGQ_CM : CldrData = CldrData {
    parents: &[
        &LOC_AGQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AGQ_CM : CldrTree = CldrTree {
    data: &LOC_AGQ_CM,
    children: &[
    ],
};

pub static LOC_AK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AK : CldrTree = CldrTree {
    data: &LOC_AK,
    children: &[
        ("GH", &TAG_AK_GH),
    ],
};

pub static LOC_AK_GH : CldrData = CldrData {
    parents: &[
        &LOC_AK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AK_GH : CldrTree = CldrTree {
    data: &LOC_AK_GH,
    children: &[
    ],
};

pub static LOC_AM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AM : CldrTree = CldrTree {
    data: &LOC_AM,
    children: &[
        ("ET", &TAG_AM_ET),
    ],
};

pub static LOC_AM_ET : CldrData = CldrData {
    parents: &[
        &LOC_AM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AM_ET : CldrTree = CldrTree {
    data: &LOC_AM_ET,
    children: &[
    ],
};

pub static LOC_AR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
        (Item::DecimalSeparator, 22),
        (Item::GroupSeparator, 24),
        (Item::PlusSign, 27),
        (Item::MinusSign, 30),
        (Item::PercentSign, 34),
        (Item::PerMilleSign, 36),
        (Item::EngineeringExponent, 40),
        (Item::CommonExponent, 47),
        (Item::NotANumberSymbol, 61),
    ],
    data: "٠١٢٣٤٥٦٧٨٩٫٬؜+؜-٪؜؉اس×١٠^ليس رقم",
};

pub static TAG_AR : CldrTree = CldrTree {
    data: &LOC_AR,
    children: &[
        ("001", &TAG_AR_001),
        ("AE", &TAG_AR_AE),
        ("BH", &TAG_AR_BH),
        ("DJ", &TAG_AR_DJ),
        ("DZ", &TAG_AR_DZ),
        ("EG", &TAG_AR_EG),
        ("EH", &TAG_AR_EH),
        ("ER", &TAG_AR_ER),
        ("IL", &TAG_AR_IL),
        ("IQ", &TAG_AR_IQ),
        ("JO", &TAG_AR_JO),
        ("KM", &TAG_AR_KM),
        ("KW", &TAG_AR_KW),
        ("LB", &TAG_AR_LB),
        ("LY", &TAG_AR_LY),
        ("MA", &TAG_AR_MA),
        ("MR", &TAG_AR_MR),
        ("OM", &TAG_AR_OM),
        ("PS", &TAG_AR_PS),
        ("QA", &TAG_AR_QA),
        ("SA", &TAG_AR_SA),
        ("SD", &TAG_AR_SD),
        ("SO", &TAG_AR_SO),
        ("SS", &TAG_AR_SS),
        ("SY", &TAG_AR_SY),
        ("TD", &TAG_AR_TD),
        ("TN", &TAG_AR_TN),
        ("YE", &TAG_AR_YE),
    ],
};

pub static LOC_AR_001 : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_001 : CldrTree = CldrTree {
    data: &LOC_AR_001,
    children: &[
    ],
};

pub static LOC_AR_AE : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_AE : CldrTree = CldrTree {
    data: &LOC_AR_AE,
    children: &[
    ],
};

pub static LOC_AR_BH : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_BH : CldrTree = CldrTree {
    data: &LOC_AR_BH,
    children: &[
    ],
};

pub static LOC_AR_DJ : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_DJ : CldrTree = CldrTree {
    data: &LOC_AR_DJ,
    children: &[
    ],
};

pub static LOC_AR_DZ : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
        (Item::DecimalDigits, 10),
        (Item::DecimalSeparator, 11),
        (Item::GroupSeparator, 12),
    ],
    data: "0123456789,.",
};

pub static TAG_AR_DZ : CldrTree = CldrTree {
    data: &LOC_AR_DZ,
    children: &[
    ],
};

pub static LOC_AR_EG : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_EG : CldrTree = CldrTree {
    data: &LOC_AR_EG,
    children: &[
    ],
};

pub static LOC_AR_EH : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
        (Item::DecimalDigits, 10),
    ],
    data: "0123456789",
};

pub static TAG_AR_EH : CldrTree = CldrTree {
    data: &LOC_AR_EH,
    children: &[
    ],
};

pub static LOC_AR_ER : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_ER : CldrTree = CldrTree {
    data: &LOC_AR_ER,
    children: &[
    ],
};

pub static LOC_AR_IL : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_IL : CldrTree = CldrTree {
    data: &LOC_AR_IL,
    children: &[
    ],
};

pub static LOC_AR_IQ : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_IQ : CldrTree = CldrTree {
    data: &LOC_AR_IQ,
    children: &[
    ],
};

pub static LOC_AR_JO : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_JO : CldrTree = CldrTree {
    data: &LOC_AR_JO,
    children: &[
    ],
};

pub static LOC_AR_KM : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_KM : CldrTree = CldrTree {
    data: &LOC_AR_KM,
    children: &[
    ],
};

pub static LOC_AR_KW : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_KW : CldrTree = CldrTree {
    data: &LOC_AR_KW,
    children: &[
    ],
};

pub static LOC_AR_LB : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_LB : CldrTree = CldrTree {
    data: &LOC_AR_LB,
    children: &[
    ],
};

pub static LOC_AR_LY : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
        (Item::DecimalDigits, 10),
        (Item::DecimalSeparator, 11),
        (Item::GroupSeparator, 12),
    ],
    data: "0123456789,.",
};

pub static TAG_AR_LY : CldrTree = CldrTree {
    data: &LOC_AR_LY,
    children: &[
    ],
};

pub static LOC_AR_MA : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
        (Item::DecimalDigits, 10),
        (Item::DecimalSeparator, 11),
        (Item::GroupSeparator, 12),
    ],
    data: "0123456789,.",
};

pub static TAG_AR_MA : CldrTree = CldrTree {
    data: &LOC_AR_MA,
    children: &[
    ],
};

pub static LOC_AR_MR : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_MR : CldrTree = CldrTree {
    data: &LOC_AR_MR,
    children: &[
    ],
};

pub static LOC_AR_OM : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_OM : CldrTree = CldrTree {
    data: &LOC_AR_OM,
    children: &[
    ],
};

pub static LOC_AR_PS : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_PS : CldrTree = CldrTree {
    data: &LOC_AR_PS,
    children: &[
    ],
};

pub static LOC_AR_QA : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_QA : CldrTree = CldrTree {
    data: &LOC_AR_QA,
    children: &[
    ],
};

pub static LOC_AR_SA : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_SA : CldrTree = CldrTree {
    data: &LOC_AR_SA,
    children: &[
    ],
};

pub static LOC_AR_SD : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_SD : CldrTree = CldrTree {
    data: &LOC_AR_SD,
    children: &[
    ],
};

pub static LOC_AR_SO : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_SO : CldrTree = CldrTree {
    data: &LOC_AR_SO,
    children: &[
    ],
};

pub static LOC_AR_SS : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_SS : CldrTree = CldrTree {
    data: &LOC_AR_SS,
    children: &[
    ],
};

pub static LOC_AR_SY : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_SY : CldrTree = CldrTree {
    data: &LOC_AR_SY,
    children: &[
    ],
};

pub static LOC_AR_TD : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_TD : CldrTree = CldrTree {
    data: &LOC_AR_TD,
    children: &[
    ],
};

pub static LOC_AR_TN : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
        (Item::DecimalDigits, 10),
        (Item::DecimalSeparator, 11),
        (Item::GroupSeparator, 12),
    ],
    data: "0123456789,.",
};

pub static TAG_AR_TN : CldrTree = CldrTree {
    data: &LOC_AR_TN,
    children: &[
    ],
};

pub static LOC_AR_YE : CldrData = CldrData {
    parents: &[
        &LOC_AR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AR_YE : CldrTree = CldrTree {
    data: &LOC_AR_YE,
    children: &[
    ],
};

pub static LOC_AS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
    ],
    data: "০১২৩৪৫৬৭৮৯",
};

pub static TAG_AS : CldrTree = CldrTree {
    data: &LOC_AS,
    children: &[
        ("IN", &TAG_AS_IN),
    ],
};

pub static LOC_AS_IN : CldrData = CldrData {
    parents: &[
        &LOC_AS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AS_IN : CldrTree = CldrTree {
    data: &LOC_AS_IN,
    children: &[
    ],
};

pub static LOC_ASA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ASA : CldrTree = CldrTree {
    data: &LOC_ASA,
    children: &[
        ("TZ", &TAG_ASA_TZ),
    ],
};

pub static LOC_ASA_TZ : CldrData = CldrData {
    parents: &[
        &LOC_ASA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ASA_TZ : CldrTree = CldrTree {
    data: &LOC_ASA_TZ,
    children: &[
    ],
};

pub static LOC_AST : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::NotANumberSymbol, 4),
    ],
    data: ",.ND",
};

pub static TAG_AST : CldrTree = CldrTree {
    data: &LOC_AST,
    children: &[
        ("ES", &TAG_AST_ES),
    ],
};

pub static LOC_AST_ES : CldrData = CldrData {
    parents: &[
        &LOC_AST,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AST_ES : CldrTree = CldrTree {
    data: &LOC_AST_ES,
    children: &[
    ],
};

pub static LOC_AZ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_AZ : CldrTree = CldrTree {
    data: &LOC_AZ,
    children: &[
        ("Cyrl", &TAG_AZ_CYRL),
        ("Latn", &TAG_AZ_LATN),
    ],
};

pub static LOC_AZ_CYRL : CldrData = CldrData {
    parents: &[
        &LOC_AZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AZ_CYRL : CldrTree = CldrTree {
    data: &LOC_AZ_CYRL,
    children: &[
        ("AZ", &TAG_AZ_CYRL_AZ),
    ],
};

pub static LOC_AZ_CYRL_AZ : CldrData = CldrData {
    parents: &[
        &LOC_AZ_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AZ_CYRL_AZ : CldrTree = CldrTree {
    data: &LOC_AZ_CYRL_AZ,
    children: &[
    ],
};

pub static LOC_AZ_LATN : CldrData = CldrData {
    parents: &[
        &LOC_AZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AZ_LATN : CldrTree = CldrTree {
    data: &LOC_AZ_LATN,
    children: &[
        ("AZ", &TAG_AZ_LATN_AZ),
    ],
};

pub static LOC_AZ_LATN_AZ : CldrData = CldrData {
    parents: &[
        &LOC_AZ_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_AZ_LATN_AZ : CldrTree = CldrTree {
    data: &LOC_AZ_LATN_AZ,
    children: &[
    ],
};

pub static LOC_BAS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_BAS : CldrTree = CldrTree {
    data: &LOC_BAS,
    children: &[
        ("CM", &TAG_BAS_CM),
    ],
};

pub static LOC_BAS_CM : CldrData = CldrData {
    parents: &[
        &LOC_BAS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BAS_CM : CldrTree = CldrTree {
    data: &LOC_BAS_CM,
    children: &[
    ],
};

pub static LOC_BE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinGroupingDigits, 4),
    ],
    data: ", 2",
};

pub static TAG_BE : CldrTree = CldrTree {
    data: &LOC_BE,
    children: &[
        ("BY", &TAG_BE_BY),
    ],
};

pub static LOC_BE_BY : CldrData = CldrData {
    parents: &[
        &LOC_BE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BE_BY : CldrTree = CldrTree {
    data: &LOC_BE_BY,
    children: &[
    ],
};

pub static LOC_BEM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BEM : CldrTree = CldrTree {
    data: &LOC_BEM,
    children: &[
        ("ZM", &TAG_BEM_ZM),
    ],
};

pub static LOC_BEM_ZM : CldrData = CldrData {
    parents: &[
        &LOC_BEM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BEM_ZM : CldrTree = CldrTree {
    data: &LOC_BEM_ZM,
    children: &[
    ],
};

pub static LOC_BEZ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BEZ : CldrTree = CldrTree {
    data: &LOC_BEZ,
    children: &[
        ("TZ", &TAG_BEZ_TZ),
    ],
};

pub static LOC_BEZ_TZ : CldrData = CldrData {
    parents: &[
        &LOC_BEZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BEZ_TZ : CldrTree = CldrTree {
    data: &LOC_BEZ_TZ,
    children: &[
    ],
};

pub static LOC_BG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::CommonExponent, 7),
        (Item::MinGroupingDigits, 8),
    ],
    data: ", ·102",
};

pub static TAG_BG : CldrTree = CldrTree {
    data: &LOC_BG,
    children: &[
        ("BG", &TAG_BG_BG),
    ],
};

pub static LOC_BG_BG : CldrData = CldrData {
    parents: &[
        &LOC_BG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BG_BG : CldrTree = CldrTree {
    data: &LOC_BG_BG,
    children: &[
    ],
};

pub static LOC_BM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BM : CldrTree = CldrTree {
    data: &LOC_BM,
    children: &[
        ("ML", &TAG_BM_ML),
    ],
};

pub static LOC_BM_ML : CldrData = CldrData {
    parents: &[
        &LOC_BM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BM_ML : CldrTree = CldrTree {
    data: &LOC_BM_ML,
    children: &[
    ],
};

pub static LOC_BN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
        (Item::CommonExponent, 39),
    ],
    data: "০১২৩৪৫৬৭৮৯×১০^",
};

pub static TAG_BN : CldrTree = CldrTree {
    data: &LOC_BN,
    children: &[
        ("BD", &TAG_BN_BD),
        ("IN", &TAG_BN_IN),
    ],
};

pub static LOC_BN_BD : CldrData = CldrData {
    parents: &[
        &LOC_BN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BN_BD : CldrTree = CldrTree {
    data: &LOC_BN_BD,
    children: &[
    ],
};

pub static LOC_BN_IN : CldrData = CldrData {
    parents: &[
        &LOC_BN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BN_IN : CldrTree = CldrTree {
    data: &LOC_BN_IN,
    children: &[
    ],
};

pub static LOC_BO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BO : CldrTree = CldrTree {
    data: &LOC_BO,
    children: &[
        ("CN", &TAG_BO_CN),
        ("IN", &TAG_BO_IN),
    ],
};

pub static LOC_BO_CN : CldrData = CldrData {
    parents: &[
        &LOC_BO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BO_CN : CldrTree = CldrTree {
    data: &LOC_BO_CN,
    children: &[
    ],
};

pub static LOC_BO_IN : CldrData = CldrData {
    parents: &[
        &LOC_BO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BO_IN : CldrTree = CldrTree {
    data: &LOC_BO_IN,
    children: &[
    ],
};

pub static LOC_BR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_BR : CldrTree = CldrTree {
    data: &LOC_BR,
    children: &[
        ("FR", &TAG_BR_FR),
    ],
};

pub static LOC_BR_FR : CldrData = CldrData {
    parents: &[
        &LOC_BR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BR_FR : CldrTree = CldrTree {
    data: &LOC_BR_FR,
    children: &[
    ],
};

pub static LOC_BRX : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_BRX : CldrTree = CldrTree {
    data: &LOC_BRX,
    children: &[
        ("IN", &TAG_BRX_IN),
    ],
};

pub static LOC_BRX_IN : CldrData = CldrData {
    parents: &[
        &LOC_BRX,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BRX_IN : CldrTree = CldrTree {
    data: &LOC_BRX_IN,
    children: &[
    ],
};

pub static LOC_BS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_BS : CldrTree = CldrTree {
    data: &LOC_BS,
    children: &[
        ("Cyrl", &TAG_BS_CYRL),
        ("Latn", &TAG_BS_LATN),
    ],
};

pub static LOC_BS_CYRL : CldrData = CldrData {
    parents: &[
        &LOC_BS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BS_CYRL : CldrTree = CldrTree {
    data: &LOC_BS_CYRL,
    children: &[
        ("BA", &TAG_BS_CYRL_BA),
    ],
};

pub static LOC_BS_CYRL_BA : CldrData = CldrData {
    parents: &[
        &LOC_BS_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BS_CYRL_BA : CldrTree = CldrTree {
    data: &LOC_BS_CYRL_BA,
    children: &[
    ],
};

pub static LOC_BS_LATN : CldrData = CldrData {
    parents: &[
        &LOC_BS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BS_LATN : CldrTree = CldrTree {
    data: &LOC_BS_LATN,
    children: &[
        ("BA", &TAG_BS_LATN_BA),
    ],
};

pub static LOC_BS_LATN_BA : CldrData = CldrData {
    parents: &[
        &LOC_BS_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_BS_LATN_BA : CldrTree = CldrTree {
    data: &LOC_BS_LATN_BA,
    children: &[
    ],
};

pub static LOC_CA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_CA : CldrTree = CldrTree {
    data: &LOC_CA,
    children: &[
        ("AD", &TAG_CA_AD),
        ("ES", &TAG_CA_ES),
        ("FR", &TAG_CA_FR),
        ("IT", &TAG_CA_IT),
    ],
};

pub static LOC_CA_AD : CldrData = CldrData {
    parents: &[
        &LOC_CA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CA_AD : CldrTree = CldrTree {
    data: &LOC_CA_AD,
    children: &[
    ],
};

pub static LOC_CA_ES : CldrData = CldrData {
    parents: &[
        &LOC_CA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CA_ES : CldrTree = CldrTree {
    data: &LOC_CA_ES,
    children: &[
        ("valencia", &TAG_CA_ES_VALENCIA),
    ],
};

pub static LOC_CA_ES_VALENCIA : CldrData = CldrData {
    parents: &[
        &LOC_CA_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CA_ES_VALENCIA : CldrTree = CldrTree {
    data: &LOC_CA_ES_VALENCIA,
    children: &[
    ],
};

pub static LOC_CA_FR : CldrData = CldrData {
    parents: &[
        &LOC_CA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CA_FR : CldrTree = CldrTree {
    data: &LOC_CA_FR,
    children: &[
    ],
};

pub static LOC_CA_IT : CldrData = CldrData {
    parents: &[
        &LOC_CA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CA_IT : CldrTree = CldrTree {
    data: &LOC_CA_IT,
    children: &[
    ],
};

pub static LOC_CE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::NotANumberSymbol, 22),
    ],
    data: "Терхьаш дац",
};

pub static TAG_CE : CldrTree = CldrTree {
    data: &LOC_CE,
    children: &[
        ("RU", &TAG_CE_RU),
    ],
};

pub static LOC_CE_RU : CldrData = CldrData {
    parents: &[
        &LOC_CE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CE_RU : CldrTree = CldrTree {
    data: &LOC_CE_RU,
    children: &[
    ],
};

pub static LOC_CGG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CGG : CldrTree = CldrTree {
    data: &LOC_CGG,
    children: &[
        ("UG", &TAG_CGG_UG),
    ],
};

pub static LOC_CGG_UG : CldrData = CldrData {
    parents: &[
        &LOC_CGG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CGG_UG : CldrTree = CldrTree {
    data: &LOC_CGG_UG,
    children: &[
    ],
};

pub static LOC_CHR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CHR : CldrTree = CldrTree {
    data: &LOC_CHR,
    children: &[
        ("US", &TAG_CHR_US),
    ],
};

pub static LOC_CHR_US : CldrData = CldrData {
    parents: &[
        &LOC_CHR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CHR_US : CldrTree = CldrTree {
    data: &LOC_CHR_US,
    children: &[
    ],
};

pub static LOC_CKB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
        (Item::DecimalSeparator, 22),
        (Item::GroupSeparator, 24),
        (Item::PercentSign, 26),
        (Item::PerMilleSign, 28),
        (Item::EngineeringExponent, 32),
        (Item::CommonExponent, 39),
    ],
    data: "٠١٢٣٤٥٦٧٨٩٫٬٪؉اس×١٠^",
};

pub static TAG_CKB : CldrTree = CldrTree {
    data: &LOC_CKB,
    children: &[
        ("IQ", &TAG_CKB_IQ),
        ("IR", &TAG_CKB_IR),
    ],
};

pub static LOC_CKB_IQ : CldrData = CldrData {
    parents: &[
        &LOC_CKB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CKB_IQ : CldrTree = CldrTree {
    data: &LOC_CKB_IQ,
    children: &[
    ],
};

pub static LOC_CKB_IR : CldrData = CldrData {
    parents: &[
        &LOC_CKB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CKB_IR : CldrTree = CldrTree {
    data: &LOC_CKB_IR,
    children: &[
    ],
};

pub static LOC_CS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_CS : CldrTree = CldrTree {
    data: &LOC_CS,
    children: &[
        ("CZ", &TAG_CS_CZ),
    ],
};

pub static LOC_CS_CZ : CldrData = CldrData {
    parents: &[
        &LOC_CS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CS_CZ : CldrTree = CldrTree {
    data: &LOC_CS_CZ,
    children: &[
    ],
};

pub static LOC_CU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_CU : CldrTree = CldrTree {
    data: &LOC_CU,
    children: &[
        ("RU", &TAG_CU_RU),
    ],
};

pub static LOC_CU_RU : CldrData = CldrData {
    parents: &[
        &LOC_CU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CU_RU : CldrTree = CldrTree {
    data: &LOC_CU_RU,
    children: &[
    ],
};

pub static LOC_CY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CY : CldrTree = CldrTree {
    data: &LOC_CY,
    children: &[
        ("GB", &TAG_CY_GB),
    ],
};

pub static LOC_CY_GB : CldrData = CldrData {
    parents: &[
        &LOC_CY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_CY_GB : CldrTree = CldrTree {
    data: &LOC_CY_GB,
    children: &[
    ],
};

pub static LOC_DA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_DA : CldrTree = CldrTree {
    data: &LOC_DA,
    children: &[
        ("DK", &TAG_DA_DK),
        ("GL", &TAG_DA_GL),
    ],
};

pub static LOC_DA_DK : CldrData = CldrData {
    parents: &[
        &LOC_DA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DA_DK : CldrTree = CldrTree {
    data: &LOC_DA_DK,
    children: &[
    ],
};

pub static LOC_DA_GL : CldrData = CldrData {
    parents: &[
        &LOC_DA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DA_GL : CldrTree = CldrTree {
    data: &LOC_DA_GL,
    children: &[
    ],
};

pub static LOC_DAV : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DAV : CldrTree = CldrTree {
    data: &LOC_DAV,
    children: &[
        ("KE", &TAG_DAV_KE),
    ],
};

pub static LOC_DAV_KE : CldrData = CldrData {
    parents: &[
        &LOC_DAV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DAV_KE : CldrTree = CldrTree {
    data: &LOC_DAV_KE,
    children: &[
    ],
};

pub static LOC_DE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::CommonExponent, 6),
    ],
    data: ",.·10",
};

pub static TAG_DE : CldrTree = CldrTree {
    data: &LOC_DE,
    children: &[
        ("AT", &TAG_DE_AT),
        ("BE", &TAG_DE_BE),
        ("CH", &TAG_DE_CH),
        ("DE", &TAG_DE_DE),
        ("IT", &TAG_DE_IT),
        ("LI", &TAG_DE_LI),
        ("LU", &TAG_DE_LU),
    ],
};

pub static LOC_DE_AT : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_DE_AT : CldrTree = CldrTree {
    data: &LOC_DE_AT,
    children: &[
    ],
};

pub static LOC_DE_BE : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DE_BE : CldrTree = CldrTree {
    data: &LOC_DE_BE,
    children: &[
    ],
};

pub static LOC_DE_CH : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 4),
    ],
    data: ".’",
};

pub static TAG_DE_CH : CldrTree = CldrTree {
    data: &LOC_DE_CH,
    children: &[
    ],
};

pub static LOC_DE_DE : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DE_DE : CldrTree = CldrTree {
    data: &LOC_DE_DE,
    children: &[
    ],
};

pub static LOC_DE_IT : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DE_IT : CldrTree = CldrTree {
    data: &LOC_DE_IT,
    children: &[
    ],
};

pub static LOC_DE_LI : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 4),
    ],
    data: ".’",
};

pub static TAG_DE_LI : CldrTree = CldrTree {
    data: &LOC_DE_LI,
    children: &[
    ],
};

pub static LOC_DE_LU : CldrData = CldrData {
    parents: &[
        &LOC_DE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DE_LU : CldrTree = CldrTree {
    data: &LOC_DE_LU,
    children: &[
    ],
};

pub static LOC_DJE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_DJE : CldrTree = CldrTree {
    data: &LOC_DJE,
    children: &[
        ("NE", &TAG_DJE_NE),
    ],
};

pub static LOC_DJE_NE : CldrData = CldrData {
    parents: &[
        &LOC_DJE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DJE_NE : CldrTree = CldrTree {
    data: &LOC_DJE_NE,
    children: &[
    ],
};

pub static LOC_DSB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::CommonExponent, 6),
    ],
    data: ",.·10",
};

pub static TAG_DSB : CldrTree = CldrTree {
    data: &LOC_DSB,
    children: &[
        ("DE", &TAG_DSB_DE),
    ],
};

pub static LOC_DSB_DE : CldrData = CldrData {
    parents: &[
        &LOC_DSB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DSB_DE : CldrTree = CldrTree {
    data: &LOC_DSB_DE,
    children: &[
    ],
};

pub static LOC_DUA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_DUA : CldrTree = CldrTree {
    data: &LOC_DUA,
    children: &[
        ("CM", &TAG_DUA_CM),
    ],
};

pub static LOC_DUA_CM : CldrData = CldrData {
    parents: &[
        &LOC_DUA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DUA_CM : CldrTree = CldrTree {
    data: &LOC_DUA_CM,
    children: &[
    ],
};

pub static LOC_DYO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_DYO : CldrTree = CldrTree {
    data: &LOC_DYO,
    children: &[
        ("SN", &TAG_DYO_SN),
    ],
};

pub static LOC_DYO_SN : CldrData = CldrData {
    parents: &[
        &LOC_DYO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DYO_SN : CldrTree = CldrTree {
    data: &LOC_DYO_SN,
    children: &[
    ],
};

pub static LOC_DZ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
        (Item::InfinitySymbol, 54),
        (Item::NotANumberSymbol, 69),
    ],
    data: "༠༡༢༣༤༥༦༧༨༩གྲངས་མེདཨང་མད",
};

pub static TAG_DZ : CldrTree = CldrTree {
    data: &LOC_DZ,
    children: &[
        ("BT", &TAG_DZ_BT),
    ],
};

pub static LOC_DZ_BT : CldrData = CldrData {
    parents: &[
        &LOC_DZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_DZ_BT : CldrTree = CldrTree {
    data: &LOC_DZ_BT,
    children: &[
    ],
};

pub static LOC_EBU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EBU : CldrTree = CldrTree {
    data: &LOC_EBU,
    children: &[
        ("KE", &TAG_EBU_KE),
    ],
};

pub static LOC_EBU_KE : CldrData = CldrData {
    parents: &[
        &LOC_EBU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EBU_KE : CldrTree = CldrTree {
    data: &LOC_EBU_KE,
    children: &[
    ],
};

pub static LOC_EE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::NotANumberSymbol, 3),
    ],
    data: "mnn",
};

pub static TAG_EE : CldrTree = CldrTree {
    data: &LOC_EE,
    children: &[
        ("GH", &TAG_EE_GH),
        ("TG", &TAG_EE_TG),
    ],
};

pub static LOC_EE_GH : CldrData = CldrData {
    parents: &[
        &LOC_EE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EE_GH : CldrTree = CldrTree {
    data: &LOC_EE_GH,
    children: &[
    ],
};

pub static LOC_EE_TG : CldrData = CldrData {
    parents: &[
        &LOC_EE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EE_TG : CldrTree = CldrTree {
    data: &LOC_EE_TG,
    children: &[
    ],
};

pub static LOC_EL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::EngineeringExponent, 3),
    ],
    data: ",.e",
};

pub static TAG_EL : CldrTree = CldrTree {
    data: &LOC_EL,
    children: &[
        ("CY", &TAG_EL_CY),
        ("GR", &TAG_EL_GR),
    ],
};

pub static LOC_EL_CY : CldrData = CldrData {
    parents: &[
        &LOC_EL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EL_CY : CldrTree = CldrTree {
    data: &LOC_EL_CY,
    children: &[
    ],
};

pub static LOC_EL_GR : CldrData = CldrData {
    parents: &[
        &LOC_EL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EL_GR : CldrTree = CldrTree {
    data: &LOC_EL_GR,
    children: &[
    ],
};

pub static LOC_EN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN : CldrTree = CldrTree {
    data: &LOC_EN,
    children: &[
        ("001", &TAG_EN_001),
        ("150", &TAG_EN_150),
        ("AG", &TAG_EN_AG),
        ("AI", &TAG_EN_AI),
        ("AS", &TAG_EN_AS),
        ("AT", &TAG_EN_AT),
        ("AU", &TAG_EN_AU),
        ("BB", &TAG_EN_BB),
        ("BE", &TAG_EN_BE),
        ("BI", &TAG_EN_BI),
        ("BM", &TAG_EN_BM),
        ("BS", &TAG_EN_BS),
        ("BW", &TAG_EN_BW),
        ("BZ", &TAG_EN_BZ),
        ("CA", &TAG_EN_CA),
        ("CC", &TAG_EN_CC),
        ("CH", &TAG_EN_CH),
        ("CK", &TAG_EN_CK),
        ("CM", &TAG_EN_CM),
        ("CX", &TAG_EN_CX),
        ("CY", &TAG_EN_CY),
        ("DE", &TAG_EN_DE),
        ("DG", &TAG_EN_DG),
        ("DK", &TAG_EN_DK),
        ("DM", &TAG_EN_DM),
        ("ER", &TAG_EN_ER),
        ("FI", &TAG_EN_FI),
        ("FJ", &TAG_EN_FJ),
        ("FK", &TAG_EN_FK),
        ("FM", &TAG_EN_FM),
        ("GB", &TAG_EN_GB),
        ("GD", &TAG_EN_GD),
        ("GG", &TAG_EN_GG),
        ("GH", &TAG_EN_GH),
        ("GI", &TAG_EN_GI),
        ("GM", &TAG_EN_GM),
        ("GU", &TAG_EN_GU),
        ("GY", &TAG_EN_GY),
        ("HK", &TAG_EN_HK),
        ("IE", &TAG_EN_IE),
        ("IL", &TAG_EN_IL),
        ("IM", &TAG_EN_IM),
        ("IN", &TAG_EN_IN),
        ("IO", &TAG_EN_IO),
        ("JE", &TAG_EN_JE),
        ("JM", &TAG_EN_JM),
        ("KE", &TAG_EN_KE),
        ("KI", &TAG_EN_KI),
        ("KN", &TAG_EN_KN),
        ("KY", &TAG_EN_KY),
        ("LC", &TAG_EN_LC),
        ("LR", &TAG_EN_LR),
        ("LS", &TAG_EN_LS),
        ("MG", &TAG_EN_MG),
        ("MH", &TAG_EN_MH),
        ("MO", &TAG_EN_MO),
        ("MP", &TAG_EN_MP),
        ("MS", &TAG_EN_MS),
        ("MT", &TAG_EN_MT),
        ("MU", &TAG_EN_MU),
        ("MW", &TAG_EN_MW),
        ("MY", &TAG_EN_MY),
        ("NA", &TAG_EN_NA),
        ("NF", &TAG_EN_NF),
        ("NG", &TAG_EN_NG),
        ("NL", &TAG_EN_NL),
        ("NR", &TAG_EN_NR),
        ("NU", &TAG_EN_NU),
        ("NZ", &TAG_EN_NZ),
        ("PG", &TAG_EN_PG),
        ("PH", &TAG_EN_PH),
        ("PK", &TAG_EN_PK),
        ("PN", &TAG_EN_PN),
        ("PR", &TAG_EN_PR),
        ("PW", &TAG_EN_PW),
        ("RW", &TAG_EN_RW),
        ("SB", &TAG_EN_SB),
        ("SC", &TAG_EN_SC),
        ("SD", &TAG_EN_SD),
        ("SE", &TAG_EN_SE),
        ("SG", &TAG_EN_SG),
        ("SH", &TAG_EN_SH),
        ("SI", &TAG_EN_SI),
        ("SL", &TAG_EN_SL),
        ("SS", &TAG_EN_SS),
        ("SX", &TAG_EN_SX),
        ("SZ", &TAG_EN_SZ),
        ("TC", &TAG_EN_TC),
        ("TK", &TAG_EN_TK),
        ("TO", &TAG_EN_TO),
        ("TT", &TAG_EN_TT),
        ("TV", &TAG_EN_TV),
        ("TZ", &TAG_EN_TZ),
        ("UG", &TAG_EN_UG),
        ("UM", &TAG_EN_UM),
        ("US", &TAG_EN_US),
        ("VC", &TAG_EN_VC),
        ("VG", &TAG_EN_VG),
        ("VI", &TAG_EN_VI),
        ("VU", &TAG_EN_VU),
        ("WS", &TAG_EN_WS),
        ("ZA", &TAG_EN_ZA),
        ("ZM", &TAG_EN_ZM),
        ("ZW", &TAG_EN_ZW),
    ],
};

pub static LOC_EN_001 : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_001 : CldrTree = CldrTree {
    data: &LOC_EN_001,
    children: &[
    ],
};

pub static LOC_EN_150 : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_EN_150 : CldrTree = CldrTree {
    data: &LOC_EN_150,
    children: &[
    ],
};

pub static LOC_EN_AG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_AG : CldrTree = CldrTree {
    data: &LOC_EN_AG,
    children: &[
    ],
};

pub static LOC_EN_AI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_AI : CldrTree = CldrTree {
    data: &LOC_EN_AI,
    children: &[
    ],
};

pub static LOC_EN_AS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_AS : CldrTree = CldrTree {
    data: &LOC_EN_AS,
    children: &[
    ],
};

pub static LOC_EN_AT : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::CommonExponent, 4),
    ],
    data: "·10",
};

pub static TAG_EN_AT : CldrTree = CldrTree {
    data: &LOC_EN_AT,
    children: &[
    ],
};

pub static LOC_EN_AU : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::EngineeringExponent, 1),
    ],
    data: "e",
};

pub static TAG_EN_AU : CldrTree = CldrTree {
    data: &LOC_EN_AU,
    children: &[
    ],
};

pub static LOC_EN_BB : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BB : CldrTree = CldrTree {
    data: &LOC_EN_BB,
    children: &[
    ],
};

pub static LOC_EN_BE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_EN_BE : CldrTree = CldrTree {
    data: &LOC_EN_BE,
    children: &[
    ],
};

pub static LOC_EN_BI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BI : CldrTree = CldrTree {
    data: &LOC_EN_BI,
    children: &[
    ],
};

pub static LOC_EN_BM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BM : CldrTree = CldrTree {
    data: &LOC_EN_BM,
    children: &[
    ],
};

pub static LOC_EN_BS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BS : CldrTree = CldrTree {
    data: &LOC_EN_BS,
    children: &[
    ],
};

pub static LOC_EN_BW : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BW : CldrTree = CldrTree {
    data: &LOC_EN_BW,
    children: &[
    ],
};

pub static LOC_EN_BZ : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_BZ : CldrTree = CldrTree {
    data: &LOC_EN_BZ,
    children: &[
    ],
};

pub static LOC_EN_CA : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CA : CldrTree = CldrTree {
    data: &LOC_EN_CA,
    children: &[
    ],
};

pub static LOC_EN_CC : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CC : CldrTree = CldrTree {
    data: &LOC_EN_CC,
    children: &[
    ],
};

pub static LOC_EN_CH : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::CommonExponent, 4),
    ],
    data: "·10",
};

pub static TAG_EN_CH : CldrTree = CldrTree {
    data: &LOC_EN_CH,
    children: &[
    ],
};

pub static LOC_EN_CK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CK : CldrTree = CldrTree {
    data: &LOC_EN_CK,
    children: &[
    ],
};

pub static LOC_EN_CM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CM : CldrTree = CldrTree {
    data: &LOC_EN_CM,
    children: &[
    ],
};

pub static LOC_EN_CX : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CX : CldrTree = CldrTree {
    data: &LOC_EN_CX,
    children: &[
    ],
};

pub static LOC_EN_CY : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_CY : CldrTree = CldrTree {
    data: &LOC_EN_CY,
    children: &[
    ],
};

pub static LOC_EN_DE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::CommonExponent, 4),
    ],
    data: "·10",
};

pub static TAG_EN_DE : CldrTree = CldrTree {
    data: &LOC_EN_DE,
    children: &[
    ],
};

pub static LOC_EN_DG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_DG : CldrTree = CldrTree {
    data: &LOC_EN_DG,
    children: &[
    ],
};

pub static LOC_EN_DK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_DK : CldrTree = CldrTree {
    data: &LOC_EN_DK,
    children: &[
    ],
};

pub static LOC_EN_DM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_DM : CldrTree = CldrTree {
    data: &LOC_EN_DM,
    children: &[
    ],
};

pub static LOC_EN_ER : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_ER : CldrTree = CldrTree {
    data: &LOC_EN_ER,
    children: &[
    ],
};

pub static LOC_EN_FI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_EN_FI : CldrTree = CldrTree {
    data: &LOC_EN_FI,
    children: &[
    ],
};

pub static LOC_EN_FJ : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_FJ : CldrTree = CldrTree {
    data: &LOC_EN_FJ,
    children: &[
    ],
};

pub static LOC_EN_FK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_FK : CldrTree = CldrTree {
    data: &LOC_EN_FK,
    children: &[
    ],
};

pub static LOC_EN_FM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_FM : CldrTree = CldrTree {
    data: &LOC_EN_FM,
    children: &[
    ],
};

pub static LOC_EN_GB : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GB : CldrTree = CldrTree {
    data: &LOC_EN_GB,
    children: &[
    ],
};

pub static LOC_EN_GD : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GD : CldrTree = CldrTree {
    data: &LOC_EN_GD,
    children: &[
    ],
};

pub static LOC_EN_GG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GG : CldrTree = CldrTree {
    data: &LOC_EN_GG,
    children: &[
    ],
};

pub static LOC_EN_GH : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GH : CldrTree = CldrTree {
    data: &LOC_EN_GH,
    children: &[
    ],
};

pub static LOC_EN_GI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GI : CldrTree = CldrTree {
    data: &LOC_EN_GI,
    children: &[
    ],
};

pub static LOC_EN_GM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GM : CldrTree = CldrTree {
    data: &LOC_EN_GM,
    children: &[
    ],
};

pub static LOC_EN_GU : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GU : CldrTree = CldrTree {
    data: &LOC_EN_GU,
    children: &[
    ],
};

pub static LOC_EN_GY : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_GY : CldrTree = CldrTree {
    data: &LOC_EN_GY,
    children: &[
    ],
};

pub static LOC_EN_HK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_HK : CldrTree = CldrTree {
    data: &LOC_EN_HK,
    children: &[
    ],
};

pub static LOC_EN_IE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_IE : CldrTree = CldrTree {
    data: &LOC_EN_IE,
    children: &[
    ],
};

pub static LOC_EN_IL : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_IL : CldrTree = CldrTree {
    data: &LOC_EN_IL,
    children: &[
    ],
};

pub static LOC_EN_IM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_IM : CldrTree = CldrTree {
    data: &LOC_EN_IM,
    children: &[
    ],
};

pub static LOC_EN_IN : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_EN_IN : CldrTree = CldrTree {
    data: &LOC_EN_IN,
    children: &[
    ],
};

pub static LOC_EN_IO : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_IO : CldrTree = CldrTree {
    data: &LOC_EN_IO,
    children: &[
    ],
};

pub static LOC_EN_JE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_JE : CldrTree = CldrTree {
    data: &LOC_EN_JE,
    children: &[
    ],
};

pub static LOC_EN_JM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_JM : CldrTree = CldrTree {
    data: &LOC_EN_JM,
    children: &[
    ],
};

pub static LOC_EN_KE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_KE : CldrTree = CldrTree {
    data: &LOC_EN_KE,
    children: &[
    ],
};

pub static LOC_EN_KI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_KI : CldrTree = CldrTree {
    data: &LOC_EN_KI,
    children: &[
    ],
};

pub static LOC_EN_KN : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_KN : CldrTree = CldrTree {
    data: &LOC_EN_KN,
    children: &[
    ],
};

pub static LOC_EN_KY : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_KY : CldrTree = CldrTree {
    data: &LOC_EN_KY,
    children: &[
    ],
};

pub static LOC_EN_LC : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_LC : CldrTree = CldrTree {
    data: &LOC_EN_LC,
    children: &[
    ],
};

pub static LOC_EN_LR : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_LR : CldrTree = CldrTree {
    data: &LOC_EN_LR,
    children: &[
    ],
};

pub static LOC_EN_LS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_LS : CldrTree = CldrTree {
    data: &LOC_EN_LS,
    children: &[
    ],
};

pub static LOC_EN_MG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MG : CldrTree = CldrTree {
    data: &LOC_EN_MG,
    children: &[
    ],
};

pub static LOC_EN_MH : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MH : CldrTree = CldrTree {
    data: &LOC_EN_MH,
    children: &[
    ],
};

pub static LOC_EN_MO : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MO : CldrTree = CldrTree {
    data: &LOC_EN_MO,
    children: &[
    ],
};

pub static LOC_EN_MP : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MP : CldrTree = CldrTree {
    data: &LOC_EN_MP,
    children: &[
    ],
};

pub static LOC_EN_MS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MS : CldrTree = CldrTree {
    data: &LOC_EN_MS,
    children: &[
    ],
};

pub static LOC_EN_MT : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MT : CldrTree = CldrTree {
    data: &LOC_EN_MT,
    children: &[
    ],
};

pub static LOC_EN_MU : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MU : CldrTree = CldrTree {
    data: &LOC_EN_MU,
    children: &[
    ],
};

pub static LOC_EN_MW : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MW : CldrTree = CldrTree {
    data: &LOC_EN_MW,
    children: &[
    ],
};

pub static LOC_EN_MY : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_MY : CldrTree = CldrTree {
    data: &LOC_EN_MY,
    children: &[
    ],
};

pub static LOC_EN_NA : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NA : CldrTree = CldrTree {
    data: &LOC_EN_NA,
    children: &[
    ],
};

pub static LOC_EN_NF : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NF : CldrTree = CldrTree {
    data: &LOC_EN_NF,
    children: &[
    ],
};

pub static LOC_EN_NG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NG : CldrTree = CldrTree {
    data: &LOC_EN_NG,
    children: &[
    ],
};

pub static LOC_EN_NL : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NL : CldrTree = CldrTree {
    data: &LOC_EN_NL,
    children: &[
    ],
};

pub static LOC_EN_NR : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NR : CldrTree = CldrTree {
    data: &LOC_EN_NR,
    children: &[
    ],
};

pub static LOC_EN_NU : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NU : CldrTree = CldrTree {
    data: &LOC_EN_NU,
    children: &[
    ],
};

pub static LOC_EN_NZ : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_NZ : CldrTree = CldrTree {
    data: &LOC_EN_NZ,
    children: &[
    ],
};

pub static LOC_EN_PG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PG : CldrTree = CldrTree {
    data: &LOC_EN_PG,
    children: &[
    ],
};

pub static LOC_EN_PH : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PH : CldrTree = CldrTree {
    data: &LOC_EN_PH,
    children: &[
    ],
};

pub static LOC_EN_PK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PK : CldrTree = CldrTree {
    data: &LOC_EN_PK,
    children: &[
    ],
};

pub static LOC_EN_PN : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PN : CldrTree = CldrTree {
    data: &LOC_EN_PN,
    children: &[
    ],
};

pub static LOC_EN_PR : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PR : CldrTree = CldrTree {
    data: &LOC_EN_PR,
    children: &[
    ],
};

pub static LOC_EN_PW : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_PW : CldrTree = CldrTree {
    data: &LOC_EN_PW,
    children: &[
    ],
};

pub static LOC_EN_RW : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_RW : CldrTree = CldrTree {
    data: &LOC_EN_RW,
    children: &[
    ],
};

pub static LOC_EN_SB : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SB : CldrTree = CldrTree {
    data: &LOC_EN_SB,
    children: &[
    ],
};

pub static LOC_EN_SC : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SC : CldrTree = CldrTree {
    data: &LOC_EN_SC,
    children: &[
    ],
};

pub static LOC_EN_SD : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SD : CldrTree = CldrTree {
    data: &LOC_EN_SD,
    children: &[
    ],
};

pub static LOC_EN_SE : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::GroupSeparator, 2),
        (Item::EngineeringExponent, 7),
    ],
    data: " ×10^",
};

pub static TAG_EN_SE : CldrTree = CldrTree {
    data: &LOC_EN_SE,
    children: &[
    ],
};

pub static LOC_EN_SG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SG : CldrTree = CldrTree {
    data: &LOC_EN_SG,
    children: &[
    ],
};

pub static LOC_EN_SH : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SH : CldrTree = CldrTree {
    data: &LOC_EN_SH,
    children: &[
    ],
};

pub static LOC_EN_SI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::EngineeringExponent, 1),
    ],
    data: "e",
};

pub static TAG_EN_SI : CldrTree = CldrTree {
    data: &LOC_EN_SI,
    children: &[
    ],
};

pub static LOC_EN_SL : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SL : CldrTree = CldrTree {
    data: &LOC_EN_SL,
    children: &[
    ],
};

pub static LOC_EN_SS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SS : CldrTree = CldrTree {
    data: &LOC_EN_SS,
    children: &[
    ],
};

pub static LOC_EN_SX : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SX : CldrTree = CldrTree {
    data: &LOC_EN_SX,
    children: &[
    ],
};

pub static LOC_EN_SZ : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_SZ : CldrTree = CldrTree {
    data: &LOC_EN_SZ,
    children: &[
    ],
};

pub static LOC_EN_TC : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TC : CldrTree = CldrTree {
    data: &LOC_EN_TC,
    children: &[
    ],
};

pub static LOC_EN_TK : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TK : CldrTree = CldrTree {
    data: &LOC_EN_TK,
    children: &[
    ],
};

pub static LOC_EN_TO : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TO : CldrTree = CldrTree {
    data: &LOC_EN_TO,
    children: &[
    ],
};

pub static LOC_EN_TT : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TT : CldrTree = CldrTree {
    data: &LOC_EN_TT,
    children: &[
    ],
};

pub static LOC_EN_TV : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TV : CldrTree = CldrTree {
    data: &LOC_EN_TV,
    children: &[
    ],
};

pub static LOC_EN_TZ : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_TZ : CldrTree = CldrTree {
    data: &LOC_EN_TZ,
    children: &[
    ],
};

pub static LOC_EN_UG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_UG : CldrTree = CldrTree {
    data: &LOC_EN_UG,
    children: &[
    ],
};

pub static LOC_EN_UM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_UM : CldrTree = CldrTree {
    data: &LOC_EN_UM,
    children: &[
    ],
};

pub static LOC_EN_US : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_US : CldrTree = CldrTree {
    data: &LOC_EN_US,
    children: &[
        ("posix", &TAG_EN_US_POSIX),
    ],
};

pub static LOC_EN_US_POSIX : CldrData = CldrData {
    parents: &[
        &LOC_EN_US,
    ],
    index: &[
        (Item::PerMilleSign, 4),
        (Item::InfinitySymbol, 7),
        (Item::Grouping, 7),
    ],
    data: "0/00INF",
};

pub static TAG_EN_US_POSIX : CldrTree = CldrTree {
    data: &LOC_EN_US_POSIX,
    children: &[
    ],
};

pub static LOC_EN_VC : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_VC : CldrTree = CldrTree {
    data: &LOC_EN_VC,
    children: &[
    ],
};

pub static LOC_EN_VG : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_VG : CldrTree = CldrTree {
    data: &LOC_EN_VG,
    children: &[
    ],
};

pub static LOC_EN_VI : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_VI : CldrTree = CldrTree {
    data: &LOC_EN_VI,
    children: &[
    ],
};

pub static LOC_EN_VU : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_VU : CldrTree = CldrTree {
    data: &LOC_EN_VU,
    children: &[
    ],
};

pub static LOC_EN_WS : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_WS : CldrTree = CldrTree {
    data: &LOC_EN_WS,
    children: &[
    ],
};

pub static LOC_EN_ZA : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_EN_ZA : CldrTree = CldrTree {
    data: &LOC_EN_ZA,
    children: &[
    ],
};

pub static LOC_EN_ZM : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_ZM : CldrTree = CldrTree {
    data: &LOC_EN_ZM,
    children: &[
    ],
};

pub static LOC_EN_ZW : CldrData = CldrData {
    parents: &[
        &LOC_EN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EN_ZW : CldrTree = CldrTree {
    data: &LOC_EN_ZW,
    children: &[
    ],
};

pub static LOC_EO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
    ],
    data: ", −",
};

pub static TAG_EO : CldrTree = CldrTree {
    data: &LOC_EO,
    children: &[
        ("001", &TAG_EO_001),
    ],
};

pub static LOC_EO_001 : CldrData = CldrData {
    parents: &[
        &LOC_EO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EO_001 : CldrTree = CldrTree {
    data: &LOC_EO_001,
    children: &[
    ],
};

pub static LOC_ES : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::MinGroupingDigits, 3),
    ],
    data: ",.2",
};

pub static TAG_ES : CldrTree = CldrTree {
    data: &LOC_ES,
    children: &[
        ("419", &TAG_ES_419),
        ("AR", &TAG_ES_AR),
        ("BO", &TAG_ES_BO),
        ("BR", &TAG_ES_BR),
        ("BZ", &TAG_ES_BZ),
        ("CL", &TAG_ES_CL),
        ("CO", &TAG_ES_CO),
        ("CR", &TAG_ES_CR),
        ("CU", &TAG_ES_CU),
        ("DO", &TAG_ES_DO),
        ("EA", &TAG_ES_EA),
        ("EC", &TAG_ES_EC),
        ("ES", &TAG_ES_ES),
        ("GQ", &TAG_ES_GQ),
        ("GT", &TAG_ES_GT),
        ("HN", &TAG_ES_HN),
        ("IC", &TAG_ES_IC),
        ("MX", &TAG_ES_MX),
        ("NI", &TAG_ES_NI),
        ("PA", &TAG_ES_PA),
        ("PE", &TAG_ES_PE),
        ("PH", &TAG_ES_PH),
        ("PR", &TAG_ES_PR),
        ("PY", &TAG_ES_PY),
        ("SV", &TAG_ES_SV),
        ("US", &TAG_ES_US),
        ("UY", &TAG_ES_UY),
        ("VE", &TAG_ES_VE),
    ],
};

pub static LOC_ES_419 : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ".,",
};

pub static TAG_ES_419 : CldrTree = CldrTree {
    data: &LOC_ES_419,
    children: &[
    ],
};

pub static LOC_ES_AR : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_AR : CldrTree = CldrTree {
    data: &LOC_ES_AR,
    children: &[
    ],
};

pub static LOC_ES_BO : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_BO : CldrTree = CldrTree {
    data: &LOC_ES_BO,
    children: &[
    ],
};

pub static LOC_ES_BR : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_BR : CldrTree = CldrTree {
    data: &LOC_ES_BR,
    children: &[
    ],
};

pub static LOC_ES_BZ : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_BZ : CldrTree = CldrTree {
    data: &LOC_ES_BZ,
    children: &[
    ],
};

pub static LOC_ES_CL : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_CL : CldrTree = CldrTree {
    data: &LOC_ES_CL,
    children: &[
    ],
};

pub static LOC_ES_CO : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_CO : CldrTree = CldrTree {
    data: &LOC_ES_CO,
    children: &[
    ],
};

pub static LOC_ES_CR : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_ES_CR : CldrTree = CldrTree {
    data: &LOC_ES_CR,
    children: &[
    ],
};

pub static LOC_ES_CU : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_CU : CldrTree = CldrTree {
    data: &LOC_ES_CU,
    children: &[
    ],
};

pub static LOC_ES_DO : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_DO : CldrTree = CldrTree {
    data: &LOC_ES_DO,
    children: &[
    ],
};

pub static LOC_ES_EA : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_EA : CldrTree = CldrTree {
    data: &LOC_ES_EA,
    children: &[
    ],
};

pub static LOC_ES_EC : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_EC : CldrTree = CldrTree {
    data: &LOC_ES_EC,
    children: &[
    ],
};

pub static LOC_ES_ES : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_ES : CldrTree = CldrTree {
    data: &LOC_ES_ES,
    children: &[
    ],
};

pub static LOC_ES_GQ : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_GQ : CldrTree = CldrTree {
    data: &LOC_ES_GQ,
    children: &[
    ],
};

pub static LOC_ES_GT : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_GT : CldrTree = CldrTree {
    data: &LOC_ES_GT,
    children: &[
    ],
};

pub static LOC_ES_HN : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_HN : CldrTree = CldrTree {
    data: &LOC_ES_HN,
    children: &[
    ],
};

pub static LOC_ES_IC : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_IC : CldrTree = CldrTree {
    data: &LOC_ES_IC,
    children: &[
    ],
};

pub static LOC_ES_MX : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_MX : CldrTree = CldrTree {
    data: &LOC_ES_MX,
    children: &[
    ],
};

pub static LOC_ES_NI : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_NI : CldrTree = CldrTree {
    data: &LOC_ES_NI,
    children: &[
    ],
};

pub static LOC_ES_PA : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_PA : CldrTree = CldrTree {
    data: &LOC_ES_PA,
    children: &[
    ],
};

pub static LOC_ES_PE : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_PE : CldrTree = CldrTree {
    data: &LOC_ES_PE,
    children: &[
    ],
};

pub static LOC_ES_PH : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_PH : CldrTree = CldrTree {
    data: &LOC_ES_PH,
    children: &[
    ],
};

pub static LOC_ES_PR : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_PR : CldrTree = CldrTree {
    data: &LOC_ES_PR,
    children: &[
    ],
};

pub static LOC_ES_PY : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_PY : CldrTree = CldrTree {
    data: &LOC_ES_PY,
    children: &[
    ],
};

pub static LOC_ES_SV : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_SV : CldrTree = CldrTree {
    data: &LOC_ES_SV,
    children: &[
    ],
};

pub static LOC_ES_US : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_US : CldrTree = CldrTree {
    data: &LOC_ES_US,
    children: &[
    ],
};

pub static LOC_ES_UY : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_UY : CldrTree = CldrTree {
    data: &LOC_ES_UY,
    children: &[
    ],
};

pub static LOC_ES_VE : CldrData = CldrData {
    parents: &[
        &LOC_ES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ES_VE : CldrTree = CldrTree {
    data: &LOC_ES_VE,
    children: &[
    ],
};

pub static LOC_ET : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::EngineeringExponent, 11),
        (Item::MinGroupingDigits, 12),
    ],
    data: ", −×10^2",
};

pub static TAG_ET : CldrTree = CldrTree {
    data: &LOC_ET,
    children: &[
        ("EE", &TAG_ET_EE),
    ],
};

pub static LOC_ET_EE : CldrData = CldrData {
    parents: &[
        &LOC_ET,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ET_EE : CldrTree = CldrTree {
    data: &LOC_ET_EE,
    children: &[
    ],
};

pub static LOC_EU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_EU : CldrTree = CldrTree {
    data: &LOC_EU,
    children: &[
        ("ES", &TAG_EU_ES),
    ],
};

pub static LOC_EU_ES : CldrData = CldrData {
    parents: &[
        &LOC_EU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EU_ES : CldrTree = CldrTree {
    data: &LOC_EU_ES,
    children: &[
    ],
};

pub static LOC_EWO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_EWO : CldrTree = CldrTree {
    data: &LOC_EWO,
    children: &[
        ("CM", &TAG_EWO_CM),
    ],
};

pub static LOC_EWO_CM : CldrData = CldrData {
    parents: &[
        &LOC_EWO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_EWO_CM : CldrTree = CldrTree {
    data: &LOC_EWO_CM,
    children: &[
    ],
};

pub static LOC_FA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
        (Item::DecimalSeparator, 22),
        (Item::GroupSeparator, 24),
        (Item::PlusSign, 28),
        (Item::MinusSign, 34),
        (Item::PercentSign, 39),
        (Item::PerMilleSign, 41),
        (Item::EngineeringExponent, 48),
        (Item::CommonExponent, 55),
        (Item::NotANumberSymbol, 65),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹٫٬‎+‎−‎٪؉×۱۰^×۱۰^ناعدد",
};

pub static TAG_FA : CldrTree = CldrTree {
    data: &LOC_FA,
    children: &[
        ("AF", &TAG_FA_AF),
        ("IR", &TAG_FA_IR),
    ],
};

pub static LOC_FA_AF : CldrData = CldrData {
    parents: &[
        &LOC_FA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FA_AF : CldrTree = CldrTree {
    data: &LOC_FA_AF,
    children: &[
    ],
};

pub static LOC_FA_IR : CldrData = CldrData {
    parents: &[
        &LOC_FA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FA_IR : CldrTree = CldrTree {
    data: &LOC_FA_IR,
    children: &[
    ],
};

pub static LOC_FF : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_FF : CldrTree = CldrTree {
    data: &LOC_FF,
    children: &[
        ("CM", &TAG_FF_CM),
        ("GN", &TAG_FF_GN),
        ("MR", &TAG_FF_MR),
        ("SN", &TAG_FF_SN),
    ],
};

pub static LOC_FF_CM : CldrData = CldrData {
    parents: &[
        &LOC_FF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FF_CM : CldrTree = CldrTree {
    data: &LOC_FF_CM,
    children: &[
    ],
};

pub static LOC_FF_GN : CldrData = CldrData {
    parents: &[
        &LOC_FF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FF_GN : CldrTree = CldrTree {
    data: &LOC_FF_GN,
    children: &[
    ],
};

pub static LOC_FF_MR : CldrData = CldrData {
    parents: &[
        &LOC_FF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FF_MR : CldrTree = CldrTree {
    data: &LOC_FF_MR,
    children: &[
    ],
};

pub static LOC_FF_SN : CldrData = CldrData {
    parents: &[
        &LOC_FF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FF_SN : CldrTree = CldrTree {
    data: &LOC_FF_SN,
    children: &[
    ],
};

pub static LOC_FI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::NotANumberSymbol, 14),
    ],
    data: ", −epäluku",
};

pub static TAG_FI : CldrTree = CldrTree {
    data: &LOC_FI,
    children: &[
        ("FI", &TAG_FI_FI),
    ],
};

pub static LOC_FI_FI : CldrData = CldrData {
    parents: &[
        &LOC_FI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FI_FI : CldrTree = CldrTree {
    data: &LOC_FI_FI,
    children: &[
    ],
};

pub static LOC_FIL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FIL : CldrTree = CldrTree {
    data: &LOC_FIL,
    children: &[
        ("PH", &TAG_FIL_PH),
    ],
};

pub static LOC_FIL_PH : CldrData = CldrData {
    parents: &[
        &LOC_FIL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FIL_PH : CldrTree = CldrTree {
    data: &LOC_FIL_PH,
    children: &[
    ],
};

pub static LOC_FO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::MinusSign, 5),
    ],
    data: ",.−",
};

pub static TAG_FO : CldrTree = CldrTree {
    data: &LOC_FO,
    children: &[
        ("DK", &TAG_FO_DK),
        ("FO", &TAG_FO_FO),
    ],
};

pub static LOC_FO_DK : CldrData = CldrData {
    parents: &[
        &LOC_FO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FO_DK : CldrTree = CldrTree {
    data: &LOC_FO_DK,
    children: &[
    ],
};

pub static LOC_FO_FO : CldrData = CldrData {
    parents: &[
        &LOC_FO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FO_FO : CldrTree = CldrTree {
    data: &LOC_FO_FO,
    children: &[
    ],
};

pub static LOC_FR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_FR : CldrTree = CldrTree {
    data: &LOC_FR,
    children: &[
        ("BE", &TAG_FR_BE),
        ("BF", &TAG_FR_BF),
        ("BI", &TAG_FR_BI),
        ("BJ", &TAG_FR_BJ),
        ("BL", &TAG_FR_BL),
        ("CA", &TAG_FR_CA),
        ("CD", &TAG_FR_CD),
        ("CF", &TAG_FR_CF),
        ("CG", &TAG_FR_CG),
        ("CH", &TAG_FR_CH),
        ("CI", &TAG_FR_CI),
        ("CM", &TAG_FR_CM),
        ("DJ", &TAG_FR_DJ),
        ("DZ", &TAG_FR_DZ),
        ("FR", &TAG_FR_FR),
        ("GA", &TAG_FR_GA),
        ("GF", &TAG_FR_GF),
        ("GN", &TAG_FR_GN),
        ("GP", &TAG_FR_GP),
        ("GQ", &TAG_FR_GQ),
        ("HT", &TAG_FR_HT),
        ("KM", &TAG_FR_KM),
        ("LU", &TAG_FR_LU),
        ("MA", &TAG_FR_MA),
        ("MC", &TAG_FR_MC),
        ("MF", &TAG_FR_MF),
        ("MG", &TAG_FR_MG),
        ("ML", &TAG_FR_ML),
        ("MQ", &TAG_FR_MQ),
        ("MR", &TAG_FR_MR),
        ("MU", &TAG_FR_MU),
        ("NC", &TAG_FR_NC),
        ("NE", &TAG_FR_NE),
        ("PF", &TAG_FR_PF),
        ("PM", &TAG_FR_PM),
        ("RE", &TAG_FR_RE),
        ("RW", &TAG_FR_RW),
        ("SC", &TAG_FR_SC),
        ("SN", &TAG_FR_SN),
        ("SY", &TAG_FR_SY),
        ("TD", &TAG_FR_TD),
        ("TG", &TAG_FR_TG),
        ("TN", &TAG_FR_TN),
        ("VU", &TAG_FR_VU),
        ("WF", &TAG_FR_WF),
        ("YT", &TAG_FR_YT),
    ],
};

pub static LOC_FR_BE : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_BE : CldrTree = CldrTree {
    data: &LOC_FR_BE,
    children: &[
    ],
};

pub static LOC_FR_BF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_BF : CldrTree = CldrTree {
    data: &LOC_FR_BF,
    children: &[
    ],
};

pub static LOC_FR_BI : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_BI : CldrTree = CldrTree {
    data: &LOC_FR_BI,
    children: &[
    ],
};

pub static LOC_FR_BJ : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_BJ : CldrTree = CldrTree {
    data: &LOC_FR_BJ,
    children: &[
    ],
};

pub static LOC_FR_BL : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_BL : CldrTree = CldrTree {
    data: &LOC_FR_BL,
    children: &[
    ],
};

pub static LOC_FR_CA : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CA : CldrTree = CldrTree {
    data: &LOC_FR_CA,
    children: &[
    ],
};

pub static LOC_FR_CD : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CD : CldrTree = CldrTree {
    data: &LOC_FR_CD,
    children: &[
    ],
};

pub static LOC_FR_CF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CF : CldrTree = CldrTree {
    data: &LOC_FR_CF,
    children: &[
    ],
};

pub static LOC_FR_CG : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CG : CldrTree = CldrTree {
    data: &LOC_FR_CG,
    children: &[
    ],
};

pub static LOC_FR_CH : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CH : CldrTree = CldrTree {
    data: &LOC_FR_CH,
    children: &[
    ],
};

pub static LOC_FR_CI : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CI : CldrTree = CldrTree {
    data: &LOC_FR_CI,
    children: &[
    ],
};

pub static LOC_FR_CM : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_CM : CldrTree = CldrTree {
    data: &LOC_FR_CM,
    children: &[
    ],
};

pub static LOC_FR_DJ : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_DJ : CldrTree = CldrTree {
    data: &LOC_FR_DJ,
    children: &[
    ],
};

pub static LOC_FR_DZ : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_DZ : CldrTree = CldrTree {
    data: &LOC_FR_DZ,
    children: &[
    ],
};

pub static LOC_FR_FR : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_FR : CldrTree = CldrTree {
    data: &LOC_FR_FR,
    children: &[
    ],
};

pub static LOC_FR_GA : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_GA : CldrTree = CldrTree {
    data: &LOC_FR_GA,
    children: &[
    ],
};

pub static LOC_FR_GF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_GF : CldrTree = CldrTree {
    data: &LOC_FR_GF,
    children: &[
    ],
};

pub static LOC_FR_GN : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_GN : CldrTree = CldrTree {
    data: &LOC_FR_GN,
    children: &[
    ],
};

pub static LOC_FR_GP : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_GP : CldrTree = CldrTree {
    data: &LOC_FR_GP,
    children: &[
    ],
};

pub static LOC_FR_GQ : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_GQ : CldrTree = CldrTree {
    data: &LOC_FR_GQ,
    children: &[
    ],
};

pub static LOC_FR_HT : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_HT : CldrTree = CldrTree {
    data: &LOC_FR_HT,
    children: &[
    ],
};

pub static LOC_FR_KM : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_KM : CldrTree = CldrTree {
    data: &LOC_FR_KM,
    children: &[
    ],
};

pub static LOC_FR_LU : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
        (Item::GroupSeparator, 1),
    ],
    data: ".",
};

pub static TAG_FR_LU : CldrTree = CldrTree {
    data: &LOC_FR_LU,
    children: &[
    ],
};

pub static LOC_FR_MA : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
        (Item::GroupSeparator, 1),
    ],
    data: ".",
};

pub static TAG_FR_MA : CldrTree = CldrTree {
    data: &LOC_FR_MA,
    children: &[
    ],
};

pub static LOC_FR_MC : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MC : CldrTree = CldrTree {
    data: &LOC_FR_MC,
    children: &[
    ],
};

pub static LOC_FR_MF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MF : CldrTree = CldrTree {
    data: &LOC_FR_MF,
    children: &[
    ],
};

pub static LOC_FR_MG : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MG : CldrTree = CldrTree {
    data: &LOC_FR_MG,
    children: &[
    ],
};

pub static LOC_FR_ML : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_ML : CldrTree = CldrTree {
    data: &LOC_FR_ML,
    children: &[
    ],
};

pub static LOC_FR_MQ : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MQ : CldrTree = CldrTree {
    data: &LOC_FR_MQ,
    children: &[
    ],
};

pub static LOC_FR_MR : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MR : CldrTree = CldrTree {
    data: &LOC_FR_MR,
    children: &[
    ],
};

pub static LOC_FR_MU : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_MU : CldrTree = CldrTree {
    data: &LOC_FR_MU,
    children: &[
    ],
};

pub static LOC_FR_NC : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_NC : CldrTree = CldrTree {
    data: &LOC_FR_NC,
    children: &[
    ],
};

pub static LOC_FR_NE : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_NE : CldrTree = CldrTree {
    data: &LOC_FR_NE,
    children: &[
    ],
};

pub static LOC_FR_PF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_PF : CldrTree = CldrTree {
    data: &LOC_FR_PF,
    children: &[
    ],
};

pub static LOC_FR_PM : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_PM : CldrTree = CldrTree {
    data: &LOC_FR_PM,
    children: &[
    ],
};

pub static LOC_FR_RE : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_RE : CldrTree = CldrTree {
    data: &LOC_FR_RE,
    children: &[
    ],
};

pub static LOC_FR_RW : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_RW : CldrTree = CldrTree {
    data: &LOC_FR_RW,
    children: &[
    ],
};

pub static LOC_FR_SC : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_SC : CldrTree = CldrTree {
    data: &LOC_FR_SC,
    children: &[
    ],
};

pub static LOC_FR_SN : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_SN : CldrTree = CldrTree {
    data: &LOC_FR_SN,
    children: &[
    ],
};

pub static LOC_FR_SY : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_SY : CldrTree = CldrTree {
    data: &LOC_FR_SY,
    children: &[
    ],
};

pub static LOC_FR_TD : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_TD : CldrTree = CldrTree {
    data: &LOC_FR_TD,
    children: &[
    ],
};

pub static LOC_FR_TG : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_TG : CldrTree = CldrTree {
    data: &LOC_FR_TG,
    children: &[
    ],
};

pub static LOC_FR_TN : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_TN : CldrTree = CldrTree {
    data: &LOC_FR_TN,
    children: &[
    ],
};

pub static LOC_FR_VU : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_VU : CldrTree = CldrTree {
    data: &LOC_FR_VU,
    children: &[
    ],
};

pub static LOC_FR_WF : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_WF : CldrTree = CldrTree {
    data: &LOC_FR_WF,
    children: &[
    ],
};

pub static LOC_FR_YT : CldrData = CldrData {
    parents: &[
        &LOC_FR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FR_YT : CldrTree = CldrTree {
    data: &LOC_FR_YT,
    children: &[
    ],
};

pub static LOC_FUR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_FUR : CldrTree = CldrTree {
    data: &LOC_FUR,
    children: &[
        ("IT", &TAG_FUR_IT),
    ],
};

pub static LOC_FUR_IT : CldrData = CldrData {
    parents: &[
        &LOC_FUR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FUR_IT : CldrTree = CldrTree {
    data: &LOC_FUR_IT,
    children: &[
    ],
};

pub static LOC_FY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_FY : CldrTree = CldrTree {
    data: &LOC_FY,
    children: &[
        ("NL", &TAG_FY_NL),
    ],
};

pub static LOC_FY_NL : CldrData = CldrData {
    parents: &[
        &LOC_FY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_FY_NL : CldrTree = CldrTree {
    data: &LOC_FY_NL,
    children: &[
    ],
};

pub static LOC_GA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GA : CldrTree = CldrTree {
    data: &LOC_GA,
    children: &[
        ("IE", &TAG_GA_IE),
    ],
};

pub static LOC_GA_IE : CldrData = CldrData {
    parents: &[
        &LOC_GA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GA_IE : CldrTree = CldrTree {
    data: &LOC_GA_IE,
    children: &[
    ],
};

pub static LOC_GD : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GD : CldrTree = CldrTree {
    data: &LOC_GD,
    children: &[
        ("GB", &TAG_GD_GB),
    ],
};

pub static LOC_GD_GB : CldrData = CldrData {
    parents: &[
        &LOC_GD,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GD_GB : CldrTree = CldrTree {
    data: &LOC_GD_GB,
    children: &[
    ],
};

pub static LOC_GL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_GL : CldrTree = CldrTree {
    data: &LOC_GL,
    children: &[
        ("ES", &TAG_GL_ES),
    ],
};

pub static LOC_GL_ES : CldrData = CldrData {
    parents: &[
        &LOC_GL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GL_ES : CldrTree = CldrTree {
    data: &LOC_GL_ES,
    children: &[
    ],
};

pub static LOC_GSW : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
    ],
    data: "’−",
};

pub static TAG_GSW : CldrTree = CldrTree {
    data: &LOC_GSW,
    children: &[
        ("CH", &TAG_GSW_CH),
        ("FR", &TAG_GSW_FR),
        ("LI", &TAG_GSW_LI),
    ],
};

pub static LOC_GSW_CH : CldrData = CldrData {
    parents: &[
        &LOC_GSW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GSW_CH : CldrTree = CldrTree {
    data: &LOC_GSW_CH,
    children: &[
    ],
};

pub static LOC_GSW_FR : CldrData = CldrData {
    parents: &[
        &LOC_GSW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GSW_FR : CldrTree = CldrTree {
    data: &LOC_GSW_FR,
    children: &[
    ],
};

pub static LOC_GSW_LI : CldrData = CldrData {
    parents: &[
        &LOC_GSW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GSW_LI : CldrTree = CldrTree {
    data: &LOC_GSW_LI,
    children: &[
    ],
};

pub static LOC_GU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_GU : CldrTree = CldrTree {
    data: &LOC_GU,
    children: &[
        ("IN", &TAG_GU_IN),
    ],
};

pub static LOC_GU_IN : CldrData = CldrData {
    parents: &[
        &LOC_GU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GU_IN : CldrTree = CldrTree {
    data: &LOC_GU_IN,
    children: &[
    ],
};

pub static LOC_GUZ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GUZ : CldrTree = CldrTree {
    data: &LOC_GUZ,
    children: &[
        ("KE", &TAG_GUZ_KE),
    ],
};

pub static LOC_GUZ_KE : CldrData = CldrData {
    parents: &[
        &LOC_GUZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GUZ_KE : CldrTree = CldrTree {
    data: &LOC_GUZ_KE,
    children: &[
    ],
};

pub static LOC_GV : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GV : CldrTree = CldrTree {
    data: &LOC_GV,
    children: &[
        ("IM", &TAG_GV_IM),
    ],
};

pub static LOC_GV_IM : CldrData = CldrData {
    parents: &[
        &LOC_GV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_GV_IM : CldrTree = CldrTree {
    data: &LOC_GV_IM,
    children: &[
    ],
};

pub static LOC_HA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HA : CldrTree = CldrTree {
    data: &LOC_HA,
    children: &[
        ("GH", &TAG_HA_GH),
        ("NE", &TAG_HA_NE),
        ("NG", &TAG_HA_NG),
    ],
};

pub static LOC_HA_GH : CldrData = CldrData {
    parents: &[
        &LOC_HA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HA_GH : CldrTree = CldrTree {
    data: &LOC_HA_GH,
    children: &[
    ],
};

pub static LOC_HA_NE : CldrData = CldrData {
    parents: &[
        &LOC_HA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HA_NE : CldrTree = CldrTree {
    data: &LOC_HA_NE,
    children: &[
    ],
};

pub static LOC_HA_NG : CldrData = CldrData {
    parents: &[
        &LOC_HA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HA_NG : CldrTree = CldrTree {
    data: &LOC_HA_NG,
    children: &[
    ],
};

pub static LOC_HAW : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HAW : CldrTree = CldrTree {
    data: &LOC_HAW,
    children: &[
        ("US", &TAG_HAW_US),
    ],
};

pub static LOC_HAW_US : CldrData = CldrData {
    parents: &[
        &LOC_HAW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HAW_US : CldrTree = CldrTree {
    data: &LOC_HAW_US,
    children: &[
    ],
};

pub static LOC_HE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::PlusSign, 4),
        (Item::MinusSign, 8),
    ],
    data: "‎+‎-",
};

pub static TAG_HE : CldrTree = CldrTree {
    data: &LOC_HE,
    children: &[
        ("IL", &TAG_HE_IL),
    ],
};

pub static LOC_HE_IL : CldrData = CldrData {
    parents: &[
        &LOC_HE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HE_IL : CldrTree = CldrTree {
    data: &LOC_HE_IL,
    children: &[
    ],
};

pub static LOC_HI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_HI : CldrTree = CldrTree {
    data: &LOC_HI,
    children: &[
        ("IN", &TAG_HI_IN),
    ],
};

pub static LOC_HI_IN : CldrData = CldrData {
    parents: &[
        &LOC_HI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HI_IN : CldrTree = CldrTree {
    data: &LOC_HI_IN,
    children: &[
    ],
};

pub static LOC_HR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_HR : CldrTree = CldrTree {
    data: &LOC_HR,
    children: &[
        ("BA", &TAG_HR_BA),
        ("HR", &TAG_HR_HR),
    ],
};

pub static LOC_HR_BA : CldrData = CldrData {
    parents: &[
        &LOC_HR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HR_BA : CldrTree = CldrTree {
    data: &LOC_HR_BA,
    children: &[
    ],
};

pub static LOC_HR_HR : CldrData = CldrData {
    parents: &[
        &LOC_HR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HR_HR : CldrTree = CldrTree {
    data: &LOC_HR_HR,
    children: &[
    ],
};

pub static LOC_HSB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::CommonExponent, 6),
    ],
    data: ",.·10",
};

pub static TAG_HSB : CldrTree = CldrTree {
    data: &LOC_HSB,
    children: &[
        ("DE", &TAG_HSB_DE),
    ],
};

pub static LOC_HSB_DE : CldrData = CldrData {
    parents: &[
        &LOC_HSB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HSB_DE : CldrTree = CldrTree {
    data: &LOC_HSB_DE,
    children: &[
    ],
};

pub static LOC_HU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_HU : CldrTree = CldrTree {
    data: &LOC_HU,
    children: &[
        ("HU", &TAG_HU_HU),
    ],
};

pub static LOC_HU_HU : CldrData = CldrData {
    parents: &[
        &LOC_HU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HU_HU : CldrTree = CldrTree {
    data: &LOC_HU_HU,
    children: &[
    ],
};

pub static LOC_HY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 9),
    ],
    data: ", ՈչԹ",
};

pub static TAG_HY : CldrTree = CldrTree {
    data: &LOC_HY,
    children: &[
        ("AM", &TAG_HY_AM),
    ],
};

pub static LOC_HY_AM : CldrData = CldrData {
    parents: &[
        &LOC_HY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_HY_AM : CldrTree = CldrTree {
    data: &LOC_HY_AM,
    children: &[
    ],
};

pub static LOC_ID : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_ID : CldrTree = CldrTree {
    data: &LOC_ID,
    children: &[
        ("ID", &TAG_ID_ID),
    ],
};

pub static LOC_ID_ID : CldrData = CldrData {
    parents: &[
        &LOC_ID,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ID_ID : CldrTree = CldrTree {
    data: &LOC_ID_ID,
    children: &[
    ],
};

pub static LOC_IG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IG : CldrTree = CldrTree {
    data: &LOC_IG,
    children: &[
        ("NG", &TAG_IG_NG),
    ],
};

pub static LOC_IG_NG : CldrData = CldrData {
    parents: &[
        &LOC_IG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IG_NG : CldrTree = CldrTree {
    data: &LOC_IG_NG,
    children: &[
    ],
};

pub static LOC_II : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_II : CldrTree = CldrTree {
    data: &LOC_II,
    children: &[
        ("CN", &TAG_II_CN),
    ],
};

pub static LOC_II_CN : CldrData = CldrData {
    parents: &[
        &LOC_II,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_II_CN : CldrTree = CldrTree {
    data: &LOC_II_CN,
    children: &[
    ],
};

pub static LOC_IS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_IS : CldrTree = CldrTree {
    data: &LOC_IS,
    children: &[
        ("IS", &TAG_IS_IS),
    ],
};

pub static LOC_IS_IS : CldrData = CldrData {
    parents: &[
        &LOC_IS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IS_IS : CldrTree = CldrTree {
    data: &LOC_IS_IS,
    children: &[
    ],
};

pub static LOC_IT : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_IT : CldrTree = CldrTree {
    data: &LOC_IT,
    children: &[
        ("CH", &TAG_IT_CH),
        ("IT", &TAG_IT_IT),
        ("SM", &TAG_IT_SM),
        ("VA", &TAG_IT_VA),
    ],
};

pub static LOC_IT_CH : CldrData = CldrData {
    parents: &[
        &LOC_IT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 4),
    ],
    data: ".’",
};

pub static TAG_IT_CH : CldrTree = CldrTree {
    data: &LOC_IT_CH,
    children: &[
    ],
};

pub static LOC_IT_IT : CldrData = CldrData {
    parents: &[
        &LOC_IT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IT_IT : CldrTree = CldrTree {
    data: &LOC_IT_IT,
    children: &[
    ],
};

pub static LOC_IT_SM : CldrData = CldrData {
    parents: &[
        &LOC_IT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IT_SM : CldrTree = CldrTree {
    data: &LOC_IT_SM,
    children: &[
    ],
};

pub static LOC_IT_VA : CldrData = CldrData {
    parents: &[
        &LOC_IT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_IT_VA : CldrTree = CldrTree {
    data: &LOC_IT_VA,
    children: &[
    ],
};

pub static LOC_JA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_JA : CldrTree = CldrTree {
    data: &LOC_JA,
    children: &[
        ("JP", &TAG_JA_JP),
    ],
};

pub static LOC_JA_JP : CldrData = CldrData {
    parents: &[
        &LOC_JA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_JA_JP : CldrTree = CldrTree {
    data: &LOC_JA_JP,
    children: &[
    ],
};

pub static LOC_JGO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_JGO : CldrTree = CldrTree {
    data: &LOC_JGO,
    children: &[
        ("CM", &TAG_JGO_CM),
    ],
};

pub static LOC_JGO_CM : CldrData = CldrData {
    parents: &[
        &LOC_JGO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_JGO_CM : CldrTree = CldrTree {
    data: &LOC_JGO_CM,
    children: &[
    ],
};

pub static LOC_JMC : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_JMC : CldrTree = CldrTree {
    data: &LOC_JMC,
    children: &[
        ("TZ", &TAG_JMC_TZ),
    ],
};

pub static LOC_JMC_TZ : CldrData = CldrData {
    parents: &[
        &LOC_JMC,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_JMC_TZ : CldrTree = CldrTree {
    data: &LOC_JMC_TZ,
    children: &[
    ],
};

pub static LOC_KA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 43),
        (Item::MinGroupingDigits, 44),
    ],
    data: ", არ არის რიცხვი2",
};

pub static TAG_KA : CldrTree = CldrTree {
    data: &LOC_KA,
    children: &[
        ("GE", &TAG_KA_GE),
    ],
};

pub static LOC_KA_GE : CldrData = CldrData {
    parents: &[
        &LOC_KA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KA_GE : CldrTree = CldrTree {
    data: &LOC_KA_GE,
    children: &[
    ],
};

pub static LOC_KAB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_KAB : CldrTree = CldrTree {
    data: &LOC_KAB,
    children: &[
        ("DZ", &TAG_KAB_DZ),
    ],
};

pub static LOC_KAB_DZ : CldrData = CldrData {
    parents: &[
        &LOC_KAB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KAB_DZ : CldrTree = CldrTree {
    data: &LOC_KAB_DZ,
    children: &[
    ],
};

pub static LOC_KAM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KAM : CldrTree = CldrTree {
    data: &LOC_KAM,
    children: &[
        ("KE", &TAG_KAM_KE),
    ],
};

pub static LOC_KAM_KE : CldrData = CldrData {
    parents: &[
        &LOC_KAM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KAM_KE : CldrTree = CldrTree {
    data: &LOC_KAM_KE,
    children: &[
    ],
};

pub static LOC_KDE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KDE : CldrTree = CldrTree {
    data: &LOC_KDE,
    children: &[
        ("TZ", &TAG_KDE_TZ),
    ],
};

pub static LOC_KDE_TZ : CldrData = CldrData {
    parents: &[
        &LOC_KDE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KDE_TZ : CldrTree = CldrTree {
    data: &LOC_KDE_TZ,
    children: &[
    ],
};

pub static LOC_KEA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_KEA : CldrTree = CldrTree {
    data: &LOC_KEA,
    children: &[
        ("CV", &TAG_KEA_CV),
    ],
};

pub static LOC_KEA_CV : CldrData = CldrData {
    parents: &[
        &LOC_KEA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KEA_CV : CldrTree = CldrTree {
    data: &LOC_KEA_CV,
    children: &[
    ],
};

pub static LOC_KHQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_KHQ : CldrTree = CldrTree {
    data: &LOC_KHQ,
    children: &[
        ("ML", &TAG_KHQ_ML),
    ],
};

pub static LOC_KHQ_ML : CldrData = CldrData {
    parents: &[
        &LOC_KHQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KHQ_ML : CldrTree = CldrTree {
    data: &LOC_KHQ_ML,
    children: &[
    ],
};

pub static LOC_KI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KI : CldrTree = CldrTree {
    data: &LOC_KI,
    children: &[
        ("KE", &TAG_KI_KE),
    ],
};

pub static LOC_KI_KE : CldrData = CldrData {
    parents: &[
        &LOC_KI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KI_KE : CldrTree = CldrTree {
    data: &LOC_KI_KE,
    children: &[
    ],
};

pub static LOC_KK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 19),
    ],
    data: ", сан емес",
};

pub static TAG_KK : CldrTree = CldrTree {
    data: &LOC_KK,
    children: &[
        ("KZ", &TAG_KK_KZ),
    ],
};

pub static LOC_KK_KZ : CldrData = CldrData {
    parents: &[
        &LOC_KK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KK_KZ : CldrTree = CldrTree {
    data: &LOC_KK_KZ,
    children: &[
    ],
};

pub static LOC_KKJ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_KKJ : CldrTree = CldrTree {
    data: &LOC_KKJ,
    children: &[
        ("CM", &TAG_KKJ_CM),
    ],
};

pub static LOC_KKJ_CM : CldrData = CldrData {
    parents: &[
        &LOC_KKJ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KKJ_CM : CldrTree = CldrTree {
    data: &LOC_KKJ_CM,
    children: &[
    ],
};

pub static LOC_KL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::MinusSign, 5),
        (Item::EngineeringExponent, 10),
        (Item::CommonExponent, 14),
        (Item::NotANumberSymbol, 20),
    ],
    data: ",.−×10^·10¤¤¤",
};

pub static TAG_KL : CldrTree = CldrTree {
    data: &LOC_KL,
    children: &[
        ("GL", &TAG_KL_GL),
    ],
};

pub static LOC_KL_GL : CldrData = CldrData {
    parents: &[
        &LOC_KL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KL_GL : CldrTree = CldrTree {
    data: &LOC_KL_GL,
    children: &[
    ],
};

pub static LOC_KLN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KLN : CldrTree = CldrTree {
    data: &LOC_KLN,
    children: &[
        ("KE", &TAG_KLN_KE),
    ],
};

pub static LOC_KLN_KE : CldrData = CldrData {
    parents: &[
        &LOC_KLN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KLN_KE : CldrTree = CldrTree {
    data: &LOC_KLN_KE,
    children: &[
    ],
};

pub static LOC_KM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_KM : CldrTree = CldrTree {
    data: &LOC_KM,
    children: &[
        ("KH", &TAG_KM_KH),
    ],
};

pub static LOC_KM_KH : CldrData = CldrData {
    parents: &[
        &LOC_KM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KM_KH : CldrTree = CldrTree {
    data: &LOC_KM_KH,
    children: &[
    ],
};

pub static LOC_KN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KN : CldrTree = CldrTree {
    data: &LOC_KN,
    children: &[
        ("IN", &TAG_KN_IN),
    ],
};

pub static LOC_KN_IN : CldrData = CldrData {
    parents: &[
        &LOC_KN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KN_IN : CldrTree = CldrTree {
    data: &LOC_KN_IN,
    children: &[
    ],
};

pub static LOC_KO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KO : CldrTree = CldrTree {
    data: &LOC_KO,
    children: &[
        ("KP", &TAG_KO_KP),
        ("KR", &TAG_KO_KR),
    ],
};

pub static LOC_KO_KP : CldrData = CldrData {
    parents: &[
        &LOC_KO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KO_KP : CldrTree = CldrTree {
    data: &LOC_KO_KP,
    children: &[
    ],
};

pub static LOC_KO_KR : CldrData = CldrData {
    parents: &[
        &LOC_KO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KO_KR : CldrTree = CldrTree {
    data: &LOC_KO_KR,
    children: &[
    ],
};

pub static LOC_KOK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_KOK : CldrTree = CldrTree {
    data: &LOC_KOK,
    children: &[
        ("IN", &TAG_KOK_IN),
    ],
};

pub static LOC_KOK_IN : CldrData = CldrData {
    parents: &[
        &LOC_KOK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KOK_IN : CldrTree = CldrTree {
    data: &LOC_KOK_IN,
    children: &[
    ],
};

pub static LOC_KS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹",
};

pub static TAG_KS : CldrTree = CldrTree {
    data: &LOC_KS,
    children: &[
        ("IN", &TAG_KS_IN),
    ],
};

pub static LOC_KS_IN : CldrData = CldrData {
    parents: &[
        &LOC_KS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KS_IN : CldrTree = CldrTree {
    data: &LOC_KS_IN,
    children: &[
    ],
};

pub static LOC_KSB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KSB : CldrTree = CldrTree {
    data: &LOC_KSB,
    children: &[
        ("TZ", &TAG_KSB_TZ),
    ],
};

pub static LOC_KSB_TZ : CldrData = CldrData {
    parents: &[
        &LOC_KSB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KSB_TZ : CldrTree = CldrTree {
    data: &LOC_KSB_TZ,
    children: &[
    ],
};

pub static LOC_KSF : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_KSF : CldrTree = CldrTree {
    data: &LOC_KSF,
    children: &[
        ("CM", &TAG_KSF_CM),
    ],
};

pub static LOC_KSF_CM : CldrData = CldrData {
    parents: &[
        &LOC_KSF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KSF_CM : CldrTree = CldrTree {
    data: &LOC_KSF_CM,
    children: &[
    ],
};

pub static LOC_KSH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::EngineeringExponent, 11),
        (Item::NotANumberSymbol, 17),
    ],
    data: ", −×10^¤¤¤",
};

pub static TAG_KSH : CldrTree = CldrTree {
    data: &LOC_KSH,
    children: &[
        ("DE", &TAG_KSH_DE),
    ],
};

pub static LOC_KSH_DE : CldrData = CldrData {
    parents: &[
        &LOC_KSH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KSH_DE : CldrTree = CldrTree {
    data: &LOC_KSH_DE,
    children: &[
    ],
};

pub static LOC_KW : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KW : CldrTree = CldrTree {
    data: &LOC_KW,
    children: &[
        ("GB", &TAG_KW_GB),
    ],
};

pub static LOC_KW_GB : CldrData = CldrData {
    parents: &[
        &LOC_KW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KW_GB : CldrTree = CldrTree {
    data: &LOC_KW_GB,
    children: &[
    ],
};

pub static LOC_KY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 19),
    ],
    data: ", сан эмес",
};

pub static TAG_KY : CldrTree = CldrTree {
    data: &LOC_KY,
    children: &[
        ("KG", &TAG_KY_KG),
    ],
};

pub static LOC_KY_KG : CldrData = CldrData {
    parents: &[
        &LOC_KY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_KY_KG : CldrTree = CldrTree {
    data: &LOC_KY_KG,
    children: &[
    ],
};

pub static LOC_LAG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LAG : CldrTree = CldrTree {
    data: &LOC_LAG,
    children: &[
        ("TZ", &TAG_LAG_TZ),
    ],
};

pub static LOC_LAG_TZ : CldrData = CldrData {
    parents: &[
        &LOC_LAG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LAG_TZ : CldrTree = CldrTree {
    data: &LOC_LAG_TZ,
    children: &[
    ],
};

pub static LOC_LB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_LB : CldrTree = CldrTree {
    data: &LOC_LB,
    children: &[
        ("LU", &TAG_LB_LU),
    ],
};

pub static LOC_LB_LU : CldrData = CldrData {
    parents: &[
        &LOC_LB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LB_LU : CldrTree = CldrTree {
    data: &LOC_LB_LU,
    children: &[
    ],
};

pub static LOC_LG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LG : CldrTree = CldrTree {
    data: &LOC_LG,
    children: &[
        ("UG", &TAG_LG_UG),
    ],
};

pub static LOC_LG_UG : CldrData = CldrData {
    parents: &[
        &LOC_LG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LG_UG : CldrTree = CldrTree {
    data: &LOC_LG_UG,
    children: &[
    ],
};

pub static LOC_LKT : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LKT : CldrTree = CldrTree {
    data: &LOC_LKT,
    children: &[
        ("US", &TAG_LKT_US),
    ],
};

pub static LOC_LKT_US : CldrData = CldrData {
    parents: &[
        &LOC_LKT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LKT_US : CldrTree = CldrTree {
    data: &LOC_LKT_US,
    children: &[
    ],
};

pub static LOC_LN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_LN : CldrTree = CldrTree {
    data: &LOC_LN,
    children: &[
        ("AO", &TAG_LN_AO),
        ("CD", &TAG_LN_CD),
        ("CF", &TAG_LN_CF),
        ("CG", &TAG_LN_CG),
    ],
};

pub static LOC_LN_AO : CldrData = CldrData {
    parents: &[
        &LOC_LN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LN_AO : CldrTree = CldrTree {
    data: &LOC_LN_AO,
    children: &[
    ],
};

pub static LOC_LN_CD : CldrData = CldrData {
    parents: &[
        &LOC_LN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LN_CD : CldrTree = CldrTree {
    data: &LOC_LN_CD,
    children: &[
    ],
};

pub static LOC_LN_CF : CldrData = CldrData {
    parents: &[
        &LOC_LN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LN_CF : CldrTree = CldrTree {
    data: &LOC_LN_CF,
    children: &[
    ],
};

pub static LOC_LN_CG : CldrData = CldrData {
    parents: &[
        &LOC_LN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LN_CG : CldrTree = CldrTree {
    data: &LOC_LN_CG,
    children: &[
    ],
};

pub static LOC_LO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::NotANumberSymbol, 47),
    ],
    data: ",.ບໍ່​ແມ່ນ​ໂຕ​ເລກ",
};

pub static TAG_LO : CldrTree = CldrTree {
    data: &LOC_LO,
    children: &[
        ("LA", &TAG_LO_LA),
    ],
};

pub static LOC_LO_LA : CldrData = CldrData {
    parents: &[
        &LOC_LO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LO_LA : CldrTree = CldrTree {
    data: &LOC_LO_LA,
    children: &[
    ],
};

pub static LOC_LRC : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹",
};

pub static TAG_LRC : CldrTree = CldrTree {
    data: &LOC_LRC,
    children: &[
        ("IQ", &TAG_LRC_IQ),
        ("IR", &TAG_LRC_IR),
    ],
};

pub static LOC_LRC_IQ : CldrData = CldrData {
    parents: &[
        &LOC_LRC,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LRC_IQ : CldrTree = CldrTree {
    data: &LOC_LRC_IQ,
    children: &[
    ],
};

pub static LOC_LRC_IR : CldrData = CldrData {
    parents: &[
        &LOC_LRC,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LRC_IR : CldrTree = CldrTree {
    data: &LOC_LRC_IR,
    children: &[
    ],
};

pub static LOC_LT : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::EngineeringExponent, 11),
    ],
    data: ", −×10^",
};

pub static TAG_LT : CldrTree = CldrTree {
    data: &LOC_LT,
    children: &[
        ("LT", &TAG_LT_LT),
    ],
};

pub static LOC_LT_LT : CldrData = CldrData {
    parents: &[
        &LOC_LT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LT_LT : CldrTree = CldrTree {
    data: &LOC_LT_LT,
    children: &[
    ],
};

pub static LOC_LU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_LU : CldrTree = CldrTree {
    data: &LOC_LU,
    children: &[
        ("CD", &TAG_LU_CD),
    ],
};

pub static LOC_LU_CD : CldrData = CldrData {
    parents: &[
        &LOC_LU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LU_CD : CldrTree = CldrTree {
    data: &LOC_LU_CD,
    children: &[
    ],
};

pub static LOC_LUO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LUO : CldrTree = CldrTree {
    data: &LOC_LUO,
    children: &[
        ("KE", &TAG_LUO_KE),
    ],
};

pub static LOC_LUO_KE : CldrData = CldrData {
    parents: &[
        &LOC_LUO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LUO_KE : CldrTree = CldrTree {
    data: &LOC_LUO_KE,
    children: &[
    ],
};

pub static LOC_LUY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LUY : CldrTree = CldrTree {
    data: &LOC_LUY,
    children: &[
        ("KE", &TAG_LUY_KE),
    ],
};

pub static LOC_LUY_KE : CldrData = CldrData {
    parents: &[
        &LOC_LUY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LUY_KE : CldrTree = CldrTree {
    data: &LOC_LUY_KE,
    children: &[
    ],
};

pub static LOC_LV : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 5),
        (Item::MinGroupingDigits, 6),
    ],
    data: ", NS2",
};

pub static TAG_LV : CldrTree = CldrTree {
    data: &LOC_LV,
    children: &[
        ("LV", &TAG_LV_LV),
    ],
};

pub static LOC_LV_LV : CldrData = CldrData {
    parents: &[
        &LOC_LV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_LV_LV : CldrTree = CldrTree {
    data: &LOC_LV_LV,
    children: &[
    ],
};

pub static LOC_MAS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MAS : CldrTree = CldrTree {
    data: &LOC_MAS,
    children: &[
        ("KE", &TAG_MAS_KE),
        ("TZ", &TAG_MAS_TZ),
    ],
};

pub static LOC_MAS_KE : CldrData = CldrData {
    parents: &[
        &LOC_MAS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MAS_KE : CldrTree = CldrTree {
    data: &LOC_MAS_KE,
    children: &[
    ],
};

pub static LOC_MAS_TZ : CldrData = CldrData {
    parents: &[
        &LOC_MAS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MAS_TZ : CldrTree = CldrTree {
    data: &LOC_MAS_TZ,
    children: &[
    ],
};

pub static LOC_MER : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MER : CldrTree = CldrTree {
    data: &LOC_MER,
    children: &[
        ("KE", &TAG_MER_KE),
    ],
};

pub static LOC_MER_KE : CldrData = CldrData {
    parents: &[
        &LOC_MER,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MER_KE : CldrTree = CldrTree {
    data: &LOC_MER_KE,
    children: &[
    ],
};

pub static LOC_MFE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_MFE : CldrTree = CldrTree {
    data: &LOC_MFE,
    children: &[
        ("MU", &TAG_MFE_MU),
    ],
};

pub static LOC_MFE_MU : CldrData = CldrData {
    parents: &[
        &LOC_MFE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MFE_MU : CldrTree = CldrTree {
    data: &LOC_MFE_MU,
    children: &[
    ],
};

pub static LOC_MG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MG : CldrTree = CldrTree {
    data: &LOC_MG,
    children: &[
        ("MG", &TAG_MG_MG),
    ],
};

pub static LOC_MG_MG : CldrData = CldrData {
    parents: &[
        &LOC_MG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MG_MG : CldrTree = CldrTree {
    data: &LOC_MG_MG,
    children: &[
    ],
};

pub static LOC_MGH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_MGH : CldrTree = CldrTree {
    data: &LOC_MGH,
    children: &[
        ("MZ", &TAG_MGH_MZ),
    ],
};

pub static LOC_MGH_MZ : CldrData = CldrData {
    parents: &[
        &LOC_MGH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MGH_MZ : CldrTree = CldrTree {
    data: &LOC_MGH_MZ,
    children: &[
    ],
};

pub static LOC_MGO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MGO : CldrTree = CldrTree {
    data: &LOC_MGO,
    children: &[
        ("CM", &TAG_MGO_CM),
    ],
};

pub static LOC_MGO_CM : CldrData = CldrData {
    parents: &[
        &LOC_MGO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MGO_CM : CldrTree = CldrTree {
    data: &LOC_MGO_CM,
    children: &[
    ],
};

pub static LOC_MK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_MK : CldrTree = CldrTree {
    data: &LOC_MK,
    children: &[
        ("MK", &TAG_MK_MK),
    ],
};

pub static LOC_MK_MK : CldrData = CldrData {
    parents: &[
        &LOC_MK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MK_MK : CldrTree = CldrTree {
    data: &LOC_MK_MK,
    children: &[
    ],
};

pub static LOC_ML : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_ML : CldrTree = CldrTree {
    data: &LOC_ML,
    children: &[
        ("IN", &TAG_ML_IN),
    ],
};

pub static LOC_ML_IN : CldrData = CldrData {
    parents: &[
        &LOC_ML,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ML_IN : CldrTree = CldrTree {
    data: &LOC_ML_IN,
    children: &[
    ],
};

pub static LOC_MN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MN : CldrTree = CldrTree {
    data: &LOC_MN,
    children: &[
        ("MN", &TAG_MN_MN),
    ],
};

pub static LOC_MN_MN : CldrData = CldrData {
    parents: &[
        &LOC_MN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MN_MN : CldrTree = CldrTree {
    data: &LOC_MN_MN,
    children: &[
    ],
};

pub static LOC_MR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
        (Item::CommonExponent, 39),
        (Item::Grouping, 42),
    ],
    data: "०१२३४५६७८९×१०^2;3",
};

pub static TAG_MR : CldrTree = CldrTree {
    data: &LOC_MR,
    children: &[
        ("IN", &TAG_MR_IN),
    ],
};

pub static LOC_MR_IN : CldrData = CldrData {
    parents: &[
        &LOC_MR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MR_IN : CldrTree = CldrTree {
    data: &LOC_MR_IN,
    children: &[
    ],
};

pub static LOC_MS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MS : CldrTree = CldrTree {
    data: &LOC_MS,
    children: &[
        ("BN", &TAG_MS_BN),
        ("MY", &TAG_MS_MY),
        ("SG", &TAG_MS_SG),
    ],
};

pub static LOC_MS_BN : CldrData = CldrData {
    parents: &[
        &LOC_MS,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_MS_BN : CldrTree = CldrTree {
    data: &LOC_MS_BN,
    children: &[
    ],
};

pub static LOC_MS_MY : CldrData = CldrData {
    parents: &[
        &LOC_MS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MS_MY : CldrTree = CldrTree {
    data: &LOC_MS_MY,
    children: &[
    ],
};

pub static LOC_MS_SG : CldrData = CldrData {
    parents: &[
        &LOC_MS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MS_SG : CldrTree = CldrTree {
    data: &LOC_MS_SG,
    children: &[
    ],
};

pub static LOC_MT : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MT : CldrTree = CldrTree {
    data: &LOC_MT,
    children: &[
        ("MT", &TAG_MT_MT),
    ],
};

pub static LOC_MT_MT : CldrData = CldrData {
    parents: &[
        &LOC_MT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MT_MT : CldrTree = CldrTree {
    data: &LOC_MT_MT,
    children: &[
    ],
};

pub static LOC_MUA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_MUA : CldrTree = CldrTree {
    data: &LOC_MUA,
    children: &[
        ("CM", &TAG_MUA_CM),
    ],
};

pub static LOC_MUA_CM : CldrData = CldrData {
    parents: &[
        &LOC_MUA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MUA_CM : CldrTree = CldrTree {
    data: &LOC_MUA_CM,
    children: &[
    ],
};

pub static LOC_MY : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
        (Item::CommonExponent, 39),
        (Item::NotANumberSymbol, 78),
    ],
    data: "၀၁၂၃၄၅၆၇၈၉×၁၀^ဂဏန်းမဟုတ်သော",
};

pub static TAG_MY : CldrTree = CldrTree {
    data: &LOC_MY,
    children: &[
        ("MM", &TAG_MY_MM),
    ],
};

pub static LOC_MY_MM : CldrData = CldrData {
    parents: &[
        &LOC_MY,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MY_MM : CldrTree = CldrTree {
    data: &LOC_MY_MM,
    children: &[
    ],
};

pub static LOC_MZN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹",
};

pub static TAG_MZN : CldrTree = CldrTree {
    data: &LOC_MZN,
    children: &[
        ("IR", &TAG_MZN_IR),
    ],
};

pub static LOC_MZN_IR : CldrData = CldrData {
    parents: &[
        &LOC_MZN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_MZN_IR : CldrTree = CldrTree {
    data: &LOC_MZN_IR,
    children: &[
    ],
};

pub static LOC_NAQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NAQ : CldrTree = CldrTree {
    data: &LOC_NAQ,
    children: &[
        ("NA", &TAG_NAQ_NA),
    ],
};

pub static LOC_NAQ_NA : CldrData = CldrData {
    parents: &[
        &LOC_NAQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NAQ_NA : CldrTree = CldrTree {
    data: &LOC_NAQ_NA,
    children: &[
    ],
};

pub static LOC_NB : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
    ],
    data: ", −",
};

pub static TAG_NB : CldrTree = CldrTree {
    data: &LOC_NB,
    children: &[
        ("NO", &TAG_NB_NO),
        ("SJ", &TAG_NB_SJ),
    ],
};

pub static LOC_NB_NO : CldrData = CldrData {
    parents: &[
        &LOC_NB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NB_NO : CldrTree = CldrTree {
    data: &LOC_NB_NO,
    children: &[
    ],
};

pub static LOC_NB_SJ : CldrData = CldrData {
    parents: &[
        &LOC_NB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NB_SJ : CldrTree = CldrTree {
    data: &LOC_NB_SJ,
    children: &[
    ],
};

pub static LOC_ND : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ND : CldrTree = CldrTree {
    data: &LOC_ND,
    children: &[
        ("ZW", &TAG_ND_ZW),
    ],
};

pub static LOC_ND_ZW : CldrData = CldrData {
    parents: &[
        &LOC_ND,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ND_ZW : CldrTree = CldrTree {
    data: &LOC_ND_ZW,
    children: &[
    ],
};

pub static LOC_NDS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_NDS : CldrTree = CldrTree {
    data: &LOC_NDS,
    children: &[
        ("DE", &TAG_NDS_DE),
        ("NL", &TAG_NDS_NL),
    ],
};

pub static LOC_NDS_DE : CldrData = CldrData {
    parents: &[
        &LOC_NDS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NDS_DE : CldrTree = CldrTree {
    data: &LOC_NDS_DE,
    children: &[
    ],
};

pub static LOC_NDS_NL : CldrData = CldrData {
    parents: &[
        &LOC_NDS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NDS_NL : CldrTree = CldrTree {
    data: &LOC_NDS_NL,
    children: &[
    ],
};

pub static LOC_NE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 30),
        (Item::CommonExponent, 39),
    ],
    data: "०१२३४५६७८९×१०^",
};

pub static TAG_NE : CldrTree = CldrTree {
    data: &LOC_NE,
    children: &[
        ("IN", &TAG_NE_IN),
        ("NP", &TAG_NE_NP),
    ],
};

pub static LOC_NE_IN : CldrData = CldrData {
    parents: &[
        &LOC_NE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NE_IN : CldrTree = CldrTree {
    data: &LOC_NE_IN,
    children: &[
    ],
};

pub static LOC_NE_NP : CldrData = CldrData {
    parents: &[
        &LOC_NE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NE_NP : CldrTree = CldrTree {
    data: &LOC_NE_NP,
    children: &[
    ],
};

pub static LOC_NL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_NL : CldrTree = CldrTree {
    data: &LOC_NL,
    children: &[
        ("AW", &TAG_NL_AW),
        ("BE", &TAG_NL_BE),
        ("BQ", &TAG_NL_BQ),
        ("CW", &TAG_NL_CW),
        ("NL", &TAG_NL_NL),
        ("SR", &TAG_NL_SR),
        ("SX", &TAG_NL_SX),
    ],
};

pub static LOC_NL_AW : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_AW : CldrTree = CldrTree {
    data: &LOC_NL_AW,
    children: &[
    ],
};

pub static LOC_NL_BE : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_BE : CldrTree = CldrTree {
    data: &LOC_NL_BE,
    children: &[
    ],
};

pub static LOC_NL_BQ : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_BQ : CldrTree = CldrTree {
    data: &LOC_NL_BQ,
    children: &[
    ],
};

pub static LOC_NL_CW : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_CW : CldrTree = CldrTree {
    data: &LOC_NL_CW,
    children: &[
    ],
};

pub static LOC_NL_NL : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_NL : CldrTree = CldrTree {
    data: &LOC_NL_NL,
    children: &[
    ],
};

pub static LOC_NL_SR : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_SR : CldrTree = CldrTree {
    data: &LOC_NL_SR,
    children: &[
    ],
};

pub static LOC_NL_SX : CldrData = CldrData {
    parents: &[
        &LOC_NL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NL_SX : CldrTree = CldrTree {
    data: &LOC_NL_SX,
    children: &[
    ],
};

pub static LOC_NMG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_NMG : CldrTree = CldrTree {
    data: &LOC_NMG,
    children: &[
        ("CM", &TAG_NMG_CM),
    ],
};

pub static LOC_NMG_CM : CldrData = CldrData {
    parents: &[
        &LOC_NMG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NMG_CM : CldrTree = CldrTree {
    data: &LOC_NMG_CM,
    children: &[
    ],
};

pub static LOC_NN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
    ],
    data: ", −",
};

pub static TAG_NN : CldrTree = CldrTree {
    data: &LOC_NN,
    children: &[
        ("NO", &TAG_NN_NO),
    ],
};

pub static LOC_NN_NO : CldrData = CldrData {
    parents: &[
        &LOC_NN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NN_NO : CldrTree = CldrTree {
    data: &LOC_NN_NO,
    children: &[
    ],
};

pub static LOC_NNH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_NNH : CldrTree = CldrTree {
    data: &LOC_NNH,
    children: &[
        ("CM", &TAG_NNH_CM),
    ],
};

pub static LOC_NNH_CM : CldrData = CldrData {
    parents: &[
        &LOC_NNH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NNH_CM : CldrTree = CldrTree {
    data: &LOC_NNH_CM,
    children: &[
    ],
};

pub static LOC_NUS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NUS : CldrTree = CldrTree {
    data: &LOC_NUS,
    children: &[
        ("SS", &TAG_NUS_SS),
    ],
};

pub static LOC_NUS_SS : CldrData = CldrData {
    parents: &[
        &LOC_NUS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NUS_SS : CldrTree = CldrTree {
    data: &LOC_NUS_SS,
    children: &[
    ],
};

pub static LOC_NYN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NYN : CldrTree = CldrTree {
    data: &LOC_NYN,
    children: &[
        ("UG", &TAG_NYN_UG),
    ],
};

pub static LOC_NYN_UG : CldrData = CldrData {
    parents: &[
        &LOC_NYN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_NYN_UG : CldrTree = CldrTree {
    data: &LOC_NYN_UG,
    children: &[
    ],
};

pub static LOC_OM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OM : CldrTree = CldrTree {
    data: &LOC_OM,
    children: &[
        ("ET", &TAG_OM_ET),
        ("KE", &TAG_OM_KE),
    ],
};

pub static LOC_OM_ET : CldrData = CldrData {
    parents: &[
        &LOC_OM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OM_ET : CldrTree = CldrTree {
    data: &LOC_OM_ET,
    children: &[
    ],
};

pub static LOC_OM_KE : CldrData = CldrData {
    parents: &[
        &LOC_OM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OM_KE : CldrTree = CldrTree {
    data: &LOC_OM_KE,
    children: &[
    ],
};

pub static LOC_OR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_OR : CldrTree = CldrTree {
    data: &LOC_OR,
    children: &[
        ("IN", &TAG_OR_IN),
    ],
};

pub static LOC_OR_IN : CldrData = CldrData {
    parents: &[
        &LOC_OR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OR_IN : CldrTree = CldrTree {
    data: &LOC_OR_IN,
    children: &[
    ],
};

pub static LOC_OS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 7),
    ],
    data: ", НН",
};

pub static TAG_OS : CldrTree = CldrTree {
    data: &LOC_OS,
    children: &[
        ("GE", &TAG_OS_GE),
        ("RU", &TAG_OS_RU),
    ],
};

pub static LOC_OS_GE : CldrData = CldrData {
    parents: &[
        &LOC_OS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OS_GE : CldrTree = CldrTree {
    data: &LOC_OS_GE,
    children: &[
    ],
};

pub static LOC_OS_RU : CldrData = CldrData {
    parents: &[
        &LOC_OS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_OS_RU : CldrTree = CldrTree {
    data: &LOC_OS_RU,
    children: &[
    ],
};

pub static LOC_PA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_PA : CldrTree = CldrTree {
    data: &LOC_PA,
    children: &[
        ("Arab", &TAG_PA_ARAB),
        ("Guru", &TAG_PA_GURU),
    ],
};

pub static LOC_PA_ARAB : CldrData = CldrData {
    parents: &[
        &LOC_PA,
    ],
    index: &[
        (Item::DecimalDigits, 20),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹",
};

pub static TAG_PA_ARAB : CldrTree = CldrTree {
    data: &LOC_PA_ARAB,
    children: &[
        ("PK", &TAG_PA_ARAB_PK),
    ],
};

pub static LOC_PA_ARAB_PK : CldrData = CldrData {
    parents: &[
        &LOC_PA_ARAB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PA_ARAB_PK : CldrTree = CldrTree {
    data: &LOC_PA_ARAB_PK,
    children: &[
    ],
};

pub static LOC_PA_GURU : CldrData = CldrData {
    parents: &[
        &LOC_PA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PA_GURU : CldrTree = CldrTree {
    data: &LOC_PA_GURU,
    children: &[
        ("IN", &TAG_PA_GURU_IN),
    ],
};

pub static LOC_PA_GURU_IN : CldrData = CldrData {
    parents: &[
        &LOC_PA_GURU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PA_GURU_IN : CldrTree = CldrTree {
    data: &LOC_PA_GURU_IN,
    children: &[
    ],
};

pub static LOC_PL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinGroupingDigits, 4),
    ],
    data: ", 2",
};

pub static TAG_PL : CldrTree = CldrTree {
    data: &LOC_PL,
    children: &[
        ("PL", &TAG_PL_PL),
    ],
};

pub static LOC_PL_PL : CldrData = CldrData {
    parents: &[
        &LOC_PL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PL_PL : CldrTree = CldrTree {
    data: &LOC_PL_PL,
    children: &[
    ],
};

pub static LOC_PRG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_PRG : CldrTree = CldrTree {
    data: &LOC_PRG,
    children: &[
        ("001", &TAG_PRG_001),
    ],
};

pub static LOC_PRG_001 : CldrData = CldrData {
    parents: &[
        &LOC_PRG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PRG_001 : CldrTree = CldrTree {
    data: &LOC_PRG_001,
    children: &[
    ],
};

pub static LOC_PS : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalDigits, 20),
        (Item::DecimalSeparator, 22),
        (Item::GroupSeparator, 24),
        (Item::PlusSign, 31),
        (Item::MinusSign, 38),
        (Item::PercentSign, 40),
        (Item::PerMilleSign, 42),
        (Item::EngineeringExponent, 49),
        (Item::CommonExponent, 56),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹٫٬‎+‎‎-‎٪؉×۱۰^×۱۰^",
};

pub static TAG_PS : CldrTree = CldrTree {
    data: &LOC_PS,
    children: &[
        ("AF", &TAG_PS_AF),
    ],
};

pub static LOC_PS_AF : CldrData = CldrData {
    parents: &[
        &LOC_PS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PS_AF : CldrTree = CldrTree {
    data: &LOC_PS_AF,
    children: &[
    ],
};

pub static LOC_PT : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_PT : CldrTree = CldrTree {
    data: &LOC_PT,
    children: &[
        ("AO", &TAG_PT_AO),
        ("BR", &TAG_PT_BR),
        ("CH", &TAG_PT_CH),
        ("CV", &TAG_PT_CV),
        ("GQ", &TAG_PT_GQ),
        ("GW", &TAG_PT_GW),
        ("LU", &TAG_PT_LU),
        ("MO", &TAG_PT_MO),
        ("MZ", &TAG_PT_MZ),
        ("PT", &TAG_PT_PT),
        ("ST", &TAG_PT_ST),
        ("TL", &TAG_PT_TL),
    ],
};

pub static LOC_PT_AO : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_AO : CldrTree = CldrTree {
    data: &LOC_PT_AO,
    children: &[
    ],
};

pub static LOC_PT_BR : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_BR : CldrTree = CldrTree {
    data: &LOC_PT_BR,
    children: &[
    ],
};

pub static LOC_PT_CH : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_CH : CldrTree = CldrTree {
    data: &LOC_PT_CH,
    children: &[
    ],
};

pub static LOC_PT_CV : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_CV : CldrTree = CldrTree {
    data: &LOC_PT_CV,
    children: &[
    ],
};

pub static LOC_PT_GQ : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_GQ : CldrTree = CldrTree {
    data: &LOC_PT_GQ,
    children: &[
    ],
};

pub static LOC_PT_GW : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_GW : CldrTree = CldrTree {
    data: &LOC_PT_GW,
    children: &[
    ],
};

pub static LOC_PT_LU : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_LU : CldrTree = CldrTree {
    data: &LOC_PT_LU,
    children: &[
    ],
};

pub static LOC_PT_MO : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_MO : CldrTree = CldrTree {
    data: &LOC_PT_MO,
    children: &[
    ],
};

pub static LOC_PT_MZ : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_MZ : CldrTree = CldrTree {
    data: &LOC_PT_MZ,
    children: &[
    ],
};

pub static LOC_PT_PT : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_PT_PT : CldrTree = CldrTree {
    data: &LOC_PT_PT,
    children: &[
    ],
};

pub static LOC_PT_ST : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_ST : CldrTree = CldrTree {
    data: &LOC_PT_ST,
    children: &[
    ],
};

pub static LOC_PT_TL : CldrData = CldrData {
    parents: &[
        &LOC_PT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_PT_TL : CldrTree = CldrTree {
    data: &LOC_PT_TL,
    children: &[
    ],
};

pub static LOC_QU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_QU : CldrTree = CldrTree {
    data: &LOC_QU,
    children: &[
        ("BO", &TAG_QU_BO),
        ("EC", &TAG_QU_EC),
        ("PE", &TAG_QU_PE),
    ],
};

pub static LOC_QU_BO : CldrData = CldrData {
    parents: &[
        &LOC_QU,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_QU_BO : CldrTree = CldrTree {
    data: &LOC_QU_BO,
    children: &[
    ],
};

pub static LOC_QU_EC : CldrData = CldrData {
    parents: &[
        &LOC_QU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_QU_EC : CldrTree = CldrTree {
    data: &LOC_QU_EC,
    children: &[
    ],
};

pub static LOC_QU_PE : CldrData = CldrData {
    parents: &[
        &LOC_QU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_QU_PE : CldrTree = CldrTree {
    data: &LOC_QU_PE,
    children: &[
    ],
};

pub static LOC_RM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
    ],
    data: "’−",
};

pub static TAG_RM : CldrTree = CldrTree {
    data: &LOC_RM,
    children: &[
        ("CH", &TAG_RM_CH),
    ],
};

pub static LOC_RM_CH : CldrData = CldrData {
    parents: &[
        &LOC_RM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RM_CH : CldrTree = CldrTree {
    data: &LOC_RM_CH,
    children: &[
    ],
};

pub static LOC_RN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_RN : CldrTree = CldrTree {
    data: &LOC_RN,
    children: &[
        ("BI", &TAG_RN_BI),
    ],
};

pub static LOC_RN_BI : CldrData = CldrData {
    parents: &[
        &LOC_RN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RN_BI : CldrTree = CldrTree {
    data: &LOC_RN_BI,
    children: &[
    ],
};

pub static LOC_RO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_RO : CldrTree = CldrTree {
    data: &LOC_RO,
    children: &[
        ("MD", &TAG_RO_MD),
        ("RO", &TAG_RO_RO),
    ],
};

pub static LOC_RO_MD : CldrData = CldrData {
    parents: &[
        &LOC_RO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RO_MD : CldrTree = CldrTree {
    data: &LOC_RO_MD,
    children: &[
    ],
};

pub static LOC_RO_RO : CldrData = CldrData {
    parents: &[
        &LOC_RO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RO_RO : CldrTree = CldrTree {
    data: &LOC_RO_RO,
    children: &[
    ],
};

pub static LOC_ROF : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ROF : CldrTree = CldrTree {
    data: &LOC_ROF,
    children: &[
        ("TZ", &TAG_ROF_TZ),
    ],
};

pub static LOC_ROF_TZ : CldrData = CldrData {
    parents: &[
        &LOC_ROF,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ROF_TZ : CldrTree = CldrTree {
    data: &LOC_ROF_TZ,
    children: &[
    ],
};

pub static LOC_RU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 19),
    ],
    data: ", не число",
};

pub static TAG_RU : CldrTree = CldrTree {
    data: &LOC_RU,
    children: &[
        ("BY", &TAG_RU_BY),
        ("KG", &TAG_RU_KG),
        ("KZ", &TAG_RU_KZ),
        ("MD", &TAG_RU_MD),
        ("RU", &TAG_RU_RU),
        ("UA", &TAG_RU_UA),
    ],
};

pub static LOC_RU_BY : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_BY : CldrTree = CldrTree {
    data: &LOC_RU_BY,
    children: &[
    ],
};

pub static LOC_RU_KG : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_KG : CldrTree = CldrTree {
    data: &LOC_RU_KG,
    children: &[
    ],
};

pub static LOC_RU_KZ : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_KZ : CldrTree = CldrTree {
    data: &LOC_RU_KZ,
    children: &[
    ],
};

pub static LOC_RU_MD : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_MD : CldrTree = CldrTree {
    data: &LOC_RU_MD,
    children: &[
    ],
};

pub static LOC_RU_RU : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_RU : CldrTree = CldrTree {
    data: &LOC_RU_RU,
    children: &[
    ],
};

pub static LOC_RU_UA : CldrData = CldrData {
    parents: &[
        &LOC_RU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RU_UA : CldrTree = CldrTree {
    data: &LOC_RU_UA,
    children: &[
    ],
};

pub static LOC_RW : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_RW : CldrTree = CldrTree {
    data: &LOC_RW,
    children: &[
        ("RW", &TAG_RW_RW),
    ],
};

pub static LOC_RW_RW : CldrData = CldrData {
    parents: &[
        &LOC_RW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RW_RW : CldrTree = CldrTree {
    data: &LOC_RW_RW,
    children: &[
    ],
};

pub static LOC_RWK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RWK : CldrTree = CldrTree {
    data: &LOC_RWK,
    children: &[
        ("TZ", &TAG_RWK_TZ),
    ],
};

pub static LOC_RWK_TZ : CldrData = CldrData {
    parents: &[
        &LOC_RWK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_RWK_TZ : CldrTree = CldrTree {
    data: &LOC_RWK_TZ,
    children: &[
    ],
};

pub static LOC_SAH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 31),
    ],
    data: ", чыыһыла буотах",
};

pub static TAG_SAH : CldrTree = CldrTree {
    data: &LOC_SAH,
    children: &[
        ("RU", &TAG_SAH_RU),
    ],
};

pub static LOC_SAH_RU : CldrData = CldrData {
    parents: &[
        &LOC_SAH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SAH_RU : CldrTree = CldrTree {
    data: &LOC_SAH_RU,
    children: &[
    ],
};

pub static LOC_SAQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SAQ : CldrTree = CldrTree {
    data: &LOC_SAQ,
    children: &[
        ("KE", &TAG_SAQ_KE),
    ],
};

pub static LOC_SAQ_KE : CldrData = CldrData {
    parents: &[
        &LOC_SAQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SAQ_KE : CldrTree = CldrTree {
    data: &LOC_SAQ_KE,
    children: &[
    ],
};

pub static LOC_SBP : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SBP : CldrTree = CldrTree {
    data: &LOC_SBP,
    children: &[
        ("TZ", &TAG_SBP_TZ),
    ],
};

pub static LOC_SBP_TZ : CldrData = CldrData {
    parents: &[
        &LOC_SBP,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SBP_TZ : CldrTree = CldrTree {
    data: &LOC_SBP_TZ,
    children: &[
    ],
};

pub static LOC_SE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::EngineeringExponent, 11),
        (Item::CommonExponent, 15),
        (Item::NotANumberSymbol, 21),
    ],
    data: ", −·10^·10¤¤¤",
};

pub static TAG_SE : CldrTree = CldrTree {
    data: &LOC_SE,
    children: &[
        ("FI", &TAG_SE_FI),
        ("NO", &TAG_SE_NO),
        ("SE", &TAG_SE_SE),
    ],
};

pub static LOC_SE_FI : CldrData = CldrData {
    parents: &[
        &LOC_SE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SE_FI : CldrTree = CldrTree {
    data: &LOC_SE_FI,
    children: &[
    ],
};

pub static LOC_SE_NO : CldrData = CldrData {
    parents: &[
        &LOC_SE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SE_NO : CldrTree = CldrTree {
    data: &LOC_SE_NO,
    children: &[
    ],
};

pub static LOC_SE_SE : CldrData = CldrData {
    parents: &[
        &LOC_SE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SE_SE : CldrTree = CldrTree {
    data: &LOC_SE_SE,
    children: &[
    ],
};

pub static LOC_SEH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_SEH : CldrTree = CldrTree {
    data: &LOC_SEH,
    children: &[
        ("MZ", &TAG_SEH_MZ),
    ],
};

pub static LOC_SEH_MZ : CldrData = CldrData {
    parents: &[
        &LOC_SEH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SEH_MZ : CldrTree = CldrTree {
    data: &LOC_SEH_MZ,
    children: &[
    ],
};

pub static LOC_SES : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_SES : CldrTree = CldrTree {
    data: &LOC_SES,
    children: &[
        ("ML", &TAG_SES_ML),
    ],
};

pub static LOC_SES_ML : CldrData = CldrData {
    parents: &[
        &LOC_SES,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SES_ML : CldrTree = CldrTree {
    data: &LOC_SES_ML,
    children: &[
    ],
};

pub static LOC_SG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_SG : CldrTree = CldrTree {
    data: &LOC_SG,
    children: &[
        ("CF", &TAG_SG_CF),
    ],
};

pub static LOC_SG_CF : CldrData = CldrData {
    parents: &[
        &LOC_SG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SG_CF : CldrTree = CldrTree {
    data: &LOC_SG_CF,
    children: &[
    ],
};

pub static LOC_SHI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_SHI : CldrTree = CldrTree {
    data: &LOC_SHI,
    children: &[
        ("Latn", &TAG_SHI_LATN),
        ("Tfng", &TAG_SHI_TFNG),
    ],
};

pub static LOC_SHI_LATN : CldrData = CldrData {
    parents: &[
        &LOC_SHI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SHI_LATN : CldrTree = CldrTree {
    data: &LOC_SHI_LATN,
    children: &[
        ("MA", &TAG_SHI_LATN_MA),
    ],
};

pub static LOC_SHI_LATN_MA : CldrData = CldrData {
    parents: &[
        &LOC_SHI_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SHI_LATN_MA : CldrTree = CldrTree {
    data: &LOC_SHI_LATN_MA,
    children: &[
    ],
};

pub static LOC_SHI_TFNG : CldrData = CldrData {
    parents: &[
        &LOC_SHI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SHI_TFNG : CldrTree = CldrTree {
    data: &LOC_SHI_TFNG,
    children: &[
        ("MA", &TAG_SHI_TFNG_MA),
    ],
};

pub static LOC_SHI_TFNG_MA : CldrData = CldrData {
    parents: &[
        &LOC_SHI_TFNG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SHI_TFNG_MA : CldrTree = CldrTree {
    data: &LOC_SHI_TFNG_MA,
    children: &[
    ],
};

pub static LOC_SI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SI : CldrTree = CldrTree {
    data: &LOC_SI,
    children: &[
        ("LK", &TAG_SI_LK),
    ],
};

pub static LOC_SI_LK : CldrData = CldrData {
    parents: &[
        &LOC_SI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SI_LK : CldrTree = CldrTree {
    data: &LOC_SI_LK,
    children: &[
    ],
};

pub static LOC_SK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::EngineeringExponent, 4),
    ],
    data: ", e",
};

pub static TAG_SK : CldrTree = CldrTree {
    data: &LOC_SK,
    children: &[
        ("SK", &TAG_SK_SK),
    ],
};

pub static LOC_SK_SK : CldrData = CldrData {
    parents: &[
        &LOC_SK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SK_SK : CldrTree = CldrTree {
    data: &LOC_SK_SK,
    children: &[
    ],
};

pub static LOC_SL : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
        (Item::MinusSign, 5),
        (Item::EngineeringExponent, 6),
    ],
    data: ",.−e",
};

pub static TAG_SL : CldrTree = CldrTree {
    data: &LOC_SL,
    children: &[
        ("SI", &TAG_SL_SI),
    ],
};

pub static LOC_SL_SI : CldrData = CldrData {
    parents: &[
        &LOC_SL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SL_SI : CldrTree = CldrTree {
    data: &LOC_SL_SI,
    children: &[
    ],
};

pub static LOC_SMN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 10),
    ],
    data: ", epiloho",
};

pub static TAG_SMN : CldrTree = CldrTree {
    data: &LOC_SMN,
    children: &[
        ("FI", &TAG_SMN_FI),
    ],
};

pub static LOC_SMN_FI : CldrData = CldrData {
    parents: &[
        &LOC_SMN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SMN_FI : CldrTree = CldrTree {
    data: &LOC_SMN_FI,
    children: &[
    ],
};

pub static LOC_SN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SN : CldrTree = CldrTree {
    data: &LOC_SN,
    children: &[
        ("ZW", &TAG_SN_ZW),
    ],
};

pub static LOC_SN_ZW : CldrData = CldrData {
    parents: &[
        &LOC_SN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SN_ZW : CldrTree = CldrTree {
    data: &LOC_SN_ZW,
    children: &[
    ],
};

pub static LOC_SO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SO : CldrTree = CldrTree {
    data: &LOC_SO,
    children: &[
        ("DJ", &TAG_SO_DJ),
        ("ET", &TAG_SO_ET),
        ("KE", &TAG_SO_KE),
        ("SO", &TAG_SO_SO),
    ],
};

pub static LOC_SO_DJ : CldrData = CldrData {
    parents: &[
        &LOC_SO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SO_DJ : CldrTree = CldrTree {
    data: &LOC_SO_DJ,
    children: &[
    ],
};

pub static LOC_SO_ET : CldrData = CldrData {
    parents: &[
        &LOC_SO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SO_ET : CldrTree = CldrTree {
    data: &LOC_SO_ET,
    children: &[
    ],
};

pub static LOC_SO_KE : CldrData = CldrData {
    parents: &[
        &LOC_SO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SO_KE : CldrTree = CldrTree {
    data: &LOC_SO_KE,
    children: &[
    ],
};

pub static LOC_SO_SO : CldrData = CldrData {
    parents: &[
        &LOC_SO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SO_SO : CldrTree = CldrTree {
    data: &LOC_SO_SO,
    children: &[
    ],
};

pub static LOC_SQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_SQ : CldrTree = CldrTree {
    data: &LOC_SQ,
    children: &[
        ("AL", &TAG_SQ_AL),
        ("MK", &TAG_SQ_MK),
        ("XK", &TAG_SQ_XK),
    ],
};

pub static LOC_SQ_AL : CldrData = CldrData {
    parents: &[
        &LOC_SQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SQ_AL : CldrTree = CldrTree {
    data: &LOC_SQ_AL,
    children: &[
    ],
};

pub static LOC_SQ_MK : CldrData = CldrData {
    parents: &[
        &LOC_SQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SQ_MK : CldrTree = CldrTree {
    data: &LOC_SQ_MK,
    children: &[
    ],
};

pub static LOC_SQ_XK : CldrData = CldrData {
    parents: &[
        &LOC_SQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SQ_XK : CldrTree = CldrTree {
    data: &LOC_SQ_XK,
    children: &[
    ],
};

pub static LOC_SR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_SR : CldrTree = CldrTree {
    data: &LOC_SR,
    children: &[
        ("Cyrl", &TAG_SR_CYRL),
        ("Latn", &TAG_SR_LATN),
    ],
};

pub static LOC_SR_CYRL : CldrData = CldrData {
    parents: &[
        &LOC_SR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_CYRL : CldrTree = CldrTree {
    data: &LOC_SR_CYRL,
    children: &[
        ("BA", &TAG_SR_CYRL_BA),
        ("ME", &TAG_SR_CYRL_ME),
        ("RS", &TAG_SR_CYRL_RS),
        ("XK", &TAG_SR_CYRL_XK),
    ],
};

pub static LOC_SR_CYRL_BA : CldrData = CldrData {
    parents: &[
        &LOC_SR_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_CYRL_BA : CldrTree = CldrTree {
    data: &LOC_SR_CYRL_BA,
    children: &[
    ],
};

pub static LOC_SR_CYRL_ME : CldrData = CldrData {
    parents: &[
        &LOC_SR_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_CYRL_ME : CldrTree = CldrTree {
    data: &LOC_SR_CYRL_ME,
    children: &[
    ],
};

pub static LOC_SR_CYRL_RS : CldrData = CldrData {
    parents: &[
        &LOC_SR_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_CYRL_RS : CldrTree = CldrTree {
    data: &LOC_SR_CYRL_RS,
    children: &[
    ],
};

pub static LOC_SR_CYRL_XK : CldrData = CldrData {
    parents: &[
        &LOC_SR_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_CYRL_XK : CldrTree = CldrTree {
    data: &LOC_SR_CYRL_XK,
    children: &[
    ],
};

pub static LOC_SR_LATN : CldrData = CldrData {
    parents: &[
        &LOC_SR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_LATN : CldrTree = CldrTree {
    data: &LOC_SR_LATN,
    children: &[
        ("BA", &TAG_SR_LATN_BA),
        ("ME", &TAG_SR_LATN_ME),
        ("RS", &TAG_SR_LATN_RS),
        ("XK", &TAG_SR_LATN_XK),
    ],
};

pub static LOC_SR_LATN_BA : CldrData = CldrData {
    parents: &[
        &LOC_SR_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_LATN_BA : CldrTree = CldrTree {
    data: &LOC_SR_LATN_BA,
    children: &[
    ],
};

pub static LOC_SR_LATN_ME : CldrData = CldrData {
    parents: &[
        &LOC_SR_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_LATN_ME : CldrTree = CldrTree {
    data: &LOC_SR_LATN_ME,
    children: &[
    ],
};

pub static LOC_SR_LATN_RS : CldrData = CldrData {
    parents: &[
        &LOC_SR_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_LATN_RS : CldrTree = CldrTree {
    data: &LOC_SR_LATN_RS,
    children: &[
    ],
};

pub static LOC_SR_LATN_XK : CldrData = CldrData {
    parents: &[
        &LOC_SR_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SR_LATN_XK : CldrTree = CldrTree {
    data: &LOC_SR_LATN_XK,
    children: &[
    ],
};

pub static LOC_SV : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::MinusSign, 6),
        (Item::EngineeringExponent, 11),
        (Item::NotANumberSymbol, 17),
    ],
    data: ", −×10^¤¤¤",
};

pub static TAG_SV : CldrTree = CldrTree {
    data: &LOC_SV,
    children: &[
        ("AX", &TAG_SV_AX),
        ("FI", &TAG_SV_FI),
        ("SE", &TAG_SV_SE),
    ],
};

pub static LOC_SV_AX : CldrData = CldrData {
    parents: &[
        &LOC_SV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SV_AX : CldrTree = CldrTree {
    data: &LOC_SV_AX,
    children: &[
    ],
};

pub static LOC_SV_FI : CldrData = CldrData {
    parents: &[
        &LOC_SV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SV_FI : CldrTree = CldrTree {
    data: &LOC_SV_FI,
    children: &[
    ],
};

pub static LOC_SV_SE : CldrData = CldrData {
    parents: &[
        &LOC_SV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SV_SE : CldrTree = CldrTree {
    data: &LOC_SV_SE,
    children: &[
    ],
};

pub static LOC_SW : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SW : CldrTree = CldrTree {
    data: &LOC_SW,
    children: &[
        ("CD", &TAG_SW_CD),
        ("KE", &TAG_SW_KE),
        ("TZ", &TAG_SW_TZ),
        ("UG", &TAG_SW_UG),
    ],
};

pub static LOC_SW_CD : CldrData = CldrData {
    parents: &[
        &LOC_SW,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_SW_CD : CldrTree = CldrTree {
    data: &LOC_SW_CD,
    children: &[
    ],
};

pub static LOC_SW_KE : CldrData = CldrData {
    parents: &[
        &LOC_SW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SW_KE : CldrTree = CldrTree {
    data: &LOC_SW_KE,
    children: &[
    ],
};

pub static LOC_SW_TZ : CldrData = CldrData {
    parents: &[
        &LOC_SW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SW_TZ : CldrTree = CldrTree {
    data: &LOC_SW_TZ,
    children: &[
    ],
};

pub static LOC_SW_UG : CldrData = CldrData {
    parents: &[
        &LOC_SW,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_SW_UG : CldrTree = CldrTree {
    data: &LOC_SW_UG,
    children: &[
    ],
};

pub static LOC_TA : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_TA : CldrTree = CldrTree {
    data: &LOC_TA,
    children: &[
        ("IN", &TAG_TA_IN),
        ("LK", &TAG_TA_LK),
        ("MY", &TAG_TA_MY),
        ("SG", &TAG_TA_SG),
    ],
};

pub static LOC_TA_IN : CldrData = CldrData {
    parents: &[
        &LOC_TA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TA_IN : CldrTree = CldrTree {
    data: &LOC_TA_IN,
    children: &[
    ],
};

pub static LOC_TA_LK : CldrData = CldrData {
    parents: &[
        &LOC_TA,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TA_LK : CldrTree = CldrTree {
    data: &LOC_TA_LK,
    children: &[
    ],
};

pub static LOC_TA_MY : CldrData = CldrData {
    parents: &[
        &LOC_TA,
    ],
    index: &[
        (Item::Grouping, 1),
    ],
    data: "3",
};

pub static TAG_TA_MY : CldrTree = CldrTree {
    data: &LOC_TA_MY,
    children: &[
    ],
};

pub static LOC_TA_SG : CldrData = CldrData {
    parents: &[
        &LOC_TA,
    ],
    index: &[
        (Item::Grouping, 1),
    ],
    data: "3",
};

pub static TAG_TA_SG : CldrTree = CldrTree {
    data: &LOC_TA_SG,
    children: &[
    ],
};

pub static LOC_TE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::Grouping, 3),
    ],
    data: "2;3",
};

pub static TAG_TE : CldrTree = CldrTree {
    data: &LOC_TE,
    children: &[
        ("IN", &TAG_TE_IN),
    ],
};

pub static LOC_TE_IN : CldrData = CldrData {
    parents: &[
        &LOC_TE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TE_IN : CldrTree = CldrTree {
    data: &LOC_TE_IN,
    children: &[
    ],
};

pub static LOC_TEO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TEO : CldrTree = CldrTree {
    data: &LOC_TEO,
    children: &[
        ("KE", &TAG_TEO_KE),
        ("UG", &TAG_TEO_UG),
    ],
};

pub static LOC_TEO_KE : CldrData = CldrData {
    parents: &[
        &LOC_TEO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TEO_KE : CldrTree = CldrTree {
    data: &LOC_TEO_KE,
    children: &[
    ],
};

pub static LOC_TEO_UG : CldrData = CldrData {
    parents: &[
        &LOC_TEO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TEO_UG : CldrTree = CldrTree {
    data: &LOC_TEO_UG,
    children: &[
    ],
};

pub static LOC_TH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TH : CldrTree = CldrTree {
    data: &LOC_TH,
    children: &[
        ("TH", &TAG_TH_TH),
    ],
};

pub static LOC_TH_TH : CldrData = CldrData {
    parents: &[
        &LOC_TH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TH_TH : CldrTree = CldrTree {
    data: &LOC_TH_TH,
    children: &[
    ],
};

pub static LOC_TI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TI : CldrTree = CldrTree {
    data: &LOC_TI,
    children: &[
        ("ER", &TAG_TI_ER),
        ("ET", &TAG_TI_ET),
    ],
};

pub static LOC_TI_ER : CldrData = CldrData {
    parents: &[
        &LOC_TI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TI_ER : CldrTree = CldrTree {
    data: &LOC_TI_ER,
    children: &[
    ],
};

pub static LOC_TI_ET : CldrData = CldrData {
    parents: &[
        &LOC_TI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TI_ET : CldrTree = CldrTree {
    data: &LOC_TI_ET,
    children: &[
    ],
};

pub static LOC_TK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 12),
    ],
    data: ", san däl",
};

pub static TAG_TK : CldrTree = CldrTree {
    data: &LOC_TK,
    children: &[
        ("TM", &TAG_TK_TM),
    ],
};

pub static LOC_TK_TM : CldrData = CldrData {
    parents: &[
        &LOC_TK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TK_TM : CldrTree = CldrTree {
    data: &LOC_TK_TM,
    children: &[
    ],
};

pub static LOC_TO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::NotANumberSymbol, 2),
    ],
    data: "TF",
};

pub static TAG_TO : CldrTree = CldrTree {
    data: &LOC_TO,
    children: &[
        ("TO", &TAG_TO_TO),
    ],
};

pub static LOC_TO_TO : CldrData = CldrData {
    parents: &[
        &LOC_TO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TO_TO : CldrTree = CldrTree {
    data: &LOC_TO_TO,
    children: &[
    ],
};

pub static LOC_TR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_TR : CldrTree = CldrTree {
    data: &LOC_TR,
    children: &[
        ("CY", &TAG_TR_CY),
        ("TR", &TAG_TR_TR),
    ],
};

pub static LOC_TR_CY : CldrData = CldrData {
    parents: &[
        &LOC_TR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TR_CY : CldrTree = CldrTree {
    data: &LOC_TR_CY,
    children: &[
    ],
};

pub static LOC_TR_TR : CldrData = CldrData {
    parents: &[
        &LOC_TR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TR_TR : CldrTree = CldrTree {
    data: &LOC_TR_TR,
    children: &[
    ],
};

pub static LOC_TWQ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::GroupSeparator, 2),
    ],
    data: " ",
};

pub static TAG_TWQ : CldrTree = CldrTree {
    data: &LOC_TWQ,
    children: &[
        ("NE", &TAG_TWQ_NE),
    ],
};

pub static LOC_TWQ_NE : CldrData = CldrData {
    parents: &[
        &LOC_TWQ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TWQ_NE : CldrTree = CldrTree {
    data: &LOC_TWQ_NE,
    children: &[
    ],
};

pub static LOC_TZM : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_TZM : CldrTree = CldrTree {
    data: &LOC_TZM,
    children: &[
        ("MA", &TAG_TZM_MA),
    ],
};

pub static LOC_TZM_MA : CldrData = CldrData {
    parents: &[
        &LOC_TZM,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_TZM_MA : CldrTree = CldrTree {
    data: &LOC_TZM_MA,
    children: &[
    ],
};

pub static LOC_UG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UG : CldrTree = CldrTree {
    data: &LOC_UG,
    children: &[
        ("CN", &TAG_UG_CN),
    ],
};

pub static LOC_UG_CN : CldrData = CldrData {
    parents: &[
        &LOC_UG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UG_CN : CldrTree = CldrTree {
    data: &LOC_UG_CN,
    children: &[
    ],
};

pub static LOC_UK : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::EngineeringExponent, 5),
    ],
    data: ", Е",
};

pub static TAG_UK : CldrTree = CldrTree {
    data: &LOC_UK,
    children: &[
        ("UA", &TAG_UK_UA),
    ],
};

pub static LOC_UK_UA : CldrData = CldrData {
    parents: &[
        &LOC_UK,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UK_UA : CldrTree = CldrTree {
    data: &LOC_UK_UA,
    children: &[
    ],
};

pub static LOC_UR : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::PlusSign, 4),
        (Item::MinusSign, 8),
    ],
    data: "‎+‎-",
};

pub static TAG_UR : CldrTree = CldrTree {
    data: &LOC_UR,
    children: &[
        ("IN", &TAG_UR_IN),
        ("PK", &TAG_UR_PK),
    ],
};

pub static LOC_UR_IN : CldrData = CldrData {
    parents: &[
        &LOC_UR,
    ],
    index: &[
        (Item::DecimalDigits, 20),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹",
};

pub static TAG_UR_IN : CldrTree = CldrTree {
    data: &LOC_UR_IN,
    children: &[
    ],
};

pub static LOC_UR_PK : CldrData = CldrData {
    parents: &[
        &LOC_UR,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UR_PK : CldrTree = CldrTree {
    data: &LOC_UR_PK,
    children: &[
    ],
};

pub static LOC_UZ : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
        (Item::NotANumberSymbol, 21),
    ],
    data: ", haqiqiy son emas",
};

pub static TAG_UZ : CldrTree = CldrTree {
    data: &LOC_UZ,
    children: &[
        ("Arab", &TAG_UZ_ARAB),
        ("Cyrl", &TAG_UZ_CYRL),
        ("Latn", &TAG_UZ_LATN),
    ],
};

pub static LOC_UZ_ARAB : CldrData = CldrData {
    parents: &[
        &LOC_UZ,
    ],
    index: &[
        (Item::DecimalDigits, 20),
        (Item::DecimalSeparator, 22),
        (Item::GroupSeparator, 24),
        (Item::PercentSign, 26),
        (Item::EngineeringExponent, 33),
    ],
    data: "۰۱۲۳۴۵۶۷۸۹٫٬٪×۱۰^",
};

pub static TAG_UZ_ARAB : CldrTree = CldrTree {
    data: &LOC_UZ_ARAB,
    children: &[
        ("AF", &TAG_UZ_ARAB_AF),
    ],
};

pub static LOC_UZ_ARAB_AF : CldrData = CldrData {
    parents: &[
        &LOC_UZ_ARAB,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UZ_ARAB_AF : CldrTree = CldrTree {
    data: &LOC_UZ_ARAB_AF,
    children: &[
    ],
};

pub static LOC_UZ_CYRL : CldrData = CldrData {
    parents: &[
        &LOC_UZ,
    ],
    index: &[
        (Item::NotANumberSymbol, 32),
    ],
    data: "ҳақиқий сон эмас",
};

pub static TAG_UZ_CYRL : CldrTree = CldrTree {
    data: &LOC_UZ_CYRL,
    children: &[
        ("UZ", &TAG_UZ_CYRL_UZ),
    ],
};

pub static LOC_UZ_CYRL_UZ : CldrData = CldrData {
    parents: &[
        &LOC_UZ_CYRL,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UZ_CYRL_UZ : CldrTree = CldrTree {
    data: &LOC_UZ_CYRL_UZ,
    children: &[
    ],
};

pub static LOC_UZ_LATN : CldrData = CldrData {
    parents: &[
        &LOC_UZ,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UZ_LATN : CldrTree = CldrTree {
    data: &LOC_UZ_LATN,
    children: &[
        ("UZ", &TAG_UZ_LATN_UZ),
    ],
};

pub static LOC_UZ_LATN_UZ : CldrData = CldrData {
    parents: &[
        &LOC_UZ_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_UZ_LATN_UZ : CldrTree = CldrTree {
    data: &LOC_UZ_LATN_UZ,
    children: &[
    ],
};

pub static LOC_VAI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VAI : CldrTree = CldrTree {
    data: &LOC_VAI,
    children: &[
        ("Latn", &TAG_VAI_LATN),
        ("Vaii", &TAG_VAI_VAII),
    ],
};

pub static LOC_VAI_LATN : CldrData = CldrData {
    parents: &[
        &LOC_VAI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VAI_LATN : CldrTree = CldrTree {
    data: &LOC_VAI_LATN,
    children: &[
        ("LR", &TAG_VAI_LATN_LR),
    ],
};

pub static LOC_VAI_LATN_LR : CldrData = CldrData {
    parents: &[
        &LOC_VAI_LATN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VAI_LATN_LR : CldrTree = CldrTree {
    data: &LOC_VAI_LATN_LR,
    children: &[
    ],
};

pub static LOC_VAI_VAII : CldrData = CldrData {
    parents: &[
        &LOC_VAI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VAI_VAII : CldrTree = CldrTree {
    data: &LOC_VAI_VAII,
    children: &[
        ("LR", &TAG_VAI_VAII_LR),
    ],
};

pub static LOC_VAI_VAII_LR : CldrData = CldrData {
    parents: &[
        &LOC_VAI_VAII,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VAI_VAII_LR : CldrTree = CldrTree {
    data: &LOC_VAI_VAII_LR,
    children: &[
    ],
};

pub static LOC_VI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 2),
    ],
    data: ",.",
};

pub static TAG_VI : CldrTree = CldrTree {
    data: &LOC_VI,
    children: &[
        ("VN", &TAG_VI_VN),
    ],
};

pub static LOC_VI_VN : CldrData = CldrData {
    parents: &[
        &LOC_VI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VI_VN : CldrTree = CldrTree {
    data: &LOC_VI_VN,
    children: &[
    ],
};

pub static LOC_VO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VO : CldrTree = CldrTree {
    data: &LOC_VO,
    children: &[
        ("001", &TAG_VO_001),
    ],
};

pub static LOC_VO_001 : CldrData = CldrData {
    parents: &[
        &LOC_VO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VO_001 : CldrTree = CldrTree {
    data: &LOC_VO_001,
    children: &[
    ],
};

pub static LOC_VUN : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VUN : CldrTree = CldrTree {
    data: &LOC_VUN,
    children: &[
        ("TZ", &TAG_VUN_TZ),
    ],
};

pub static LOC_VUN_TZ : CldrData = CldrData {
    parents: &[
        &LOC_VUN,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_VUN_TZ : CldrTree = CldrTree {
    data: &LOC_VUN_TZ,
    children: &[
    ],
};

pub static LOC_WAE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 4),
    ],
    data: ",’",
};

pub static TAG_WAE : CldrTree = CldrTree {
    data: &LOC_WAE,
    children: &[
        ("CH", &TAG_WAE_CH),
    ],
};

pub static LOC_WAE_CH : CldrData = CldrData {
    parents: &[
        &LOC_WAE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_WAE_CH : CldrTree = CldrTree {
    data: &LOC_WAE_CH,
    children: &[
    ],
};

pub static LOC_XOG : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_XOG : CldrTree = CldrTree {
    data: &LOC_XOG,
    children: &[
        ("UG", &TAG_XOG_UG),
    ],
};

pub static LOC_XOG_UG : CldrData = CldrData {
    parents: &[
        &LOC_XOG,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_XOG_UG : CldrTree = CldrTree {
    data: &LOC_XOG_UG,
    children: &[
    ],
};

pub static LOC_YAV : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_YAV : CldrTree = CldrTree {
    data: &LOC_YAV,
    children: &[
        ("CM", &TAG_YAV_CM),
    ],
};

pub static LOC_YAV_CM : CldrData = CldrData {
    parents: &[
        &LOC_YAV,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YAV_CM : CldrTree = CldrTree {
    data: &LOC_YAV_CM,
    children: &[
    ],
};

pub static LOC_YI : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YI : CldrTree = CldrTree {
    data: &LOC_YI,
    children: &[
        ("001", &TAG_YI_001),
    ],
};

pub static LOC_YI_001 : CldrData = CldrData {
    parents: &[
        &LOC_YI,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YI_001 : CldrTree = CldrTree {
    data: &LOC_YI_001,
    children: &[
    ],
};

pub static LOC_YO : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YO : CldrTree = CldrTree {
    data: &LOC_YO,
    children: &[
        ("BJ", &TAG_YO_BJ),
        ("NG", &TAG_YO_NG),
    ],
};

pub static LOC_YO_BJ : CldrData = CldrData {
    parents: &[
        &LOC_YO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YO_BJ : CldrTree = CldrTree {
    data: &LOC_YO_BJ,
    children: &[
    ],
};

pub static LOC_YO_NG : CldrData = CldrData {
    parents: &[
        &LOC_YO,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YO_NG : CldrTree = CldrTree {
    data: &LOC_YO_NG,
    children: &[
    ],
};

pub static LOC_YUE : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::NotANumberSymbol, 9),
    ],
    data: "非數值",
};

pub static TAG_YUE : CldrTree = CldrTree {
    data: &LOC_YUE,
    children: &[
        ("HK", &TAG_YUE_HK),
    ],
};

pub static LOC_YUE_HK : CldrData = CldrData {
    parents: &[
        &LOC_YUE,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_YUE_HK : CldrTree = CldrTree {
    data: &LOC_YUE_HK,
    children: &[
    ],
};

pub static LOC_ZGH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
        (Item::DecimalSeparator, 1),
        (Item::GroupSeparator, 3),
    ],
    data: ", ",
};

pub static TAG_ZGH : CldrTree = CldrTree {
    data: &LOC_ZGH,
    children: &[
        ("MA", &TAG_ZGH_MA),
    ],
};

pub static LOC_ZGH_MA : CldrData = CldrData {
    parents: &[
        &LOC_ZGH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZGH_MA : CldrTree = CldrTree {
    data: &LOC_ZGH_MA,
    children: &[
    ],
};

pub static LOC_ZH : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH : CldrTree = CldrTree {
    data: &LOC_ZH,
    children: &[
        ("Hans", &TAG_ZH_HANS),
        ("Hant", &TAG_ZH_HANT),
    ],
};

pub static LOC_ZH_HANS : CldrData = CldrData {
    parents: &[
        &LOC_ZH,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANS : CldrTree = CldrTree {
    data: &LOC_ZH_HANS,
    children: &[
        ("CN", &TAG_ZH_HANS_CN),
        ("HK", &TAG_ZH_HANS_HK),
        ("MO", &TAG_ZH_HANS_MO),
        ("SG", &TAG_ZH_HANS_SG),
    ],
};

pub static LOC_ZH_HANS_CN : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANS_CN : CldrTree = CldrTree {
    data: &LOC_ZH_HANS_CN,
    children: &[
    ],
};

pub static LOC_ZH_HANS_HK : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANS_HK : CldrTree = CldrTree {
    data: &LOC_ZH_HANS_HK,
    children: &[
    ],
};

pub static LOC_ZH_HANS_MO : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANS_MO : CldrTree = CldrTree {
    data: &LOC_ZH_HANS_MO,
    children: &[
    ],
};

pub static LOC_ZH_HANS_SG : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANS,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANS_SG : CldrTree = CldrTree {
    data: &LOC_ZH_HANS_SG,
    children: &[
    ],
};

pub static LOC_ZH_HANT : CldrData = CldrData {
    parents: &[
        &LOC_ZH,
    ],
    index: &[
        (Item::NotANumberSymbol, 9),
    ],
    data: "非數值",
};

pub static TAG_ZH_HANT : CldrTree = CldrTree {
    data: &LOC_ZH_HANT,
    children: &[
        ("HK", &TAG_ZH_HANT_HK),
        ("MO", &TAG_ZH_HANT_MO),
        ("TW", &TAG_ZH_HANT_TW),
    ],
};

pub static LOC_ZH_HANT_HK : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANT_HK : CldrTree = CldrTree {
    data: &LOC_ZH_HANT_HK,
    children: &[
    ],
};

pub static LOC_ZH_HANT_MO : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANT_MO : CldrTree = CldrTree {
    data: &LOC_ZH_HANT_MO,
    children: &[
    ],
};

pub static LOC_ZH_HANT_TW : CldrData = CldrData {
    parents: &[
        &LOC_ZH_HANT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZH_HANT_TW : CldrTree = CldrTree {
    data: &LOC_ZH_HANT_TW,
    children: &[
    ],
};

pub static LOC_ZU : CldrData = CldrData {
    parents: &[
        &LOC_ROOT,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZU : CldrTree = CldrTree {
    data: &LOC_ZU,
    children: &[
        ("ZA", &TAG_ZU_ZA),
    ],
};

pub static LOC_ZU_ZA : CldrData = CldrData {
    parents: &[
        &LOC_ZU,
    ],
    index: &[
    ],
    data: "",
};

pub static TAG_ZU_ZA : CldrTree = CldrTree {
    data: &LOC_ZU_ZA,
    children: &[
    ],
};



//! ISO 4217 currency codes used by BAI2's "Currency Code" field.
//!
//! See `docs/CURRENCY_CODE.md` for the technical spec.

/// A BAI2 currency code (02/03-record "Currency Code" field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrencyCode {
    /// Afghani (AFN).
    Afn,
    /// Lek (ALL).
    All,
    /// Algerian Dinar (DZD).
    Dzd,
    /// U.S. Dollar (USD).
    Usd,
    /// Euro (EUR).
    Eur,
    /// Kwanza (AOA).
    Aoa,
    /// East Caribbean Dollar (XCD).
    Xcd,
    /// Argentine Peso (ARS).
    Ars,
    /// Armenian Dram (AMD).
    Amd,
    /// Aruban Guilder (AWG).
    Awg,
    /// Australian Dollar (AUD).
    Aud,
    /// Azerbaijanian Manat (AZM).
    Azm,
    /// Bahamian Dollar (BSD).
    Bsd,
    /// Bahraini Dinar (BHD).
    Bhd,
    /// Taka (BDT).
    Bdt,
    /// Barbados Dollar (BBD).
    Bbd,
    /// Belarussian Ruble (BYR).
    Byr,
    /// Belize Dollar (BZD).
    Bzd,
    /// CFA Franc BCEAO (XOF).
    Xof,
    /// Bermudian Dollar (BMD).
    Bmd,
    /// Indian Rupee (INR).
    Inr,
    /// Ngultrum (BTN).
    Btn,
    /// Boliviano (BOB).
    Bob,
    /// Mvdol (BOV).
    Bov,
    /// Convertible Marks (BAM).
    Bam,
    /// Pula (BWP).
    Bwp,
    /// Norwegian Krone (NOK).
    Nok,
    /// Brazilian Real (BRL).
    Brl,
    /// Brunei Dollar (BND).
    Bnd,
    /// Bulgarian Lev (BGN).
    Bgn,
    /// Kyat (BUK).
    Buk,
    /// Burundi Franc (BIF).
    Bif,
    /// Riel (KHR).
    Khr,
    /// CFA Franc BEAC (XAF).
    Xaf,
    /// Canadian Dollar (CAD).
    Cad,
    /// Cape Verde Escudo (CVE).
    Cve,
    /// Cayman Islands Dollar (KYD).
    Kyd,
    /// Chilean Peso (CLP).
    Clp,
    /// Unidades de fomento (CLF).
    Clf,
    /// Yuan Renminbi (CNY).
    Cny,
    /// Colombian Peso (COP).
    Cop,
    /// Unidad de Valor Real (COU).
    Cou,
    /// Comoro Franc (KMF).
    Kmf,
    /// Franc Congolais (CDF).
    Cdf,
    /// New Zealand Dollar (NZD).
    Nzd,
    /// Costa Rican Colon (CRC).
    Crc,
    /// Croatian Kuna (HRK).
    Hrk,
    /// Cuban Peso (CUP).
    Cup,
    /// Cyprus Pound (CYP).
    Cyp,
    /// Czech Koruna (CZK).
    Czk,
    /// Danish Krone (DKK).
    Dkk,
    /// Djibouti Franc (DJF).
    Djf,
    /// Dominican Peso (DOP).
    Dop,
    /// Egyptian Pound (EGP).
    Egp,
    /// El Salvador Colon (SVC).
    Svc,
    /// Nakfa (ERN).
    Ern,
    /// Kroon (EEK).
    Eek,
    /// Ethiopian Birr (ETB).
    Etb,
    /// Falkland Islands Pound (FKP).
    Fkp,
    /// Fiji Dollar (FJD).
    Fjd,
    /// CFP Franc (XPF).
    Xpf,
    /// Dalasi (GMD).
    Gmd,
    /// Lari (GEL).
    Gel,
    /// Cedi (GHC).
    Ghc,
    /// Gibraltar Pound (GIP).
    Gip,
    /// Quetzal (GTQ).
    Gtq,
    /// Guinea Franc (GNF).
    Gnf,
    /// Guinea-Bissau Peso (GWP).
    Gwp,
    /// Guyana Dollar (GYD).
    Gyd,
    /// Gourde (HTG).
    Htg,
    /// Lempira (HNL).
    Hnl,
    /// Hong Kong Dollar (HKD).
    Hkd,
    /// Forint (HUF).
    Huf,
    /// Iceland Krona (ISK).
    Isk,
    /// Rupiah (IDR).
    Idr,
    /// Iranian Rial (IRR).
    Irr,
    /// Iraqi Dinar (IQD).
    Iqd,
    /// New Israeli Sheqel (ILS).
    Ils,
    /// Jamaican Dollar (JMD).
    Jmd,
    /// Yen (JPY).
    Jpy,
    /// Jordanian Dinar (JOD).
    Jod,
    /// Tenge (KZT).
    Kzt,
    /// Kenyan Shilling (KES).
    Kes,
    /// North Korean Won (KPW).
    Kpw,
    /// Won (KRW).
    Krw,
    /// Kuwaiti Dinar (KWD).
    Kwd,
    /// Som (KGS).
    Kgs,
    /// Kip (LAK).
    Lak,
    /// Latvian Lats (LVL).
    Lvl,
    /// Lebanese Pound (LBP).
    Lbp,
    /// Loti (LSL).
    Lsl,
    /// Rand (ZAR).
    Zar,
    /// Liberian Dollar (LRD).
    Lrd,
    /// Lybian Dinar (LYD).
    Lyd,
    /// Swiss Franc (CHF).
    Chf,
    /// Lithuanian Litas (LTL).
    Ltl,
    /// Pataca (MOP).
    Mop,
    /// Denar (MKD).
    Mkd,
    /// Ariary (MGA).
    Mga,
    /// Malagasy Franc (MGF).
    Mgf,
    /// Kwacha (MWK).
    Mwk,
    /// Malaysian Ringgit (MYR).
    Myr,
    /// Rufiyaa (MVR).
    Mvr,
    /// Maltese Lira (MTL).
    Mtl,
    /// Ouguiya (MRO).
    Mro,
    /// Mauritius Rupee (MUR).
    Mur,
    /// Mexican Peso (MXN).
    Mxn,
    /// Mexican Unidad de Inversion (UDI) (MXV).
    Mxv,
    /// Moldovan Leu (MDL).
    Mdl,
    /// Tugrik (MNT).
    Mnt,
    /// Moroccan Dirham (MAD).
    Mad,
    /// Metical (MZM).
    Mzm,
    /// Kyat (MMK).
    Mmk,
    /// Namibia Dollar (NAD).
    Nad,
    /// Nepalese Rupee (NPR).
    Npr,
    /// Netherlands Antillian Guilder (ANG).
    Ang,
    /// Cordoba Oro (NIO).
    Nio,
    /// Naira (NGN).
    Ngn,
    /// Rial Omani (OMR).
    Omr,
    /// Pakistan Rupee (PKR).
    Pkr,
    /// Balboa (PAB).
    Pab,
    /// Kina (PGK).
    Pgk,
    /// Guarani (PYG).
    Pyg,
    /// Nuevo Sol (PEN).
    Pen,
    /// Philippine Peso (PHP).
    Php,
    /// Zloty (PLN).
    Pln,
    /// Qatari Rial (QAR).
    Qar,
    /// Leu (ROL).
    Rol,
    /// Russian Ruble (RUB).
    Rub,
    /// Russian Ruble (RUR).
    Rur,
    /// Rwanda Franc (RWF).
    Rwf,
    /// St. Helena Pound (SHP).
    Shp,
    /// Tala (WST).
    Wst,
    /// Dobra (STD).
    Std,
    /// Saudi Riyal (SAR).
    Sar,
    /// Serbian Dinar (CSD).
    Csd,
    /// Seychelles Rupee (SCR).
    Scr,
    /// Leone (SLL).
    Sll,
    /// Singapore Dollar (SGD).
    Sgd,
    /// Slovak Koruna (SKK).
    Skk,
    /// Tolar (SIT).
    Sit,
    /// Solomon Islands Dollar (SBD).
    Sbd,
    /// Somali Shilling (SOS).
    Sos,
    /// Sri Lanka Rupee (LKR).
    Lkr,
    /// Sudanese Dinar (SDD).
    Sdd,
    /// Suriname Dollar (SRD).
    Srd,
    /// Lilangeni (SZL).
    Szl,
    /// Swedish Krona (SEK).
    Sek,
    /// Syrian Pound (SYP).
    Syp,
    /// New Taiwan Dollar (TWD).
    Twd,
    /// Somoni (TJS).
    Tjs,
    /// Tanzanian Shilling (TZS).
    Tzs,
    /// Baht (THB).
    Thb,
    /// Pa’anga (TOP).
    Top,
    /// Trinidad & Tobago Dollar (TTD).
    Ttd,
    /// Tunisian Dinar (TND).
    Tnd,
    /// Turkish Lira (TRL).
    Trl,
    /// Manat (TMM).
    Tmm,
    /// Uganda Shilling (UGX).
    Ugx,
    /// Hryvnia (UAH).
    Uah,
    /// UAE Dirham (AED).
    Aed,
    /// Pound Sterling (GBP).
    Gbp,
    /// U.S. Dollar (Next Day) (USN).
    Usn,
    /// U.S. Dollar (Same Day) (USS).
    Uss,
    /// Peso Uruguayo (UYU).
    Uyu,
    /// Uzbekistan Sum (UZS).
    Uzs,
    /// Vatu (VUV).
    Vuv,
    /// Bolivar (VEB).
    Veb,
    /// Dong (VND).
    Vnd,
    /// Yemeni Rial (YER).
    Yer,
    /// Yemeni Dinar (YDD).
    Ydd,
    /// New Yugoslavian Dinar (YUD).
    Yud,
    /// Zaire (ZRZ).
    Zrz,
    /// Kwacha (ZMK).
    Zmk,
    /// Zimbabwe Dollar (ZWD).
    Zwd,
    /// Silver (XAG).
    Xag,
    /// Gold (XAU).
    Xau,
    /// European Composite Unit (EURCO) (XBA).
    Xba,
    /// European Monetary Unit (EMU-6) (XBB).
    Xbb,
    /// European Unit of Account (EUA-9) (XBC).
    Xbc,
    /// European Unit of Account (EUA-17) (XBD).
    Xbd,
    /// A currency code not defined in Appendix B, or blank/absent input.
    Unknown,
}

impl CurrencyCode {
    /// The three-letter ISO 4217 code, e.g. "USD". Empty for `Unknown`.
    pub fn code(self) -> &'static str {
        match self {
            Self::Afn => "AFN",
            Self::All => "ALL",
            Self::Dzd => "DZD",
            Self::Usd => "USD",
            Self::Eur => "EUR",
            Self::Aoa => "AOA",
            Self::Xcd => "XCD",
            Self::Ars => "ARS",
            Self::Amd => "AMD",
            Self::Awg => "AWG",
            Self::Aud => "AUD",
            Self::Azm => "AZM",
            Self::Bsd => "BSD",
            Self::Bhd => "BHD",
            Self::Bdt => "BDT",
            Self::Bbd => "BBD",
            Self::Byr => "BYR",
            Self::Bzd => "BZD",
            Self::Xof => "XOF",
            Self::Bmd => "BMD",
            Self::Inr => "INR",
            Self::Btn => "BTN",
            Self::Bob => "BOB",
            Self::Bov => "BOV",
            Self::Bam => "BAM",
            Self::Bwp => "BWP",
            Self::Nok => "NOK",
            Self::Brl => "BRL",
            Self::Bnd => "BND",
            Self::Bgn => "BGN",
            Self::Buk => "BUK",
            Self::Bif => "BIF",
            Self::Khr => "KHR",
            Self::Xaf => "XAF",
            Self::Cad => "CAD",
            Self::Cve => "CVE",
            Self::Kyd => "KYD",
            Self::Clp => "CLP",
            Self::Clf => "CLF",
            Self::Cny => "CNY",
            Self::Cop => "COP",
            Self::Cou => "COU",
            Self::Kmf => "KMF",
            Self::Cdf => "CDF",
            Self::Nzd => "NZD",
            Self::Crc => "CRC",
            Self::Hrk => "HRK",
            Self::Cup => "CUP",
            Self::Cyp => "CYP",
            Self::Czk => "CZK",
            Self::Dkk => "DKK",
            Self::Djf => "DJF",
            Self::Dop => "DOP",
            Self::Egp => "EGP",
            Self::Svc => "SVC",
            Self::Ern => "ERN",
            Self::Eek => "EEK",
            Self::Etb => "ETB",
            Self::Fkp => "FKP",
            Self::Fjd => "FJD",
            Self::Xpf => "XPF",
            Self::Gmd => "GMD",
            Self::Gel => "GEL",
            Self::Ghc => "GHC",
            Self::Gip => "GIP",
            Self::Gtq => "GTQ",
            Self::Gnf => "GNF",
            Self::Gwp => "GWP",
            Self::Gyd => "GYD",
            Self::Htg => "HTG",
            Self::Hnl => "HNL",
            Self::Hkd => "HKD",
            Self::Huf => "HUF",
            Self::Isk => "ISK",
            Self::Idr => "IDR",
            Self::Irr => "IRR",
            Self::Iqd => "IQD",
            Self::Ils => "ILS",
            Self::Jmd => "JMD",
            Self::Jpy => "JPY",
            Self::Jod => "JOD",
            Self::Kzt => "KZT",
            Self::Kes => "KES",
            Self::Kpw => "KPW",
            Self::Krw => "KRW",
            Self::Kwd => "KWD",
            Self::Kgs => "KGS",
            Self::Lak => "LAK",
            Self::Lvl => "LVL",
            Self::Lbp => "LBP",
            Self::Lsl => "LSL",
            Self::Zar => "ZAR",
            Self::Lrd => "LRD",
            Self::Lyd => "LYD",
            Self::Chf => "CHF",
            Self::Ltl => "LTL",
            Self::Mop => "MOP",
            Self::Mkd => "MKD",
            Self::Mga => "MGA",
            Self::Mgf => "MGF",
            Self::Mwk => "MWK",
            Self::Myr => "MYR",
            Self::Mvr => "MVR",
            Self::Mtl => "MTL",
            Self::Mro => "MRO",
            Self::Mur => "MUR",
            Self::Mxn => "MXN",
            Self::Mxv => "MXV",
            Self::Mdl => "MDL",
            Self::Mnt => "MNT",
            Self::Mad => "MAD",
            Self::Mzm => "MZM",
            Self::Mmk => "MMK",
            Self::Nad => "NAD",
            Self::Npr => "NPR",
            Self::Ang => "ANG",
            Self::Nio => "NIO",
            Self::Ngn => "NGN",
            Self::Omr => "OMR",
            Self::Pkr => "PKR",
            Self::Pab => "PAB",
            Self::Pgk => "PGK",
            Self::Pyg => "PYG",
            Self::Pen => "PEN",
            Self::Php => "PHP",
            Self::Pln => "PLN",
            Self::Qar => "QAR",
            Self::Rol => "ROL",
            Self::Rub => "RUB",
            Self::Rur => "RUR",
            Self::Rwf => "RWF",
            Self::Shp => "SHP",
            Self::Wst => "WST",
            Self::Std => "STD",
            Self::Sar => "SAR",
            Self::Csd => "CSD",
            Self::Scr => "SCR",
            Self::Sll => "SLL",
            Self::Sgd => "SGD",
            Self::Skk => "SKK",
            Self::Sit => "SIT",
            Self::Sbd => "SBD",
            Self::Sos => "SOS",
            Self::Lkr => "LKR",
            Self::Sdd => "SDD",
            Self::Srd => "SRD",
            Self::Szl => "SZL",
            Self::Sek => "SEK",
            Self::Syp => "SYP",
            Self::Twd => "TWD",
            Self::Tjs => "TJS",
            Self::Tzs => "TZS",
            Self::Thb => "THB",
            Self::Top => "TOP",
            Self::Ttd => "TTD",
            Self::Tnd => "TND",
            Self::Trl => "TRL",
            Self::Tmm => "TMM",
            Self::Ugx => "UGX",
            Self::Uah => "UAH",
            Self::Aed => "AED",
            Self::Gbp => "GBP",
            Self::Usn => "USN",
            Self::Uss => "USS",
            Self::Uyu => "UYU",
            Self::Uzs => "UZS",
            Self::Vuv => "VUV",
            Self::Veb => "VEB",
            Self::Vnd => "VND",
            Self::Yer => "YER",
            Self::Ydd => "YDD",
            Self::Yud => "YUD",
            Self::Zrz => "ZRZ",
            Self::Zmk => "ZMK",
            Self::Zwd => "ZWD",
            Self::Xag => "XAG",
            Self::Xau => "XAU",
            Self::Xba => "XBA",
            Self::Xbb => "XBB",
            Self::Xbc => "XBC",
            Self::Xbd => "XBD",
            Self::Unknown => "",
        }
    }

    /// The currency's name, as transcribed from Appendix B.
    pub fn description(self) -> &'static str {
        match self {
            Self::Afn => "Afghani",
            Self::All => "Lek",
            Self::Dzd => "Algerian Dinar",
            Self::Usd => "U.S. Dollar",
            Self::Eur => "Euro",
            Self::Aoa => "Kwanza",
            Self::Xcd => "East Caribbean Dollar",
            Self::Ars => "Argentine Peso",
            Self::Amd => "Armenian Dram",
            Self::Awg => "Aruban Guilder",
            Self::Aud => "Australian Dollar",
            Self::Azm => "Azerbaijanian Manat",
            Self::Bsd => "Bahamian Dollar",
            Self::Bhd => "Bahraini Dinar",
            Self::Bdt => "Taka",
            Self::Bbd => "Barbados Dollar",
            Self::Byr => "Belarussian Ruble",
            Self::Bzd => "Belize Dollar",
            Self::Xof => "CFA Franc BCEAO",
            Self::Bmd => "Bermudian Dollar",
            Self::Inr => "Indian Rupee",
            Self::Btn => "Ngultrum",
            Self::Bob => "Boliviano",
            Self::Bov => "Mvdol",
            Self::Bam => "Convertible Marks",
            Self::Bwp => "Pula",
            Self::Nok => "Norwegian Krone",
            Self::Brl => "Brazilian Real",
            Self::Bnd => "Brunei Dollar",
            Self::Bgn => "Bulgarian Lev",
            Self::Buk => "Kyat",
            Self::Bif => "Burundi Franc",
            Self::Khr => "Riel",
            Self::Xaf => "CFA Franc BEAC",
            Self::Cad => "Canadian Dollar",
            Self::Cve => "Cape Verde Escudo",
            Self::Kyd => "Cayman Islands Dollar",
            Self::Clp => "Chilean Peso",
            Self::Clf => "Unidades de fomento",
            Self::Cny => "Yuan Renminbi",
            Self::Cop => "Colombian Peso",
            Self::Cou => "Unidad de Valor Real",
            Self::Kmf => "Comoro Franc",
            Self::Cdf => "Franc Congolais",
            Self::Nzd => "New Zealand Dollar",
            Self::Crc => "Costa Rican Colon",
            Self::Hrk => "Croatian Kuna",
            Self::Cup => "Cuban Peso",
            Self::Cyp => "Cyprus Pound",
            Self::Czk => "Czech Koruna",
            Self::Dkk => "Danish Krone",
            Self::Djf => "Djibouti Franc",
            Self::Dop => "Dominican Peso",
            Self::Egp => "Egyptian Pound",
            Self::Svc => "El Salvador Colon",
            Self::Ern => "Nakfa",
            Self::Eek => "Kroon",
            Self::Etb => "Ethiopian Birr",
            Self::Fkp => "Falkland Islands Pound",
            Self::Fjd => "Fiji Dollar",
            Self::Xpf => "CFP Franc",
            Self::Gmd => "Dalasi",
            Self::Gel => "Lari",
            Self::Ghc => "Cedi",
            Self::Gip => "Gibraltar Pound",
            Self::Gtq => "Quetzal",
            Self::Gnf => "Guinea Franc",
            Self::Gwp => "Guinea-Bissau Peso",
            Self::Gyd => "Guyana Dollar",
            Self::Htg => "Gourde",
            Self::Hnl => "Lempira",
            Self::Hkd => "Hong Kong Dollar",
            Self::Huf => "Forint",
            Self::Isk => "Iceland Krona",
            Self::Idr => "Rupiah",
            Self::Irr => "Iranian Rial",
            Self::Iqd => "Iraqi Dinar",
            Self::Ils => "New Israeli Sheqel",
            Self::Jmd => "Jamaican Dollar",
            Self::Jpy => "Yen",
            Self::Jod => "Jordanian Dinar",
            Self::Kzt => "Tenge",
            Self::Kes => "Kenyan Shilling",
            Self::Kpw => "North Korean Won",
            Self::Krw => "Won",
            Self::Kwd => "Kuwaiti Dinar",
            Self::Kgs => "Som",
            Self::Lak => "Kip",
            Self::Lvl => "Latvian Lats",
            Self::Lbp => "Lebanese Pound",
            Self::Lsl => "Loti",
            Self::Zar => "Rand",
            Self::Lrd => "Liberian Dollar",
            Self::Lyd => "Lybian Dinar",
            Self::Chf => "Swiss Franc",
            Self::Ltl => "Lithuanian Litas",
            Self::Mop => "Pataca",
            Self::Mkd => "Denar",
            Self::Mga => "Ariary",
            Self::Mgf => "Malagasy Franc",
            Self::Mwk => "Kwacha",
            Self::Myr => "Malaysian Ringgit",
            Self::Mvr => "Rufiyaa",
            Self::Mtl => "Maltese Lira",
            Self::Mro => "Ouguiya",
            Self::Mur => "Mauritius Rupee",
            Self::Mxn => "Mexican Peso",
            Self::Mxv => "Mexican Unidad de Inversion (UDI)",
            Self::Mdl => "Moldovan Leu",
            Self::Mnt => "Tugrik",
            Self::Mad => "Moroccan Dirham",
            Self::Mzm => "Metical",
            Self::Mmk => "Kyat",
            Self::Nad => "Namibia Dollar",
            Self::Npr => "Nepalese Rupee",
            Self::Ang => "Netherlands Antillian Guilder",
            Self::Nio => "Cordoba Oro",
            Self::Ngn => "Naira",
            Self::Omr => "Rial Omani",
            Self::Pkr => "Pakistan Rupee",
            Self::Pab => "Balboa",
            Self::Pgk => "Kina",
            Self::Pyg => "Guarani",
            Self::Pen => "Nuevo Sol",
            Self::Php => "Philippine Peso",
            Self::Pln => "Zloty",
            Self::Qar => "Qatari Rial",
            Self::Rol => "Leu",
            Self::Rub => "Russian Ruble",
            Self::Rur => "Russian Ruble",
            Self::Rwf => "Rwanda Franc",
            Self::Shp => "St. Helena Pound",
            Self::Wst => "Tala",
            Self::Std => "Dobra",
            Self::Sar => "Saudi Riyal",
            Self::Csd => "Serbian Dinar",
            Self::Scr => "Seychelles Rupee",
            Self::Sll => "Leone",
            Self::Sgd => "Singapore Dollar",
            Self::Skk => "Slovak Koruna",
            Self::Sit => "Tolar",
            Self::Sbd => "Solomon Islands Dollar",
            Self::Sos => "Somali Shilling",
            Self::Lkr => "Sri Lanka Rupee",
            Self::Sdd => "Sudanese Dinar",
            Self::Srd => "Suriname Dollar",
            Self::Szl => "Lilangeni",
            Self::Sek => "Swedish Krona",
            Self::Syp => "Syrian Pound",
            Self::Twd => "New Taiwan Dollar",
            Self::Tjs => "Somoni",
            Self::Tzs => "Tanzanian Shilling",
            Self::Thb => "Baht",
            Self::Top => "Pa’anga",
            Self::Ttd => "Trinidad & Tobago Dollar",
            Self::Tnd => "Tunisian Dinar",
            Self::Trl => "Turkish Lira",
            Self::Tmm => "Manat",
            Self::Ugx => "Uganda Shilling",
            Self::Uah => "Hryvnia",
            Self::Aed => "UAE Dirham",
            Self::Gbp => "Pound Sterling",
            Self::Usn => "U.S. Dollar (Next Day)",
            Self::Uss => "U.S. Dollar (Same Day)",
            Self::Uyu => "Peso Uruguayo",
            Self::Uzs => "Uzbekistan Sum",
            Self::Vuv => "Vatu",
            Self::Veb => "Bolivar",
            Self::Vnd => "Dong",
            Self::Yer => "Yemeni Rial",
            Self::Ydd => "Yemeni Dinar",
            Self::Yud => "New Yugoslavian Dinar",
            Self::Zrz => "Zaire",
            Self::Zmk => "Kwacha",
            Self::Zwd => "Zimbabwe Dollar",
            Self::Xag => "Silver",
            Self::Xau => "Gold",
            Self::Xba => "European Composite Unit (EURCO)",
            Self::Xbb => "European Monetary Unit (EMU-6)",
            Self::Xbc => "European Unit of Account (EUA-9)",
            Self::Xbd => "European Unit of Account (EUA-17)",
            Self::Unknown => "Unknown currency",
        }
    }

    /// The number of implied decimal places in Amount/Funds Type fields
    /// denominated in this currency. Defaults to 2 per Appendix B's
    /// "Implied Decimals" section; `Unknown` also defaults to 2 since a
    /// blank Currency Code field defaults to the group/account currency
    /// (itself defaulting to USD), not to an undefined decimal count.
    pub fn decimals(self) -> u32 {
        match self {
            Self::Afn => 2,
            Self::All => 2,
            Self::Dzd => 2,
            Self::Usd => 2,
            Self::Eur => 2,
            Self::Aoa => 2,
            Self::Xcd => 2,
            Self::Ars => 2,
            Self::Amd => 2,
            Self::Awg => 2,
            Self::Aud => 2,
            Self::Azm => 2,
            Self::Bsd => 2,
            Self::Bhd => 3,
            Self::Bdt => 2,
            Self::Bbd => 2,
            Self::Byr => 2,
            Self::Bzd => 2,
            Self::Xof => 0,
            Self::Bmd => 2,
            Self::Inr => 2,
            Self::Btn => 2,
            Self::Bob => 2,
            Self::Bov => 2,
            Self::Bam => 2,
            Self::Bwp => 2,
            Self::Nok => 2,
            Self::Brl => 2,
            Self::Bnd => 2,
            Self::Bgn => 2,
            Self::Buk => 2,
            Self::Bif => 2,
            Self::Khr => 2,
            Self::Xaf => 0,
            Self::Cad => 2,
            Self::Cve => 2,
            Self::Kyd => 2,
            Self::Clp => 2,
            Self::Clf => 2,
            Self::Cny => 2,
            Self::Cop => 2,
            Self::Cou => 2,
            Self::Kmf => 0,
            Self::Cdf => 2,
            Self::Nzd => 2,
            Self::Crc => 2,
            Self::Hrk => 2,
            Self::Cup => 2,
            Self::Cyp => 2,
            Self::Czk => 2,
            Self::Dkk => 2,
            Self::Djf => 2,
            Self::Dop => 2,
            Self::Egp => 3,
            Self::Svc => 2,
            Self::Ern => 2,
            Self::Eek => 2,
            Self::Etb => 2,
            Self::Fkp => 2,
            Self::Fjd => 2,
            Self::Xpf => 0,
            Self::Gmd => 2,
            Self::Gel => 2,
            Self::Ghc => 2,
            Self::Gip => 2,
            Self::Gtq => 2,
            Self::Gnf => 2,
            Self::Gwp => 2,
            Self::Gyd => 2,
            Self::Htg => 2,
            Self::Hnl => 2,
            Self::Hkd => 2,
            Self::Huf => 2,
            Self::Isk => 2,
            Self::Idr => 2,
            Self::Irr => 2,
            Self::Iqd => 3,
            Self::Ils => 2,
            Self::Jmd => 2,
            Self::Jpy => 0,
            Self::Jod => 3,
            Self::Kzt => 2,
            Self::Kes => 2,
            Self::Kpw => 2,
            Self::Krw => 2,
            Self::Kwd => 3,
            Self::Kgs => 2,
            Self::Lak => 2,
            Self::Lvl => 2,
            Self::Lbp => 2,
            Self::Lsl => 2,
            Self::Zar => 2,
            Self::Lrd => 2,
            Self::Lyd => 3,
            Self::Chf => 2,
            Self::Ltl => 2,
            Self::Mop => 2,
            Self::Mkd => 2,
            Self::Mga => 2,
            Self::Mgf => 2,
            Self::Mwk => 2,
            Self::Myr => 2,
            Self::Mvr => 2,
            Self::Mtl => 3,
            Self::Mro => 1,
            Self::Mur => 2,
            Self::Mxn => 2,
            Self::Mxv => 2,
            Self::Mdl => 2,
            Self::Mnt => 2,
            Self::Mad => 2,
            Self::Mzm => 2,
            Self::Mmk => 2,
            Self::Nad => 2,
            Self::Npr => 2,
            Self::Ang => 2,
            Self::Nio => 2,
            Self::Ngn => 2,
            Self::Omr => 3,
            Self::Pkr => 2,
            Self::Pab => 2,
            Self::Pgk => 2,
            Self::Pyg => 2,
            Self::Pen => 2,
            Self::Php => 2,
            Self::Pln => 2,
            Self::Qar => 2,
            Self::Rol => 2,
            Self::Rub => 2,
            Self::Rur => 2,
            Self::Rwf => 2,
            Self::Shp => 2,
            Self::Wst => 2,
            Self::Std => 2,
            Self::Sar => 2,
            Self::Csd => 2,
            Self::Scr => 2,
            Self::Sll => 2,
            Self::Sgd => 2,
            Self::Skk => 2,
            Self::Sit => 2,
            Self::Sbd => 2,
            Self::Sos => 2,
            Self::Lkr => 2,
            Self::Sdd => 2,
            Self::Srd => 2,
            Self::Szl => 2,
            Self::Sek => 2,
            Self::Syp => 2,
            Self::Twd => 2,
            Self::Tjs => 2,
            Self::Tzs => 2,
            Self::Thb => 2,
            Self::Top => 2,
            Self::Ttd => 2,
            Self::Tnd => 3,
            Self::Trl => 2,
            Self::Tmm => 2,
            Self::Ugx => 2,
            Self::Uah => 2,
            Self::Aed => 2,
            Self::Gbp => 2,
            Self::Usn => 2,
            Self::Uss => 2,
            Self::Uyu => 2,
            Self::Uzs => 2,
            Self::Vuv => 2,
            Self::Veb => 2,
            Self::Vnd => 2,
            Self::Yer => 2,
            Self::Ydd => 3,
            Self::Yud => 2,
            Self::Zrz => 2,
            Self::Zmk => 2,
            Self::Zwd => 2,
            Self::Xag => 2,
            Self::Xau => 2,
            Self::Xba => 2,
            Self::Xbb => 2,
            Self::Xbc => 2,
            Self::Xbd => 2,
            Self::Unknown => 2,
        }
    }
}

/// Maps a 3-letter code's bytes to an index in [`CURRENCY_CODE_TABLE`].
const fn table_index(bytes: [u8; 3]) -> usize {
    (bytes[0] - b'A') as usize * 676 + (bytes[1] - b'A') as usize * 26 + (bytes[2] - b'A') as usize
}

/// `(code, variant)` pairs used to populate [`CURRENCY_CODE_TABLE`] at compile time.
const CURRENCY_CODE_ENTRIES: [(&str, CurrencyCode); 181] = [
    ("AFN", CurrencyCode::Afn),
    ("ALL", CurrencyCode::All),
    ("DZD", CurrencyCode::Dzd),
    ("USD", CurrencyCode::Usd),
    ("EUR", CurrencyCode::Eur),
    ("AOA", CurrencyCode::Aoa),
    ("XCD", CurrencyCode::Xcd),
    ("ARS", CurrencyCode::Ars),
    ("AMD", CurrencyCode::Amd),
    ("AWG", CurrencyCode::Awg),
    ("AUD", CurrencyCode::Aud),
    ("AZM", CurrencyCode::Azm),
    ("BSD", CurrencyCode::Bsd),
    ("BHD", CurrencyCode::Bhd),
    ("BDT", CurrencyCode::Bdt),
    ("BBD", CurrencyCode::Bbd),
    ("BYR", CurrencyCode::Byr),
    ("BZD", CurrencyCode::Bzd),
    ("XOF", CurrencyCode::Xof),
    ("BMD", CurrencyCode::Bmd),
    ("INR", CurrencyCode::Inr),
    ("BTN", CurrencyCode::Btn),
    ("BOB", CurrencyCode::Bob),
    ("BOV", CurrencyCode::Bov),
    ("BAM", CurrencyCode::Bam),
    ("BWP", CurrencyCode::Bwp),
    ("NOK", CurrencyCode::Nok),
    ("BRL", CurrencyCode::Brl),
    ("BND", CurrencyCode::Bnd),
    ("BGN", CurrencyCode::Bgn),
    ("BUK", CurrencyCode::Buk),
    ("BIF", CurrencyCode::Bif),
    ("KHR", CurrencyCode::Khr),
    ("XAF", CurrencyCode::Xaf),
    ("CAD", CurrencyCode::Cad),
    ("CVE", CurrencyCode::Cve),
    ("KYD", CurrencyCode::Kyd),
    ("CLP", CurrencyCode::Clp),
    ("CLF", CurrencyCode::Clf),
    ("CNY", CurrencyCode::Cny),
    ("COP", CurrencyCode::Cop),
    ("COU", CurrencyCode::Cou),
    ("KMF", CurrencyCode::Kmf),
    ("CDF", CurrencyCode::Cdf),
    ("NZD", CurrencyCode::Nzd),
    ("CRC", CurrencyCode::Crc),
    ("HRK", CurrencyCode::Hrk),
    ("CUP", CurrencyCode::Cup),
    ("CYP", CurrencyCode::Cyp),
    ("CZK", CurrencyCode::Czk),
    ("DKK", CurrencyCode::Dkk),
    ("DJF", CurrencyCode::Djf),
    ("DOP", CurrencyCode::Dop),
    ("EGP", CurrencyCode::Egp),
    ("SVC", CurrencyCode::Svc),
    ("ERN", CurrencyCode::Ern),
    ("EEK", CurrencyCode::Eek),
    ("ETB", CurrencyCode::Etb),
    ("FKP", CurrencyCode::Fkp),
    ("FJD", CurrencyCode::Fjd),
    ("XPF", CurrencyCode::Xpf),
    ("GMD", CurrencyCode::Gmd),
    ("GEL", CurrencyCode::Gel),
    ("GHC", CurrencyCode::Ghc),
    ("GIP", CurrencyCode::Gip),
    ("GTQ", CurrencyCode::Gtq),
    ("GNF", CurrencyCode::Gnf),
    ("GWP", CurrencyCode::Gwp),
    ("GYD", CurrencyCode::Gyd),
    ("HTG", CurrencyCode::Htg),
    ("HNL", CurrencyCode::Hnl),
    ("HKD", CurrencyCode::Hkd),
    ("HUF", CurrencyCode::Huf),
    ("ISK", CurrencyCode::Isk),
    ("IDR", CurrencyCode::Idr),
    ("IRR", CurrencyCode::Irr),
    ("IQD", CurrencyCode::Iqd),
    ("ILS", CurrencyCode::Ils),
    ("JMD", CurrencyCode::Jmd),
    ("JPY", CurrencyCode::Jpy),
    ("JOD", CurrencyCode::Jod),
    ("KZT", CurrencyCode::Kzt),
    ("KES", CurrencyCode::Kes),
    ("KPW", CurrencyCode::Kpw),
    ("KRW", CurrencyCode::Krw),
    ("KWD", CurrencyCode::Kwd),
    ("KGS", CurrencyCode::Kgs),
    ("LAK", CurrencyCode::Lak),
    ("LVL", CurrencyCode::Lvl),
    ("LBP", CurrencyCode::Lbp),
    ("LSL", CurrencyCode::Lsl),
    ("ZAR", CurrencyCode::Zar),
    ("LRD", CurrencyCode::Lrd),
    ("LYD", CurrencyCode::Lyd),
    ("CHF", CurrencyCode::Chf),
    ("LTL", CurrencyCode::Ltl),
    ("MOP", CurrencyCode::Mop),
    ("MKD", CurrencyCode::Mkd),
    ("MGA", CurrencyCode::Mga),
    ("MGF", CurrencyCode::Mgf),
    ("MWK", CurrencyCode::Mwk),
    ("MYR", CurrencyCode::Myr),
    ("MVR", CurrencyCode::Mvr),
    ("MTL", CurrencyCode::Mtl),
    ("MRO", CurrencyCode::Mro),
    ("MUR", CurrencyCode::Mur),
    ("MXN", CurrencyCode::Mxn),
    ("MXV", CurrencyCode::Mxv),
    ("MDL", CurrencyCode::Mdl),
    ("MNT", CurrencyCode::Mnt),
    ("MAD", CurrencyCode::Mad),
    ("MZM", CurrencyCode::Mzm),
    ("MMK", CurrencyCode::Mmk),
    ("NAD", CurrencyCode::Nad),
    ("NPR", CurrencyCode::Npr),
    ("ANG", CurrencyCode::Ang),
    ("NIO", CurrencyCode::Nio),
    ("NGN", CurrencyCode::Ngn),
    ("OMR", CurrencyCode::Omr),
    ("PKR", CurrencyCode::Pkr),
    ("PAB", CurrencyCode::Pab),
    ("PGK", CurrencyCode::Pgk),
    ("PYG", CurrencyCode::Pyg),
    ("PEN", CurrencyCode::Pen),
    ("PHP", CurrencyCode::Php),
    ("PLN", CurrencyCode::Pln),
    ("QAR", CurrencyCode::Qar),
    ("ROL", CurrencyCode::Rol),
    ("RUB", CurrencyCode::Rub),
    ("RUR", CurrencyCode::Rur),
    ("RWF", CurrencyCode::Rwf),
    ("SHP", CurrencyCode::Shp),
    ("WST", CurrencyCode::Wst),
    ("STD", CurrencyCode::Std),
    ("SAR", CurrencyCode::Sar),
    ("CSD", CurrencyCode::Csd),
    ("SCR", CurrencyCode::Scr),
    ("SLL", CurrencyCode::Sll),
    ("SGD", CurrencyCode::Sgd),
    ("SKK", CurrencyCode::Skk),
    ("SIT", CurrencyCode::Sit),
    ("SBD", CurrencyCode::Sbd),
    ("SOS", CurrencyCode::Sos),
    ("LKR", CurrencyCode::Lkr),
    ("SDD", CurrencyCode::Sdd),
    ("SRD", CurrencyCode::Srd),
    ("SZL", CurrencyCode::Szl),
    ("SEK", CurrencyCode::Sek),
    ("SYP", CurrencyCode::Syp),
    ("TWD", CurrencyCode::Twd),
    ("TJS", CurrencyCode::Tjs),
    ("TZS", CurrencyCode::Tzs),
    ("THB", CurrencyCode::Thb),
    ("TOP", CurrencyCode::Top),
    ("TTD", CurrencyCode::Ttd),
    ("TND", CurrencyCode::Tnd),
    ("TRL", CurrencyCode::Trl),
    ("TMM", CurrencyCode::Tmm),
    ("UGX", CurrencyCode::Ugx),
    ("UAH", CurrencyCode::Uah),
    ("AED", CurrencyCode::Aed),
    ("GBP", CurrencyCode::Gbp),
    ("USN", CurrencyCode::Usn),
    ("USS", CurrencyCode::Uss),
    ("UYU", CurrencyCode::Uyu),
    ("UZS", CurrencyCode::Uzs),
    ("VUV", CurrencyCode::Vuv),
    ("VEB", CurrencyCode::Veb),
    ("VND", CurrencyCode::Vnd),
    ("YER", CurrencyCode::Yer),
    ("YDD", CurrencyCode::Ydd),
    ("YUD", CurrencyCode::Yud),
    ("ZRZ", CurrencyCode::Zrz),
    ("ZMK", CurrencyCode::Zmk),
    ("ZWD", CurrencyCode::Zwd),
    ("XAG", CurrencyCode::Xag),
    ("XAU", CurrencyCode::Xau),
    ("XBA", CurrencyCode::Xba),
    ("XBB", CurrencyCode::Xbb),
    ("XBC", CurrencyCode::Xbc),
    ("XBD", CurrencyCode::Xbd),
];

/// Lookup table indexed by the base-26 value of a 3-letter currency code
/// (`table_index`), so `CurrencyCode::from(&str)` is a single array index
/// rather than a chain of string comparisons against 181 known codes.
///
/// Built at compile time from `CURRENCY_CODE_ENTRIES` (rather than written
/// out as 17,576 literal entries, as `TYPE_CODE_TABLE` is for `TypeCode`'s
/// dense 3-digit space) since the vast majority of the 26^3 code space is
/// undefined; a sparse entries list plus a const-evaluated build keeps the
/// source readable while still compiling down to the same flat array.
static CURRENCY_CODE_TABLE: [CurrencyCode; 17576] = {
    let mut table = [CurrencyCode::Unknown; 17576];
    let mut i = 0;
    while i < CURRENCY_CODE_ENTRIES.len() {
        let (code, variant) = CURRENCY_CODE_ENTRIES[i];
        let bytes = code.as_bytes();
        table[table_index([bytes[0], bytes[1], bytes[2]])] = variant;
        i += 1;
    }
    table
};

impl From<&str> for CurrencyCode {
    /// Parses a 3-letter ISO 4217 currency code string into a [`CurrencyCode`].
    ///
    /// Returns [`CurrencyCode::Unknown`] for any code not defined in Appendix
    /// B, including any string that isn't exactly 3 uppercase ASCII letters
    /// (e.g. a blank/defaulted field, or "--").
    fn from(code: &str) -> Self {
        let bytes = code.as_bytes();
        if let [a @ b'A'..=b'Z', b @ b'A'..=b'Z', c @ b'A'..=b'Z'] = *bytes {
            CURRENCY_CODE_TABLE[table_index([a, b, c])]
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_unknown_code_yields_unknown_variant() {
        assert_eq!(CurrencyCode::from("ZZZ"), CurrencyCode::Unknown);
        assert_eq!(CurrencyCode::from(""), CurrencyCode::Unknown);
        assert_eq!(CurrencyCode::from("--"), CurrencyCode::Unknown);
        assert_eq!(CurrencyCode::from("us"), CurrencyCode::Unknown);
        assert_eq!(CurrencyCode::from("USDD"), CurrencyCode::Unknown);
        assert_eq!(CurrencyCode::Unknown.code(), "");
        assert_eq!(CurrencyCode::Unknown.decimals(), 2);
    }

    #[test]
    fn every_defined_code_round_trips_through_from_and_code() {
        assert_eq!(CurrencyCode::from("AFN"), CurrencyCode::Afn);
        assert_eq!(CurrencyCode::Afn.code(), "AFN");
        assert_eq!(CurrencyCode::from("ALL"), CurrencyCode::All);
        assert_eq!(CurrencyCode::All.code(), "ALL");
        assert_eq!(CurrencyCode::from("DZD"), CurrencyCode::Dzd);
        assert_eq!(CurrencyCode::Dzd.code(), "DZD");
        assert_eq!(CurrencyCode::from("USD"), CurrencyCode::Usd);
        assert_eq!(CurrencyCode::Usd.code(), "USD");
        assert_eq!(CurrencyCode::from("EUR"), CurrencyCode::Eur);
        assert_eq!(CurrencyCode::Eur.code(), "EUR");
        assert_eq!(CurrencyCode::from("AOA"), CurrencyCode::Aoa);
        assert_eq!(CurrencyCode::Aoa.code(), "AOA");
        assert_eq!(CurrencyCode::from("XCD"), CurrencyCode::Xcd);
        assert_eq!(CurrencyCode::Xcd.code(), "XCD");
        assert_eq!(CurrencyCode::from("ARS"), CurrencyCode::Ars);
        assert_eq!(CurrencyCode::Ars.code(), "ARS");
        assert_eq!(CurrencyCode::from("AMD"), CurrencyCode::Amd);
        assert_eq!(CurrencyCode::Amd.code(), "AMD");
        assert_eq!(CurrencyCode::from("AWG"), CurrencyCode::Awg);
        assert_eq!(CurrencyCode::Awg.code(), "AWG");
        assert_eq!(CurrencyCode::from("AUD"), CurrencyCode::Aud);
        assert_eq!(CurrencyCode::Aud.code(), "AUD");
        assert_eq!(CurrencyCode::from("AZM"), CurrencyCode::Azm);
        assert_eq!(CurrencyCode::Azm.code(), "AZM");
        assert_eq!(CurrencyCode::from("BSD"), CurrencyCode::Bsd);
        assert_eq!(CurrencyCode::Bsd.code(), "BSD");
        assert_eq!(CurrencyCode::from("BHD"), CurrencyCode::Bhd);
        assert_eq!(CurrencyCode::Bhd.code(), "BHD");
        assert_eq!(CurrencyCode::from("BDT"), CurrencyCode::Bdt);
        assert_eq!(CurrencyCode::Bdt.code(), "BDT");
        assert_eq!(CurrencyCode::from("BBD"), CurrencyCode::Bbd);
        assert_eq!(CurrencyCode::Bbd.code(), "BBD");
        assert_eq!(CurrencyCode::from("BYR"), CurrencyCode::Byr);
        assert_eq!(CurrencyCode::Byr.code(), "BYR");
        assert_eq!(CurrencyCode::from("BZD"), CurrencyCode::Bzd);
        assert_eq!(CurrencyCode::Bzd.code(), "BZD");
        assert_eq!(CurrencyCode::from("XOF"), CurrencyCode::Xof);
        assert_eq!(CurrencyCode::Xof.code(), "XOF");
        assert_eq!(CurrencyCode::from("BMD"), CurrencyCode::Bmd);
        assert_eq!(CurrencyCode::Bmd.code(), "BMD");
        assert_eq!(CurrencyCode::from("INR"), CurrencyCode::Inr);
        assert_eq!(CurrencyCode::Inr.code(), "INR");
        assert_eq!(CurrencyCode::from("BTN"), CurrencyCode::Btn);
        assert_eq!(CurrencyCode::Btn.code(), "BTN");
        assert_eq!(CurrencyCode::from("BOB"), CurrencyCode::Bob);
        assert_eq!(CurrencyCode::Bob.code(), "BOB");
        assert_eq!(CurrencyCode::from("BOV"), CurrencyCode::Bov);
        assert_eq!(CurrencyCode::Bov.code(), "BOV");
        assert_eq!(CurrencyCode::from("BAM"), CurrencyCode::Bam);
        assert_eq!(CurrencyCode::Bam.code(), "BAM");
        assert_eq!(CurrencyCode::from("BWP"), CurrencyCode::Bwp);
        assert_eq!(CurrencyCode::Bwp.code(), "BWP");
        assert_eq!(CurrencyCode::from("NOK"), CurrencyCode::Nok);
        assert_eq!(CurrencyCode::Nok.code(), "NOK");
        assert_eq!(CurrencyCode::from("BRL"), CurrencyCode::Brl);
        assert_eq!(CurrencyCode::Brl.code(), "BRL");
        assert_eq!(CurrencyCode::from("BND"), CurrencyCode::Bnd);
        assert_eq!(CurrencyCode::Bnd.code(), "BND");
        assert_eq!(CurrencyCode::from("BGN"), CurrencyCode::Bgn);
        assert_eq!(CurrencyCode::Bgn.code(), "BGN");
        assert_eq!(CurrencyCode::from("BUK"), CurrencyCode::Buk);
        assert_eq!(CurrencyCode::Buk.code(), "BUK");
        assert_eq!(CurrencyCode::from("BIF"), CurrencyCode::Bif);
        assert_eq!(CurrencyCode::Bif.code(), "BIF");
        assert_eq!(CurrencyCode::from("KHR"), CurrencyCode::Khr);
        assert_eq!(CurrencyCode::Khr.code(), "KHR");
        assert_eq!(CurrencyCode::from("XAF"), CurrencyCode::Xaf);
        assert_eq!(CurrencyCode::Xaf.code(), "XAF");
        assert_eq!(CurrencyCode::from("CAD"), CurrencyCode::Cad);
        assert_eq!(CurrencyCode::Cad.code(), "CAD");
        assert_eq!(CurrencyCode::from("CVE"), CurrencyCode::Cve);
        assert_eq!(CurrencyCode::Cve.code(), "CVE");
        assert_eq!(CurrencyCode::from("KYD"), CurrencyCode::Kyd);
        assert_eq!(CurrencyCode::Kyd.code(), "KYD");
        assert_eq!(CurrencyCode::from("CLP"), CurrencyCode::Clp);
        assert_eq!(CurrencyCode::Clp.code(), "CLP");
        assert_eq!(CurrencyCode::from("CLF"), CurrencyCode::Clf);
        assert_eq!(CurrencyCode::Clf.code(), "CLF");
        assert_eq!(CurrencyCode::from("CNY"), CurrencyCode::Cny);
        assert_eq!(CurrencyCode::Cny.code(), "CNY");
        assert_eq!(CurrencyCode::from("COP"), CurrencyCode::Cop);
        assert_eq!(CurrencyCode::Cop.code(), "COP");
        assert_eq!(CurrencyCode::from("COU"), CurrencyCode::Cou);
        assert_eq!(CurrencyCode::Cou.code(), "COU");
        assert_eq!(CurrencyCode::from("KMF"), CurrencyCode::Kmf);
        assert_eq!(CurrencyCode::Kmf.code(), "KMF");
        assert_eq!(CurrencyCode::from("CDF"), CurrencyCode::Cdf);
        assert_eq!(CurrencyCode::Cdf.code(), "CDF");
        assert_eq!(CurrencyCode::from("NZD"), CurrencyCode::Nzd);
        assert_eq!(CurrencyCode::Nzd.code(), "NZD");
        assert_eq!(CurrencyCode::from("CRC"), CurrencyCode::Crc);
        assert_eq!(CurrencyCode::Crc.code(), "CRC");
        assert_eq!(CurrencyCode::from("HRK"), CurrencyCode::Hrk);
        assert_eq!(CurrencyCode::Hrk.code(), "HRK");
        assert_eq!(CurrencyCode::from("CUP"), CurrencyCode::Cup);
        assert_eq!(CurrencyCode::Cup.code(), "CUP");
        assert_eq!(CurrencyCode::from("CYP"), CurrencyCode::Cyp);
        assert_eq!(CurrencyCode::Cyp.code(), "CYP");
        assert_eq!(CurrencyCode::from("CZK"), CurrencyCode::Czk);
        assert_eq!(CurrencyCode::Czk.code(), "CZK");
        assert_eq!(CurrencyCode::from("DKK"), CurrencyCode::Dkk);
        assert_eq!(CurrencyCode::Dkk.code(), "DKK");
        assert_eq!(CurrencyCode::from("DJF"), CurrencyCode::Djf);
        assert_eq!(CurrencyCode::Djf.code(), "DJF");
        assert_eq!(CurrencyCode::from("DOP"), CurrencyCode::Dop);
        assert_eq!(CurrencyCode::Dop.code(), "DOP");
        assert_eq!(CurrencyCode::from("EGP"), CurrencyCode::Egp);
        assert_eq!(CurrencyCode::Egp.code(), "EGP");
        assert_eq!(CurrencyCode::from("SVC"), CurrencyCode::Svc);
        assert_eq!(CurrencyCode::Svc.code(), "SVC");
        assert_eq!(CurrencyCode::from("ERN"), CurrencyCode::Ern);
        assert_eq!(CurrencyCode::Ern.code(), "ERN");
        assert_eq!(CurrencyCode::from("EEK"), CurrencyCode::Eek);
        assert_eq!(CurrencyCode::Eek.code(), "EEK");
        assert_eq!(CurrencyCode::from("ETB"), CurrencyCode::Etb);
        assert_eq!(CurrencyCode::Etb.code(), "ETB");
        assert_eq!(CurrencyCode::from("FKP"), CurrencyCode::Fkp);
        assert_eq!(CurrencyCode::Fkp.code(), "FKP");
        assert_eq!(CurrencyCode::from("FJD"), CurrencyCode::Fjd);
        assert_eq!(CurrencyCode::Fjd.code(), "FJD");
        assert_eq!(CurrencyCode::from("XPF"), CurrencyCode::Xpf);
        assert_eq!(CurrencyCode::Xpf.code(), "XPF");
        assert_eq!(CurrencyCode::from("GMD"), CurrencyCode::Gmd);
        assert_eq!(CurrencyCode::Gmd.code(), "GMD");
        assert_eq!(CurrencyCode::from("GEL"), CurrencyCode::Gel);
        assert_eq!(CurrencyCode::Gel.code(), "GEL");
        assert_eq!(CurrencyCode::from("GHC"), CurrencyCode::Ghc);
        assert_eq!(CurrencyCode::Ghc.code(), "GHC");
        assert_eq!(CurrencyCode::from("GIP"), CurrencyCode::Gip);
        assert_eq!(CurrencyCode::Gip.code(), "GIP");
        assert_eq!(CurrencyCode::from("GTQ"), CurrencyCode::Gtq);
        assert_eq!(CurrencyCode::Gtq.code(), "GTQ");
        assert_eq!(CurrencyCode::from("GNF"), CurrencyCode::Gnf);
        assert_eq!(CurrencyCode::Gnf.code(), "GNF");
        assert_eq!(CurrencyCode::from("GWP"), CurrencyCode::Gwp);
        assert_eq!(CurrencyCode::Gwp.code(), "GWP");
        assert_eq!(CurrencyCode::from("GYD"), CurrencyCode::Gyd);
        assert_eq!(CurrencyCode::Gyd.code(), "GYD");
        assert_eq!(CurrencyCode::from("HTG"), CurrencyCode::Htg);
        assert_eq!(CurrencyCode::Htg.code(), "HTG");
        assert_eq!(CurrencyCode::from("HNL"), CurrencyCode::Hnl);
        assert_eq!(CurrencyCode::Hnl.code(), "HNL");
        assert_eq!(CurrencyCode::from("HKD"), CurrencyCode::Hkd);
        assert_eq!(CurrencyCode::Hkd.code(), "HKD");
        assert_eq!(CurrencyCode::from("HUF"), CurrencyCode::Huf);
        assert_eq!(CurrencyCode::Huf.code(), "HUF");
        assert_eq!(CurrencyCode::from("ISK"), CurrencyCode::Isk);
        assert_eq!(CurrencyCode::Isk.code(), "ISK");
        assert_eq!(CurrencyCode::from("IDR"), CurrencyCode::Idr);
        assert_eq!(CurrencyCode::Idr.code(), "IDR");
        assert_eq!(CurrencyCode::from("IRR"), CurrencyCode::Irr);
        assert_eq!(CurrencyCode::Irr.code(), "IRR");
        assert_eq!(CurrencyCode::from("IQD"), CurrencyCode::Iqd);
        assert_eq!(CurrencyCode::Iqd.code(), "IQD");
        assert_eq!(CurrencyCode::from("ILS"), CurrencyCode::Ils);
        assert_eq!(CurrencyCode::Ils.code(), "ILS");
        assert_eq!(CurrencyCode::from("JMD"), CurrencyCode::Jmd);
        assert_eq!(CurrencyCode::Jmd.code(), "JMD");
        assert_eq!(CurrencyCode::from("JPY"), CurrencyCode::Jpy);
        assert_eq!(CurrencyCode::Jpy.code(), "JPY");
        assert_eq!(CurrencyCode::from("JOD"), CurrencyCode::Jod);
        assert_eq!(CurrencyCode::Jod.code(), "JOD");
        assert_eq!(CurrencyCode::from("KZT"), CurrencyCode::Kzt);
        assert_eq!(CurrencyCode::Kzt.code(), "KZT");
        assert_eq!(CurrencyCode::from("KES"), CurrencyCode::Kes);
        assert_eq!(CurrencyCode::Kes.code(), "KES");
        assert_eq!(CurrencyCode::from("KPW"), CurrencyCode::Kpw);
        assert_eq!(CurrencyCode::Kpw.code(), "KPW");
        assert_eq!(CurrencyCode::from("KRW"), CurrencyCode::Krw);
        assert_eq!(CurrencyCode::Krw.code(), "KRW");
        assert_eq!(CurrencyCode::from("KWD"), CurrencyCode::Kwd);
        assert_eq!(CurrencyCode::Kwd.code(), "KWD");
        assert_eq!(CurrencyCode::from("KGS"), CurrencyCode::Kgs);
        assert_eq!(CurrencyCode::Kgs.code(), "KGS");
        assert_eq!(CurrencyCode::from("LAK"), CurrencyCode::Lak);
        assert_eq!(CurrencyCode::Lak.code(), "LAK");
        assert_eq!(CurrencyCode::from("LVL"), CurrencyCode::Lvl);
        assert_eq!(CurrencyCode::Lvl.code(), "LVL");
        assert_eq!(CurrencyCode::from("LBP"), CurrencyCode::Lbp);
        assert_eq!(CurrencyCode::Lbp.code(), "LBP");
        assert_eq!(CurrencyCode::from("LSL"), CurrencyCode::Lsl);
        assert_eq!(CurrencyCode::Lsl.code(), "LSL");
        assert_eq!(CurrencyCode::from("ZAR"), CurrencyCode::Zar);
        assert_eq!(CurrencyCode::Zar.code(), "ZAR");
        assert_eq!(CurrencyCode::from("LRD"), CurrencyCode::Lrd);
        assert_eq!(CurrencyCode::Lrd.code(), "LRD");
        assert_eq!(CurrencyCode::from("LYD"), CurrencyCode::Lyd);
        assert_eq!(CurrencyCode::Lyd.code(), "LYD");
        assert_eq!(CurrencyCode::from("CHF"), CurrencyCode::Chf);
        assert_eq!(CurrencyCode::Chf.code(), "CHF");
        assert_eq!(CurrencyCode::from("LTL"), CurrencyCode::Ltl);
        assert_eq!(CurrencyCode::Ltl.code(), "LTL");
        assert_eq!(CurrencyCode::from("MOP"), CurrencyCode::Mop);
        assert_eq!(CurrencyCode::Mop.code(), "MOP");
        assert_eq!(CurrencyCode::from("MKD"), CurrencyCode::Mkd);
        assert_eq!(CurrencyCode::Mkd.code(), "MKD");
        assert_eq!(CurrencyCode::from("MGA"), CurrencyCode::Mga);
        assert_eq!(CurrencyCode::Mga.code(), "MGA");
        assert_eq!(CurrencyCode::from("MGF"), CurrencyCode::Mgf);
        assert_eq!(CurrencyCode::Mgf.code(), "MGF");
        assert_eq!(CurrencyCode::from("MWK"), CurrencyCode::Mwk);
        assert_eq!(CurrencyCode::Mwk.code(), "MWK");
        assert_eq!(CurrencyCode::from("MYR"), CurrencyCode::Myr);
        assert_eq!(CurrencyCode::Myr.code(), "MYR");
        assert_eq!(CurrencyCode::from("MVR"), CurrencyCode::Mvr);
        assert_eq!(CurrencyCode::Mvr.code(), "MVR");
        assert_eq!(CurrencyCode::from("MTL"), CurrencyCode::Mtl);
        assert_eq!(CurrencyCode::Mtl.code(), "MTL");
        assert_eq!(CurrencyCode::from("MRO"), CurrencyCode::Mro);
        assert_eq!(CurrencyCode::Mro.code(), "MRO");
        assert_eq!(CurrencyCode::from("MUR"), CurrencyCode::Mur);
        assert_eq!(CurrencyCode::Mur.code(), "MUR");
        assert_eq!(CurrencyCode::from("MXN"), CurrencyCode::Mxn);
        assert_eq!(CurrencyCode::Mxn.code(), "MXN");
        assert_eq!(CurrencyCode::from("MXV"), CurrencyCode::Mxv);
        assert_eq!(CurrencyCode::Mxv.code(), "MXV");
        assert_eq!(CurrencyCode::from("MDL"), CurrencyCode::Mdl);
        assert_eq!(CurrencyCode::Mdl.code(), "MDL");
        assert_eq!(CurrencyCode::from("MNT"), CurrencyCode::Mnt);
        assert_eq!(CurrencyCode::Mnt.code(), "MNT");
        assert_eq!(CurrencyCode::from("MAD"), CurrencyCode::Mad);
        assert_eq!(CurrencyCode::Mad.code(), "MAD");
        assert_eq!(CurrencyCode::from("MZM"), CurrencyCode::Mzm);
        assert_eq!(CurrencyCode::Mzm.code(), "MZM");
        assert_eq!(CurrencyCode::from("MMK"), CurrencyCode::Mmk);
        assert_eq!(CurrencyCode::Mmk.code(), "MMK");
        assert_eq!(CurrencyCode::from("NAD"), CurrencyCode::Nad);
        assert_eq!(CurrencyCode::Nad.code(), "NAD");
        assert_eq!(CurrencyCode::from("NPR"), CurrencyCode::Npr);
        assert_eq!(CurrencyCode::Npr.code(), "NPR");
        assert_eq!(CurrencyCode::from("ANG"), CurrencyCode::Ang);
        assert_eq!(CurrencyCode::Ang.code(), "ANG");
        assert_eq!(CurrencyCode::from("NIO"), CurrencyCode::Nio);
        assert_eq!(CurrencyCode::Nio.code(), "NIO");
        assert_eq!(CurrencyCode::from("NGN"), CurrencyCode::Ngn);
        assert_eq!(CurrencyCode::Ngn.code(), "NGN");
        assert_eq!(CurrencyCode::from("OMR"), CurrencyCode::Omr);
        assert_eq!(CurrencyCode::Omr.code(), "OMR");
        assert_eq!(CurrencyCode::from("PKR"), CurrencyCode::Pkr);
        assert_eq!(CurrencyCode::Pkr.code(), "PKR");
        assert_eq!(CurrencyCode::from("PAB"), CurrencyCode::Pab);
        assert_eq!(CurrencyCode::Pab.code(), "PAB");
        assert_eq!(CurrencyCode::from("PGK"), CurrencyCode::Pgk);
        assert_eq!(CurrencyCode::Pgk.code(), "PGK");
        assert_eq!(CurrencyCode::from("PYG"), CurrencyCode::Pyg);
        assert_eq!(CurrencyCode::Pyg.code(), "PYG");
        assert_eq!(CurrencyCode::from("PEN"), CurrencyCode::Pen);
        assert_eq!(CurrencyCode::Pen.code(), "PEN");
        assert_eq!(CurrencyCode::from("PHP"), CurrencyCode::Php);
        assert_eq!(CurrencyCode::Php.code(), "PHP");
        assert_eq!(CurrencyCode::from("PLN"), CurrencyCode::Pln);
        assert_eq!(CurrencyCode::Pln.code(), "PLN");
        assert_eq!(CurrencyCode::from("QAR"), CurrencyCode::Qar);
        assert_eq!(CurrencyCode::Qar.code(), "QAR");
        assert_eq!(CurrencyCode::from("ROL"), CurrencyCode::Rol);
        assert_eq!(CurrencyCode::Rol.code(), "ROL");
        assert_eq!(CurrencyCode::from("RUB"), CurrencyCode::Rub);
        assert_eq!(CurrencyCode::Rub.code(), "RUB");
        assert_eq!(CurrencyCode::from("RUR"), CurrencyCode::Rur);
        assert_eq!(CurrencyCode::Rur.code(), "RUR");
        assert_eq!(CurrencyCode::from("RWF"), CurrencyCode::Rwf);
        assert_eq!(CurrencyCode::Rwf.code(), "RWF");
        assert_eq!(CurrencyCode::from("SHP"), CurrencyCode::Shp);
        assert_eq!(CurrencyCode::Shp.code(), "SHP");
        assert_eq!(CurrencyCode::from("WST"), CurrencyCode::Wst);
        assert_eq!(CurrencyCode::Wst.code(), "WST");
        assert_eq!(CurrencyCode::from("STD"), CurrencyCode::Std);
        assert_eq!(CurrencyCode::Std.code(), "STD");
        assert_eq!(CurrencyCode::from("SAR"), CurrencyCode::Sar);
        assert_eq!(CurrencyCode::Sar.code(), "SAR");
        assert_eq!(CurrencyCode::from("CSD"), CurrencyCode::Csd);
        assert_eq!(CurrencyCode::Csd.code(), "CSD");
        assert_eq!(CurrencyCode::from("SCR"), CurrencyCode::Scr);
        assert_eq!(CurrencyCode::Scr.code(), "SCR");
        assert_eq!(CurrencyCode::from("SLL"), CurrencyCode::Sll);
        assert_eq!(CurrencyCode::Sll.code(), "SLL");
        assert_eq!(CurrencyCode::from("SGD"), CurrencyCode::Sgd);
        assert_eq!(CurrencyCode::Sgd.code(), "SGD");
        assert_eq!(CurrencyCode::from("SKK"), CurrencyCode::Skk);
        assert_eq!(CurrencyCode::Skk.code(), "SKK");
        assert_eq!(CurrencyCode::from("SIT"), CurrencyCode::Sit);
        assert_eq!(CurrencyCode::Sit.code(), "SIT");
        assert_eq!(CurrencyCode::from("SBD"), CurrencyCode::Sbd);
        assert_eq!(CurrencyCode::Sbd.code(), "SBD");
        assert_eq!(CurrencyCode::from("SOS"), CurrencyCode::Sos);
        assert_eq!(CurrencyCode::Sos.code(), "SOS");
        assert_eq!(CurrencyCode::from("LKR"), CurrencyCode::Lkr);
        assert_eq!(CurrencyCode::Lkr.code(), "LKR");
        assert_eq!(CurrencyCode::from("SDD"), CurrencyCode::Sdd);
        assert_eq!(CurrencyCode::Sdd.code(), "SDD");
        assert_eq!(CurrencyCode::from("SRD"), CurrencyCode::Srd);
        assert_eq!(CurrencyCode::Srd.code(), "SRD");
        assert_eq!(CurrencyCode::from("SZL"), CurrencyCode::Szl);
        assert_eq!(CurrencyCode::Szl.code(), "SZL");
        assert_eq!(CurrencyCode::from("SEK"), CurrencyCode::Sek);
        assert_eq!(CurrencyCode::Sek.code(), "SEK");
        assert_eq!(CurrencyCode::from("SYP"), CurrencyCode::Syp);
        assert_eq!(CurrencyCode::Syp.code(), "SYP");
        assert_eq!(CurrencyCode::from("TWD"), CurrencyCode::Twd);
        assert_eq!(CurrencyCode::Twd.code(), "TWD");
        assert_eq!(CurrencyCode::from("TJS"), CurrencyCode::Tjs);
        assert_eq!(CurrencyCode::Tjs.code(), "TJS");
        assert_eq!(CurrencyCode::from("TZS"), CurrencyCode::Tzs);
        assert_eq!(CurrencyCode::Tzs.code(), "TZS");
        assert_eq!(CurrencyCode::from("THB"), CurrencyCode::Thb);
        assert_eq!(CurrencyCode::Thb.code(), "THB");
        assert_eq!(CurrencyCode::from("TOP"), CurrencyCode::Top);
        assert_eq!(CurrencyCode::Top.code(), "TOP");
        assert_eq!(CurrencyCode::from("TTD"), CurrencyCode::Ttd);
        assert_eq!(CurrencyCode::Ttd.code(), "TTD");
        assert_eq!(CurrencyCode::from("TND"), CurrencyCode::Tnd);
        assert_eq!(CurrencyCode::Tnd.code(), "TND");
        assert_eq!(CurrencyCode::from("TRL"), CurrencyCode::Trl);
        assert_eq!(CurrencyCode::Trl.code(), "TRL");
        assert_eq!(CurrencyCode::from("TMM"), CurrencyCode::Tmm);
        assert_eq!(CurrencyCode::Tmm.code(), "TMM");
        assert_eq!(CurrencyCode::from("UGX"), CurrencyCode::Ugx);
        assert_eq!(CurrencyCode::Ugx.code(), "UGX");
        assert_eq!(CurrencyCode::from("UAH"), CurrencyCode::Uah);
        assert_eq!(CurrencyCode::Uah.code(), "UAH");
        assert_eq!(CurrencyCode::from("AED"), CurrencyCode::Aed);
        assert_eq!(CurrencyCode::Aed.code(), "AED");
        assert_eq!(CurrencyCode::from("GBP"), CurrencyCode::Gbp);
        assert_eq!(CurrencyCode::Gbp.code(), "GBP");
        assert_eq!(CurrencyCode::from("USN"), CurrencyCode::Usn);
        assert_eq!(CurrencyCode::Usn.code(), "USN");
        assert_eq!(CurrencyCode::from("USS"), CurrencyCode::Uss);
        assert_eq!(CurrencyCode::Uss.code(), "USS");
        assert_eq!(CurrencyCode::from("UYU"), CurrencyCode::Uyu);
        assert_eq!(CurrencyCode::Uyu.code(), "UYU");
        assert_eq!(CurrencyCode::from("UZS"), CurrencyCode::Uzs);
        assert_eq!(CurrencyCode::Uzs.code(), "UZS");
        assert_eq!(CurrencyCode::from("VUV"), CurrencyCode::Vuv);
        assert_eq!(CurrencyCode::Vuv.code(), "VUV");
        assert_eq!(CurrencyCode::from("VEB"), CurrencyCode::Veb);
        assert_eq!(CurrencyCode::Veb.code(), "VEB");
        assert_eq!(CurrencyCode::from("VND"), CurrencyCode::Vnd);
        assert_eq!(CurrencyCode::Vnd.code(), "VND");
        assert_eq!(CurrencyCode::from("YER"), CurrencyCode::Yer);
        assert_eq!(CurrencyCode::Yer.code(), "YER");
        assert_eq!(CurrencyCode::from("YDD"), CurrencyCode::Ydd);
        assert_eq!(CurrencyCode::Ydd.code(), "YDD");
        assert_eq!(CurrencyCode::from("YUD"), CurrencyCode::Yud);
        assert_eq!(CurrencyCode::Yud.code(), "YUD");
        assert_eq!(CurrencyCode::from("ZRZ"), CurrencyCode::Zrz);
        assert_eq!(CurrencyCode::Zrz.code(), "ZRZ");
        assert_eq!(CurrencyCode::from("ZMK"), CurrencyCode::Zmk);
        assert_eq!(CurrencyCode::Zmk.code(), "ZMK");
        assert_eq!(CurrencyCode::from("ZWD"), CurrencyCode::Zwd);
        assert_eq!(CurrencyCode::Zwd.code(), "ZWD");
        assert_eq!(CurrencyCode::from("XAG"), CurrencyCode::Xag);
        assert_eq!(CurrencyCode::Xag.code(), "XAG");
        assert_eq!(CurrencyCode::from("XAU"), CurrencyCode::Xau);
        assert_eq!(CurrencyCode::Xau.code(), "XAU");
        assert_eq!(CurrencyCode::from("XBA"), CurrencyCode::Xba);
        assert_eq!(CurrencyCode::Xba.code(), "XBA");
        assert_eq!(CurrencyCode::from("XBB"), CurrencyCode::Xbb);
        assert_eq!(CurrencyCode::Xbb.code(), "XBB");
        assert_eq!(CurrencyCode::from("XBC"), CurrencyCode::Xbc);
        assert_eq!(CurrencyCode::Xbc.code(), "XBC");
        assert_eq!(CurrencyCode::from("XBD"), CurrencyCode::Xbd);
        assert_eq!(CurrencyCode::Xbd.code(), "XBD");
    }

    #[test]
    fn decimal_exceptions_are_correct() {
        assert_eq!(CurrencyCode::Jpy.decimals(), 0);
        assert_eq!(CurrencyCode::Xof.decimals(), 0);
        assert_eq!(CurrencyCode::Xaf.decimals(), 0);
        assert_eq!(CurrencyCode::Kmf.decimals(), 0);
        assert_eq!(CurrencyCode::Xpf.decimals(), 0);
        assert_eq!(CurrencyCode::Mro.decimals(), 1);
        assert_eq!(CurrencyCode::Bhd.decimals(), 3);
        assert_eq!(CurrencyCode::Kwd.decimals(), 3);
        assert_eq!(CurrencyCode::Tnd.decimals(), 3);
        // Overridden from the spec's own (stale, pre-Euro) 0-decimal listing.
        assert_eq!(CurrencyCode::Eur.decimals(), 2);
        assert_eq!(CurrencyCode::Brl.decimals(), 2);
        // Not listed in any exception section.
        assert_eq!(CurrencyCode::Usd.decimals(), 2);
        assert_eq!(CurrencyCode::Gbp.decimals(), 2);
    }
}

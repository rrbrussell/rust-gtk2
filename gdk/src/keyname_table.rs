//! Hello, the keyname table is used for translating between keycode and key names.

/// This is an entry in the Keyname Table.
pub struct KeynameTableEntry<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub keycode: u32,
}

/// The Table of known keynames.
pub struct KeynameTable<'a> {
    table: Vec<KeynameTableEntry<'a>>,
}

impl KeynameTable<'_> {
    /// [`keyvalue_to_name`] converts a d key value or keycode into a human readable name.
    /// Returns None if the keycode was not found.
    pub fn keyvalue_to_name(&self, value: u32) -> Option<String> {
        for kv in self.table.iter() {
            if kv.keycode == value {
                return Some(String::from(kv.name));
            }
        }
        return None;
    }

    pub fn keyvalue_from_name(&self, name: &str) -> Option<u32> {
        for kv in self.table.iter() {
            if kv.name == name {
                return Some(kv.keycode);
            }
        }
        return None;
    }

    /// This must be called before any use of the Keyname Table.
    /// Save the returned table in a cache somewhere for use.
    pub fn new() -> KeynameTable<'static> {
        return KeynameTable {
            table: vec![
                KeynameTableEntry {
                    identifier: "KEY_space",
                    name: "space",
                    keycode: 32,
                },
                KeynameTableEntry {
                    identifier: "KEY_exclam",
                    name: "exclam",
                    keycode: 33,
                },
                KeynameTableEntry {
                    identifier: "KEY_quotedbl",
                    name: "quotedbl",
                    keycode: 34,
                },
                KeynameTableEntry {
                    identifier: "KEY_numbersign",
                    name: "numbersign",
                    keycode: 35,
                },
                KeynameTableEntry {
                    identifier: "KEY_dollar",
                    name: "dollar",
                    keycode: 36,
                },
                KeynameTableEntry {
                    identifier: "KEY_percent",
                    name: "percent",
                    keycode: 37,
                },
                KeynameTableEntry {
                    identifier: "KEY_ampersand",
                    name: "ampersand",
                    keycode: 38,
                },
                KeynameTableEntry {
                    identifier: "KEY_apostrophe",
                    name: "apostrophe",
                    keycode: 39,
                },
                KeynameTableEntry {
                    identifier: "KEY_quoteright",
                    name: "quoteright",
                    keycode: 39,
                },
                KeynameTableEntry {
                    identifier: "KEY_parenleft",
                    name: "parenleft",
                    keycode: 40,
                },
                KeynameTableEntry {
                    identifier: "KEY_parenright",
                    name: "parenright",
                    keycode: 41,
                },
                KeynameTableEntry {
                    identifier: "KEY_asterisk",
                    name: "asterisk",
                    keycode: 42,
                },
                KeynameTableEntry {
                    identifier: "KEY_plus",
                    name: "plus",
                    keycode: 43,
                },
                KeynameTableEntry {
                    identifier: "KEY_comma",
                    name: "comma",
                    keycode: 44,
                },
                KeynameTableEntry {
                    identifier: "KEY_minus",
                    name: "minus",
                    keycode: 45,
                },
                KeynameTableEntry {
                    identifier: "KEY_period",
                    name: "period",
                    keycode: 46,
                },
                KeynameTableEntry {
                    identifier: "KEY_slash",
                    name: "slash",
                    keycode: 47,
                },
                KeynameTableEntry {
                    identifier: "KEY_0",
                    name: "0",
                    keycode: 48,
                },
                KeynameTableEntry {
                    identifier: "KEY_1",
                    name: "1",
                    keycode: 49,
                },
                KeynameTableEntry {
                    identifier: "KEY_2",
                    name: "2",
                    keycode: 50,
                },
                KeynameTableEntry {
                    identifier: "KEY_3",
                    name: "3",
                    keycode: 51,
                },
                KeynameTableEntry {
                    identifier: "KEY_4",
                    name: "4",
                    keycode: 52,
                },
                KeynameTableEntry {
                    identifier: "KEY_5",
                    name: "5",
                    keycode: 53,
                },
                KeynameTableEntry {
                    identifier: "KEY_6",
                    name: "6",
                    keycode: 54,
                },
                KeynameTableEntry {
                    identifier: "KEY_7",
                    name: "7",
                    keycode: 55,
                },
                KeynameTableEntry {
                    identifier: "KEY_8",
                    name: "8",
                    keycode: 56,
                },
                KeynameTableEntry {
                    identifier: "KEY_9",
                    name: "9",
                    keycode: 57,
                },
                KeynameTableEntry {
                    identifier: "KEY_colon",
                    name: "colon",
                    keycode: 58,
                },
                KeynameTableEntry {
                    identifier: "KEY_semicolon",
                    name: "semicolon",
                    keycode: 59,
                },
                KeynameTableEntry {
                    identifier: "KEY_less",
                    name: "less",
                    keycode: 60,
                },
                KeynameTableEntry {
                    identifier: "KEY_equal",
                    name: "equal",
                    keycode: 61,
                },
                KeynameTableEntry {
                    identifier: "KEY_greater",
                    name: "greater",
                    keycode: 62,
                },
                KeynameTableEntry {
                    identifier: "KEY_question",
                    name: "question",
                    keycode: 63,
                },
                KeynameTableEntry {
                    identifier: "KEY_at",
                    name: "at",
                    keycode: 64,
                },
                KeynameTableEntry {
                    identifier: "KEY_A",
                    name: "A",
                    keycode: 65,
                },
                KeynameTableEntry {
                    identifier: "KEY_B",
                    name: "B",
                    keycode: 66,
                },
                KeynameTableEntry {
                    identifier: "KEY_C",
                    name: "C",
                    keycode: 67,
                },
                KeynameTableEntry {
                    identifier: "KEY_D",
                    name: "D",
                    keycode: 68,
                },
                KeynameTableEntry {
                    identifier: "KEY_E",
                    name: "E",
                    keycode: 69,
                },
                KeynameTableEntry {
                    identifier: "KEY_F",
                    name: "F",
                    keycode: 70,
                },
                KeynameTableEntry {
                    identifier: "KEY_G",
                    name: "G",
                    keycode: 71,
                },
                KeynameTableEntry {
                    identifier: "KEY_H",
                    name: "H",
                    keycode: 72,
                },
                KeynameTableEntry {
                    identifier: "KEY_I",
                    name: "I",
                    keycode: 73,
                },
                KeynameTableEntry {
                    identifier: "KEY_J",
                    name: "J",
                    keycode: 74,
                },
                KeynameTableEntry {
                    identifier: "KEY_K",
                    name: "K",
                    keycode: 75,
                },
                KeynameTableEntry {
                    identifier: "KEY_L",
                    name: "L",
                    keycode: 76,
                },
                KeynameTableEntry {
                    identifier: "KEY_M",
                    name: "M",
                    keycode: 77,
                },
                KeynameTableEntry {
                    identifier: "KEY_N",
                    name: "N",
                    keycode: 78,
                },
                KeynameTableEntry {
                    identifier: "KEY_O",
                    name: "O",
                    keycode: 79,
                },
                KeynameTableEntry {
                    identifier: "KEY_P",
                    name: "P",
                    keycode: 80,
                },
                KeynameTableEntry {
                    identifier: "KEY_Q",
                    name: "Q",
                    keycode: 81,
                },
                KeynameTableEntry {
                    identifier: "KEY_R",
                    name: "R",
                    keycode: 82,
                },
                KeynameTableEntry {
                    identifier: "KEY_S",
                    name: "S",
                    keycode: 83,
                },
                KeynameTableEntry {
                    identifier: "KEY_T",
                    name: "T",
                    keycode: 84,
                },
                KeynameTableEntry {
                    identifier: "KEY_U",
                    name: "U",
                    keycode: 85,
                },
                KeynameTableEntry {
                    identifier: "KEY_V",
                    name: "V",
                    keycode: 86,
                },
                KeynameTableEntry {
                    identifier: "KEY_W",
                    name: "W",
                    keycode: 87,
                },
                KeynameTableEntry {
                    identifier: "KEY_X",
                    name: "X",
                    keycode: 88,
                },
                KeynameTableEntry {
                    identifier: "KEY_Y",
                    name: "Y",
                    keycode: 89,
                },
                KeynameTableEntry {
                    identifier: "KEY_Z",
                    name: "Z",
                    keycode: 90,
                },
                KeynameTableEntry {
                    identifier: "KEY_bracketleft",
                    name: "bracketleft",
                    keycode: 91,
                },
                KeynameTableEntry {
                    identifier: "KEY_backslash",
                    name: "backslash",
                    keycode: 92,
                },
                KeynameTableEntry {
                    identifier: "KEY_bracketright",
                    name: "bracketright",
                    keycode: 93,
                },
                KeynameTableEntry {
                    identifier: "KEY_asciicircum",
                    name: "asciicircum",
                    keycode: 94,
                },
                KeynameTableEntry {
                    identifier: "KEY_underscore",
                    name: "underscore",
                    keycode: 95,
                },
                KeynameTableEntry {
                    identifier: "KEY_grave",
                    name: "grave",
                    keycode: 96,
                },
                KeynameTableEntry {
                    identifier: "KEY_quoteleft",
                    name: "quoteleft",
                    keycode: 96,
                },
                KeynameTableEntry {
                    identifier: "KEY_a",
                    name: "a",
                    keycode: 97,
                },
                KeynameTableEntry {
                    identifier: "KEY_b",
                    name: "b",
                    keycode: 98,
                },
                KeynameTableEntry {
                    identifier: "KEY_c",
                    name: "c",
                    keycode: 99,
                },
                KeynameTableEntry {
                    identifier: "KEY_d",
                    name: "d",
                    keycode: 100,
                },
                KeynameTableEntry {
                    identifier: "KEY_e",
                    name: "e",
                    keycode: 101,
                },
                KeynameTableEntry {
                    identifier: "KEY_f",
                    name: "f",
                    keycode: 102,
                },
                KeynameTableEntry {
                    identifier: "KEY_g",
                    name: "g",
                    keycode: 103,
                },
                KeynameTableEntry {
                    identifier: "KEY_h",
                    name: "h",
                    keycode: 104,
                },
                KeynameTableEntry {
                    identifier: "KEY_i",
                    name: "i",
                    keycode: 105,
                },
                KeynameTableEntry {
                    identifier: "KEY_j",
                    name: "j",
                    keycode: 106,
                },
                KeynameTableEntry {
                    identifier: "KEY_k",
                    name: "k",
                    keycode: 107,
                },
                KeynameTableEntry {
                    identifier: "KEY_l",
                    name: "l",
                    keycode: 108,
                },
                KeynameTableEntry {
                    identifier: "KEY_m",
                    name: "m",
                    keycode: 109,
                },
                KeynameTableEntry {
                    identifier: "KEY_n",
                    name: "n",
                    keycode: 110,
                },
                KeynameTableEntry {
                    identifier: "KEY_o",
                    name: "o",
                    keycode: 111,
                },
                KeynameTableEntry {
                    identifier: "KEY_p",
                    name: "p",
                    keycode: 112,
                },
                KeynameTableEntry {
                    identifier: "KEY_q",
                    name: "q",
                    keycode: 113,
                },
                KeynameTableEntry {
                    identifier: "KEY_r",
                    name: "r",
                    keycode: 114,
                },
                KeynameTableEntry {
                    identifier: "KEY_s",
                    name: "s",
                    keycode: 115,
                },
                KeynameTableEntry {
                    identifier: "KEY_t",
                    name: "t",
                    keycode: 116,
                },
                KeynameTableEntry {
                    identifier: "KEY_u",
                    name: "u",
                    keycode: 117,
                },
                KeynameTableEntry {
                    identifier: "KEY_v",
                    name: "v",
                    keycode: 118,
                },
                KeynameTableEntry {
                    identifier: "KEY_w",
                    name: "w",
                    keycode: 119,
                },
                KeynameTableEntry {
                    identifier: "KEY_x",
                    name: "x",
                    keycode: 120,
                },
                KeynameTableEntry {
                    identifier: "KEY_y",
                    name: "y",
                    keycode: 121,
                },
                KeynameTableEntry {
                    identifier: "KEY_z",
                    name: "z",
                    keycode: 122,
                },
                KeynameTableEntry {
                    identifier: "KEY_braceleft",
                    name: "braceleft",
                    keycode: 123,
                },
                KeynameTableEntry {
                    identifier: "KEY_bar",
                    name: "bar",
                    keycode: 124,
                },
                KeynameTableEntry {
                    identifier: "KEY_braceright",
                    name: "braceright",
                    keycode: 125,
                },
                KeynameTableEntry {
                    identifier: "KEY_asciitilde",
                    name: "asciitilde",
                    keycode: 126,
                },
                KeynameTableEntry {
                    identifier: "KEY_nobreakspace",
                    name: "nobreakspace",
                    keycode: 160,
                },
                KeynameTableEntry {
                    identifier: "KEY_exclamdown",
                    name: "exclamdown",
                    keycode: 161,
                },
                KeynameTableEntry {
                    identifier: "KEY_cent",
                    name: "cent",
                    keycode: 162,
                },
                KeynameTableEntry {
                    identifier: "KEY_sterling",
                    name: "sterling",
                    keycode: 163,
                },
                KeynameTableEntry {
                    identifier: "KEY_currency",
                    name: "currency",
                    keycode: 164,
                },
                KeynameTableEntry {
                    identifier: "KEY_yen",
                    name: "yen",
                    keycode: 165,
                },
                KeynameTableEntry {
                    identifier: "KEY_brokenbar",
                    name: "brokenbar",
                    keycode: 166,
                },
                KeynameTableEntry {
                    identifier: "KEY_section",
                    name: "section",
                    keycode: 167,
                },
                KeynameTableEntry {
                    identifier: "KEY_diaeresis",
                    name: "diaeresis",
                    keycode: 168,
                },
                KeynameTableEntry {
                    identifier: "KEY_copyright",
                    name: "copyright",
                    keycode: 169,
                },
                KeynameTableEntry {
                    identifier: "KEY_ordfeminine",
                    name: "ordfeminine",
                    keycode: 170,
                },
                KeynameTableEntry {
                    identifier: "KEY_guillemotleft",
                    name: "guillemotleft",
                    keycode: 171,
                },
                KeynameTableEntry {
                    identifier: "KEY_notsign",
                    name: "notsign",
                    keycode: 172,
                },
                KeynameTableEntry {
                    identifier: "KEY_hyphen",
                    name: "hyphen",
                    keycode: 173,
                },
                KeynameTableEntry {
                    identifier: "KEY_registered",
                    name: "registered",
                    keycode: 174,
                },
                KeynameTableEntry {
                    identifier: "KEY_macron",
                    name: "macron",
                    keycode: 175,
                },
                KeynameTableEntry {
                    identifier: "KEY_degree",
                    name: "degree",
                    keycode: 176,
                },
                KeynameTableEntry {
                    identifier: "KEY_plusminus",
                    name: "plusminus",
                    keycode: 177,
                },
                KeynameTableEntry {
                    identifier: "KEY_twosuperior",
                    name: "twosuperior",
                    keycode: 178,
                },
                KeynameTableEntry {
                    identifier: "KEY_threesuperior",
                    name: "threesuperior",
                    keycode: 179,
                },
                KeynameTableEntry {
                    identifier: "KEY_acute",
                    name: "acute",
                    keycode: 180,
                },
                KeynameTableEntry {
                    identifier: "KEY_mu",
                    name: "mu",
                    keycode: 181,
                },
                KeynameTableEntry {
                    identifier: "KEY_paragraph",
                    name: "paragraph",
                    keycode: 182,
                },
                KeynameTableEntry {
                    identifier: "KEY_periodcentered",
                    name: "periodcentered",
                    keycode: 183,
                },
                KeynameTableEntry {
                    identifier: "KEY_cedilla",
                    name: "cedilla",
                    keycode: 184,
                },
                KeynameTableEntry {
                    identifier: "KEY_onesuperior",
                    name: "onesuperior",
                    keycode: 185,
                },
                KeynameTableEntry {
                    identifier: "KEY_masculine",
                    name: "masculine",
                    keycode: 186,
                },
                KeynameTableEntry {
                    identifier: "KEY_guillemotright",
                    name: "guillemotright",
                    keycode: 187,
                },
                KeynameTableEntry {
                    identifier: "KEY_onequarter",
                    name: "onequarter",
                    keycode: 188,
                },
                KeynameTableEntry {
                    identifier: "KEY_onehalf",
                    name: "onehalf",
                    keycode: 189,
                },
                KeynameTableEntry {
                    identifier: "KEY_threequarters",
                    name: "threequarters",
                    keycode: 190,
                },
                KeynameTableEntry {
                    identifier: "KEY_questiondown",
                    name: "questiondown",
                    keycode: 191,
                },
                KeynameTableEntry {
                    identifier: "KEY_Agrave",
                    name: "Agrave",
                    keycode: 192,
                },
                KeynameTableEntry {
                    identifier: "KEY_Aacute",
                    name: "Aacute",
                    keycode: 193,
                },
                KeynameTableEntry {
                    identifier: "KEY_Acircumflex",
                    name: "Acircumflex",
                    keycode: 194,
                },
                KeynameTableEntry {
                    identifier: "KEY_Atilde",
                    name: "Atilde",
                    keycode: 195,
                },
                KeynameTableEntry {
                    identifier: "KEY_Adiaeresis",
                    name: "Adiaeresis",
                    keycode: 196,
                },
                KeynameTableEntry {
                    identifier: "KEY_Aring",
                    name: "Aring",
                    keycode: 197,
                },
                KeynameTableEntry {
                    identifier: "KEY_AE",
                    name: "AE",
                    keycode: 198,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ccedilla",
                    name: "Ccedilla",
                    keycode: 199,
                },
                KeynameTableEntry {
                    identifier: "KEY_Egrave",
                    name: "Egrave",
                    keycode: 200,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eacute",
                    name: "Eacute",
                    keycode: 201,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ecircumflex",
                    name: "Ecircumflex",
                    keycode: 202,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ediaeresis",
                    name: "Ediaeresis",
                    keycode: 203,
                },
                KeynameTableEntry {
                    identifier: "KEY_Igrave",
                    name: "Igrave",
                    keycode: 204,
                },
                KeynameTableEntry {
                    identifier: "KEY_Iacute",
                    name: "Iacute",
                    keycode: 205,
                },
                KeynameTableEntry {
                    identifier: "KEY_Icircumflex",
                    name: "Icircumflex",
                    keycode: 206,
                },
                KeynameTableEntry {
                    identifier: "KEY_Idiaeresis",
                    name: "Idiaeresis",
                    keycode: 207,
                },
                KeynameTableEntry {
                    identifier: "KEY_ETH",
                    name: "ETH",
                    keycode: 208,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eth",
                    name: "Eth",
                    keycode: 208,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ntilde",
                    name: "Ntilde",
                    keycode: 209,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ograve",
                    name: "Ograve",
                    keycode: 210,
                },
                KeynameTableEntry {
                    identifier: "KEY_Oacute",
                    name: "Oacute",
                    keycode: 211,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ocircumflex",
                    name: "Ocircumflex",
                    keycode: 212,
                },
                KeynameTableEntry {
                    identifier: "KEY_Otilde",
                    name: "Otilde",
                    keycode: 213,
                },
                KeynameTableEntry {
                    identifier: "KEY_Odiaeresis",
                    name: "Odiaeresis",
                    keycode: 214,
                },
                KeynameTableEntry {
                    identifier: "KEY_multiply",
                    name: "multiply",
                    keycode: 215,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ooblique",
                    name: "Ooblique",
                    keycode: 216,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ugrave",
                    name: "Ugrave",
                    keycode: 217,
                },
                KeynameTableEntry {
                    identifier: "KEY_Uacute",
                    name: "Uacute",
                    keycode: 218,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ucircumflex",
                    name: "Ucircumflex",
                    keycode: 219,
                },
                KeynameTableEntry {
                    identifier: "KEY_Udiaeresis",
                    name: "Udiaeresis",
                    keycode: 220,
                },
                KeynameTableEntry {
                    identifier: "KEY_Yacute",
                    name: "Yacute",
                    keycode: 221,
                },
                KeynameTableEntry {
                    identifier: "KEY_THORN",
                    name: "THORN",
                    keycode: 222,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thorn",
                    name: "Thorn",
                    keycode: 222,
                },
                KeynameTableEntry {
                    identifier: "KEY_ssharp",
                    name: "ssharp",
                    keycode: 223,
                },
                KeynameTableEntry {
                    identifier: "KEY_agrave",
                    name: "agrave",
                    keycode: 224,
                },
                KeynameTableEntry {
                    identifier: "KEY_aacute",
                    name: "aacute",
                    keycode: 225,
                },
                KeynameTableEntry {
                    identifier: "KEY_acircumflex",
                    name: "acircumflex",
                    keycode: 226,
                },
                KeynameTableEntry {
                    identifier: "KEY_atilde",
                    name: "atilde",
                    keycode: 227,
                },
                KeynameTableEntry {
                    identifier: "KEY_adiaeresis",
                    name: "adiaeresis",
                    keycode: 228,
                },
                KeynameTableEntry {
                    identifier: "KEY_aring",
                    name: "aring",
                    keycode: 229,
                },
                KeynameTableEntry {
                    identifier: "KEY_ae",
                    name: "ae",
                    keycode: 230,
                },
                KeynameTableEntry {
                    identifier: "KEY_ccedilla",
                    name: "ccedilla",
                    keycode: 231,
                },
                KeynameTableEntry {
                    identifier: "KEY_egrave",
                    name: "egrave",
                    keycode: 232,
                },
                KeynameTableEntry {
                    identifier: "KEY_eacute",
                    name: "eacute",
                    keycode: 233,
                },
                KeynameTableEntry {
                    identifier: "KEY_ecircumflex",
                    name: "ecircumflex",
                    keycode: 234,
                },
                KeynameTableEntry {
                    identifier: "KEY_ediaeresis",
                    name: "ediaeresis",
                    keycode: 235,
                },
                KeynameTableEntry {
                    identifier: "KEY_igrave",
                    name: "igrave",
                    keycode: 236,
                },
                KeynameTableEntry {
                    identifier: "KEY_iacute",
                    name: "iacute",
                    keycode: 237,
                },
                KeynameTableEntry {
                    identifier: "KEY_icircumflex",
                    name: "icircumflex",
                    keycode: 238,
                },
                KeynameTableEntry {
                    identifier: "KEY_idiaeresis",
                    name: "idiaeresis",
                    keycode: 239,
                },
                KeynameTableEntry {
                    identifier: "KEY_eth",
                    name: "eth",
                    keycode: 240,
                },
                KeynameTableEntry {
                    identifier: "KEY_ntilde",
                    name: "ntilde",
                    keycode: 241,
                },
                KeynameTableEntry {
                    identifier: "KEY_ograve",
                    name: "ograve",
                    keycode: 242,
                },
                KeynameTableEntry {
                    identifier: "KEY_oacute",
                    name: "oacute",
                    keycode: 243,
                },
                KeynameTableEntry {
                    identifier: "KEY_ocircumflex",
                    name: "ocircumflex",
                    keycode: 244,
                },
                KeynameTableEntry {
                    identifier: "KEY_otilde",
                    name: "otilde",
                    keycode: 245,
                },
                KeynameTableEntry {
                    identifier: "KEY_odiaeresis",
                    name: "odiaeresis",
                    keycode: 246,
                },
                KeynameTableEntry {
                    identifier: "KEY_division",
                    name: "division",
                    keycode: 247,
                },
                KeynameTableEntry {
                    identifier: "KEY_oslash",
                    name: "oslash",
                    keycode: 248,
                },
                KeynameTableEntry {
                    identifier: "KEY_ugrave",
                    name: "ugrave",
                    keycode: 249,
                },
                KeynameTableEntry {
                    identifier: "KEY_uacute",
                    name: "uacute",
                    keycode: 250,
                },
                KeynameTableEntry {
                    identifier: "KEY_ucircumflex",
                    name: "ucircumflex",
                    keycode: 251,
                },
                KeynameTableEntry {
                    identifier: "KEY_udiaeresis",
                    name: "udiaeresis",
                    keycode: 252,
                },
                KeynameTableEntry {
                    identifier: "KEY_yacute",
                    name: "yacute",
                    keycode: 253,
                },
                KeynameTableEntry {
                    identifier: "KEY_thorn",
                    name: "thorn",
                    keycode: 254,
                },
                KeynameTableEntry {
                    identifier: "KEY_ydiaeresis",
                    name: "ydiaeresis",
                    keycode: 255,
                },
                KeynameTableEntry {
                    identifier: "KEY_Aogonek",
                    name: "Aogonek",
                    keycode: 417,
                },
                KeynameTableEntry {
                    identifier: "KEY_breve",
                    name: "breve",
                    keycode: 418,
                },
                KeynameTableEntry {
                    identifier: "KEY_Lstroke",
                    name: "Lstroke",
                    keycode: 419,
                },
                KeynameTableEntry {
                    identifier: "KEY_Lcaron",
                    name: "Lcaron",
                    keycode: 421,
                },
                KeynameTableEntry {
                    identifier: "KEY_Sacute",
                    name: "Sacute",
                    keycode: 422,
                },
                KeynameTableEntry {
                    identifier: "KEY_Scaron",
                    name: "Scaron",
                    keycode: 425,
                },
                KeynameTableEntry {
                    identifier: "KEY_Scedilla",
                    name: "Scedilla",
                    keycode: 426,
                },
                KeynameTableEntry {
                    identifier: "KEY_Tcaron",
                    name: "Tcaron",
                    keycode: 427,
                },
                KeynameTableEntry {
                    identifier: "KEY_Zacute",
                    name: "Zacute",
                    keycode: 428,
                },
                KeynameTableEntry {
                    identifier: "KEY_Zcaron",
                    name: "Zcaron",
                    keycode: 430,
                },
                KeynameTableEntry {
                    identifier: "KEY_Zabovedot",
                    name: "Zabovedot",
                    keycode: 431,
                },
                KeynameTableEntry {
                    identifier: "KEY_aogonek",
                    name: "aogonek",
                    keycode: 433,
                },
                KeynameTableEntry {
                    identifier: "KEY_ogonek",
                    name: "ogonek",
                    keycode: 434,
                },
                KeynameTableEntry {
                    identifier: "KEY_lstroke",
                    name: "lstroke",
                    keycode: 435,
                },
                KeynameTableEntry {
                    identifier: "KEY_lcaron",
                    name: "lcaron",
                    keycode: 437,
                },
                KeynameTableEntry {
                    identifier: "KEY_sacute",
                    name: "sacute",
                    keycode: 438,
                },
                KeynameTableEntry {
                    identifier: "KEY_caron",
                    name: "caron",
                    keycode: 439,
                },
                KeynameTableEntry {
                    identifier: "KEY_scaron",
                    name: "scaron",
                    keycode: 441,
                },
                KeynameTableEntry {
                    identifier: "KEY_scedilla",
                    name: "scedilla",
                    keycode: 442,
                },
                KeynameTableEntry {
                    identifier: "KEY_tcaron",
                    name: "tcaron",
                    keycode: 443,
                },
                KeynameTableEntry {
                    identifier: "KEY_zacute",
                    name: "zacute",
                    keycode: 444,
                },
                KeynameTableEntry {
                    identifier: "KEY_doubleacute",
                    name: "doubleacute",
                    keycode: 445,
                },
                KeynameTableEntry {
                    identifier: "KEY_zcaron",
                    name: "zcaron",
                    keycode: 446,
                },
                KeynameTableEntry {
                    identifier: "KEY_zabovedot",
                    name: "zabovedot",
                    keycode: 447,
                },
                KeynameTableEntry {
                    identifier: "KEY_Racute",
                    name: "Racute",
                    keycode: 448,
                },
                KeynameTableEntry {
                    identifier: "KEY_Abreve",
                    name: "Abreve",
                    keycode: 451,
                },
                KeynameTableEntry {
                    identifier: "KEY_Lacute",
                    name: "Lacute",
                    keycode: 453,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cacute",
                    name: "Cacute",
                    keycode: 454,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ccaron",
                    name: "Ccaron",
                    keycode: 456,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eogonek",
                    name: "Eogonek",
                    keycode: 458,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ecaron",
                    name: "Ecaron",
                    keycode: 460,
                },
                KeynameTableEntry {
                    identifier: "KEY_Dcaron",
                    name: "Dcaron",
                    keycode: 463,
                },
                KeynameTableEntry {
                    identifier: "KEY_Dstroke",
                    name: "Dstroke",
                    keycode: 464,
                },
                KeynameTableEntry {
                    identifier: "KEY_Nacute",
                    name: "Nacute",
                    keycode: 465,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ncaron",
                    name: "Ncaron",
                    keycode: 466,
                },
                KeynameTableEntry {
                    identifier: "KEY_Odoubleacute",
                    name: "Odoubleacute",
                    keycode: 469,
                },
                KeynameTableEntry {
                    identifier: "KEY_Rcaron",
                    name: "Rcaron",
                    keycode: 472,
                },
                KeynameTableEntry {
                    identifier: "KEY_Uring",
                    name: "Uring",
                    keycode: 473,
                },
                KeynameTableEntry {
                    identifier: "KEY_Udoubleacute",
                    name: "Udoubleacute",
                    keycode: 475,
                },
                KeynameTableEntry {
                    identifier: "KEY_Tcedilla",
                    name: "Tcedilla",
                    keycode: 478,
                },
                KeynameTableEntry {
                    identifier: "KEY_racute",
                    name: "racute",
                    keycode: 480,
                },
                KeynameTableEntry {
                    identifier: "KEY_abreve",
                    name: "abreve",
                    keycode: 483,
                },
                KeynameTableEntry {
                    identifier: "KEY_lacute",
                    name: "lacute",
                    keycode: 485,
                },
                KeynameTableEntry {
                    identifier: "KEY_cacute",
                    name: "cacute",
                    keycode: 486,
                },
                KeynameTableEntry {
                    identifier: "KEY_ccaron",
                    name: "ccaron",
                    keycode: 488,
                },
                KeynameTableEntry {
                    identifier: "KEY_eogonek",
                    name: "eogonek",
                    keycode: 490,
                },
                KeynameTableEntry {
                    identifier: "KEY_ecaron",
                    name: "ecaron",
                    keycode: 492,
                },
                KeynameTableEntry {
                    identifier: "KEY_dcaron",
                    name: "dcaron",
                    keycode: 495,
                },
                KeynameTableEntry {
                    identifier: "KEY_dstroke",
                    name: "dstroke",
                    keycode: 496,
                },
                KeynameTableEntry {
                    identifier: "KEY_nacute",
                    name: "nacute",
                    keycode: 497,
                },
                KeynameTableEntry {
                    identifier: "KEY_ncaron",
                    name: "ncaron",
                    keycode: 498,
                },
                KeynameTableEntry {
                    identifier: "KEY_odoubleacute",
                    name: "odoubleacute",
                    keycode: 501,
                },
                KeynameTableEntry {
                    identifier: "KEY_rcaron",
                    name: "rcaron",
                    keycode: 504,
                },
                KeynameTableEntry {
                    identifier: "KEY_uring",
                    name: "uring",
                    keycode: 505,
                },
                KeynameTableEntry {
                    identifier: "KEY_udoubleacute",
                    name: "udoubleacute",
                    keycode: 507,
                },
                KeynameTableEntry {
                    identifier: "KEY_tcedilla",
                    name: "tcedilla",
                    keycode: 510,
                },
                KeynameTableEntry {
                    identifier: "KEY_abovedot",
                    name: "abovedot",
                    keycode: 511,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hstroke",
                    name: "Hstroke",
                    keycode: 673,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hcircumflex",
                    name: "Hcircumflex",
                    keycode: 678,
                },
                KeynameTableEntry {
                    identifier: "KEY_Iabovedot",
                    name: "Iabovedot",
                    keycode: 681,
                },
                KeynameTableEntry {
                    identifier: "KEY_Gbreve",
                    name: "Gbreve",
                    keycode: 683,
                },
                KeynameTableEntry {
                    identifier: "KEY_Jcircumflex",
                    name: "Jcircumflex",
                    keycode: 684,
                },
                KeynameTableEntry {
                    identifier: "KEY_hstroke",
                    name: "hstroke",
                    keycode: 689,
                },
                KeynameTableEntry {
                    identifier: "KEY_hcircumflex",
                    name: "hcircumflex",
                    keycode: 694,
                },
                KeynameTableEntry {
                    identifier: "KEY_idotless",
                    name: "idotless",
                    keycode: 697,
                },
                KeynameTableEntry {
                    identifier: "KEY_gbreve",
                    name: "gbreve",
                    keycode: 699,
                },
                KeynameTableEntry {
                    identifier: "KEY_jcircumflex",
                    name: "jcircumflex",
                    keycode: 700,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cabovedot",
                    name: "Cabovedot",
                    keycode: 709,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ccircumflex",
                    name: "Ccircumflex",
                    keycode: 710,
                },
                KeynameTableEntry {
                    identifier: "KEY_Gabovedot",
                    name: "Gabovedot",
                    keycode: 725,
                },
                KeynameTableEntry {
                    identifier: "KEY_Gcircumflex",
                    name: "Gcircumflex",
                    keycode: 728,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ubreve",
                    name: "Ubreve",
                    keycode: 733,
                },
                KeynameTableEntry {
                    identifier: "KEY_Scircumflex",
                    name: "Scircumflex",
                    keycode: 734,
                },
                KeynameTableEntry {
                    identifier: "KEY_cabovedot",
                    name: "cabovedot",
                    keycode: 741,
                },
                KeynameTableEntry {
                    identifier: "KEY_ccircumflex",
                    name: "ccircumflex",
                    keycode: 742,
                },
                KeynameTableEntry {
                    identifier: "KEY_gabovedot",
                    name: "gabovedot",
                    keycode: 757,
                },
                KeynameTableEntry {
                    identifier: "KEY_gcircumflex",
                    name: "gcircumflex",
                    keycode: 760,
                },
                KeynameTableEntry {
                    identifier: "KEY_ubreve",
                    name: "ubreve",
                    keycode: 765,
                },
                KeynameTableEntry {
                    identifier: "KEY_scircumflex",
                    name: "scircumflex",
                    keycode: 766,
                },
                KeynameTableEntry {
                    identifier: "KEY_kappa",
                    name: "kappa",
                    keycode: 930,
                },
                KeynameTableEntry {
                    identifier: "KEY_kra",
                    name: "kra",
                    keycode: 930,
                },
                KeynameTableEntry {
                    identifier: "KEY_Rcedilla",
                    name: "Rcedilla",
                    keycode: 931,
                },
                KeynameTableEntry {
                    identifier: "KEY_Itilde",
                    name: "Itilde",
                    keycode: 933,
                },
                KeynameTableEntry {
                    identifier: "KEY_Lcedilla",
                    name: "Lcedilla",
                    keycode: 934,
                },
                KeynameTableEntry {
                    identifier: "KEY_Emacron",
                    name: "Emacron",
                    keycode: 938,
                },
                KeynameTableEntry {
                    identifier: "KEY_Gcedilla",
                    name: "Gcedilla",
                    keycode: 939,
                },
                KeynameTableEntry {
                    identifier: "KEY_Tslash",
                    name: "Tslash",
                    keycode: 940,
                },
                KeynameTableEntry {
                    identifier: "KEY_rcedilla",
                    name: "rcedilla",
                    keycode: 947,
                },
                KeynameTableEntry {
                    identifier: "KEY_itilde",
                    name: "itilde",
                    keycode: 949,
                },
                KeynameTableEntry {
                    identifier: "KEY_lcedilla",
                    name: "lcedilla",
                    keycode: 950,
                },
                KeynameTableEntry {
                    identifier: "KEY_emacron",
                    name: "emacron",
                    keycode: 954,
                },
                KeynameTableEntry {
                    identifier: "KEY_gcedilla",
                    name: "gcedilla",
                    keycode: 955,
                },
                KeynameTableEntry {
                    identifier: "KEY_tslash",
                    name: "tslash",
                    keycode: 956,
                },
                KeynameTableEntry {
                    identifier: "KEY_ENG",
                    name: "ENG",
                    keycode: 957,
                },
                KeynameTableEntry {
                    identifier: "KEY_eng",
                    name: "eng",
                    keycode: 959,
                },
                KeynameTableEntry {
                    identifier: "KEY_Amacron",
                    name: "Amacron",
                    keycode: 960,
                },
                KeynameTableEntry {
                    identifier: "KEY_Iogonek",
                    name: "Iogonek",
                    keycode: 967,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eabovedot",
                    name: "Eabovedot",
                    keycode: 972,
                },
                KeynameTableEntry {
                    identifier: "KEY_Imacron",
                    name: "Imacron",
                    keycode: 975,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ncedilla",
                    name: "Ncedilla",
                    keycode: 977,
                },
                KeynameTableEntry {
                    identifier: "KEY_Omacron",
                    name: "Omacron",
                    keycode: 978,
                },
                KeynameTableEntry {
                    identifier: "KEY_Kcedilla",
                    name: "Kcedilla",
                    keycode: 979,
                },
                KeynameTableEntry {
                    identifier: "KEY_Uogonek",
                    name: "Uogonek",
                    keycode: 985,
                },
                KeynameTableEntry {
                    identifier: "KEY_Utilde",
                    name: "Utilde",
                    keycode: 989,
                },
                KeynameTableEntry {
                    identifier: "KEY_Umacron",
                    name: "Umacron",
                    keycode: 990,
                },
                KeynameTableEntry {
                    identifier: "KEY_amacron",
                    name: "amacron",
                    keycode: 992,
                },
                KeynameTableEntry {
                    identifier: "KEY_iogonek",
                    name: "iogonek",
                    keycode: 999,
                },
                KeynameTableEntry {
                    identifier: "KEY_eabovedot",
                    name: "eabovedot",
                    keycode: 1004,
                },
                KeynameTableEntry {
                    identifier: "KEY_imacron",
                    name: "imacron",
                    keycode: 1007,
                },
                KeynameTableEntry {
                    identifier: "KEY_ncedilla",
                    name: "ncedilla",
                    keycode: 1009,
                },
                KeynameTableEntry {
                    identifier: "KEY_omacron",
                    name: "omacron",
                    keycode: 1010,
                },
                KeynameTableEntry {
                    identifier: "KEY_kcedilla",
                    name: "kcedilla",
                    keycode: 1011,
                },
                KeynameTableEntry {
                    identifier: "KEY_uogonek",
                    name: "uogonek",
                    keycode: 1017,
                },
                KeynameTableEntry {
                    identifier: "KEY_utilde",
                    name: "utilde",
                    keycode: 1021,
                },
                KeynameTableEntry {
                    identifier: "KEY_umacron",
                    name: "umacron",
                    keycode: 1022,
                },
                KeynameTableEntry {
                    identifier: "KEY_overline",
                    name: "overline",
                    keycode: 1150,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_fullstop",
                    name: "kana_fullstop",
                    keycode: 1185,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_openingbracket",
                    name: "kana_openingbracket",
                    keycode: 1186,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_closingbracket",
                    name: "kana_closingbracket",
                    keycode: 1187,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_comma",
                    name: "kana_comma",
                    keycode: 1188,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_conjunctive",
                    name: "kana_conjunctive",
                    keycode: 1189,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_middledot",
                    name: "kana_middledot",
                    keycode: 1189,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_WO",
                    name: "kana_WO",
                    keycode: 1190,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_a",
                    name: "kana_a",
                    keycode: 1191,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_i",
                    name: "kana_i",
                    keycode: 1192,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_u",
                    name: "kana_u",
                    keycode: 1193,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_e",
                    name: "kana_e",
                    keycode: 1194,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_o",
                    name: "kana_o",
                    keycode: 1195,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_ya",
                    name: "kana_ya",
                    keycode: 1196,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_yu",
                    name: "kana_yu",
                    keycode: 1197,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_yo",
                    name: "kana_yo",
                    keycode: 1198,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_tsu",
                    name: "kana_tsu",
                    keycode: 1199,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_tu",
                    name: "kana_tu",
                    keycode: 1199,
                },
                KeynameTableEntry {
                    identifier: "KEY_prolongedsound",
                    name: "prolongedsound",
                    keycode: 1200,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_A",
                    name: "kana_A",
                    keycode: 1201,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_I",
                    name: "kana_I",
                    keycode: 1202,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_U",
                    name: "kana_U",
                    keycode: 1203,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_E",
                    name: "kana_E",
                    keycode: 1204,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_O",
                    name: "kana_O",
                    keycode: 1205,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_KA",
                    name: "kana_KA",
                    keycode: 1206,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_KI",
                    name: "kana_KI",
                    keycode: 1207,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_KU",
                    name: "kana_KU",
                    keycode: 1208,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_KE",
                    name: "kana_KE",
                    keycode: 1209,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_KO",
                    name: "kana_KO",
                    keycode: 1210,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_SA",
                    name: "kana_SA",
                    keycode: 1211,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_SHI",
                    name: "kana_SHI",
                    keycode: 1212,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_SU",
                    name: "kana_SU",
                    keycode: 1213,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_SE",
                    name: "kana_SE",
                    keycode: 1214,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_SO",
                    name: "kana_SO",
                    keycode: 1215,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TA",
                    name: "kana_TA",
                    keycode: 1216,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_CHI",
                    name: "kana_CHI",
                    keycode: 1217,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TI",
                    name: "kana_TI",
                    keycode: 1217,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TSU",
                    name: "kana_TSU",
                    keycode: 1218,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TU",
                    name: "kana_TU",
                    keycode: 1218,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TE",
                    name: "kana_TE",
                    keycode: 1219,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_TO",
                    name: "kana_TO",
                    keycode: 1220,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_NA",
                    name: "kana_NA",
                    keycode: 1221,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_NI",
                    name: "kana_NI",
                    keycode: 1222,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_NU",
                    name: "kana_NU",
                    keycode: 1223,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_NE",
                    name: "kana_NE",
                    keycode: 1224,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_NO",
                    name: "kana_NO",
                    keycode: 1225,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_HA",
                    name: "kana_HA",
                    keycode: 1226,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_HI",
                    name: "kana_HI",
                    keycode: 1227,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_FU",
                    name: "kana_FU",
                    keycode: 1228,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_HU",
                    name: "kana_HU",
                    keycode: 1228,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_HE",
                    name: "kana_HE",
                    keycode: 1229,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_HO",
                    name: "kana_HO",
                    keycode: 1230,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_MA",
                    name: "kana_MA",
                    keycode: 1231,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_MI",
                    name: "kana_MI",
                    keycode: 1232,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_MU",
                    name: "kana_MU",
                    keycode: 1233,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_ME",
                    name: "kana_ME",
                    keycode: 1234,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_MO",
                    name: "kana_MO",
                    keycode: 1235,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_YA",
                    name: "kana_YA",
                    keycode: 1236,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_YU",
                    name: "kana_YU",
                    keycode: 1237,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_YO",
                    name: "kana_YO",
                    keycode: 1238,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_RA",
                    name: "kana_RA",
                    keycode: 1239,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_RI",
                    name: "kana_RI",
                    keycode: 1240,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_RU",
                    name: "kana_RU",
                    keycode: 1241,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_RE",
                    name: "kana_RE",
                    keycode: 1242,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_RO",
                    name: "kana_RO",
                    keycode: 1243,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_WA",
                    name: "kana_WA",
                    keycode: 1244,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_N",
                    name: "kana_N",
                    keycode: 1245,
                },
                KeynameTableEntry {
                    identifier: "KEY_voicedsound",
                    name: "voicedsound",
                    keycode: 1246,
                },
                KeynameTableEntry {
                    identifier: "KEY_semivoicedsound",
                    name: "semivoicedsound",
                    keycode: 1247,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_comma",
                    name: "Arabic_comma",
                    keycode: 1452,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_semicolon",
                    name: "Arabic_semicolon",
                    keycode: 1467,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_question_mark",
                    name: "Arabic_question_mark",
                    keycode: 1471,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hamza",
                    name: "Arabic_hamza",
                    keycode: 1473,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_maddaonalef",
                    name: "Arabic_maddaonalef",
                    keycode: 1474,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hamzaonalef",
                    name: "Arabic_hamzaonalef",
                    keycode: 1475,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hamzaonwaw",
                    name: "Arabic_hamzaonwaw",
                    keycode: 1476,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hamzaunderalef",
                    name: "Arabic_hamzaunderalef",
                    keycode: 1477,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hamzaonyeh",
                    name: "Arabic_hamzaonyeh",
                    keycode: 1478,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_alef",
                    name: "Arabic_alef",
                    keycode: 1479,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_beh",
                    name: "Arabic_beh",
                    keycode: 1480,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_tehmarbuta",
                    name: "Arabic_tehmarbuta",
                    keycode: 1481,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_teh",
                    name: "Arabic_teh",
                    keycode: 1482,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_theh",
                    name: "Arabic_theh",
                    keycode: 1483,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_jeem",
                    name: "Arabic_jeem",
                    keycode: 1484,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_hah",
                    name: "Arabic_hah",
                    keycode: 1485,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_khah",
                    name: "Arabic_khah",
                    keycode: 1486,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_dal",
                    name: "Arabic_dal",
                    keycode: 1487,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_thal",
                    name: "Arabic_thal",
                    keycode: 1488,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_ra",
                    name: "Arabic_ra",
                    keycode: 1489,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_zain",
                    name: "Arabic_zain",
                    keycode: 1490,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_seen",
                    name: "Arabic_seen",
                    keycode: 1491,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_sheen",
                    name: "Arabic_sheen",
                    keycode: 1492,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_sad",
                    name: "Arabic_sad",
                    keycode: 1493,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_dad",
                    name: "Arabic_dad",
                    keycode: 1494,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_tah",
                    name: "Arabic_tah",
                    keycode: 1495,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_zah",
                    name: "Arabic_zah",
                    keycode: 1496,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_ain",
                    name: "Arabic_ain",
                    keycode: 1497,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_ghain",
                    name: "Arabic_ghain",
                    keycode: 1498,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_tatweel",
                    name: "Arabic_tatweel",
                    keycode: 1504,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_feh",
                    name: "Arabic_feh",
                    keycode: 1505,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_qaf",
                    name: "Arabic_qaf",
                    keycode: 1506,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_kaf",
                    name: "Arabic_kaf",
                    keycode: 1507,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_lam",
                    name: "Arabic_lam",
                    keycode: 1508,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_meem",
                    name: "Arabic_meem",
                    keycode: 1509,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_noon",
                    name: "Arabic_noon",
                    keycode: 1510,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_ha",
                    name: "Arabic_ha",
                    keycode: 1511,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_heh",
                    name: "Arabic_heh",
                    keycode: 1511,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_waw",
                    name: "Arabic_waw",
                    keycode: 1512,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_alefmaksura",
                    name: "Arabic_alefmaksura",
                    keycode: 1513,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_yeh",
                    name: "Arabic_yeh",
                    keycode: 1514,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_fathatan",
                    name: "Arabic_fathatan",
                    keycode: 1515,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_dammatan",
                    name: "Arabic_dammatan",
                    keycode: 1516,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_kasratan",
                    name: "Arabic_kasratan",
                    keycode: 1517,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_fatha",
                    name: "Arabic_fatha",
                    keycode: 1518,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_damma",
                    name: "Arabic_damma",
                    keycode: 1519,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_kasra",
                    name: "Arabic_kasra",
                    keycode: 1520,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_shadda",
                    name: "Arabic_shadda",
                    keycode: 1521,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_sukun",
                    name: "Arabic_sukun",
                    keycode: 1522,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_dje",
                    name: "Serbian_dje",
                    keycode: 1697,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_gje",
                    name: "Macedonia_gje",
                    keycode: 1698,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_io",
                    name: "Cyrillic_io",
                    keycode: 1699,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_ie",
                    name: "Ukrainian_ie",
                    keycode: 1700,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_je",
                    name: "Ukranian_je",
                    keycode: 1700,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_dse",
                    name: "Macedonia_dse",
                    keycode: 1701,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_i",
                    name: "Ukrainian_i",
                    keycode: 1702,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_i",
                    name: "Ukranian_i",
                    keycode: 1702,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_yi",
                    name: "Ukrainian_yi",
                    keycode: 1703,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_yi",
                    name: "Ukranian_yi",
                    keycode: 1703,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_je",
                    name: "Cyrillic_je",
                    keycode: 1704,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_je",
                    name: "Serbian_je",
                    keycode: 1704,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_lje",
                    name: "Cyrillic_lje",
                    keycode: 1705,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_lje",
                    name: "Serbian_lje",
                    keycode: 1705,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_nje",
                    name: "Cyrillic_nje",
                    keycode: 1706,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_nje",
                    name: "Serbian_nje",
                    keycode: 1706,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_tshe",
                    name: "Serbian_tshe",
                    keycode: 1707,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_kje",
                    name: "Macedonia_kje",
                    keycode: 1708,
                },
                KeynameTableEntry {
                    identifier: "KEY_Byelorussian_shortu",
                    name: "Byelorussian_shortu",
                    keycode: 1710,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_dzhe",
                    name: "Cyrillic_dzhe",
                    keycode: 1711,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_dze",
                    name: "Serbian_dze",
                    keycode: 1711,
                },
                KeynameTableEntry {
                    identifier: "KEY_numerosign",
                    name: "numerosign",
                    keycode: 1712,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_DJE",
                    name: "Serbian_DJE",
                    keycode: 1713,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_GJE",
                    name: "Macedonia_GJE",
                    keycode: 1714,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_IO",
                    name: "Cyrillic_IO",
                    keycode: 1715,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_IE",
                    name: "Ukrainian_IE",
                    keycode: 1716,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_JE",
                    name: "Ukranian_JE",
                    keycode: 1716,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_DSE",
                    name: "Macedonia_DSE",
                    keycode: 1717,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_I",
                    name: "Ukrainian_I",
                    keycode: 1718,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_I",
                    name: "Ukranian_I",
                    keycode: 1718,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukrainian_YI",
                    name: "Ukrainian_YI",
                    keycode: 1719,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ukranian_YI",
                    name: "Ukranian_YI",
                    keycode: 1719,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_JE",
                    name: "Cyrillic_JE",
                    keycode: 1720,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_JE",
                    name: "Serbian_JE",
                    keycode: 1720,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_LJE",
                    name: "Cyrillic_LJE",
                    keycode: 1721,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_LJE",
                    name: "Serbian_LJE",
                    keycode: 1721,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_NJE",
                    name: "Cyrillic_NJE",
                    keycode: 1722,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_NJE",
                    name: "Serbian_NJE",
                    keycode: 1722,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_TSHE",
                    name: "Serbian_TSHE",
                    keycode: 1723,
                },
                KeynameTableEntry {
                    identifier: "KEY_Macedonia_KJE",
                    name: "Macedonia_KJE",
                    keycode: 1724,
                },
                KeynameTableEntry {
                    identifier: "KEY_Byelorussian_SHORTU",
                    name: "Byelorussian_SHORTU",
                    keycode: 1726,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_DZHE",
                    name: "Cyrillic_DZHE",
                    keycode: 1727,
                },
                KeynameTableEntry {
                    identifier: "KEY_Serbian_DZE",
                    name: "Serbian_DZE",
                    keycode: 1727,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_yu",
                    name: "Cyrillic_yu",
                    keycode: 1728,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_a",
                    name: "Cyrillic_a",
                    keycode: 1729,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_be",
                    name: "Cyrillic_be",
                    keycode: 1730,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_tse",
                    name: "Cyrillic_tse",
                    keycode: 1731,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_de",
                    name: "Cyrillic_de",
                    keycode: 1732,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ie",
                    name: "Cyrillic_ie",
                    keycode: 1733,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ef",
                    name: "Cyrillic_ef",
                    keycode: 1734,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ghe",
                    name: "Cyrillic_ghe",
                    keycode: 1735,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ha",
                    name: "Cyrillic_ha",
                    keycode: 1736,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_i",
                    name: "Cyrillic_i",
                    keycode: 1737,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_shorti",
                    name: "Cyrillic_shorti",
                    keycode: 1738,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ka",
                    name: "Cyrillic_ka",
                    keycode: 1739,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_el",
                    name: "Cyrillic_el",
                    keycode: 1740,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_em",
                    name: "Cyrillic_em",
                    keycode: 1741,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_en",
                    name: "Cyrillic_en",
                    keycode: 1742,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_o",
                    name: "Cyrillic_o",
                    keycode: 1743,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_pe",
                    name: "Cyrillic_pe",
                    keycode: 1744,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ya",
                    name: "Cyrillic_ya",
                    keycode: 1745,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_er",
                    name: "Cyrillic_er",
                    keycode: 1746,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_es",
                    name: "Cyrillic_es",
                    keycode: 1747,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_te",
                    name: "Cyrillic_te",
                    keycode: 1748,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_u",
                    name: "Cyrillic_u",
                    keycode: 1749,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_zhe",
                    name: "Cyrillic_zhe",
                    keycode: 1750,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ve",
                    name: "Cyrillic_ve",
                    keycode: 1751,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_softsign",
                    name: "Cyrillic_softsign",
                    keycode: 1752,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_yeru",
                    name: "Cyrillic_yeru",
                    keycode: 1753,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ze",
                    name: "Cyrillic_ze",
                    keycode: 1754,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_sha",
                    name: "Cyrillic_sha",
                    keycode: 1755,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_e",
                    name: "Cyrillic_e",
                    keycode: 1756,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_shcha",
                    name: "Cyrillic_shcha",
                    keycode: 1757,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_che",
                    name: "Cyrillic_che",
                    keycode: 1758,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_hardsign",
                    name: "Cyrillic_hardsign",
                    keycode: 1759,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_YU",
                    name: "Cyrillic_YU",
                    keycode: 1760,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_A",
                    name: "Cyrillic_A",
                    keycode: 1761,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_BE",
                    name: "Cyrillic_BE",
                    keycode: 1762,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_TSE",
                    name: "Cyrillic_TSE",
                    keycode: 1763,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_DE",
                    name: "Cyrillic_DE",
                    keycode: 1764,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_IE",
                    name: "Cyrillic_IE",
                    keycode: 1765,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_EF",
                    name: "Cyrillic_EF",
                    keycode: 1766,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_GHE",
                    name: "Cyrillic_GHE",
                    keycode: 1767,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_HA",
                    name: "Cyrillic_HA",
                    keycode: 1768,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_I",
                    name: "Cyrillic_I",
                    keycode: 1769,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_SHORTI",
                    name: "Cyrillic_SHORTI",
                    keycode: 1770,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_KA",
                    name: "Cyrillic_KA",
                    keycode: 1771,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_EL",
                    name: "Cyrillic_EL",
                    keycode: 1772,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_EM",
                    name: "Cyrillic_EM",
                    keycode: 1773,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_EN",
                    name: "Cyrillic_EN",
                    keycode: 1774,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_O",
                    name: "Cyrillic_O",
                    keycode: 1775,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_PE",
                    name: "Cyrillic_PE",
                    keycode: 1776,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_YA",
                    name: "Cyrillic_YA",
                    keycode: 1777,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ER",
                    name: "Cyrillic_ER",
                    keycode: 1778,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ES",
                    name: "Cyrillic_ES",
                    keycode: 1779,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_TE",
                    name: "Cyrillic_TE",
                    keycode: 1780,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_U",
                    name: "Cyrillic_U",
                    keycode: 1781,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ZHE",
                    name: "Cyrillic_ZHE",
                    keycode: 1782,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_VE",
                    name: "Cyrillic_VE",
                    keycode: 1783,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_SOFTSIGN",
                    name: "Cyrillic_SOFTSIGN",
                    keycode: 1784,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_YERU",
                    name: "Cyrillic_YERU",
                    keycode: 1785,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_ZE",
                    name: "Cyrillic_ZE",
                    keycode: 1786,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_SHA",
                    name: "Cyrillic_SHA",
                    keycode: 1787,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_E",
                    name: "Cyrillic_E",
                    keycode: 1788,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_SHCHA",
                    name: "Cyrillic_SHCHA",
                    keycode: 1789,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_CHE",
                    name: "Cyrillic_CHE",
                    keycode: 1790,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cyrillic_HARDSIGN",
                    name: "Cyrillic_HARDSIGN",
                    keycode: 1791,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_ALPHAaccent",
                    name: "Greek_ALPHAaccent",
                    keycode: 1953,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_EPSILONaccent",
                    name: "Greek_EPSILONaccent",
                    keycode: 1954,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_ETAaccent",
                    name: "Greek_ETAaccent",
                    keycode: 1955,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_IOTAaccent",
                    name: "Greek_IOTAaccent",
                    keycode: 1956,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_IOTAdieresis",
                    name: "Greek_IOTAdieresis",
                    keycode: 1957,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_IOTAdiaeresis",
                    name: "Greek_IOTAdiaeresis",
                    keycode: 1957,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_OMICRONaccent",
                    name: "Greek_OMICRONaccent",
                    keycode: 1959,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_UPSILONaccent",
                    name: "Greek_UPSILONaccent",
                    keycode: 1960,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_UPSILONdieresis",
                    name: "Greek_UPSILONdieresis",
                    keycode: 1961,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_OMEGAaccent",
                    name: "Greek_OMEGAaccent",
                    keycode: 1963,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_accentdieresis",
                    name: "Greek_accentdieresis",
                    keycode: 1966,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_horizbar",
                    name: "Greek_horizbar",
                    keycode: 1967,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_alphaaccent",
                    name: "Greek_alphaaccent",
                    keycode: 1969,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_epsilonaccent",
                    name: "Greek_epsilonaccent",
                    keycode: 1970,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_etaaccent",
                    name: "Greek_etaaccent",
                    keycode: 1971,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_iotaaccent",
                    name: "Greek_iotaaccent",
                    keycode: 1972,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_iotadieresis",
                    name: "Greek_iotadieresis",
                    keycode: 1973,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_iotaaccentdieresis",
                    name: "Greek_iotaaccentdieresis",
                    keycode: 1974,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_omicronaccent",
                    name: "Greek_omicronaccent",
                    keycode: 1975,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_upsilonaccent",
                    name: "Greek_upsilonaccent",
                    keycode: 1976,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_upsilondieresis",
                    name: "Greek_upsilondieresis",
                    keycode: 1977,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_upsilonaccentdieresis",
                    name: "Greek_upsilonaccentdieresis",
                    keycode: 1978,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_omegaaccent",
                    name: "Greek_omegaaccent",
                    keycode: 1979,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_ALPHA",
                    name: "Greek_ALPHA",
                    keycode: 1985,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_BETA",
                    name: "Greek_BETA",
                    keycode: 1986,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_GAMMA",
                    name: "Greek_GAMMA",
                    keycode: 1987,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_DELTA",
                    name: "Greek_DELTA",
                    keycode: 1988,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_EPSILON",
                    name: "Greek_EPSILON",
                    keycode: 1989,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_ZETA",
                    name: "Greek_ZETA",
                    keycode: 1990,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_ETA",
                    name: "Greek_ETA",
                    keycode: 1991,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_THETA",
                    name: "Greek_THETA",
                    keycode: 1992,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_IOTA",
                    name: "Greek_IOTA",
                    keycode: 1993,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_KAPPA",
                    name: "Greek_KAPPA",
                    keycode: 1994,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_LAMBDA",
                    name: "Greek_LAMBDA",
                    keycode: 1995,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_LAMDA",
                    name: "Greek_LAMDA",
                    keycode: 1995,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_MU",
                    name: "Greek_MU",
                    keycode: 1996,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_NU",
                    name: "Greek_NU",
                    keycode: 1997,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_XI",
                    name: "Greek_XI",
                    keycode: 1998,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_OMICRON",
                    name: "Greek_OMICRON",
                    keycode: 1999,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_PI",
                    name: "Greek_PI",
                    keycode: 2000,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_RHO",
                    name: "Greek_RHO",
                    keycode: 2001,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_SIGMA",
                    name: "Greek_SIGMA",
                    keycode: 2002,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_TAU",
                    name: "Greek_TAU",
                    keycode: 2004,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_UPSILON",
                    name: "Greek_UPSILON",
                    keycode: 2005,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_PHI",
                    name: "Greek_PHI",
                    keycode: 2006,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_CHI",
                    name: "Greek_CHI",
                    keycode: 2007,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_PSI",
                    name: "Greek_PSI",
                    keycode: 2008,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_OMEGA",
                    name: "Greek_OMEGA",
                    keycode: 2009,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_alpha",
                    name: "Greek_alpha",
                    keycode: 2017,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_beta",
                    name: "Greek_beta",
                    keycode: 2018,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_gamma",
                    name: "Greek_gamma",
                    keycode: 2019,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_delta",
                    name: "Greek_delta",
                    keycode: 2020,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_epsilon",
                    name: "Greek_epsilon",
                    keycode: 2021,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_zeta",
                    name: "Greek_zeta",
                    keycode: 2022,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_eta",
                    name: "Greek_eta",
                    keycode: 2023,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_theta",
                    name: "Greek_theta",
                    keycode: 2024,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_iota",
                    name: "Greek_iota",
                    keycode: 2025,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_kappa",
                    name: "Greek_kappa",
                    keycode: 2026,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_lambda",
                    name: "Greek_lambda",
                    keycode: 2027,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_lamda",
                    name: "Greek_lamda",
                    keycode: 2027,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_mu",
                    name: "Greek_mu",
                    keycode: 2028,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_nu",
                    name: "Greek_nu",
                    keycode: 2029,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_xi",
                    name: "Greek_xi",
                    keycode: 2030,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_omicron",
                    name: "Greek_omicron",
                    keycode: 2031,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_pi",
                    name: "Greek_pi",
                    keycode: 2032,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_rho",
                    name: "Greek_rho",
                    keycode: 2033,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_sigma",
                    name: "Greek_sigma",
                    keycode: 2034,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_finalsmallsigma",
                    name: "Greek_finalsmallsigma",
                    keycode: 2035,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_tau",
                    name: "Greek_tau",
                    keycode: 2036,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_upsilon",
                    name: "Greek_upsilon",
                    keycode: 2037,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_phi",
                    name: "Greek_phi",
                    keycode: 2038,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_chi",
                    name: "Greek_chi",
                    keycode: 2039,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_psi",
                    name: "Greek_psi",
                    keycode: 2040,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_omega",
                    name: "Greek_omega",
                    keycode: 2041,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftradical",
                    name: "leftradical",
                    keycode: 2209,
                },
                KeynameTableEntry {
                    identifier: "KEY_topleftradical",
                    name: "topleftradical",
                    keycode: 2210,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizconnector",
                    name: "horizconnector",
                    keycode: 2211,
                },
                KeynameTableEntry {
                    identifier: "KEY_topintegral",
                    name: "topintegral",
                    keycode: 2212,
                },
                KeynameTableEntry {
                    identifier: "KEY_botintegral",
                    name: "botintegral",
                    keycode: 2213,
                },
                KeynameTableEntry {
                    identifier: "KEY_vertconnector",
                    name: "vertconnector",
                    keycode: 2214,
                },
                KeynameTableEntry {
                    identifier: "KEY_topleftsqbracket",
                    name: "topleftsqbracket",
                    keycode: 2215,
                },
                KeynameTableEntry {
                    identifier: "KEY_botleftsqbracket",
                    name: "botleftsqbracket",
                    keycode: 2216,
                },
                KeynameTableEntry {
                    identifier: "KEY_toprightsqbracket",
                    name: "toprightsqbracket",
                    keycode: 2217,
                },
                KeynameTableEntry {
                    identifier: "KEY_botrightsqbracket",
                    name: "botrightsqbracket",
                    keycode: 2218,
                },
                KeynameTableEntry {
                    identifier: "KEY_topleftparens",
                    name: "topleftparens",
                    keycode: 2219,
                },
                KeynameTableEntry {
                    identifier: "KEY_botleftparens",
                    name: "botleftparens",
                    keycode: 2220,
                },
                KeynameTableEntry {
                    identifier: "KEY_toprightparens",
                    name: "toprightparens",
                    keycode: 2221,
                },
                KeynameTableEntry {
                    identifier: "KEY_botrightparens",
                    name: "botrightparens",
                    keycode: 2222,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftmiddlecurlybrace",
                    name: "leftmiddlecurlybrace",
                    keycode: 2223,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightmiddlecurlybrace",
                    name: "rightmiddlecurlybrace",
                    keycode: 2224,
                },
                KeynameTableEntry {
                    identifier: "KEY_topleftsummation",
                    name: "topleftsummation",
                    keycode: 2225,
                },
                KeynameTableEntry {
                    identifier: "KEY_botleftsummation",
                    name: "botleftsummation",
                    keycode: 2226,
                },
                KeynameTableEntry {
                    identifier: "KEY_topvertsummationconnector",
                    name: "topvertsummationconnector",
                    keycode: 2227,
                },
                KeynameTableEntry {
                    identifier: "KEY_botvertsummationconnector",
                    name: "botvertsummationconnector",
                    keycode: 2228,
                },
                KeynameTableEntry {
                    identifier: "KEY_toprightsummation",
                    name: "toprightsummation",
                    keycode: 2229,
                },
                KeynameTableEntry {
                    identifier: "KEY_botrightsummation",
                    name: "botrightsummation",
                    keycode: 2230,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightmiddlesummation",
                    name: "rightmiddlesummation",
                    keycode: 2231,
                },
                KeynameTableEntry {
                    identifier: "KEY_lessthanequal",
                    name: "lessthanequal",
                    keycode: 2236,
                },
                KeynameTableEntry {
                    identifier: "KEY_notequal",
                    name: "notequal",
                    keycode: 2237,
                },
                KeynameTableEntry {
                    identifier: "KEY_greaterthanequal",
                    name: "greaterthanequal",
                    keycode: 2238,
                },
                KeynameTableEntry {
                    identifier: "KEY_integral",
                    name: "integral",
                    keycode: 2239,
                },
                KeynameTableEntry {
                    identifier: "KEY_therefore",
                    name: "therefore",
                    keycode: 2240,
                },
                KeynameTableEntry {
                    identifier: "KEY_variation",
                    name: "variation",
                    keycode: 2241,
                },
                KeynameTableEntry {
                    identifier: "KEY_infinity",
                    name: "infinity",
                    keycode: 2242,
                },
                KeynameTableEntry {
                    identifier: "KEY_nabla",
                    name: "nabla",
                    keycode: 2245,
                },
                KeynameTableEntry {
                    identifier: "KEY_approximate",
                    name: "approximate",
                    keycode: 2248,
                },
                KeynameTableEntry {
                    identifier: "KEY_similarequal",
                    name: "similarequal",
                    keycode: 2249,
                },
                KeynameTableEntry {
                    identifier: "KEY_ifonlyif",
                    name: "ifonlyif",
                    keycode: 2253,
                },
                KeynameTableEntry {
                    identifier: "KEY_implies",
                    name: "implies",
                    keycode: 2254,
                },
                KeynameTableEntry {
                    identifier: "KEY_identical",
                    name: "identical",
                    keycode: 2255,
                },
                KeynameTableEntry {
                    identifier: "KEY_radical",
                    name: "radical",
                    keycode: 2262,
                },
                KeynameTableEntry {
                    identifier: "KEY_includedin",
                    name: "includedin",
                    keycode: 2266,
                },
                KeynameTableEntry {
                    identifier: "KEY_includes",
                    name: "includes",
                    keycode: 2267,
                },
                KeynameTableEntry {
                    identifier: "KEY_intersection",
                    name: "intersection",
                    keycode: 2268,
                },
                KeynameTableEntry {
                    identifier: "KEY_union",
                    name: "union",
                    keycode: 2269,
                },
                KeynameTableEntry {
                    identifier: "KEY_logicaland",
                    name: "logicaland",
                    keycode: 2270,
                },
                KeynameTableEntry {
                    identifier: "KEY_logicalor",
                    name: "logicalor",
                    keycode: 2271,
                },
                KeynameTableEntry {
                    identifier: "KEY_partialderivative",
                    name: "partialderivative",
                    keycode: 2287,
                },
                KeynameTableEntry {
                    identifier: "KEY_function",
                    name: "function",
                    keycode: 2294,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftarrow",
                    name: "leftarrow",
                    keycode: 2299,
                },
                KeynameTableEntry {
                    identifier: "KEY_uparrow",
                    name: "uparrow",
                    keycode: 2300,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightarrow",
                    name: "rightarrow",
                    keycode: 2301,
                },
                KeynameTableEntry {
                    identifier: "KEY_downarrow",
                    name: "downarrow",
                    keycode: 2302,
                },
                KeynameTableEntry {
                    identifier: "KEY_blank",
                    name: "blank",
                    keycode: 2527,
                },
                KeynameTableEntry {
                    identifier: "KEY_soliddiamond",
                    name: "soliddiamond",
                    keycode: 2528,
                },
                KeynameTableEntry {
                    identifier: "KEY_checkerboard",
                    name: "checkerboard",
                    keycode: 2529,
                },
                KeynameTableEntry {
                    identifier: "KEY_ht",
                    name: "ht",
                    keycode: 2530,
                },
                KeynameTableEntry {
                    identifier: "KEY_ff",
                    name: "ff",
                    keycode: 2531,
                },
                KeynameTableEntry {
                    identifier: "KEY_cr",
                    name: "cr",
                    keycode: 2532,
                },
                KeynameTableEntry {
                    identifier: "KEY_lf",
                    name: "lf",
                    keycode: 2533,
                },
                KeynameTableEntry {
                    identifier: "KEY_nl",
                    name: "nl",
                    keycode: 2536,
                },
                KeynameTableEntry {
                    identifier: "KEY_vt",
                    name: "vt",
                    keycode: 2537,
                },
                KeynameTableEntry {
                    identifier: "KEY_lowrightcorner",
                    name: "lowrightcorner",
                    keycode: 2538,
                },
                KeynameTableEntry {
                    identifier: "KEY_uprightcorner",
                    name: "uprightcorner",
                    keycode: 2539,
                },
                KeynameTableEntry {
                    identifier: "KEY_upleftcorner",
                    name: "upleftcorner",
                    keycode: 2540,
                },
                KeynameTableEntry {
                    identifier: "KEY_lowleftcorner",
                    name: "lowleftcorner",
                    keycode: 2541,
                },
                KeynameTableEntry {
                    identifier: "KEY_crossinglines",
                    name: "crossinglines",
                    keycode: 2542,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizlinescan1",
                    name: "horizlinescan1",
                    keycode: 2543,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizlinescan3",
                    name: "horizlinescan3",
                    keycode: 2544,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizlinescan5",
                    name: "horizlinescan5",
                    keycode: 2545,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizlinescan7",
                    name: "horizlinescan7",
                    keycode: 2546,
                },
                KeynameTableEntry {
                    identifier: "KEY_horizlinescan9",
                    name: "horizlinescan9",
                    keycode: 2547,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftt",
                    name: "leftt",
                    keycode: 2548,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightt",
                    name: "rightt",
                    keycode: 2549,
                },
                KeynameTableEntry {
                    identifier: "KEY_bott",
                    name: "bott",
                    keycode: 2550,
                },
                KeynameTableEntry {
                    identifier: "KEY_topt",
                    name: "topt",
                    keycode: 2551,
                },
                KeynameTableEntry {
                    identifier: "KEY_vertbar",
                    name: "vertbar",
                    keycode: 2552,
                },
                KeynameTableEntry {
                    identifier: "KEY_emspace",
                    name: "emspace",
                    keycode: 2721,
                },
                KeynameTableEntry {
                    identifier: "KEY_enspace",
                    name: "enspace",
                    keycode: 2722,
                },
                KeynameTableEntry {
                    identifier: "KEY_em3space",
                    name: "em3space",
                    keycode: 2723,
                },
                KeynameTableEntry {
                    identifier: "KEY_em4space",
                    name: "em4space",
                    keycode: 2724,
                },
                KeynameTableEntry {
                    identifier: "KEY_digitspace",
                    name: "digitspace",
                    keycode: 2725,
                },
                KeynameTableEntry {
                    identifier: "KEY_punctspace",
                    name: "punctspace",
                    keycode: 2726,
                },
                KeynameTableEntry {
                    identifier: "KEY_thinspace",
                    name: "thinspace",
                    keycode: 2727,
                },
                KeynameTableEntry {
                    identifier: "KEY_hairspace",
                    name: "hairspace",
                    keycode: 2728,
                },
                KeynameTableEntry {
                    identifier: "KEY_emdash",
                    name: "emdash",
                    keycode: 2729,
                },
                KeynameTableEntry {
                    identifier: "KEY_endash",
                    name: "endash",
                    keycode: 2730,
                },
                KeynameTableEntry {
                    identifier: "KEY_signifblank",
                    name: "signifblank",
                    keycode: 2732,
                },
                KeynameTableEntry {
                    identifier: "KEY_ellipsis",
                    name: "ellipsis",
                    keycode: 2734,
                },
                KeynameTableEntry {
                    identifier: "KEY_doubbaselinedot",
                    name: "doubbaselinedot",
                    keycode: 2735,
                },
                KeynameTableEntry {
                    identifier: "KEY_onethird",
                    name: "onethird",
                    keycode: 2736,
                },
                KeynameTableEntry {
                    identifier: "KEY_twothirds",
                    name: "twothirds",
                    keycode: 2737,
                },
                KeynameTableEntry {
                    identifier: "KEY_onefifth",
                    name: "onefifth",
                    keycode: 2738,
                },
                KeynameTableEntry {
                    identifier: "KEY_twofifths",
                    name: "twofifths",
                    keycode: 2739,
                },
                KeynameTableEntry {
                    identifier: "KEY_threefifths",
                    name: "threefifths",
                    keycode: 2740,
                },
                KeynameTableEntry {
                    identifier: "KEY_fourfifths",
                    name: "fourfifths",
                    keycode: 2741,
                },
                KeynameTableEntry {
                    identifier: "KEY_onesixth",
                    name: "onesixth",
                    keycode: 2742,
                },
                KeynameTableEntry {
                    identifier: "KEY_fivesixths",
                    name: "fivesixths",
                    keycode: 2743,
                },
                KeynameTableEntry {
                    identifier: "KEY_careof",
                    name: "careof",
                    keycode: 2744,
                },
                KeynameTableEntry {
                    identifier: "KEY_figdash",
                    name: "figdash",
                    keycode: 2747,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftanglebracket",
                    name: "leftanglebracket",
                    keycode: 2748,
                },
                KeynameTableEntry {
                    identifier: "KEY_decimalpoint",
                    name: "decimalpoint",
                    keycode: 2749,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightanglebracket",
                    name: "rightanglebracket",
                    keycode: 2750,
                },
                KeynameTableEntry {
                    identifier: "KEY_marker",
                    name: "marker",
                    keycode: 2751,
                },
                KeynameTableEntry {
                    identifier: "KEY_oneeighth",
                    name: "oneeighth",
                    keycode: 2755,
                },
                KeynameTableEntry {
                    identifier: "KEY_threeeighths",
                    name: "threeeighths",
                    keycode: 2756,
                },
                KeynameTableEntry {
                    identifier: "KEY_fiveeighths",
                    name: "fiveeighths",
                    keycode: 2757,
                },
                KeynameTableEntry {
                    identifier: "KEY_seveneighths",
                    name: "seveneighths",
                    keycode: 2758,
                },
                KeynameTableEntry {
                    identifier: "KEY_trademark",
                    name: "trademark",
                    keycode: 2761,
                },
                KeynameTableEntry {
                    identifier: "KEY_signaturemark",
                    name: "signaturemark",
                    keycode: 2762,
                },
                KeynameTableEntry {
                    identifier: "KEY_trademarkincircle",
                    name: "trademarkincircle",
                    keycode: 2763,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftopentriangle",
                    name: "leftopentriangle",
                    keycode: 2764,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightopentriangle",
                    name: "rightopentriangle",
                    keycode: 2765,
                },
                KeynameTableEntry {
                    identifier: "KEY_emopencircle",
                    name: "emopencircle",
                    keycode: 2766,
                },
                KeynameTableEntry {
                    identifier: "KEY_emopenrectangle",
                    name: "emopenrectangle",
                    keycode: 2767,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftsinglequotemark",
                    name: "leftsinglequotemark",
                    keycode: 2768,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightsinglequotemark",
                    name: "rightsinglequotemark",
                    keycode: 2769,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftdoublequotemark",
                    name: "leftdoublequotemark",
                    keycode: 2770,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightdoublequotemark",
                    name: "rightdoublequotemark",
                    keycode: 2771,
                },
                KeynameTableEntry {
                    identifier: "KEY_prescription",
                    name: "prescription",
                    keycode: 2772,
                },
                KeynameTableEntry {
                    identifier: "KEY_minutes",
                    name: "minutes",
                    keycode: 2774,
                },
                KeynameTableEntry {
                    identifier: "KEY_seconds",
                    name: "seconds",
                    keycode: 2775,
                },
                KeynameTableEntry {
                    identifier: "KEY_latincross",
                    name: "latincross",
                    keycode: 2777,
                },
                KeynameTableEntry {
                    identifier: "KEY_hexagram",
                    name: "hexagram",
                    keycode: 2778,
                },
                KeynameTableEntry {
                    identifier: "KEY_filledrectbullet",
                    name: "filledrectbullet",
                    keycode: 2779,
                },
                KeynameTableEntry {
                    identifier: "KEY_filledlefttribullet",
                    name: "filledlefttribullet",
                    keycode: 2780,
                },
                KeynameTableEntry {
                    identifier: "KEY_filledrighttribullet",
                    name: "filledrighttribullet",
                    keycode: 2781,
                },
                KeynameTableEntry {
                    identifier: "KEY_emfilledcircle",
                    name: "emfilledcircle",
                    keycode: 2782,
                },
                KeynameTableEntry {
                    identifier: "KEY_emfilledrect",
                    name: "emfilledrect",
                    keycode: 2783,
                },
                KeynameTableEntry {
                    identifier: "KEY_enopencircbullet",
                    name: "enopencircbullet",
                    keycode: 2784,
                },
                KeynameTableEntry {
                    identifier: "KEY_enopensquarebullet",
                    name: "enopensquarebullet",
                    keycode: 2785,
                },
                KeynameTableEntry {
                    identifier: "KEY_openrectbullet",
                    name: "openrectbullet",
                    keycode: 2786,
                },
                KeynameTableEntry {
                    identifier: "KEY_opentribulletup",
                    name: "opentribulletup",
                    keycode: 2787,
                },
                KeynameTableEntry {
                    identifier: "KEY_opentribulletdown",
                    name: "opentribulletdown",
                    keycode: 2788,
                },
                KeynameTableEntry {
                    identifier: "KEY_openstar",
                    name: "openstar",
                    keycode: 2789,
                },
                KeynameTableEntry {
                    identifier: "KEY_enfilledcircbullet",
                    name: "enfilledcircbullet",
                    keycode: 2790,
                },
                KeynameTableEntry {
                    identifier: "KEY_enfilledsqbullet",
                    name: "enfilledsqbullet",
                    keycode: 2791,
                },
                KeynameTableEntry {
                    identifier: "KEY_filledtribulletup",
                    name: "filledtribulletup",
                    keycode: 2792,
                },
                KeynameTableEntry {
                    identifier: "KEY_filledtribulletdown",
                    name: "filledtribulletdown",
                    keycode: 2793,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftpointer",
                    name: "leftpointer",
                    keycode: 2794,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightpointer",
                    name: "rightpointer",
                    keycode: 2795,
                },
                KeynameTableEntry {
                    identifier: "KEY_club",
                    name: "club",
                    keycode: 2796,
                },
                KeynameTableEntry {
                    identifier: "KEY_diamond",
                    name: "diamond",
                    keycode: 2797,
                },
                KeynameTableEntry {
                    identifier: "KEY_heart",
                    name: "heart",
                    keycode: 2798,
                },
                KeynameTableEntry {
                    identifier: "KEY_maltesecross",
                    name: "maltesecross",
                    keycode: 2800,
                },
                KeynameTableEntry {
                    identifier: "KEY_dagger",
                    name: "dagger",
                    keycode: 2801,
                },
                KeynameTableEntry {
                    identifier: "KEY_doubledagger",
                    name: "doubledagger",
                    keycode: 2802,
                },
                KeynameTableEntry {
                    identifier: "KEY_checkmark",
                    name: "checkmark",
                    keycode: 2803,
                },
                KeynameTableEntry {
                    identifier: "KEY_ballotcross",
                    name: "ballotcross",
                    keycode: 2804,
                },
                KeynameTableEntry {
                    identifier: "KEY_musicalsharp",
                    name: "musicalsharp",
                    keycode: 2805,
                },
                KeynameTableEntry {
                    identifier: "KEY_musicalflat",
                    name: "musicalflat",
                    keycode: 2806,
                },
                KeynameTableEntry {
                    identifier: "KEY_malesymbol",
                    name: "malesymbol",
                    keycode: 2807,
                },
                KeynameTableEntry {
                    identifier: "KEY_femalesymbol",
                    name: "femalesymbol",
                    keycode: 2808,
                },
                KeynameTableEntry {
                    identifier: "KEY_telephone",
                    name: "telephone",
                    keycode: 2809,
                },
                KeynameTableEntry {
                    identifier: "KEY_telephonerecorder",
                    name: "telephonerecorder",
                    keycode: 2810,
                },
                KeynameTableEntry {
                    identifier: "KEY_phonographcopyright",
                    name: "phonographcopyright",
                    keycode: 2811,
                },
                KeynameTableEntry {
                    identifier: "KEY_caret",
                    name: "caret",
                    keycode: 2812,
                },
                KeynameTableEntry {
                    identifier: "KEY_singlelowquotemark",
                    name: "singlelowquotemark",
                    keycode: 2813,
                },
                KeynameTableEntry {
                    identifier: "KEY_doublelowquotemark",
                    name: "doublelowquotemark",
                    keycode: 2814,
                },
                KeynameTableEntry {
                    identifier: "KEY_cursor",
                    name: "cursor",
                    keycode: 2815,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftcaret",
                    name: "leftcaret",
                    keycode: 2979,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightcaret",
                    name: "rightcaret",
                    keycode: 2982,
                },
                KeynameTableEntry {
                    identifier: "KEY_downcaret",
                    name: "downcaret",
                    keycode: 2984,
                },
                KeynameTableEntry {
                    identifier: "KEY_upcaret",
                    name: "upcaret",
                    keycode: 2985,
                },
                KeynameTableEntry {
                    identifier: "KEY_overbar",
                    name: "overbar",
                    keycode: 3008,
                },
                KeynameTableEntry {
                    identifier: "KEY_downtack",
                    name: "downtack",
                    keycode: 3010,
                },
                KeynameTableEntry {
                    identifier: "KEY_upshoe",
                    name: "upshoe",
                    keycode: 3011,
                },
                KeynameTableEntry {
                    identifier: "KEY_downstile",
                    name: "downstile",
                    keycode: 3012,
                },
                KeynameTableEntry {
                    identifier: "KEY_underbar",
                    name: "underbar",
                    keycode: 3014,
                },
                KeynameTableEntry {
                    identifier: "KEY_jot",
                    name: "jot",
                    keycode: 3018,
                },
                KeynameTableEntry {
                    identifier: "KEY_quad",
                    name: "quad",
                    keycode: 3020,
                },
                KeynameTableEntry {
                    identifier: "KEY_uptack",
                    name: "uptack",
                    keycode: 3022,
                },
                KeynameTableEntry {
                    identifier: "KEY_circle",
                    name: "circle",
                    keycode: 3023,
                },
                KeynameTableEntry {
                    identifier: "KEY_upstile",
                    name: "upstile",
                    keycode: 3027,
                },
                KeynameTableEntry {
                    identifier: "KEY_downshoe",
                    name: "downshoe",
                    keycode: 3030,
                },
                KeynameTableEntry {
                    identifier: "KEY_rightshoe",
                    name: "rightshoe",
                    keycode: 3032,
                },
                KeynameTableEntry {
                    identifier: "KEY_leftshoe",
                    name: "leftshoe",
                    keycode: 3034,
                },
                KeynameTableEntry {
                    identifier: "KEY_lefttack",
                    name: "lefttack",
                    keycode: 3036,
                },
                KeynameTableEntry {
                    identifier: "KEY_righttack",
                    name: "righttack",
                    keycode: 3068,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_doublelowline",
                    name: "hebrew_doublelowline",
                    keycode: 3295,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_aleph",
                    name: "hebrew_aleph",
                    keycode: 3296,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_bet",
                    name: "hebrew_bet",
                    keycode: 3297,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_beth",
                    name: "hebrew_beth",
                    keycode: 3297,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_gimel",
                    name: "hebrew_gimel",
                    keycode: 3298,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_gimmel",
                    name: "hebrew_gimmel",
                    keycode: 3298,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_dalet",
                    name: "hebrew_dalet",
                    keycode: 3299,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_daleth",
                    name: "hebrew_daleth",
                    keycode: 3299,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_he",
                    name: "hebrew_he",
                    keycode: 3300,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_waw",
                    name: "hebrew_waw",
                    keycode: 3301,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_zain",
                    name: "hebrew_zain",
                    keycode: 3302,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_zayin",
                    name: "hebrew_zayin",
                    keycode: 3302,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_chet",
                    name: "hebrew_chet",
                    keycode: 3303,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_het",
                    name: "hebrew_het",
                    keycode: 3303,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_tet",
                    name: "hebrew_tet",
                    keycode: 3304,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_teth",
                    name: "hebrew_teth",
                    keycode: 3304,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_yod",
                    name: "hebrew_yod",
                    keycode: 3305,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalkaph",
                    name: "hebrew_finalkaph",
                    keycode: 3306,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_kaph",
                    name: "hebrew_kaph",
                    keycode: 3307,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_lamed",
                    name: "hebrew_lamed",
                    keycode: 3308,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalmem",
                    name: "hebrew_finalmem",
                    keycode: 3309,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_mem",
                    name: "hebrew_mem",
                    keycode: 3310,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalnun",
                    name: "hebrew_finalnun",
                    keycode: 3311,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_nun",
                    name: "hebrew_nun",
                    keycode: 3312,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_samech",
                    name: "hebrew_samech",
                    keycode: 3313,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_samekh",
                    name: "hebrew_samekh",
                    keycode: 3313,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_ayin",
                    name: "hebrew_ayin",
                    keycode: 3314,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalpe",
                    name: "hebrew_finalpe",
                    keycode: 3315,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_pe",
                    name: "hebrew_pe",
                    keycode: 3316,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalzade",
                    name: "hebrew_finalzade",
                    keycode: 3317,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_finalzadi",
                    name: "hebrew_finalzadi",
                    keycode: 3317,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_zade",
                    name: "hebrew_zade",
                    keycode: 3318,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_zadi",
                    name: "hebrew_zadi",
                    keycode: 3318,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_kuf",
                    name: "hebrew_kuf",
                    keycode: 3319,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_qoph",
                    name: "hebrew_qoph",
                    keycode: 3319,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_resh",
                    name: "hebrew_resh",
                    keycode: 3320,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_shin",
                    name: "hebrew_shin",
                    keycode: 3321,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_taf",
                    name: "hebrew_taf",
                    keycode: 3322,
                },
                KeynameTableEntry {
                    identifier: "KEY_hebrew_taw",
                    name: "hebrew_taw",
                    keycode: 3322,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_kokai",
                    name: "Thai_kokai",
                    keycode: 3489,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_khokhai",
                    name: "Thai_khokhai",
                    keycode: 3490,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_khokhuat",
                    name: "Thai_khokhuat",
                    keycode: 3491,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_khokhwai",
                    name: "Thai_khokhwai",
                    keycode: 3492,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_khokhon",
                    name: "Thai_khokhon",
                    keycode: 3493,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_khorakhang",
                    name: "Thai_khorakhang",
                    keycode: 3494,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_ngongu",
                    name: "Thai_ngongu",
                    keycode: 3495,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_chochan",
                    name: "Thai_chochan",
                    keycode: 3496,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_choching",
                    name: "Thai_choching",
                    keycode: 3497,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_chochang",
                    name: "Thai_chochang",
                    keycode: 3498,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_soso",
                    name: "Thai_soso",
                    keycode: 3499,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_chochoe",
                    name: "Thai_chochoe",
                    keycode: 3500,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_yoying",
                    name: "Thai_yoying",
                    keycode: 3501,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_dochada",
                    name: "Thai_dochada",
                    keycode: 3502,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_topatak",
                    name: "Thai_topatak",
                    keycode: 3503,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thothan",
                    name: "Thai_thothan",
                    keycode: 3504,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thonangmontho",
                    name: "Thai_thonangmontho",
                    keycode: 3505,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thophuthao",
                    name: "Thai_thophuthao",
                    keycode: 3506,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_nonen",
                    name: "Thai_nonen",
                    keycode: 3507,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_dodek",
                    name: "Thai_dodek",
                    keycode: 3508,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_totao",
                    name: "Thai_totao",
                    keycode: 3509,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thothung",
                    name: "Thai_thothung",
                    keycode: 3510,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thothahan",
                    name: "Thai_thothahan",
                    keycode: 3511,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thothong",
                    name: "Thai_thothong",
                    keycode: 3512,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_nonu",
                    name: "Thai_nonu",
                    keycode: 3513,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_bobaimai",
                    name: "Thai_bobaimai",
                    keycode: 3514,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_popla",
                    name: "Thai_popla",
                    keycode: 3515,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_phophung",
                    name: "Thai_phophung",
                    keycode: 3516,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_fofa",
                    name: "Thai_fofa",
                    keycode: 3517,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_phophan",
                    name: "Thai_phophan",
                    keycode: 3518,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_fofan",
                    name: "Thai_fofan",
                    keycode: 3519,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_phosamphao",
                    name: "Thai_phosamphao",
                    keycode: 3520,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_moma",
                    name: "Thai_moma",
                    keycode: 3521,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_yoyak",
                    name: "Thai_yoyak",
                    keycode: 3522,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_rorua",
                    name: "Thai_rorua",
                    keycode: 3523,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_ru",
                    name: "Thai_ru",
                    keycode: 3524,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_loling",
                    name: "Thai_loling",
                    keycode: 3525,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lu",
                    name: "Thai_lu",
                    keycode: 3526,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_wowaen",
                    name: "Thai_wowaen",
                    keycode: 3527,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sosala",
                    name: "Thai_sosala",
                    keycode: 3528,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sorusi",
                    name: "Thai_sorusi",
                    keycode: 3529,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sosua",
                    name: "Thai_sosua",
                    keycode: 3530,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_hohip",
                    name: "Thai_hohip",
                    keycode: 3531,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lochula",
                    name: "Thai_lochula",
                    keycode: 3532,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_oang",
                    name: "Thai_oang",
                    keycode: 3533,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_honokhuk",
                    name: "Thai_honokhuk",
                    keycode: 3534,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_paiyannoi",
                    name: "Thai_paiyannoi",
                    keycode: 3535,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraa",
                    name: "Thai_saraa",
                    keycode: 3536,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maihanakat",
                    name: "Thai_maihanakat",
                    keycode: 3537,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraaa",
                    name: "Thai_saraaa",
                    keycode: 3538,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraam",
                    name: "Thai_saraam",
                    keycode: 3539,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarai",
                    name: "Thai_sarai",
                    keycode: 3540,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraii",
                    name: "Thai_saraii",
                    keycode: 3541,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraue",
                    name: "Thai_saraue",
                    keycode: 3542,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarauee",
                    name: "Thai_sarauee",
                    keycode: 3543,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarau",
                    name: "Thai_sarau",
                    keycode: 3544,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarauu",
                    name: "Thai_sarauu",
                    keycode: 3545,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_phinthu",
                    name: "Thai_phinthu",
                    keycode: 3546,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maihanakat_maitho",
                    name: "Thai_maihanakat_maitho",
                    keycode: 3550,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_baht",
                    name: "Thai_baht",
                    keycode: 3551,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarae",
                    name: "Thai_sarae",
                    keycode: 3552,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraae",
                    name: "Thai_saraae",
                    keycode: 3553,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_sarao",
                    name: "Thai_sarao",
                    keycode: 3554,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraaimaimuan",
                    name: "Thai_saraaimaimuan",
                    keycode: 3555,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_saraaimaimalai",
                    name: "Thai_saraaimaimalai",
                    keycode: 3556,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lakkhangyao",
                    name: "Thai_lakkhangyao",
                    keycode: 3557,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maiyamok",
                    name: "Thai_maiyamok",
                    keycode: 3558,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maitaikhu",
                    name: "Thai_maitaikhu",
                    keycode: 3559,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maiek",
                    name: "Thai_maiek",
                    keycode: 3560,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maitho",
                    name: "Thai_maitho",
                    keycode: 3561,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maitri",
                    name: "Thai_maitri",
                    keycode: 3562,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_maichattawa",
                    name: "Thai_maichattawa",
                    keycode: 3563,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_thanthakhat",
                    name: "Thai_thanthakhat",
                    keycode: 3564,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_nikhahit",
                    name: "Thai_nikhahit",
                    keycode: 3565,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_leksun",
                    name: "Thai_leksun",
                    keycode: 3568,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_leknung",
                    name: "Thai_leknung",
                    keycode: 3569,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_leksong",
                    name: "Thai_leksong",
                    keycode: 3570,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_leksam",
                    name: "Thai_leksam",
                    keycode: 3571,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_leksi",
                    name: "Thai_leksi",
                    keycode: 3572,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lekha",
                    name: "Thai_lekha",
                    keycode: 3573,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lekhok",
                    name: "Thai_lekhok",
                    keycode: 3574,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lekchet",
                    name: "Thai_lekchet",
                    keycode: 3575,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lekpaet",
                    name: "Thai_lekpaet",
                    keycode: 3576,
                },
                KeynameTableEntry {
                    identifier: "KEY_Thai_lekkao",
                    name: "Thai_lekkao",
                    keycode: 3577,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Kiyeog",
                    name: "Hangul_Kiyeog",
                    keycode: 3745,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SsangKiyeog",
                    name: "Hangul_SsangKiyeog",
                    keycode: 3746,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_KiyeogSios",
                    name: "Hangul_KiyeogSios",
                    keycode: 3747,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Nieun",
                    name: "Hangul_Nieun",
                    keycode: 3748,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_NieunJieuj",
                    name: "Hangul_NieunJieuj",
                    keycode: 3749,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_NieunHieuh",
                    name: "Hangul_NieunHieuh",
                    keycode: 3750,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Dikeud",
                    name: "Hangul_Dikeud",
                    keycode: 3751,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SsangDikeud",
                    name: "Hangul_SsangDikeud",
                    keycode: 3752,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Rieul",
                    name: "Hangul_Rieul",
                    keycode: 3753,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulKiyeog",
                    name: "Hangul_RieulKiyeog",
                    keycode: 3754,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulMieum",
                    name: "Hangul_RieulMieum",
                    keycode: 3755,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulPieub",
                    name: "Hangul_RieulPieub",
                    keycode: 3756,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulSios",
                    name: "Hangul_RieulSios",
                    keycode: 3757,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulTieut",
                    name: "Hangul_RieulTieut",
                    keycode: 3758,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulPhieuf",
                    name: "Hangul_RieulPhieuf",
                    keycode: 3759,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulHieuh",
                    name: "Hangul_RieulHieuh",
                    keycode: 3760,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Mieum",
                    name: "Hangul_Mieum",
                    keycode: 3761,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Pieub",
                    name: "Hangul_Pieub",
                    keycode: 3762,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SsangPieub",
                    name: "Hangul_SsangPieub",
                    keycode: 3763,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_PieubSios",
                    name: "Hangul_PieubSios",
                    keycode: 3764,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Sios",
                    name: "Hangul_Sios",
                    keycode: 3765,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SsangSios",
                    name: "Hangul_SsangSios",
                    keycode: 3766,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Ieung",
                    name: "Hangul_Ieung",
                    keycode: 3767,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Jieuj",
                    name: "Hangul_Jieuj",
                    keycode: 3768,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SsangJieuj",
                    name: "Hangul_SsangJieuj",
                    keycode: 3769,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Cieuc",
                    name: "Hangul_Cieuc",
                    keycode: 3770,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Khieuq",
                    name: "Hangul_Khieuq",
                    keycode: 3771,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Tieut",
                    name: "Hangul_Tieut",
                    keycode: 3772,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Phieuf",
                    name: "Hangul_Phieuf",
                    keycode: 3773,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Hieuh",
                    name: "Hangul_Hieuh",
                    keycode: 3774,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_A",
                    name: "Hangul_A",
                    keycode: 3775,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_AE",
                    name: "Hangul_AE",
                    keycode: 3776,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YA",
                    name: "Hangul_YA",
                    keycode: 3777,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YAE",
                    name: "Hangul_YAE",
                    keycode: 3778,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_EO",
                    name: "Hangul_EO",
                    keycode: 3779,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_E",
                    name: "Hangul_E",
                    keycode: 3780,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YEO",
                    name: "Hangul_YEO",
                    keycode: 3781,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YE",
                    name: "Hangul_YE",
                    keycode: 3782,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_O",
                    name: "Hangul_O",
                    keycode: 3783,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_WA",
                    name: "Hangul_WA",
                    keycode: 3784,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_WAE",
                    name: "Hangul_WAE",
                    keycode: 3785,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_OE",
                    name: "Hangul_OE",
                    keycode: 3786,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YO",
                    name: "Hangul_YO",
                    keycode: 3787,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_U",
                    name: "Hangul_U",
                    keycode: 3788,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_WEO",
                    name: "Hangul_WEO",
                    keycode: 3789,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_WE",
                    name: "Hangul_WE",
                    keycode: 3790,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_WI",
                    name: "Hangul_WI",
                    keycode: 3791,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YU",
                    name: "Hangul_YU",
                    keycode: 3792,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_EU",
                    name: "Hangul_EU",
                    keycode: 3793,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YI",
                    name: "Hangul_YI",
                    keycode: 3794,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_I",
                    name: "Hangul_I",
                    keycode: 3795,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Kiyeog",
                    name: "Hangul_J_Kiyeog",
                    keycode: 3796,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_SsangKiyeog",
                    name: "Hangul_J_SsangKiyeog",
                    keycode: 3797,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_KiyeogSios",
                    name: "Hangul_J_KiyeogSios",
                    keycode: 3798,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Nieun",
                    name: "Hangul_J_Nieun",
                    keycode: 3799,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_NieunJieuj",
                    name: "Hangul_J_NieunJieuj",
                    keycode: 3800,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_NieunHieuh",
                    name: "Hangul_J_NieunHieuh",
                    keycode: 3801,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Dikeud",
                    name: "Hangul_J_Dikeud",
                    keycode: 3802,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Rieul",
                    name: "Hangul_J_Rieul",
                    keycode: 3803,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulKiyeog",
                    name: "Hangul_J_RieulKiyeog",
                    keycode: 3804,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulMieum",
                    name: "Hangul_J_RieulMieum",
                    keycode: 3805,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulPieub",
                    name: "Hangul_J_RieulPieub",
                    keycode: 3806,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulSios",
                    name: "Hangul_J_RieulSios",
                    keycode: 3807,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulTieut",
                    name: "Hangul_J_RieulTieut",
                    keycode: 3808,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulPhieuf",
                    name: "Hangul_J_RieulPhieuf",
                    keycode: 3809,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_RieulHieuh",
                    name: "Hangul_J_RieulHieuh",
                    keycode: 3810,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Mieum",
                    name: "Hangul_J_Mieum",
                    keycode: 3811,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Pieub",
                    name: "Hangul_J_Pieub",
                    keycode: 3812,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_PieubSios",
                    name: "Hangul_J_PieubSios",
                    keycode: 3813,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Sios",
                    name: "Hangul_J_Sios",
                    keycode: 3814,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_SsangSios",
                    name: "Hangul_J_SsangSios",
                    keycode: 3815,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Ieung",
                    name: "Hangul_J_Ieung",
                    keycode: 3816,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Jieuj",
                    name: "Hangul_J_Jieuj",
                    keycode: 3817,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Cieuc",
                    name: "Hangul_J_Cieuc",
                    keycode: 3818,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Khieuq",
                    name: "Hangul_J_Khieuq",
                    keycode: 3819,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Tieut",
                    name: "Hangul_J_Tieut",
                    keycode: 3820,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Phieuf",
                    name: "Hangul_J_Phieuf",
                    keycode: 3821,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_Hieuh",
                    name: "Hangul_J_Hieuh",
                    keycode: 3822,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_RieulYeorinHieuh",
                    name: "Hangul_RieulYeorinHieuh",
                    keycode: 3823,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SunkyeongeumMieum",
                    name: "Hangul_SunkyeongeumMieum",
                    keycode: 3824,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SunkyeongeumPieub",
                    name: "Hangul_SunkyeongeumPieub",
                    keycode: 3825,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_PanSios",
                    name: "Hangul_PanSios",
                    keycode: 3826,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_KkogjiDalrinIeung",
                    name: "Hangul_KkogjiDalrinIeung",
                    keycode: 3827,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_SunkyeongeumPhieuf",
                    name: "Hangul_SunkyeongeumPhieuf",
                    keycode: 3828,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_YeorinHieuh",
                    name: "Hangul_YeorinHieuh",
                    keycode: 3829,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_AraeA",
                    name: "Hangul_AraeA",
                    keycode: 3830,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_AraeAE",
                    name: "Hangul_AraeAE",
                    keycode: 3831,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_PanSios",
                    name: "Hangul_J_PanSios",
                    keycode: 3832,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_KkogjiDalrinIeung",
                    name: "Hangul_J_KkogjiDalrinIeung",
                    keycode: 3833,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_J_YeorinHieuh",
                    name: "Hangul_J_YeorinHieuh",
                    keycode: 3834,
                },
                KeynameTableEntry {
                    identifier: "KEY_Korean_Won",
                    name: "Korean_Won",
                    keycode: 3839,
                },
                KeynameTableEntry {
                    identifier: "KEY_OE",
                    name: "OE",
                    keycode: 5052,
                },
                KeynameTableEntry {
                    identifier: "KEY_oe",
                    name: "oe",
                    keycode: 5053,
                },
                KeynameTableEntry {
                    identifier: "KEY_Ydiaeresis",
                    name: "Ydiaeresis",
                    keycode: 5054,
                },
                KeynameTableEntry {
                    identifier: "KEY_EcuSign",
                    name: "EcuSign",
                    keycode: 8352,
                },
                KeynameTableEntry {
                    identifier: "KEY_ColonSign",
                    name: "ColonSign",
                    keycode: 8353,
                },
                KeynameTableEntry {
                    identifier: "KEY_CruzeiroSign",
                    name: "CruzeiroSign",
                    keycode: 8354,
                },
                KeynameTableEntry {
                    identifier: "KEY_FFrancSign",
                    name: "FFrancSign",
                    keycode: 8355,
                },
                KeynameTableEntry {
                    identifier: "KEY_LiraSign",
                    name: "LiraSign",
                    keycode: 8356,
                },
                KeynameTableEntry {
                    identifier: "KEY_MillSign",
                    name: "MillSign",
                    keycode: 8357,
                },
                KeynameTableEntry {
                    identifier: "KEY_NairaSign",
                    name: "NairaSign",
                    keycode: 8358,
                },
                KeynameTableEntry {
                    identifier: "KEY_PesetaSign",
                    name: "PesetaSign",
                    keycode: 8359,
                },
                KeynameTableEntry {
                    identifier: "KEY_RupeeSign",
                    name: "RupeeSign",
                    keycode: 8360,
                },
                KeynameTableEntry {
                    identifier: "KEY_WonSign",
                    name: "WonSign",
                    keycode: 8361,
                },
                KeynameTableEntry {
                    identifier: "KEY_NewSheqelSign",
                    name: "NewSheqelSign",
                    keycode: 8362,
                },
                KeynameTableEntry {
                    identifier: "KEY_DongSign",
                    name: "DongSign",
                    keycode: 8363,
                },
                KeynameTableEntry {
                    identifier: "KEY_EuroSign",
                    name: "EuroSign",
                    keycode: 8364,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Duplicate",
                    name: "3270_Duplicate",
                    keycode: 64769,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_FieldMark",
                    name: "3270_FieldMark",
                    keycode: 64770,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Right2",
                    name: "3270_Right2",
                    keycode: 64771,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Left2",
                    name: "3270_Left2",
                    keycode: 64772,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_BackTab",
                    name: "3270_BackTab",
                    keycode: 64773,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_EraseEOF",
                    name: "3270_EraseEOF",
                    keycode: 64774,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_EraseInput",
                    name: "3270_EraseInput",
                    keycode: 64775,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Reset",
                    name: "3270_Reset",
                    keycode: 64776,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Quit",
                    name: "3270_Quit",
                    keycode: 64777,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_PA1",
                    name: "3270_PA1",
                    keycode: 64778,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_PA2",
                    name: "3270_PA2",
                    keycode: 64779,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_PA3",
                    name: "3270_PA3",
                    keycode: 64780,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Test",
                    name: "3270_Test",
                    keycode: 64781,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Attn",
                    name: "3270_Attn",
                    keycode: 64782,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_CursorBlink",
                    name: "3270_CursorBlink",
                    keycode: 64783,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_AltCursor",
                    name: "3270_AltCursor",
                    keycode: 64784,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_KeyClick",
                    name: "3270_KeyClick",
                    keycode: 64785,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Jump",
                    name: "3270_Jump",
                    keycode: 64786,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Ident",
                    name: "3270_Ident",
                    keycode: 64787,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Rule",
                    name: "3270_Rule",
                    keycode: 64788,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Copy",
                    name: "3270_Copy",
                    keycode: 64789,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Play",
                    name: "3270_Play",
                    keycode: 64790,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Setup",
                    name: "3270_Setup",
                    keycode: 64791,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Record",
                    name: "3270_Record",
                    keycode: 64792,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_ChangeScreen",
                    name: "3270_ChangeScreen",
                    keycode: 64793,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_DeleteWord",
                    name: "3270_DeleteWord",
                    keycode: 64794,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_ExSelect",
                    name: "3270_ExSelect",
                    keycode: 64795,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_CursorSelect",
                    name: "3270_CursorSelect",
                    keycode: 64796,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_PrintScreen",
                    name: "3270_PrintScreen",
                    keycode: 64797,
                },
                KeynameTableEntry {
                    identifier: "KEY_3270_Enter",
                    name: "3270_Enter",
                    keycode: 64798,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Lock",
                    name: "ISO_Lock",
                    keycode: 65025,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Level2_Latch",
                    name: "ISO_Level2_Latch",
                    keycode: 65026,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Level3_Shift",
                    name: "ISO_Level3_Shift",
                    keycode: 65027,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Level3_Latch",
                    name: "ISO_Level3_Latch",
                    keycode: 65028,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Level3_Lock",
                    name: "ISO_Level3_Lock",
                    keycode: 65029,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Group_Latch",
                    name: "ISO_Group_Latch",
                    keycode: 65030,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Group_Lock",
                    name: "ISO_Group_Lock",
                    keycode: 65031,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Next_Group",
                    name: "ISO_Next_Group",
                    keycode: 65032,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Next_Group_Lock",
                    name: "ISO_Next_Group_Lock",
                    keycode: 65033,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Prev_Group",
                    name: "ISO_Prev_Group",
                    keycode: 65034,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Prev_Group_Lock",
                    name: "ISO_Prev_Group_Lock",
                    keycode: 65035,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_First_Group",
                    name: "ISO_First_Group",
                    keycode: 65036,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_First_Group_Lock",
                    name: "ISO_First_Group_Lock",
                    keycode: 65037,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Last_Group",
                    name: "ISO_Last_Group",
                    keycode: 65038,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Last_Group_Lock",
                    name: "ISO_Last_Group_Lock",
                    keycode: 65039,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Left_Tab",
                    name: "ISO_Left_Tab",
                    keycode: 65056,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Move_Line_Up",
                    name: "ISO_Move_Line_Up",
                    keycode: 65057,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Move_Line_Down",
                    name: "ISO_Move_Line_Down",
                    keycode: 65058,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Partial_Line_Up",
                    name: "ISO_Partial_Line_Up",
                    keycode: 65059,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Partial_Line_Down",
                    name: "ISO_Partial_Line_Down",
                    keycode: 65060,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Partial_Space_Left",
                    name: "ISO_Partial_Space_Left",
                    keycode: 65061,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Partial_Space_Right",
                    name: "ISO_Partial_Space_Right",
                    keycode: 65062,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Set_Margin_Left",
                    name: "ISO_Set_Margin_Left",
                    keycode: 65063,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Set_Margin_Right",
                    name: "ISO_Set_Margin_Right",
                    keycode: 65064,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Release_Margin_Left",
                    name: "ISO_Release_Margin_Left",
                    keycode: 65065,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Release_Margin_Right",
                    name: "ISO_Release_Margin_Right",
                    keycode: 65066,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Release_Both_Margins",
                    name: "ISO_Release_Both_Margins",
                    keycode: 65067,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Fast_Cursor_Left",
                    name: "ISO_Fast_Cursor_Left",
                    keycode: 65068,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Fast_Cursor_Right",
                    name: "ISO_Fast_Cursor_Right",
                    keycode: 65069,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Fast_Cursor_Up",
                    name: "ISO_Fast_Cursor_Up",
                    keycode: 65070,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Fast_Cursor_Down",
                    name: "ISO_Fast_Cursor_Down",
                    keycode: 65071,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Continuous_Underline",
                    name: "ISO_Continuous_Underline",
                    keycode: 65072,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Discontinuous_Underline",
                    name: "ISO_Discontinuous_Underline",
                    keycode: 65073,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Emphasize",
                    name: "ISO_Emphasize",
                    keycode: 65074,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Center_Object",
                    name: "ISO_Center_Object",
                    keycode: 65075,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Enter",
                    name: "ISO_Enter",
                    keycode: 65076,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_grave",
                    name: "dead_grave",
                    keycode: 65104,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_acute",
                    name: "dead_acute",
                    keycode: 65105,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_circumflex",
                    name: "dead_circumflex",
                    keycode: 65106,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_tilde",
                    name: "dead_tilde",
                    keycode: 65107,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_macron",
                    name: "dead_macron",
                    keycode: 65108,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_breve",
                    name: "dead_breve",
                    keycode: 65109,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_abovedot",
                    name: "dead_abovedot",
                    keycode: 65110,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_diaeresis",
                    name: "dead_diaeresis",
                    keycode: 65111,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_abovering",
                    name: "dead_abovering",
                    keycode: 65112,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_doubleacute",
                    name: "dead_doubleacute",
                    keycode: 65113,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_caron",
                    name: "dead_caron",
                    keycode: 65114,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_cedilla",
                    name: "dead_cedilla",
                    keycode: 65115,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_ogonek",
                    name: "dead_ogonek",
                    keycode: 65116,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_iota",
                    name: "dead_iota",
                    keycode: 65117,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_voiced_sound",
                    name: "dead_voiced_sound",
                    keycode: 65118,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_semivoiced_sound",
                    name: "dead_semivoiced_sound",
                    keycode: 65119,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_belowdot",
                    name: "dead_belowdot",
                    keycode: 65120,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_hook",
                    name: "dead_hook",
                    keycode: 65121,
                },
                KeynameTableEntry {
                    identifier: "KEY_dead_horn",
                    name: "dead_horn",
                    keycode: 65122,
                },
                KeynameTableEntry {
                    identifier: "KEY_AccessX_Enable",
                    name: "AccessX_Enable",
                    keycode: 65136,
                },
                KeynameTableEntry {
                    identifier: "KEY_AccessX_Feedback_Enable",
                    name: "AccessX_Feedback_Enable",
                    keycode: 65137,
                },
                KeynameTableEntry {
                    identifier: "KEY_RepeatKeys_Enable",
                    name: "RepeatKeys_Enable",
                    keycode: 65138,
                },
                KeynameTableEntry {
                    identifier: "KEY_SlowKeys_Enable",
                    name: "SlowKeys_Enable",
                    keycode: 65139,
                },
                KeynameTableEntry {
                    identifier: "KEY_BounceKeys_Enable",
                    name: "BounceKeys_Enable",
                    keycode: 65140,
                },
                KeynameTableEntry {
                    identifier: "KEY_StickyKeys_Enable",
                    name: "StickyKeys_Enable",
                    keycode: 65141,
                },
                KeynameTableEntry {
                    identifier: "KEY_MouseKeys_Enable",
                    name: "MouseKeys_Enable",
                    keycode: 65142,
                },
                KeynameTableEntry {
                    identifier: "KEY_MouseKeys_Accel_Enable",
                    name: "MouseKeys_Accel_Enable",
                    keycode: 65143,
                },
                KeynameTableEntry {
                    identifier: "KEY_Overlay1_Enable",
                    name: "Overlay1_Enable",
                    keycode: 65144,
                },
                KeynameTableEntry {
                    identifier: "KEY_Overlay2_Enable",
                    name: "Overlay2_Enable",
                    keycode: 65145,
                },
                KeynameTableEntry {
                    identifier: "KEY_AudibleBell_Enable",
                    name: "AudibleBell_Enable",
                    keycode: 65146,
                },
                KeynameTableEntry {
                    identifier: "KEY_First_Virtual_Screen",
                    name: "First_Virtual_Screen",
                    keycode: 65232,
                },
                KeynameTableEntry {
                    identifier: "KEY_Prev_Virtual_Screen",
                    name: "Prev_Virtual_Screen",
                    keycode: 65233,
                },
                KeynameTableEntry {
                    identifier: "KEY_Next_Virtual_Screen",
                    name: "Next_Virtual_Screen",
                    keycode: 65234,
                },
                KeynameTableEntry {
                    identifier: "KEY_Last_Virtual_Screen",
                    name: "Last_Virtual_Screen",
                    keycode: 65236,
                },
                KeynameTableEntry {
                    identifier: "KEY_Terminate_Server",
                    name: "Terminate_Server",
                    keycode: 65237,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Left",
                    name: "Pointer_Left",
                    keycode: 65248,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Right",
                    name: "Pointer_Right",
                    keycode: 65249,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Up",
                    name: "Pointer_Up",
                    keycode: 65250,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Down",
                    name: "Pointer_Down",
                    keycode: 65251,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_UpLeft",
                    name: "Pointer_UpLeft",
                    keycode: 65252,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_UpRight",
                    name: "Pointer_UpRight",
                    keycode: 65253,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DownLeft",
                    name: "Pointer_DownLeft",
                    keycode: 65254,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DownRight",
                    name: "Pointer_DownRight",
                    keycode: 65255,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button_Dflt",
                    name: "Pointer_Button_Dflt",
                    keycode: 65256,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button1",
                    name: "Pointer_Button1",
                    keycode: 65257,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button2",
                    name: "Pointer_Button2",
                    keycode: 65258,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button3",
                    name: "Pointer_Button3",
                    keycode: 65259,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button4",
                    name: "Pointer_Button4",
                    keycode: 65260,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Button5",
                    name: "Pointer_Button5",
                    keycode: 65261,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick_Dflt",
                    name: "Pointer_DblClick_Dflt",
                    keycode: 65262,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick1",
                    name: "Pointer_DblClick1",
                    keycode: 65263,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick2",
                    name: "Pointer_DblClick2",
                    keycode: 65264,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick3",
                    name: "Pointer_DblClick3",
                    keycode: 65265,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick4",
                    name: "Pointer_DblClick4",
                    keycode: 65266,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DblClick5",
                    name: "Pointer_DblClick5",
                    keycode: 65267,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag_Dflt",
                    name: "Pointer_Drag_Dflt",
                    keycode: 65268,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag1",
                    name: "Pointer_Drag1",
                    keycode: 65269,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag2",
                    name: "Pointer_Drag2",
                    keycode: 65270,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag3",
                    name: "Pointer_Drag3",
                    keycode: 65271,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag4",
                    name: "Pointer_Drag4",
                    keycode: 65272,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_EnableKeys",
                    name: "Pointer_EnableKeys",
                    keycode: 65273,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Accelerate",
                    name: "Pointer_Accelerate",
                    keycode: 65274,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DfltBtnNext",
                    name: "Pointer_DfltBtnNext",
                    keycode: 65275,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_DfltBtnPrev",
                    name: "Pointer_DfltBtnPrev",
                    keycode: 65276,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pointer_Drag5",
                    name: "Pointer_Drag5",
                    keycode: 65277,
                },
                KeynameTableEntry {
                    identifier: "KEY_BackSpace",
                    name: "BackSpace",
                    keycode: 65288,
                },
                KeynameTableEntry {
                    identifier: "KEY_Tab",
                    name: "Tab",
                    keycode: 65289,
                },
                KeynameTableEntry {
                    identifier: "KEY_Linefeed",
                    name: "Linefeed",
                    keycode: 65290,
                },
                KeynameTableEntry {
                    identifier: "KEY_Clear",
                    name: "Clear",
                    keycode: 65291,
                },
                KeynameTableEntry {
                    identifier: "KEY_Return",
                    name: "Return",
                    keycode: 65293,
                },
                KeynameTableEntry {
                    identifier: "KEY_Pause",
                    name: "Pause",
                    keycode: 65299,
                },
                KeynameTableEntry {
                    identifier: "KEY_Scroll_Lock",
                    name: "Scroll_Lock",
                    keycode: 65300,
                },
                KeynameTableEntry {
                    identifier: "KEY_Sys_Req",
                    name: "Sys_Req",
                    keycode: 65301,
                },
                KeynameTableEntry {
                    identifier: "KEY_Escape",
                    name: "Escape",
                    keycode: 65307,
                },
                KeynameTableEntry {
                    identifier: "KEY_Multi_key",
                    name: "Multi_key",
                    keycode: 65312,
                },
                KeynameTableEntry {
                    identifier: "KEY_Kanji",
                    name: "Kanji",
                    keycode: 65313,
                },
                KeynameTableEntry {
                    identifier: "KEY_Muhenkan",
                    name: "Muhenkan",
                    keycode: 65314,
                },
                KeynameTableEntry {
                    identifier: "KEY_Henkan",
                    name: "Henkan",
                    keycode: 65315,
                },
                KeynameTableEntry {
                    identifier: "KEY_Henkan_Mode",
                    name: "Henkan_Mode",
                    keycode: 65315,
                },
                KeynameTableEntry {
                    identifier: "KEY_Romaji",
                    name: "Romaji",
                    keycode: 65316,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hiragana",
                    name: "Hiragana",
                    keycode: 65317,
                },
                KeynameTableEntry {
                    identifier: "KEY_Katakana",
                    name: "Katakana",
                    keycode: 65318,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hiragana_Katakana",
                    name: "Hiragana_Katakana",
                    keycode: 65319,
                },
                KeynameTableEntry {
                    identifier: "KEY_Zenkaku",
                    name: "Zenkaku",
                    keycode: 65320,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hankaku",
                    name: "Hankaku",
                    keycode: 65321,
                },
                KeynameTableEntry {
                    identifier: "KEY_Zenkaku_Hankaku",
                    name: "Zenkaku_Hankaku",
                    keycode: 65322,
                },
                KeynameTableEntry {
                    identifier: "KEY_Touroku",
                    name: "Touroku",
                    keycode: 65323,
                },
                KeynameTableEntry {
                    identifier: "KEY_Massyo",
                    name: "Massyo",
                    keycode: 65324,
                },
                KeynameTableEntry {
                    identifier: "KEY_Kana_Lock",
                    name: "Kana_Lock",
                    keycode: 65325,
                },
                KeynameTableEntry {
                    identifier: "KEY_Kana_Shift",
                    name: "Kana_Shift",
                    keycode: 65326,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eisu_Shift",
                    name: "Eisu_Shift",
                    keycode: 65327,
                },
                KeynameTableEntry {
                    identifier: "KEY_Eisu_toggle",
                    name: "Eisu_toggle",
                    keycode: 65328,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul",
                    name: "Hangul",
                    keycode: 65329,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Start",
                    name: "Hangul_Start",
                    keycode: 65330,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_End",
                    name: "Hangul_End",
                    keycode: 65331,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Hanja",
                    name: "Hangul_Hanja",
                    keycode: 65332,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Jamo",
                    name: "Hangul_Jamo",
                    keycode: 65333,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Romaja",
                    name: "Hangul_Romaja",
                    keycode: 65334,
                },
                KeynameTableEntry {
                    identifier: "KEY_Codeinput",
                    name: "Codeinput",
                    keycode: 65335,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Jeonja",
                    name: "Hangul_Jeonja",
                    keycode: 65336,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Banja",
                    name: "Hangul_Banja",
                    keycode: 65337,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_PreHanja",
                    name: "Hangul_PreHanja",
                    keycode: 65338,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_PostHanja",
                    name: "Hangul_PostHanja",
                    keycode: 65339,
                },
                KeynameTableEntry {
                    identifier: "KEY_SingleCandidate",
                    name: "SingleCandidate",
                    keycode: 65340,
                },
                KeynameTableEntry {
                    identifier: "KEY_MultipleCandidate",
                    name: "MultipleCandidate",
                    keycode: 65341,
                },
                KeynameTableEntry {
                    identifier: "KEY_PreviousCandidate",
                    name: "PreviousCandidate",
                    keycode: 65342,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_Special",
                    name: "Hangul_Special",
                    keycode: 65343,
                },
                KeynameTableEntry {
                    identifier: "KEY_Home",
                    name: "Home",
                    keycode: 65360,
                },
                KeynameTableEntry {
                    identifier: "KEY_Left",
                    name: "Left",
                    keycode: 65361,
                },
                KeynameTableEntry {
                    identifier: "KEY_Up",
                    name: "Up",
                    keycode: 65362,
                },
                KeynameTableEntry {
                    identifier: "KEY_Right",
                    name: "Right",
                    keycode: 65363,
                },
                KeynameTableEntry {
                    identifier: "KEY_Down",
                    name: "Down",
                    keycode: 65364,
                },
                KeynameTableEntry {
                    identifier: "KEY_Page_Up",
                    name: "Page_Up",
                    keycode: 65365,
                },
                KeynameTableEntry {
                    identifier: "KEY_Prior",
                    name: "Prior",
                    keycode: 65365,
                },
                KeynameTableEntry {
                    identifier: "KEY_Page_Down",
                    name: "Page_Down",
                    keycode: 65366,
                },
                KeynameTableEntry {
                    identifier: "KEY_Next",
                    name: "Next",
                    keycode: 65366,
                },
                KeynameTableEntry {
                    identifier: "KEY_End",
                    name: "End",
                    keycode: 65367,
                },
                KeynameTableEntry {
                    identifier: "KEY_Begin",
                    name: "Begin",
                    keycode: 65368,
                },
                KeynameTableEntry {
                    identifier: "KEY_Select",
                    name: "Select",
                    keycode: 65376,
                },
                KeynameTableEntry {
                    identifier: "KEY_Print",
                    name: "Print",
                    keycode: 65377,
                },
                KeynameTableEntry {
                    identifier: "KEY_Execute",
                    name: "Execute",
                    keycode: 65378,
                },
                KeynameTableEntry {
                    identifier: "KEY_Insert",
                    name: "Insert",
                    keycode: 65379,
                },
                KeynameTableEntry {
                    identifier: "KEY_Undo",
                    name: "Undo",
                    keycode: 65381,
                },
                KeynameTableEntry {
                    identifier: "KEY_Redo",
                    name: "Redo",
                    keycode: 65382,
                },
                KeynameTableEntry {
                    identifier: "KEY_Menu",
                    name: "Menu",
                    keycode: 65383,
                },
                KeynameTableEntry {
                    identifier: "KEY_Find",
                    name: "Find",
                    keycode: 65384,
                },
                KeynameTableEntry {
                    identifier: "KEY_Cancel",
                    name: "Cancel",
                    keycode: 65385,
                },
                KeynameTableEntry {
                    identifier: "KEY_Help",
                    name: "Help",
                    keycode: 65386,
                },
                KeynameTableEntry {
                    identifier: "KEY_Break",
                    name: "Break",
                    keycode: 65387,
                },
                KeynameTableEntry {
                    identifier: "KEY_Arabic_switch",
                    name: "Arabic_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_Greek_switch",
                    name: "Greek_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hangul_switch",
                    name: "Hangul_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hebrew_switch",
                    name: "Hebrew_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_ISO_Group_Shift",
                    name: "ISO_Group_Shift",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_Mode_switch",
                    name: "Mode_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_kana_switch",
                    name: "kana_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_script_switch",
                    name: "script_switch",
                    keycode: 65406,
                },
                KeynameTableEntry {
                    identifier: "KEY_Num_Lock",
                    name: "Num_Lock",
                    keycode: 65407,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Space",
                    name: "KP_Space",
                    keycode: 65408,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Tab",
                    name: "KP_Tab",
                    keycode: 65417,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Enter",
                    name: "KP_Enter",
                    keycode: 65421,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_F1",
                    name: "KP_F1",
                    keycode: 65425,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_F2",
                    name: "KP_F2",
                    keycode: 65426,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_F3",
                    name: "KP_F3",
                    keycode: 65427,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_F4",
                    name: "KP_F4",
                    keycode: 65428,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Home",
                    name: "KP_Home",
                    keycode: 65429,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Left",
                    name: "KP_Left",
                    keycode: 65430,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Up",
                    name: "KP_Up",
                    keycode: 65431,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Right",
                    name: "KP_Right",
                    keycode: 65432,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Down",
                    name: "KP_Down",
                    keycode: 65433,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Page_Up",
                    name: "KP_Page_Up",
                    keycode: 65434,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Prior",
                    name: "KP_Prior",
                    keycode: 65434,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Page_Down",
                    name: "KP_Page_Down",
                    keycode: 65435,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Next",
                    name: "KP_Next",
                    keycode: 65435,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_End",
                    name: "KP_End",
                    keycode: 65436,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Begin",
                    name: "KP_Begin",
                    keycode: 65437,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Insert",
                    name: "KP_Insert",
                    keycode: 65438,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Delete",
                    name: "KP_Delete",
                    keycode: 65439,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Multiply",
                    name: "KP_Multiply",
                    keycode: 65450,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Add",
                    name: "KP_Add",
                    keycode: 65451,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Separator",
                    name: "KP_Separator",
                    keycode: 65452,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Subtract",
                    name: "KP_Subtract",
                    keycode: 65453,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Decimal",
                    name: "KP_Decimal",
                    keycode: 65454,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Divide",
                    name: "KP_Divide",
                    keycode: 65455,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_0",
                    name: "KP_0",
                    keycode: 65456,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_1",
                    name: "KP_1",
                    keycode: 65457,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_2",
                    name: "KP_2",
                    keycode: 65458,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_3",
                    name: "KP_3",
                    keycode: 65459,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_4",
                    name: "KP_4",
                    keycode: 65460,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_5",
                    name: "KP_5",
                    keycode: 65461,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_6",
                    name: "KP_6",
                    keycode: 65462,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_7",
                    name: "KP_7",
                    keycode: 65463,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_8",
                    name: "KP_8",
                    keycode: 65464,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_9",
                    name: "KP_9",
                    keycode: 65465,
                },
                KeynameTableEntry {
                    identifier: "KEY_KP_Equal",
                    name: "KP_Equal",
                    keycode: 65469,
                },
                KeynameTableEntry {
                    identifier: "KEY_F1",
                    name: "F1",
                    keycode: 65470,
                },
                KeynameTableEntry {
                    identifier: "KEY_F2",
                    name: "F2",
                    keycode: 65471,
                },
                KeynameTableEntry {
                    identifier: "KEY_F3",
                    name: "F3",
                    keycode: 65472,
                },
                KeynameTableEntry {
                    identifier: "KEY_F4",
                    name: "F4",
                    keycode: 65473,
                },
                KeynameTableEntry {
                    identifier: "KEY_F5",
                    name: "F5",
                    keycode: 65474,
                },
                KeynameTableEntry {
                    identifier: "KEY_F6",
                    name: "F6",
                    keycode: 65475,
                },
                KeynameTableEntry {
                    identifier: "KEY_F7",
                    name: "F7",
                    keycode: 65476,
                },
                KeynameTableEntry {
                    identifier: "KEY_F8",
                    name: "F8",
                    keycode: 65477,
                },
                KeynameTableEntry {
                    identifier: "KEY_F9",
                    name: "F9",
                    keycode: 65478,
                },
                KeynameTableEntry {
                    identifier: "KEY_F10",
                    name: "F10",
                    keycode: 65479,
                },
                KeynameTableEntry {
                    identifier: "KEY_F11",
                    name: "F11",
                    keycode: 65480,
                },
                KeynameTableEntry {
                    identifier: "KEY_F12",
                    name: "F12",
                    keycode: 65481,
                },
                KeynameTableEntry {
                    identifier: "KEY_F13",
                    name: "F13",
                    keycode: 65482,
                },
                KeynameTableEntry {
                    identifier: "KEY_F14",
                    name: "F14",
                    keycode: 65483,
                },
                KeynameTableEntry {
                    identifier: "KEY_F15",
                    name: "F15",
                    keycode: 65484,
                },
                KeynameTableEntry {
                    identifier: "KEY_F16",
                    name: "F16",
                    keycode: 65485,
                },
                KeynameTableEntry {
                    identifier: "KEY_F17",
                    name: "F17",
                    keycode: 65486,
                },
                KeynameTableEntry {
                    identifier: "KEY_F18",
                    name: "F18",
                    keycode: 65487,
                },
                KeynameTableEntry {
                    identifier: "KEY_F19",
                    name: "F19",
                    keycode: 65488,
                },
                KeynameTableEntry {
                    identifier: "KEY_F20",
                    name: "F20",
                    keycode: 65489,
                },
                KeynameTableEntry {
                    identifier: "KEY_F21",
                    name: "F21",
                    keycode: 65490,
                },
                KeynameTableEntry {
                    identifier: "KEY_F22",
                    name: "F22",
                    keycode: 65491,
                },
                KeynameTableEntry {
                    identifier: "KEY_F23",
                    name: "F23",
                    keycode: 65492,
                },
                KeynameTableEntry {
                    identifier: "KEY_F24",
                    name: "F24",
                    keycode: 65493,
                },
                KeynameTableEntry {
                    identifier: "KEY_F25",
                    name: "F25",
                    keycode: 65494,
                },
                KeynameTableEntry {
                    identifier: "KEY_F26",
                    name: "F26",
                    keycode: 65495,
                },
                KeynameTableEntry {
                    identifier: "KEY_F27",
                    name: "F27",
                    keycode: 65496,
                },
                KeynameTableEntry {
                    identifier: "KEY_F28",
                    name: "F28",
                    keycode: 65497,
                },
                KeynameTableEntry {
                    identifier: "KEY_F29",
                    name: "F29",
                    keycode: 65498,
                },
                KeynameTableEntry {
                    identifier: "KEY_F30",
                    name: "F30",
                    keycode: 65499,
                },
                KeynameTableEntry {
                    identifier: "KEY_F31",
                    name: "F31",
                    keycode: 65500,
                },
                KeynameTableEntry {
                    identifier: "KEY_F32",
                    name: "F32",
                    keycode: 65501,
                },
                KeynameTableEntry {
                    identifier: "KEY_F33",
                    name: "F33",
                    keycode: 65502,
                },
                KeynameTableEntry {
                    identifier: "KEY_F34",
                    name: "F34",
                    keycode: 65503,
                },
                KeynameTableEntry {
                    identifier: "KEY_F35",
                    name: "F35",
                    keycode: 65504,
                },
                KeynameTableEntry {
                    identifier: "KEY_Shift_L",
                    name: "Shift_L",
                    keycode: 65505,
                },
                KeynameTableEntry {
                    identifier: "KEY_Shift_R",
                    name: "Shift_R",
                    keycode: 65506,
                },
                KeynameTableEntry {
                    identifier: "KEY_Control_L",
                    name: "Control_L",
                    keycode: 65507,
                },
                KeynameTableEntry {
                    identifier: "KEY_Control_R",
                    name: "Control_R",
                    keycode: 65508,
                },
                KeynameTableEntry {
                    identifier: "KEY_Caps_Lock",
                    name: "Caps_Lock",
                    keycode: 65509,
                },
                KeynameTableEntry {
                    identifier: "KEY_Shift_Lock",
                    name: "Shift_Lock",
                    keycode: 65510,
                },
                KeynameTableEntry {
                    identifier: "KEY_Meta_L",
                    name: "Meta_L",
                    keycode: 65511,
                },
                KeynameTableEntry {
                    identifier: "KEY_Meta_R",
                    name: "Meta_R",
                    keycode: 65512,
                },
                KeynameTableEntry {
                    identifier: "KEY_Alt_L",
                    name: "Alt_L",
                    keycode: 65513,
                },
                KeynameTableEntry {
                    identifier: "KEY_Alt_R",
                    name: "Alt_R",
                    keycode: 65514,
                },
                KeynameTableEntry {
                    identifier: "KEY_Super_L",
                    name: "Super_L",
                    keycode: 65515,
                },
                KeynameTableEntry {
                    identifier: "KEY_Super_R",
                    name: "Super_R",
                    keycode: 65516,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hyper_L",
                    name: "Hyper_L",
                    keycode: 65517,
                },
                KeynameTableEntry {
                    identifier: "KEY_Hyper_R",
                    name: "Hyper_R",
                    keycode: 65518,
                },
                KeynameTableEntry {
                    identifier: "KEY_Delete",
                    name: "Delete",
                    keycode: 65535,
                },
                KeynameTableEntry {
                    identifier: "KEY_VoidSymbol",
                    name: "VoidSymbol",
                    keycode: 16777215,
                },
            ],
        };
    }
}

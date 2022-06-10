use iron_ingot::TreeMap;
use std::borrow::Cow;

#[test]
fn test_bulk_put() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.bulk_put(
    [
      (-23, "aV"),
      (-22, "aU"),
      (-21, "aT"),
      (-20, "aS"),
      (-19, "aR"),
      (-18, "aQ"),
      (-17, "aP"),
      (-16, "aO"),
      (-15, "aN"),
      (-14, "aM"),
      (-13, "aL"),
      (-12, "aK"),
      (-11, "aJ"),
      (-10, "aI"),
      (-9, "aH"),
      (-8, "aG"),
      (-7, "ai"),
      (-6, "ae"),
      (-5, "aj"),
      (-4, "aW"),
      (-3, "ak"),
      (-2, "af"),
      (-1, "al"),
      (1, "am"),
      (2, "ag"),
      (3, "an"),
      (4, "aX"),
      (5, "ao"),
      (6, "ah"),
      (7, "ap"),
      (8, "aq"),
      (9, "ar"),
      (10, "as"),
      (11, "at"),
      (12, "au"),
      (13, "av"),
      (14, "aw"),
      (15, "ax"),
      (16, "ay"),
      (17, "az"),
      (18, "aA"),
      (19, "aB"),
      (20, "aC"),
      (21, "aD"),
      (22, "aE"),
      (23, "aF"),
    ]
    .into_iter(),
  );
  assert_eq!(tree_map.get(-24), None);
  assert_eq!(tree_map.get(-23), Some(&"aV"));
  assert_eq!(tree_map.get(-22), Some(&"aU"));
  assert_eq!(tree_map.get(-21), Some(&"aT"));
  assert_eq!(tree_map.get(-20), Some(&"aS"));
  assert_eq!(tree_map.get(-19), Some(&"aR"));
  assert_eq!(tree_map.get(-18), Some(&"aQ"));
  assert_eq!(tree_map.get(-17), Some(&"aP"));
  assert_eq!(tree_map.get(-16), Some(&"aO"));
  assert_eq!(tree_map.get(-15), Some(&"aN"));
  assert_eq!(tree_map.get(-14), Some(&"aM"));
  assert_eq!(tree_map.get(-13), Some(&"aL"));
  assert_eq!(tree_map.get(-12), Some(&"aK"));
  assert_eq!(tree_map.get(-11), Some(&"aJ"));
  assert_eq!(tree_map.get(-10), Some(&"aI"));
  assert_eq!(tree_map.get(-9), Some(&"aH"));
  assert_eq!(tree_map.get(-8), Some(&"aG"));
  assert_eq!(tree_map.get(-7), Some(&"ai"));
  assert_eq!(tree_map.get(-6), Some(&"ae"));
  assert_eq!(tree_map.get(-5), Some(&"aj"));
  assert_eq!(tree_map.get(-4), Some(&"aW"));
  assert_eq!(tree_map.get(-3), Some(&"ak"));
  assert_eq!(tree_map.get(-2), Some(&"af"));
  assert_eq!(tree_map.get(-1), Some(&"al"));
  assert_eq!(tree_map.get(1), Some(&"am"));
  assert_eq!(tree_map.get(2), Some(&"ag"));
  assert_eq!(tree_map.get(3), Some(&"an"));
  assert_eq!(tree_map.get(4), Some(&"aX"));
  assert_eq!(tree_map.get(5), Some(&"ao"));
  assert_eq!(tree_map.get(6), Some(&"ah"));
  assert_eq!(tree_map.get(7), Some(&"ap"));
  assert_eq!(tree_map.get(8), Some(&"aq"));
  assert_eq!(tree_map.get(9), Some(&"ar"));
  assert_eq!(tree_map.get(10), Some(&"as"));
  assert_eq!(tree_map.get(11), Some(&"at"));
  assert_eq!(tree_map.get(12), Some(&"au"));
  assert_eq!(tree_map.get(13), Some(&"av"));
  assert_eq!(tree_map.get(14), Some(&"aw"));
  assert_eq!(tree_map.get(15), Some(&"ax"));
  assert_eq!(tree_map.get(16), Some(&"ay"));
  assert_eq!(tree_map.get(17), Some(&"az"));
  assert_eq!(tree_map.get(18), Some(&"aA"));
  assert_eq!(tree_map.get(19), Some(&"aB"));
  assert_eq!(tree_map.get(20), Some(&"aC"));
  assert_eq!(tree_map.get(21), Some(&"aD"));
  assert_eq!(tree_map.get(22), Some(&"aE"));
  assert_eq!(tree_map.get(23), Some(&"aF"));
  assert_eq!(tree_map.get(-24), None);
}

#[test]
fn test_put() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(0, "a");
  assert_eq!(tree_map.get(0), Some(&"a"));
  tree_map.put(0, "b");
  assert_eq!(tree_map.get(0), Some(&"b"));
  tree_map.put(-4, "c");
  assert_eq!(tree_map.get(-4), Some(&"c"));
  assert_eq!(tree_map.get(0), Some(&"b"));
  tree_map.put(4, "d");
  assert_eq!(tree_map.get(-4), Some(&"c"));
  assert_eq!(tree_map.get(0), Some(&"b"));
  assert_eq!(tree_map.get(4), Some(&"d"));
  tree_map.put(-6, "e");
  tree_map.put(-2, "f");
  tree_map.put(2, "g");
  tree_map.put(6, "h");
  tree_map.put(-7, "i");
  tree_map.put(-5, "j");
  tree_map.put(-3, "k");
  tree_map.put(-1, "l");
  tree_map.put(1, "m");
  tree_map.put(3, "n");
  tree_map.put(5, "o");
  tree_map.put(7, "p");
  assert_eq!(tree_map.get(-7), Some(&"i"));
  assert_eq!(tree_map.get(-6), Some(&"e"));
  assert_eq!(tree_map.get(-5), Some(&"j"));
  assert_eq!(tree_map.get(-3), Some(&"k"));
  assert_eq!(tree_map.get(-2), Some(&"f"));
  assert_eq!(tree_map.get(-1), Some(&"l"));
  assert_eq!(tree_map.get(1), Some(&"m"));
  assert_eq!(tree_map.get(2), Some(&"g"));
  assert_eq!(tree_map.get(3), Some(&"n"));
  assert_eq!(tree_map.get(5), Some(&"o"));
  assert_eq!(tree_map.get(6), Some(&"h"));
  assert_eq!(tree_map.get(7), Some(&"p"));
  tree_map.put(8, "q");
  tree_map.put(9, "r");
  tree_map.put(10, "s");
  tree_map.put(11, "t");
  tree_map.put(12, "u");
  tree_map.put(13, "v");
  tree_map.put(14, "w");
  tree_map.put(15, "x");
  tree_map.put(16, "y");
  tree_map.put(17, "z");
  tree_map.put(18, "A");
  tree_map.put(19, "B");
  tree_map.put(20, "C");
  tree_map.put(21, "D");
  tree_map.put(22, "E");
  tree_map.put(23, "F");
  assert_eq!(tree_map.get(-7), Some(&"i"));
  assert_eq!(tree_map.get(-6), Some(&"e"));
  assert_eq!(tree_map.get(-5), Some(&"j"));
  assert_eq!(tree_map.get(-3), Some(&"k"));
  assert_eq!(tree_map.get(-2), Some(&"f"));
  assert_eq!(tree_map.get(-1), Some(&"l"));
  assert_eq!(tree_map.get(1), Some(&"m"));
  assert_eq!(tree_map.get(2), Some(&"g"));
  assert_eq!(tree_map.get(3), Some(&"n"));
  assert_eq!(tree_map.get(5), Some(&"o"));
  assert_eq!(tree_map.get(6), Some(&"h"));
  assert_eq!(tree_map.get(7), Some(&"p"));
  assert_eq!(tree_map.get(8), Some(&"q"));
  assert_eq!(tree_map.get(9), Some(&"r"));
  assert_eq!(tree_map.get(10), Some(&"s"));
  assert_eq!(tree_map.get(11), Some(&"t"));
  assert_eq!(tree_map.get(12), Some(&"u"));
  assert_eq!(tree_map.get(13), Some(&"v"));
  assert_eq!(tree_map.get(14), Some(&"w"));
  assert_eq!(tree_map.get(15), Some(&"x"));
  assert_eq!(tree_map.get(16), Some(&"y"));
  assert_eq!(tree_map.get(17), Some(&"z"));
  assert_eq!(tree_map.get(18), Some(&"A"));
  assert_eq!(tree_map.get(19), Some(&"B"));
  assert_eq!(tree_map.get(20), Some(&"C"));
  assert_eq!(tree_map.get(21), Some(&"D"));
  assert_eq!(tree_map.get(22), Some(&"E"));
  assert_eq!(tree_map.get(23), Some(&"F"));
  tree_map.put(-8, "G");
  tree_map.put(-9, "H");
  tree_map.put(-10, "I");
  tree_map.put(-11, "J");
  tree_map.put(-12, "K");
  tree_map.put(-13, "L");
  tree_map.put(-14, "M");
  tree_map.put(-15, "N");
  tree_map.put(-16, "O");
  tree_map.put(-17, "P");
  tree_map.put(-18, "Q");
  tree_map.put(-19, "R");
  tree_map.put(-20, "S");
  tree_map.put(-21, "T");
  tree_map.put(-22, "U");
  tree_map.put(-23, "V");
  assert_eq!(tree_map.get(-23), Some(&"V"));
  assert_eq!(tree_map.get(-22), Some(&"U"));
  assert_eq!(tree_map.get(-21), Some(&"T"));
  assert_eq!(tree_map.get(-20), Some(&"S"));
  assert_eq!(tree_map.get(-19), Some(&"R"));
  assert_eq!(tree_map.get(-18), Some(&"Q"));
  assert_eq!(tree_map.get(-17), Some(&"P"));
  assert_eq!(tree_map.get(-16), Some(&"O"));
  assert_eq!(tree_map.get(-15), Some(&"N"));
  assert_eq!(tree_map.get(-14), Some(&"M"));
  assert_eq!(tree_map.get(-13), Some(&"L"));
  assert_eq!(tree_map.get(-12), Some(&"K"));
  assert_eq!(tree_map.get(-11), Some(&"J"));
  assert_eq!(tree_map.get(-10), Some(&"I"));
  assert_eq!(tree_map.get(-9), Some(&"H"));
  assert_eq!(tree_map.get(-8), Some(&"G"));
  assert_eq!(tree_map.get(-7), Some(&"i"));
  assert_eq!(tree_map.get(-6), Some(&"e"));
  assert_eq!(tree_map.get(-5), Some(&"j"));
  assert_eq!(tree_map.get(-3), Some(&"k"));
  assert_eq!(tree_map.get(-2), Some(&"f"));
  assert_eq!(tree_map.get(-1), Some(&"l"));
  assert_eq!(tree_map.get(1), Some(&"m"));
  assert_eq!(tree_map.get(2), Some(&"g"));
  assert_eq!(tree_map.get(3), Some(&"n"));
  assert_eq!(tree_map.get(5), Some(&"o"));
  assert_eq!(tree_map.get(6), Some(&"h"));
  assert_eq!(tree_map.get(7), Some(&"p"));
  assert_eq!(tree_map.get(8), Some(&"q"));
  assert_eq!(tree_map.get(9), Some(&"r"));
  assert_eq!(tree_map.get(10), Some(&"s"));
  assert_eq!(tree_map.get(11), Some(&"t"));
  assert_eq!(tree_map.get(12), Some(&"u"));
  assert_eq!(tree_map.get(13), Some(&"v"));
  assert_eq!(tree_map.get(14), Some(&"w"));
  assert_eq!(tree_map.get(15), Some(&"x"));
  assert_eq!(tree_map.get(16), Some(&"y"));
  assert_eq!(tree_map.get(17), Some(&"z"));
  assert_eq!(tree_map.get(18), Some(&"A"));
  assert_eq!(tree_map.get(19), Some(&"B"));
  assert_eq!(tree_map.get(20), Some(&"C"));
  assert_eq!(tree_map.get(21), Some(&"D"));
  assert_eq!(tree_map.get(22), Some(&"E"));
  assert_eq!(tree_map.get(23), Some(&"F"));
  tree_map.put(-4, "W");
  tree_map.put(4, "X");
  assert_eq!(tree_map.get(-23), Some(&"V"));
  assert_eq!(tree_map.get(-22), Some(&"U"));
  assert_eq!(tree_map.get(-21), Some(&"T"));
  assert_eq!(tree_map.get(-20), Some(&"S"));
  assert_eq!(tree_map.get(-19), Some(&"R"));
  assert_eq!(tree_map.get(-18), Some(&"Q"));
  assert_eq!(tree_map.get(-17), Some(&"P"));
  assert_eq!(tree_map.get(-16), Some(&"O"));
  assert_eq!(tree_map.get(-15), Some(&"N"));
  assert_eq!(tree_map.get(-14), Some(&"M"));
  assert_eq!(tree_map.get(-13), Some(&"L"));
  assert_eq!(tree_map.get(-12), Some(&"K"));
  assert_eq!(tree_map.get(-11), Some(&"J"));
  assert_eq!(tree_map.get(-10), Some(&"I"));
  assert_eq!(tree_map.get(-9), Some(&"H"));
  assert_eq!(tree_map.get(-8), Some(&"G"));
  assert_eq!(tree_map.get(-7), Some(&"i"));
  assert_eq!(tree_map.get(-6), Some(&"e"));
  assert_eq!(tree_map.get(-5), Some(&"j"));
  assert_eq!(tree_map.get(-4), Some(&"W"));
  assert_eq!(tree_map.get(-3), Some(&"k"));
  assert_eq!(tree_map.get(-2), Some(&"f"));
  assert_eq!(tree_map.get(-1), Some(&"l"));
  assert_eq!(tree_map.get(1), Some(&"m"));
  assert_eq!(tree_map.get(2), Some(&"g"));
  assert_eq!(tree_map.get(3), Some(&"n"));
  assert_eq!(tree_map.get(4), Some(&"X"));
  assert_eq!(tree_map.get(5), Some(&"o"));
  assert_eq!(tree_map.get(6), Some(&"h"));
  assert_eq!(tree_map.get(7), Some(&"p"));
  assert_eq!(tree_map.get(8), Some(&"q"));
  assert_eq!(tree_map.get(9), Some(&"r"));
  assert_eq!(tree_map.get(10), Some(&"s"));
  assert_eq!(tree_map.get(11), Some(&"t"));
  assert_eq!(tree_map.get(12), Some(&"u"));
  assert_eq!(tree_map.get(13), Some(&"v"));
  assert_eq!(tree_map.get(14), Some(&"w"));
  assert_eq!(tree_map.get(15), Some(&"x"));
  assert_eq!(tree_map.get(16), Some(&"y"));
  assert_eq!(tree_map.get(17), Some(&"z"));
  assert_eq!(tree_map.get(18), Some(&"A"));
  assert_eq!(tree_map.get(19), Some(&"B"));
  assert_eq!(tree_map.get(20), Some(&"C"));
  assert_eq!(tree_map.get(21), Some(&"D"));
  assert_eq!(tree_map.get(22), Some(&"E"));
  assert_eq!(tree_map.get(23), Some(&"F"));
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert_eq!(tree_map.get(-23), Some(&"aV"));
  assert_eq!(tree_map.get(-22), Some(&"aU"));
  assert_eq!(tree_map.get(-21), Some(&"aT"));
  assert_eq!(tree_map.get(-20), Some(&"aS"));
  assert_eq!(tree_map.get(-19), Some(&"aR"));
  assert_eq!(tree_map.get(-18), Some(&"aQ"));
  assert_eq!(tree_map.get(-17), Some(&"aP"));
  assert_eq!(tree_map.get(-16), Some(&"aO"));
  assert_eq!(tree_map.get(-15), Some(&"aN"));
  assert_eq!(tree_map.get(-14), Some(&"aM"));
  assert_eq!(tree_map.get(-13), Some(&"aL"));
  assert_eq!(tree_map.get(-12), Some(&"aK"));
  assert_eq!(tree_map.get(-11), Some(&"aJ"));
  assert_eq!(tree_map.get(-10), Some(&"aI"));
  assert_eq!(tree_map.get(-9), Some(&"aH"));
  assert_eq!(tree_map.get(-8), Some(&"aG"));
  assert_eq!(tree_map.get(-7), Some(&"ai"));
  assert_eq!(tree_map.get(-6), Some(&"ae"));
  assert_eq!(tree_map.get(-5), Some(&"aj"));
  assert_eq!(tree_map.get(-4), Some(&"aW"));
  assert_eq!(tree_map.get(-3), Some(&"ak"));
  assert_eq!(tree_map.get(-2), Some(&"af"));
  assert_eq!(tree_map.get(-1), Some(&"al"));
  assert_eq!(tree_map.get(1), Some(&"am"));
  assert_eq!(tree_map.get(2), Some(&"ag"));
  assert_eq!(tree_map.get(3), Some(&"an"));
  assert_eq!(tree_map.get(4), Some(&"aX"));
  assert_eq!(tree_map.get(5), Some(&"ao"));
  assert_eq!(tree_map.get(6), Some(&"ah"));
  assert_eq!(tree_map.get(7), Some(&"ap"));
  assert_eq!(tree_map.get(8), Some(&"aq"));
  assert_eq!(tree_map.get(9), Some(&"ar"));
  assert_eq!(tree_map.get(10), Some(&"as"));
  assert_eq!(tree_map.get(11), Some(&"at"));
  assert_eq!(tree_map.get(12), Some(&"au"));
  assert_eq!(tree_map.get(13), Some(&"av"));
  assert_eq!(tree_map.get(14), Some(&"aw"));
  assert_eq!(tree_map.get(15), Some(&"ax"));
  assert_eq!(tree_map.get(16), Some(&"ay"));
  assert_eq!(tree_map.get(17), Some(&"az"));
  assert_eq!(tree_map.get(18), Some(&"aA"));
  assert_eq!(tree_map.get(19), Some(&"aB"));
  assert_eq!(tree_map.get(20), Some(&"aC"));
  assert_eq!(tree_map.get(21), Some(&"aD"));
  assert_eq!(tree_map.get(22), Some(&"aE"));
  assert_eq!(tree_map.get(23), Some(&"aF"));
}

#[test]
fn test_has() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert!(!tree_map.has(-24));
  assert!(tree_map.has(-23));
  assert!(tree_map.has(-22));
  assert!(tree_map.has(-21));
  assert!(tree_map.has(-20));
  assert!(tree_map.has(-19));
  assert!(tree_map.has(-18));
  assert!(tree_map.has(-17));
  assert!(tree_map.has(-16));
  assert!(tree_map.has(-15));
  assert!(tree_map.has(-14));
  assert!(tree_map.has(-13));
  assert!(tree_map.has(-12));
  assert!(tree_map.has(-11));
  assert!(tree_map.has(-10));
  assert!(tree_map.has(-9));
  assert!(tree_map.has(-8));
  assert!(tree_map.has(-7));
  assert!(tree_map.has(-6));
  assert!(tree_map.has(-5));
  assert!(tree_map.has(-4));
  assert!(tree_map.has(-3));
  assert!(tree_map.has(-2));
  assert!(tree_map.has(-1));
  assert!(tree_map.has(1));
  assert!(tree_map.has(2));
  assert!(tree_map.has(3));
  assert!(tree_map.has(4));
  assert!(tree_map.has(5));
  assert!(tree_map.has(6));
  assert!(tree_map.has(7));
  assert!(tree_map.has(8));
  assert!(tree_map.has(9));
  assert!(tree_map.has(10));
  assert!(tree_map.has(11));
  assert!(tree_map.has(12));
  assert!(tree_map.has(13));
  assert!(tree_map.has(14));
  assert!(tree_map.has(15));
  assert!(tree_map.has(16));
  assert!(tree_map.has(17));
  assert!(tree_map.has(18));
  assert!(tree_map.has(19));
  assert!(tree_map.has(20));
  assert!(tree_map.has(21));
  assert!(tree_map.has(22));
  assert!(tree_map.has(23));
  assert!(!tree_map.has(-24));
}

#[test]
fn test_get() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert_eq!(tree_map.get(-24), None);
  assert_eq!(tree_map.get(-23), Some(&"aV"));
  assert_eq!(tree_map.get(-22), Some(&"aU"));
  assert_eq!(tree_map.get(-21), Some(&"aT"));
  assert_eq!(tree_map.get(-20), Some(&"aS"));
  assert_eq!(tree_map.get(-19), Some(&"aR"));
  assert_eq!(tree_map.get(-18), Some(&"aQ"));
  assert_eq!(tree_map.get(-17), Some(&"aP"));
  assert_eq!(tree_map.get(-16), Some(&"aO"));
  assert_eq!(tree_map.get(-15), Some(&"aN"));
  assert_eq!(tree_map.get(-14), Some(&"aM"));
  assert_eq!(tree_map.get(-13), Some(&"aL"));
  assert_eq!(tree_map.get(-12), Some(&"aK"));
  assert_eq!(tree_map.get(-11), Some(&"aJ"));
  assert_eq!(tree_map.get(-10), Some(&"aI"));
  assert_eq!(tree_map.get(-9), Some(&"aH"));
  assert_eq!(tree_map.get(-8), Some(&"aG"));
  assert_eq!(tree_map.get(-7), Some(&"ai"));
  assert_eq!(tree_map.get(-6), Some(&"ae"));
  assert_eq!(tree_map.get(-5), Some(&"aj"));
  assert_eq!(tree_map.get(-4), Some(&"aW"));
  assert_eq!(tree_map.get(-3), Some(&"ak"));
  assert_eq!(tree_map.get(-2), Some(&"af"));
  assert_eq!(tree_map.get(-1), Some(&"al"));
  assert_eq!(tree_map.get(1), Some(&"am"));
  assert_eq!(tree_map.get(2), Some(&"ag"));
  assert_eq!(tree_map.get(3), Some(&"an"));
  assert_eq!(tree_map.get(4), Some(&"aX"));
  assert_eq!(tree_map.get(5), Some(&"ao"));
  assert_eq!(tree_map.get(6), Some(&"ah"));
  assert_eq!(tree_map.get(7), Some(&"ap"));
  assert_eq!(tree_map.get(8), Some(&"aq"));
  assert_eq!(tree_map.get(9), Some(&"ar"));
  assert_eq!(tree_map.get(10), Some(&"as"));
  assert_eq!(tree_map.get(11), Some(&"at"));
  assert_eq!(tree_map.get(12), Some(&"au"));
  assert_eq!(tree_map.get(13), Some(&"av"));
  assert_eq!(tree_map.get(14), Some(&"aw"));
  assert_eq!(tree_map.get(15), Some(&"ax"));
  assert_eq!(tree_map.get(16), Some(&"ay"));
  assert_eq!(tree_map.get(17), Some(&"az"));
  assert_eq!(tree_map.get(18), Some(&"aA"));
  assert_eq!(tree_map.get(19), Some(&"aB"));
  assert_eq!(tree_map.get(20), Some(&"aC"));
  assert_eq!(tree_map.get(21), Some(&"aD"));
  assert_eq!(tree_map.get(22), Some(&"aE"));
  assert_eq!(tree_map.get(23), Some(&"aF"));
  assert_eq!(tree_map.get(-24), None);
}

#[test]
fn test_index() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert_eq!(tree_map[-23], "aV");
  assert_eq!(tree_map[-22], "aU");
  assert_eq!(tree_map[-21], "aT");
  assert_eq!(tree_map[-20], "aS");
  assert_eq!(tree_map[-19], "aR");
  assert_eq!(tree_map[-18], "aQ");
  assert_eq!(tree_map[-17], "aP");
  assert_eq!(tree_map[-16], "aO");
  assert_eq!(tree_map[-15], "aN");
  assert_eq!(tree_map[-14], "aM");
  assert_eq!(tree_map[-13], "aL");
  assert_eq!(tree_map[-12], "aK");
  assert_eq!(tree_map[-11], "aJ");
  assert_eq!(tree_map[-10], "aI");
  assert_eq!(tree_map[-9], "aH");
  assert_eq!(tree_map[-8], "aG");
  assert_eq!(tree_map[-7], "ai");
  assert_eq!(tree_map[-6], "ae");
  assert_eq!(tree_map[-5], "aj");
  assert_eq!(tree_map[-4], "aW");
  assert_eq!(tree_map[-3], "ak");
  assert_eq!(tree_map[-2], "af");
  assert_eq!(tree_map[-1], "al");
  assert_eq!(tree_map[1], "am");
  assert_eq!(tree_map[2], "ag");
  assert_eq!(tree_map[3], "an");
  assert_eq!(tree_map[4], "aX");
  assert_eq!(tree_map[5], "ao");
  assert_eq!(tree_map[6], "ah");
  assert_eq!(tree_map[7], "ap");
  assert_eq!(tree_map[8], "aq");
  assert_eq!(tree_map[9], "ar");
  assert_eq!(tree_map[10], "as");
  assert_eq!(tree_map[11], "at");
  assert_eq!(tree_map[12], "au");
  assert_eq!(tree_map[13], "av");
  assert_eq!(tree_map[14], "aw");
  assert_eq!(tree_map[15], "ax");
  assert_eq!(tree_map[16], "ay");
  assert_eq!(tree_map[17], "az");
  assert_eq!(tree_map[18], "aA");
  assert_eq!(tree_map[19], "aB");
  assert_eq!(tree_map[20], "aC");
  assert_eq!(tree_map[21], "aD");
  assert_eq!(tree_map[22], "aE");
  assert_eq!(tree_map[23], "aF");
}

#[test]
fn test_get_mut() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  *tree_map.get_mut(-23).unwrap() = "V";
  *tree_map.get_mut(-22).unwrap() = "U";
  *tree_map.get_mut(-21).unwrap() = "T";
  *tree_map.get_mut(-20).unwrap() = "S";
  *tree_map.get_mut(-19).unwrap() = "R";
  *tree_map.get_mut(-18).unwrap() = "Q";
  *tree_map.get_mut(-17).unwrap() = "P";
  *tree_map.get_mut(-16).unwrap() = "O";
  *tree_map.get_mut(-15).unwrap() = "N";
  *tree_map.get_mut(-14).unwrap() = "M";
  *tree_map.get_mut(-13).unwrap() = "L";
  *tree_map.get_mut(-12).unwrap() = "K";
  *tree_map.get_mut(-11).unwrap() = "J";
  *tree_map.get_mut(-10).unwrap() = "I";
  *tree_map.get_mut(-9).unwrap() = "H";
  *tree_map.get_mut(-8).unwrap() = "G";
  *tree_map.get_mut(-7).unwrap() = "i";
  *tree_map.get_mut(-6).unwrap() = "e";
  *tree_map.get_mut(-5).unwrap() = "j";
  *tree_map.get_mut(-4).unwrap() = "W";
  *tree_map.get_mut(-3).unwrap() = "k";
  *tree_map.get_mut(-2).unwrap() = "f";
  *tree_map.get_mut(-1).unwrap() = "l";
  *tree_map.get_mut(1).unwrap() = "m";
  *tree_map.get_mut(2).unwrap() = "g";
  *tree_map.get_mut(3).unwrap() = "n";
  *tree_map.get_mut(4).unwrap() = "X";
  *tree_map.get_mut(5).unwrap() = "o";
  *tree_map.get_mut(6).unwrap() = "h";
  *tree_map.get_mut(7).unwrap() = "p";
  *tree_map.get_mut(8).unwrap() = "q";
  *tree_map.get_mut(9).unwrap() = "r";
  *tree_map.get_mut(10).unwrap() = "s";
  *tree_map.get_mut(11).unwrap() = "t";
  *tree_map.get_mut(12).unwrap() = "u";
  *tree_map.get_mut(13).unwrap() = "v";
  *tree_map.get_mut(14).unwrap() = "w";
  *tree_map.get_mut(15).unwrap() = "x";
  *tree_map.get_mut(16).unwrap() = "y";
  *tree_map.get_mut(17).unwrap() = "z";
  *tree_map.get_mut(18).unwrap() = "A";
  *tree_map.get_mut(19).unwrap() = "B";
  *tree_map.get_mut(20).unwrap() = "C";
  *tree_map.get_mut(21).unwrap() = "D";
  *tree_map.get_mut(22).unwrap() = "E";
  *tree_map.get_mut(23).unwrap() = "F";
  assert_eq!(tree_map[-23], "V");
  assert_eq!(tree_map[-22], "U");
  assert_eq!(tree_map[-21], "T");
  assert_eq!(tree_map[-20], "S");
  assert_eq!(tree_map[-19], "R");
  assert_eq!(tree_map[-18], "Q");
  assert_eq!(tree_map[-17], "P");
  assert_eq!(tree_map[-16], "O");
  assert_eq!(tree_map[-15], "N");
  assert_eq!(tree_map[-14], "M");
  assert_eq!(tree_map[-13], "L");
  assert_eq!(tree_map[-12], "K");
  assert_eq!(tree_map[-11], "J");
  assert_eq!(tree_map[-10], "I");
  assert_eq!(tree_map[-9], "H");
  assert_eq!(tree_map[-8], "G");
  assert_eq!(tree_map[-7], "i");
  assert_eq!(tree_map[-6], "e");
  assert_eq!(tree_map[-5], "j");
  assert_eq!(tree_map[-4], "W");
  assert_eq!(tree_map[-3], "k");
  assert_eq!(tree_map[-2], "f");
  assert_eq!(tree_map[-1], "l");
  assert_eq!(tree_map[1], "m");
  assert_eq!(tree_map[2], "g");
  assert_eq!(tree_map[3], "n");
  assert_eq!(tree_map[4], "X");
  assert_eq!(tree_map[5], "o");
  assert_eq!(tree_map[6], "h");
  assert_eq!(tree_map[7], "p");
  assert_eq!(tree_map[8], "q");
  assert_eq!(tree_map[9], "r");
  assert_eq!(tree_map[10], "s");
  assert_eq!(tree_map[11], "t");
  assert_eq!(tree_map[12], "u");
  assert_eq!(tree_map[13], "v");
  assert_eq!(tree_map[14], "w");
  assert_eq!(tree_map[15], "x");
  assert_eq!(tree_map[16], "y");
  assert_eq!(tree_map[17], "z");
  assert_eq!(tree_map[18], "A");
  assert_eq!(tree_map[19], "B");
  assert_eq!(tree_map[20], "C");
  assert_eq!(tree_map[21], "D");
  assert_eq!(tree_map[22], "E");
  assert_eq!(tree_map[23], "F");
}

#[test]
fn test_index_mut() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  tree_map[-23] = "V";
  tree_map[-22] = "U";
  tree_map[-21] = "T";
  tree_map[-20] = "S";
  tree_map[-19] = "R";
  tree_map[-18] = "Q";
  tree_map[-17] = "P";
  tree_map[-16] = "O";
  tree_map[-15] = "N";
  tree_map[-14] = "M";
  tree_map[-13] = "L";
  tree_map[-12] = "K";
  tree_map[-11] = "J";
  tree_map[-10] = "I";
  tree_map[-9] = "H";
  tree_map[-8] = "G";
  tree_map[-7] = "i";
  tree_map[-6] = "e";
  tree_map[-5] = "j";
  tree_map[-4] = "W";
  tree_map[-3] = "k";
  tree_map[-2] = "f";
  tree_map[-1] = "l";
  tree_map[1] = "m";
  tree_map[2] = "g";
  tree_map[3] = "n";
  tree_map[4] = "X";
  tree_map[5] = "o";
  tree_map[6] = "h";
  tree_map[7] = "p";
  tree_map[8] = "q";
  tree_map[9] = "r";
  tree_map[10] = "s";
  tree_map[11] = "t";
  tree_map[12] = "u";
  tree_map[13] = "v";
  tree_map[14] = "w";
  tree_map[15] = "x";
  tree_map[16] = "y";
  tree_map[17] = "z";
  tree_map[18] = "A";
  tree_map[19] = "B";
  tree_map[20] = "C";
  tree_map[21] = "D";
  tree_map[22] = "E";
  tree_map[23] = "F";
  assert_eq!(tree_map[-23], "V");
  assert_eq!(tree_map[-22], "U");
  assert_eq!(tree_map[-21], "T");
  assert_eq!(tree_map[-20], "S");
  assert_eq!(tree_map[-19], "R");
  assert_eq!(tree_map[-18], "Q");
  assert_eq!(tree_map[-17], "P");
  assert_eq!(tree_map[-16], "O");
  assert_eq!(tree_map[-15], "N");
  assert_eq!(tree_map[-14], "M");
  assert_eq!(tree_map[-13], "L");
  assert_eq!(tree_map[-12], "K");
  assert_eq!(tree_map[-11], "J");
  assert_eq!(tree_map[-10], "I");
  assert_eq!(tree_map[-9], "H");
  assert_eq!(tree_map[-8], "G");
  assert_eq!(tree_map[-7], "i");
  assert_eq!(tree_map[-6], "e");
  assert_eq!(tree_map[-5], "j");
  assert_eq!(tree_map[-4], "W");
  assert_eq!(tree_map[-3], "k");
  assert_eq!(tree_map[-2], "f");
  assert_eq!(tree_map[-1], "l");
  assert_eq!(tree_map[1], "m");
  assert_eq!(tree_map[2], "g");
  assert_eq!(tree_map[3], "n");
  assert_eq!(tree_map[4], "X");
  assert_eq!(tree_map[5], "o");
  assert_eq!(tree_map[6], "h");
  assert_eq!(tree_map[7], "p");
  assert_eq!(tree_map[8], "q");
  assert_eq!(tree_map[9], "r");
  assert_eq!(tree_map[10], "s");
  assert_eq!(tree_map[11], "t");
  assert_eq!(tree_map[12], "u");
  assert_eq!(tree_map[13], "v");
  assert_eq!(tree_map[14], "w");
  assert_eq!(tree_map[15], "x");
  assert_eq!(tree_map[16], "y");
  assert_eq!(tree_map[17], "z");
  assert_eq!(tree_map[18], "A");
  assert_eq!(tree_map[19], "B");
  assert_eq!(tree_map[20], "C");
  assert_eq!(tree_map[21], "D");
  assert_eq!(tree_map[22], "E");
  assert_eq!(tree_map[23], "F");
}

#[test]
fn test_remove() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert_eq!(tree_map.remove(-24), None);
  assert_eq!(tree_map.remove(-23), Some("aV"));
  assert_eq!(tree_map.remove(-22), Some("aU"));
  assert_eq!(tree_map.remove(-21), Some("aT"));
  assert_eq!(tree_map.remove(-20), Some("aS"));
  assert_eq!(tree_map.remove(-19), Some("aR"));
  assert_eq!(tree_map.remove(-18), Some("aQ"));
  assert_eq!(tree_map.remove(-17), Some("aP"));
  assert_eq!(tree_map.remove(-16), Some("aO"));
  assert_eq!(tree_map.remove(-15), Some("aN"));
  assert_eq!(tree_map.remove(-14), Some("aM"));
  assert_eq!(tree_map.remove(-13), Some("aL"));
  assert_eq!(tree_map.remove(-12), Some("aK"));
  assert_eq!(tree_map.remove(-11), Some("aJ"));
  assert_eq!(tree_map.remove(-10), Some("aI"));
  assert_eq!(tree_map.remove(-9), Some("aH"));
  assert_eq!(tree_map.remove(-8), Some("aG"));
  assert_eq!(tree_map.remove(-7), Some("ai"));
  assert_eq!(tree_map.remove(-6), Some("ae"));
  assert_eq!(tree_map.remove(-5), Some("aj"));
  assert_eq!(tree_map.remove(-4), Some("aW"));
  assert_eq!(tree_map.remove(-3), Some("ak"));
  assert_eq!(tree_map.remove(-2), Some("af"));
  assert_eq!(tree_map.remove(-1), Some("al"));
  assert_eq!(tree_map.remove(1), Some("am"));
  assert_eq!(tree_map.remove(2), Some("ag"));
  assert_eq!(tree_map.remove(3), Some("an"));
  assert_eq!(tree_map.remove(4), Some("aX"));
  assert_eq!(tree_map.remove(5), Some("ao"));
  assert_eq!(tree_map.remove(6), Some("ah"));
  assert_eq!(tree_map.remove(7), Some("ap"));
  assert_eq!(tree_map.remove(8), Some("aq"));
  assert_eq!(tree_map.remove(9), Some("ar"));
  assert_eq!(tree_map.remove(10), Some("as"));
  assert_eq!(tree_map.remove(11), Some("at"));
  assert_eq!(tree_map.remove(12), Some("au"));
  assert_eq!(tree_map.remove(13), Some("av"));
  assert_eq!(tree_map.remove(14), Some("aw"));
  assert_eq!(tree_map.remove(15), Some("ax"));
  assert_eq!(tree_map.remove(16), Some("ay"));
  assert_eq!(tree_map.remove(17), Some("az"));
  assert_eq!(tree_map.remove(18), Some("aA"));
  assert_eq!(tree_map.remove(19), Some("aB"));
  assert_eq!(tree_map.remove(20), Some("aC"));
  assert_eq!(tree_map.remove(21), Some("aD"));
  assert_eq!(tree_map.remove(22), Some("aE"));
  assert_eq!(tree_map.remove(23), Some("aF"));
  assert_eq!(tree_map.remove(24), None);
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  assert_eq!(tree_map.remove(24), None);
  assert_eq!(tree_map.remove(23), Some("aF"));
  assert_eq!(tree_map.remove(22), Some("aE"));
  assert_eq!(tree_map.remove(21), Some("aD"));
  assert_eq!(tree_map.remove(20), Some("aC"));
  assert_eq!(tree_map.remove(19), Some("aB"));
  assert_eq!(tree_map.remove(18), Some("aA"));
  assert_eq!(tree_map.remove(17), Some("az"));
  assert_eq!(tree_map.remove(16), Some("ay"));
  assert_eq!(tree_map.remove(15), Some("ax"));
  assert_eq!(tree_map.remove(14), Some("aw"));
  assert_eq!(tree_map.remove(13), Some("av"));
  assert_eq!(tree_map.remove(12), Some("au"));
  assert_eq!(tree_map.remove(11), Some("at"));
  assert_eq!(tree_map.remove(10), Some("as"));
  assert_eq!(tree_map.remove(9), Some("ar"));
  assert_eq!(tree_map.remove(8), Some("aq"));
  assert_eq!(tree_map.remove(7), Some("ap"));
  assert_eq!(tree_map.remove(6), Some("ah"));
  assert_eq!(tree_map.remove(5), Some("ao"));
  assert_eq!(tree_map.remove(4), Some("aX"));
  assert_eq!(tree_map.remove(3), Some("an"));
  assert_eq!(tree_map.remove(2), Some("ag"));
  assert_eq!(tree_map.remove(1), Some("am"));
  assert_eq!(tree_map.remove(-1), Some("al"));
  assert_eq!(tree_map.remove(-2), Some("af"));
  assert_eq!(tree_map.remove(-3), Some("ak"));
  assert_eq!(tree_map.remove(-4), Some("aW"));
  assert_eq!(tree_map.remove(-5), Some("aj"));
  assert_eq!(tree_map.remove(-6), Some("ae"));
  assert_eq!(tree_map.remove(-7), Some("ai"));
  assert_eq!(tree_map.remove(-8), Some("aG"));
  assert_eq!(tree_map.remove(-9), Some("aH"));
  assert_eq!(tree_map.remove(-10), Some("aI"));
  assert_eq!(tree_map.remove(-11), Some("aJ"));
  assert_eq!(tree_map.remove(-12), Some("aK"));
  assert_eq!(tree_map.remove(-13), Some("aL"));
  assert_eq!(tree_map.remove(-14), Some("aM"));
  assert_eq!(tree_map.remove(-15), Some("aN"));
  assert_eq!(tree_map.remove(-16), Some("aO"));
  assert_eq!(tree_map.remove(-17), Some("aP"));
  assert_eq!(tree_map.remove(-18), Some("aQ"));
  assert_eq!(tree_map.remove(-19), Some("aR"));
  assert_eq!(tree_map.remove(-20), Some("aS"));
  assert_eq!(tree_map.remove(-21), Some("aT"));
  assert_eq!(tree_map.remove(-22), Some("aU"));
  assert_eq!(tree_map.remove(-23), Some("aV"));
  assert_eq!(tree_map.remove(-24), None);
}

#[test]
fn test_is_empty() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  assert!(tree_map.is_empty());
  tree_map.put(0, "a");
  assert!(!tree_map.is_empty());
  tree_map.put(0, "b");
  assert!(!tree_map.is_empty());
  tree_map.put(-4, "c");
  assert!(!tree_map.is_empty());
  tree_map.put(4, "d");
  assert!(!tree_map.is_empty());
  tree_map.put(-6, "e");
  assert!(!tree_map.is_empty());
}

#[test]
fn test_len() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  assert_eq!(tree_map.len(), 0);
  tree_map.put(0, "a");
  assert_eq!(tree_map.len(), 1);
  tree_map.put(0, "b");
  assert_eq!(tree_map.len(), 1);
  tree_map.put(-4, "c");
  assert_eq!(tree_map.len(), 2);
  tree_map.put(4, "d");
  assert_eq!(tree_map.len(), 3);
  tree_map.put(-6, "e");
  assert_eq!(tree_map.len(), 4);
}

#[test]
fn test_min() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(0, "a");
  assert_eq!(tree_map.min(), Some((&0, &"a")));
  tree_map.put(0, "b");
  assert_eq!(tree_map.min(), Some((&0, &"b")));
  tree_map.put(-4, "c");
  assert_eq!(tree_map.min(), Some((&-4, &"c")));
  tree_map.put(4, "d");
  assert_eq!(tree_map.min(), Some((&-4, &"c")));
  tree_map.put(-6, "e");
  assert_eq!(tree_map.min(), Some((&-6, &"e")));
  tree_map.put(-2, "f");
  assert_eq!(tree_map.min(), Some((&-6, &"e")));
  tree_map.put(2, "g");
  assert_eq!(tree_map.min(), Some((&-6, &"e")));
  tree_map.put(6, "h");
  assert_eq!(tree_map.min(), Some((&-6, &"e")));
  tree_map.put(-7, "i");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(-5, "j");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(-3, "k");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(-1, "l");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(1, "m");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(3, "n");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(5, "o");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(7, "p");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(8, "q");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(9, "r");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(10, "s");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(11, "t");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(12, "u");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(13, "v");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(14, "w");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(15, "x");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(16, "y");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(17, "z");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(18, "A");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(19, "B");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(20, "C");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(21, "D");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(22, "E");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(23, "F");
  assert_eq!(tree_map.min(), Some((&-7, &"i")));
  tree_map.put(-8, "G");
  assert_eq!(tree_map.min(), Some((&-8, &"G")));
  tree_map.put(-9, "H");
  assert_eq!(tree_map.min(), Some((&-9, &"H")));
  tree_map.put(-10, "I");
  assert_eq!(tree_map.min(), Some((&-10, &"I")));
  tree_map.put(-11, "J");
  assert_eq!(tree_map.min(), Some((&-11, &"J")));
  tree_map.put(-12, "K");
  assert_eq!(tree_map.min(), Some((&-12, &"K")));
  tree_map.put(-13, "L");
  assert_eq!(tree_map.min(), Some((&-13, &"L")));
  tree_map.put(-14, "M");
  assert_eq!(tree_map.min(), Some((&-14, &"M")));
  tree_map.put(-15, "N");
  assert_eq!(tree_map.min(), Some((&-15, &"N")));
  tree_map.put(-16, "O");
  assert_eq!(tree_map.min(), Some((&-16, &"O")));
  tree_map.put(-17, "P");
  assert_eq!(tree_map.min(), Some((&-17, &"P")));
  tree_map.put(-18, "Q");
  assert_eq!(tree_map.min(), Some((&-18, &"Q")));
  tree_map.put(-19, "R");
  assert_eq!(tree_map.min(), Some((&-19, &"R")));
  tree_map.put(-20, "S");
  assert_eq!(tree_map.min(), Some((&-20, &"S")));
  tree_map.put(-21, "T");
  assert_eq!(tree_map.min(), Some((&-21, &"T")));
  tree_map.put(-22, "U");
  assert_eq!(tree_map.min(), Some((&-22, &"U")));
  tree_map.put(-23, "V");
  assert_eq!(tree_map.min(), Some((&-23, &"V")));
  tree_map.put(-4, "W");
  assert_eq!(tree_map.min(), Some((&-23, &"V")));
  tree_map.put(4, "X");
  assert_eq!(tree_map.min(), Some((&-23, &"V")));
}

#[test]
fn test_max() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(0, "a");
  assert_eq!(tree_map.max(), Some((&0, &"a")));
  tree_map.put(0, "b");
  assert_eq!(tree_map.max(), Some((&0, &"b")));
  tree_map.put(-4, "c");
  assert_eq!(tree_map.max(), Some((&0, &"b")));
  tree_map.put(4, "d");
  assert_eq!(tree_map.max(), Some((&4, &"d")));
  tree_map.put(-6, "e");
  assert_eq!(tree_map.max(), Some((&4, &"d")));
  tree_map.put(-2, "f");
  assert_eq!(tree_map.max(), Some((&4, &"d")));
  tree_map.put(2, "g");
  assert_eq!(tree_map.max(), Some((&4, &"d")));
  tree_map.put(6, "h");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(-7, "i");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(-5, "j");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(-3, "k");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(-1, "l");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(1, "m");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(3, "n");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(5, "o");
  assert_eq!(tree_map.max(), Some((&6, &"h")));
  tree_map.put(7, "p");
  assert_eq!(tree_map.max(), Some((&7, &"p")));
  tree_map.put(8, "q");
  assert_eq!(tree_map.max(), Some((&8, &"q")));
  tree_map.put(9, "r");
  assert_eq!(tree_map.max(), Some((&9, &"r")));
  tree_map.put(10, "s");
  assert_eq!(tree_map.max(), Some((&10, &"s")));
  tree_map.put(11, "t");
  assert_eq!(tree_map.max(), Some((&11, &"t")));
  tree_map.put(12, "u");
  assert_eq!(tree_map.max(), Some((&12, &"u")));
  tree_map.put(13, "v");
  assert_eq!(tree_map.max(), Some((&13, &"v")));
  tree_map.put(14, "w");
  assert_eq!(tree_map.max(), Some((&14, &"w")));
  tree_map.put(15, "x");
  assert_eq!(tree_map.max(), Some((&15, &"x")));
  tree_map.put(16, "y");
  assert_eq!(tree_map.max(), Some((&16, &"y")));
  tree_map.put(17, "z");
  assert_eq!(tree_map.max(), Some((&17, &"z")));
  tree_map.put(18, "A");
  assert_eq!(tree_map.max(), Some((&18, &"A")));
  tree_map.put(19, "B");
  assert_eq!(tree_map.max(), Some((&19, &"B")));
  tree_map.put(20, "C");
  assert_eq!(tree_map.max(), Some((&20, &"C")));
  tree_map.put(21, "D");
  assert_eq!(tree_map.max(), Some((&21, &"D")));
  tree_map.put(22, "E");
  assert_eq!(tree_map.max(), Some((&22, &"E")));
  tree_map.put(23, "F");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-8, "G");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-9, "H");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-10, "I");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-11, "J");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-12, "K");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-13, "L");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-14, "M");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-15, "N");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-16, "O");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-17, "P");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-18, "Q");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-19, "R");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-20, "S");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-21, "T");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-22, "U");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-23, "V");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(-4, "W");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
  tree_map.put(4, "X");
  assert_eq!(tree_map.max(), Some((&23, &"F")));
}

#[test]
fn test_to_slice_preorder() {
  assert_eq!(
    TreeMap::<i32, &str>::from([].into_iter()).to_slice_preorder(),
    Cow::Borrowed(&[])
  );
  assert_eq!(
    TreeMap::from([(1, "a")].into_iter()).to_slice_preorder(),
    Cow::Borrowed(&[(&1, &"a")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b")].into_iter()).to_slice_preorder(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c")].into_iter()).to_slice_preorder(),
    Cow::Borrowed(&[(&2, &"b"), (&1, &"a"), (&3, &"c")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")].into_iter()).to_slice_preorder(),
    Cow::Borrowed(&[(&2, &"b"), (&1, &"a"), (&3, &"c"), (&4, &"d")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")].into_iter())
      .to_slice_preorder(),
    Cow::Borrowed(&[(&2, &"b"), (&1, &"a"), (&4, &"d"), (&3, &"c"), (&5, &"e")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e"), (6, "f")].into_iter())
      .to_slice_preorder(),
    Cow::Borrowed(&[
      (&4, &"d"),
      (&2, &"b"),
      (&1, &"a"),
      (&3, &"c"),
      (&5, &"e"),
      (&6, &"f")
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g")
      ]
      .into_iter()
    )
    .to_slice_preorder(),
    Cow::Borrowed(&[
      (&4, &"d"),
      (&2, &"b"),
      (&1, &"a"),
      (&3, &"c"),
      (&6, &"f"),
      (&5, &"e"),
      (&7, &"g"),
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g"),
        (8, "h"),
      ]
      .into_iter()
    )
    .to_slice_preorder(),
    Cow::Borrowed(&[
      (&4, &"d"),
      (&2, &"b"),
      (&1, &"a"),
      (&3, &"c"),
      (&6, &"f"),
      (&5, &"e"),
      (&7, &"g"),
      (&8, &"h"),
    ])
  );
}

#[test]
fn test_to_slice() {
  assert_eq!(
    TreeMap::<i32, &str>::from([].into_iter()).to_slice(),
    Cow::Borrowed(&[])
  );
  assert_eq!(
    TreeMap::from([(1, "a")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c"), (&4, &"d")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c"), (&4, &"d"), (&5, &"e")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e"), (6, "f")].into_iter())
      .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g")
      ]
      .into_iter()
    )
    .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
      (&7, &"g"),
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g"),
        (8, "h"),
      ]
      .into_iter()
    )
    .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
      (&7, &"g"),
      (&8, &"h"),
    ])
  );
}

#[test]
fn test_to_slice_postorder() {
  assert_eq!(
    TreeMap::<i32, &str>::from([].into_iter()).to_slice(),
    Cow::Borrowed(&[])
  );
  assert_eq!(
    TreeMap::from([(1, "a")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c"), (&4, &"d")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")].into_iter()).to_slice(),
    Cow::Borrowed(&[(&1, &"a"), (&2, &"b"), (&3, &"c"), (&4, &"d"), (&5, &"e")])
  );
  assert_eq!(
    TreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e"), (6, "f")].into_iter())
      .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g")
      ]
      .into_iter()
    )
    .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
      (&7, &"g"),
    ])
  );
  assert_eq!(
    TreeMap::from(
      [
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g"),
        (8, "h"),
      ]
      .into_iter()
    )
    .to_slice(),
    Cow::Borrowed(&[
      (&1, &"a"),
      (&2, &"b"),
      (&3, &"c"),
      (&4, &"d"),
      (&5, &"e"),
      (&6, &"f"),
      (&7, &"g"),
      (&8, &"h"),
    ])
  );
}

#[test]
fn test_clear() {
  let mut tree_map = TreeMap::<i32, &str>::new(());
  tree_map.put(-23, "aV");
  tree_map.put(-22, "aU");
  tree_map.put(-21, "aT");
  tree_map.put(-20, "aS");
  tree_map.put(-19, "aR");
  tree_map.put(-18, "aQ");
  tree_map.put(-17, "aP");
  tree_map.put(-16, "aO");
  tree_map.put(-15, "aN");
  tree_map.put(-14, "aM");
  tree_map.put(-13, "aL");
  tree_map.put(-12, "aK");
  tree_map.put(-11, "aJ");
  tree_map.put(-10, "aI");
  tree_map.put(-9, "aH");
  tree_map.put(-8, "aG");
  tree_map.put(-7, "ai");
  tree_map.put(-6, "ae");
  tree_map.put(-5, "aj");
  tree_map.put(-4, "aW");
  tree_map.put(-3, "ak");
  tree_map.put(-2, "af");
  tree_map.put(-1, "al");
  tree_map.put(1, "am");
  tree_map.put(2, "ag");
  tree_map.put(3, "an");
  tree_map.put(4, "aX");
  tree_map.put(5, "ao");
  tree_map.put(6, "ah");
  tree_map.put(7, "ap");
  tree_map.put(8, "aq");
  tree_map.put(9, "ar");
  tree_map.put(10, "as");
  tree_map.put(11, "at");
  tree_map.put(12, "au");
  tree_map.put(13, "av");
  tree_map.put(14, "aw");
  tree_map.put(15, "ax");
  tree_map.put(16, "ay");
  tree_map.put(17, "az");
  tree_map.put(18, "aA");
  tree_map.put(19, "aB");
  tree_map.put(20, "aC");
  tree_map.put(21, "aD");
  tree_map.put(22, "aE");
  tree_map.put(23, "aF");
  tree_map.clear();
  assert_eq!(tree_map.get(-23), None);
  assert_eq!(tree_map.get(-22), None);
  assert_eq!(tree_map.get(-21), None);
  assert_eq!(tree_map.get(-20), None);
  assert_eq!(tree_map.get(-19), None);
  assert_eq!(tree_map.get(-18), None);
  assert_eq!(tree_map.get(-17), None);
  assert_eq!(tree_map.get(-16), None);
  assert_eq!(tree_map.get(-15), None);
  assert_eq!(tree_map.get(-14), None);
  assert_eq!(tree_map.get(-13), None);
  assert_eq!(tree_map.get(-12), None);
  assert_eq!(tree_map.get(-11), None);
  assert_eq!(tree_map.get(-10), None);
  assert_eq!(tree_map.get(-9), None);
  assert_eq!(tree_map.get(-8), None);
  assert_eq!(tree_map.get(-7), None);
  assert_eq!(tree_map.get(-6), None);
  assert_eq!(tree_map.get(-5), None);
  assert_eq!(tree_map.get(-4), None);
  assert_eq!(tree_map.get(-3), None);
  assert_eq!(tree_map.get(-2), None);
  assert_eq!(tree_map.get(-1), None);
  assert_eq!(tree_map.get(1), None);
  assert_eq!(tree_map.get(2), None);
  assert_eq!(tree_map.get(3), None);
  assert_eq!(tree_map.get(4), None);
  assert_eq!(tree_map.get(5), None);
  assert_eq!(tree_map.get(6), None);
  assert_eq!(tree_map.get(7), None);
  assert_eq!(tree_map.get(8), None);
  assert_eq!(tree_map.get(9), None);
  assert_eq!(tree_map.get(10), None);
  assert_eq!(tree_map.get(11), None);
  assert_eq!(tree_map.get(12), None);
  assert_eq!(tree_map.get(13), None);
  assert_eq!(tree_map.get(14), None);
  assert_eq!(tree_map.get(15), None);
  assert_eq!(tree_map.get(16), None);
  assert_eq!(tree_map.get(17), None);
  assert_eq!(tree_map.get(18), None);
  assert_eq!(tree_map.get(19), None);
  assert_eq!(tree_map.get(20), None);
  assert_eq!(tree_map.get(21), None);
  assert_eq!(tree_map.get(22), None);
  assert_eq!(tree_map.get(23), None);
}

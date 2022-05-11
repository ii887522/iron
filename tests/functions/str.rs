use iron_ingot::str::Substr;

#[test]
fn test_substr() {
  assert_eq!(
    "SDL2-2.0.12/lib/x86/SDL2.dll".substr("SDL", "l"),
    Some("SDL2-2.0.12/")
  );
  assert_eq!(
    "SDL2_image-2.0.5/lib/x86/libpng16-16.dll".substr("SDL", "l"),
    Some("SDL2_image-2.0.5/")
  );
  assert_eq!(
    "'SDL2_image-2.0.5/lib/x86/libpng16-16.dll".substr("image", "l"),
    Some("image-2.0.5/")
  );
  assert_eq!(
    "'SDL2_image-2.0.5/lib/x86/libpng16-16.dll".substr("image", ".dll"),
    Some("image-2.0.5/lib/x86/libpng16-16")
  );
  assert_eq!(
    "'SDL2_image-2.0.5/lib/x86/libpng16-16.dll".substr("image", "abc"),
    None
  );
}

import deepmerge from 'deepmerge';

const DEFAULT_FONT_FAMILY = '"Roboto", "Helvetica", "Arial", sans-serif';
const DEFAULT_FONT_SIZE = '10px';
const FONT_WEIGHT_LIGHT = 300;
const FONT_WEIGHT_REGULAR = 400;
const TEXT_CASE_REGULAR = 'none';
const TEXT_CASE_CAPITALIZE = 'capitalize';

/**
 * @param weight - Font weight.
 * @param size - Font size in rem unit.
 * @param lineHeight
 * @param casing - Font casing (uppercase, lowercase, capitalize, none).
 * @returns {{fontFamily: string, fontWeight: *, fontSize: *, textTransform: *}}
 */
function createElementStyles (weight, size, lineHeight, casing) {
  return {
    fontFamily: DEFAULT_FONT_FAMILY,
    fontWeight: weight,
    fontSize: `${size}rem`,
    textTransform: casing,
    lineHeight
  };
}

export function createTypography (typography = {}) {
  return deepmerge(
    {
      fontFamily: DEFAULT_FONT_FAMILY,
      fontSize: DEFAULT_FONT_SIZE,
      lineHeight: 1,
      body: createElementStyles(FONT_WEIGHT_REGULAR, 1, 1, TEXT_CASE_REGULAR),
      h1: createElementStyles(FONT_WEIGHT_REGULAR, 1.6, 1.8, TEXT_CASE_CAPITALIZE),
      h2: createElementStyles(FONT_WEIGHT_REGULAR, 1.5, 1.6, TEXT_CASE_CAPITALIZE),
      h3: createElementStyles(FONT_WEIGHT_REGULAR, 1.4, 1.4, TEXT_CASE_CAPITALIZE),
      h4: createElementStyles(FONT_WEIGHT_LIGHT, 1.3, 1.3, TEXT_CASE_REGULAR),
      h5: createElementStyles(FONT_WEIGHT_LIGHT, 1.25, 1.25, TEXT_CASE_REGULAR),
      h6: createElementStyles(FONT_WEIGHT_LIGHT, 1.25, 1.25, TEXT_CASE_REGULAR),
      p: createElementStyles(FONT_WEIGHT_REGULAR, 1, 1.1, TEXT_CASE_REGULAR),
      small: createElementStyles(FONT_WEIGHT_REGULAR, 1, 1, TEXT_CASE_REGULAR)
    },
    typography
  );
}

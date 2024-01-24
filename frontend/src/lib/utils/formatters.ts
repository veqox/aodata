export const standard_number_formatter = new Intl.NumberFormat("en", {
  notation: "standard",
});

export const compact_number_formatter = new Intl.NumberFormat("en", {
  notation: "compact",
});

export const narrow_relative_time_formatter = new Intl.RelativeTimeFormat("en-US", {
  style: "narrow",
});
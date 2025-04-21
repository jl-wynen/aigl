css = """
--slate-1: light-dark(#fcfcfd, #111113);
--slate-2: light-dark(#f9f9fb, #18191b);
--slate-3: light-dark(#f0f0f3, #212225);
--slate-4: light-dark(#e8e8ec, #272a2d);
--slate-5: light-dark(#e0e1e6, #2e3135);
--slate-6: light-dark(#d9d9e0, #363a3f);
--slate-7: light-dark(#cdced6, #43484e);
--slate-8: light-dark(#b9bbc6, #5a6169);
--slate-9: light-dark(#8b8d98, #696e77);
--slate-10: light-dark(#80838d, #777b84);
--slate-11: light-dark(#60646c, #b0b4ba);
--slate-12: light-dark(#1c2024, #edeef0);
"""

# indices as in CSS (1-based)
types = {
    "bg": 1,
    "bg_subtle": 2,
    "bg_element": 3,
    "bg_element_hovered": 4,
    "bg_element_active": 5,
    "bg_solid": 9,
    "bg_solid_hovered": 10,
    "border": 6,
    "border_element": 7,
    "border_element_hovered": 8,
    "fg_low_contrast": 11,
    "fg_high_contrast": 12,
}

lines = css.strip().split("\n")

for name, idx in types.items():
    line = lines[idx - 1]
    hex = line.rsplit("#", 1)[1].rstrip(");")
    rgb = tuple(int(hex[i : i + 2], 16) for i in (0, 2, 4))
    print(f"{name}: Color::from_rgb{rgb},")

print("-" * 24)

for name, idx in types.items():
    line = lines[idx - 1]
    hex = line.rsplit("#", 1)[1].rstrip(");")
    print(f'{name}: "#{hex}",')

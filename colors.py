gray = """
--gray-1: light-dark(#fcfcfc, #111111);
--gray-2: light-dark(#f9f9f9, #191919);
--gray-3: light-dark(#f0f0f0, #222222);
--gray-4: light-dark(#e8e8e8, #2a2a2a);
--gray-5: light-dark(#e0e0e0, #313131);
--gray-6: light-dark(#d9d9d9, #3a3a3a);
--gray-7: light-dark(#cecece, #484848);
--gray-8: light-dark(#bbbbbb, #606060);
--gray-9: light-dark(#8d8d8d, #6e6e6e);
--gray-10: light-dark(#838383, #7b7b7b);
--gray-11: light-dark(#646464, #b4b4b4);
--gray-12: light-dark(#202020, #eeeeee);
"""

mauve = """
--mauve-1: light-dark(#fdfcfd, #121113);
--mauve-2: light-dark(#faf9fb, #1a191b);
--mauve-3: light-dark(#f2eff3, #232225);
--mauve-4: light-dark(#eae7ec, #2b292d);
--mauve-5: light-dark(#e3dfe6, #323035);
--mauve-6: light-dark(#dbd8e0, #3c393f);
--mauve-7: light-dark(#d0cdd7, #49474e);
--mauve-8: light-dark(#bcbac7, #625f69);
--mauve-9: light-dark(#8e8c99, #6f6d78);
--mauve-10: light-dark(#84828e, #7c7a85);
--mauve-11: light-dark(#65636d, #b5b2bc);
--mauve-12: light-dark(#211f26, #eeeef0);
"""

ruby = """
--ruby-1: light-dark(#fffcfd, #191113);
--ruby-2: light-dark(#fff7f8, #1e1517);
--ruby-3: light-dark(#feeaed, #3a141e);
--ruby-4: light-dark(#ffdce1, #4e1325);
--ruby-5: light-dark(#ffced6, #5e1a2e);
--ruby-6: light-dark(#f8bfc8, #6f2539);
--ruby-7: light-dark(#efacb8, #883447);
--ruby-8: light-dark(#e592a3, #b3445a);
--ruby-9: light-dark(#e54666, #e54666);
--ruby-10: light-dark(#dc3b5d, #ec5a72);
--ruby-11: light-dark(#ca244d, #ff949d);
--ruby-12: light-dark(#64172b, #fed2e1);
"""

grass = """
--grass-1: light-dark(#fbfefb, #0e1511);
--grass-2: light-dark(#f5fbf5, #141a15);
--grass-3: light-dark(#e9f6e9, #1b2a1e);
--grass-4: light-dark(#daf1db, #1d3a24);
--grass-5: light-dark(#c9e8ca, #25482d);
--grass-6: light-dark(#b2ddb5, #2d5736);
--grass-7: light-dark(#94ce9a, #366740);
--grass-8: light-dark(#65ba74, #3e7949);
--grass-9: light-dark(#46a758, #46a758);
--grass-10: light-dark(#3e9b4f, #53b365);
--grass-11: light-dark(#2a7e3b, #71d083);
--grass-12: light-dark(#203c25, #c2f0c2);
"""

for line in grass.split('\n'):
    if not line.strip():
        continue
    hex = line.rsplit('#', 1)[1].rstrip(');')
    rgb = tuple(int(hex[i:i+2], 16) for i in (0, 2, 4))
    print(f"Color32::from_rgb{rgb},")

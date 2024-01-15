octaves_str = [",,", "," , "", "'", "''"]

for note in ["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"]:
    for octave in range(2,6):
        f = open("" + note + str(octave).capitalize() + ".sheet", "x")
        f.write("""tempo: 140;
device: MyDevice midi _usePort=0;
instrumentDef:lead _onDevice=MyDevice _ch=0 _pc=0;

-- melody track
[
instrument: lead;
{
"""
    + note.replace('s', '#') + octaves_str[octave - 1] +
"""
}
]""")
        f.close()
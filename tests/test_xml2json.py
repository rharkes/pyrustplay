from pyrustplay_rs import sendstring
from pathlib import Path

def test_xml2json():
    with open(Path(Path.cwd(), 'testdata', 'test1.xml'), 'rt') as f:
        myxmlstr = f.read()
    resultjson = sendstring(myxmlstr)
    with open(Path(Path.cwd(), 'testdata', 'test1.json'), 'rt') as f:
        myjsonstr = f.read()
    assert myjsonstr == resultjson
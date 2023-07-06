"""
This is pure python. You import a module and use it, without knowing if it is rust or not.
Sendstring will be imported from the compiled rust module (pyrustplay_rs.pyd) but there is a description of the function in pyrustplay_rs.pyi to help with autocomplete.
"""
from pyrustplay.pyrustplay_rs import sendstring


def runrust(mystring: str) -> bool:
    return sendstring(mystring)

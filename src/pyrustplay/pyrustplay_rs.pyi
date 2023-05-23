"""
This is a little helper file. Python does not know what rust is or what rust functions will expect.
This file helps by making a "fake" function with all the inputs and outputs.
The "real" function is in /src/rust/lib.rs
"""

def sendstring(
    mystr: str
) -> bool: ...
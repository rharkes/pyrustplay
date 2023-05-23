# Rust and Python
Some people learn by messing around with code, by adapting, breaking and fixing it. This repository is meant for those people. It just runs some very basic rust code in python.

# Installation
`pip install -e .`

# Usage
```
from pyrustplay import runrust
result = runrust("tralala")
print(f"Is the length of the string >5? {result}")
>> Is the length of the string >5? True
```
To see how it works:
* cargo.toml: contains information on how to build the rust part of this project
* pyproject.toml: contains information on how to build the python part of this project
* /src/rust/lib.rs: the rust code
* /src/pyrustplay: the python package

# Modification
As you might expect from the name, you are supposed to play around with this repository, because at the moment it just uses rust to tell you if the length of the input string is larger than 5...

"""
A minimal example of a external lambdanote extension.

Extensions can be written in any programming language
that can send and receive JSON-formatted objects
via stdin and stdout.

To define a extension in a lambdanote document use the
"define" extension like this:

| define, uppercase, python simple.py |

Every extension must support two different types of requests
"info" and "action".
"""

from sys import stdin
from json import dumps, loads

# read and deserialize the data from stdin
request = loads(stdin.read())

# responed to the request
if request['type'] == 'info':
    print(dumps({
        "name": "upper",
        "version": "0.1",
        "description": "Convert text to uppercase",
        "supportedFormats": ["html", "latex"],
        "blockSupport": True,
        "inlineSupport": True }))
else:
    print(dumps({"content": [request['arguments'][0].upper()]}))

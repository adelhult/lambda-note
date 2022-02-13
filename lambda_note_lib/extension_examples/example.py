"""
This is an example of an extension written in Python
that interfaces with the lambdanote software using JSON
objects sent via stdin and stdout.

To define a extension in a lambdanote document use the
"define" extension like this:

| define, example, python example.py |

Every extension must support two different types of requests
"info" and "action". Info messages responds with a small description
of the the extension and any potential general warnings or errors.

Action requests are what are being sent when a extension expression
actually is being translated.

Extension expressions can be used as both "block":
------ example, arg1, arg2, arg3 ----
arg0...
-------------------------------------

Or inline:
|example, arg0, arg1, arg2|
"""

from sys import stdin
from json import dumps, loads

request = loads(stdin.read())
response = ""

if request['type'] == 'info':
    response = {
        'name': 'upper',
        'version': '0.1',
        'description': 'Makes all letters uppercase',
        'supportedFormats': ['html', 'latex'],
        'blockSupport': True,
        'inlineSupport': True,
        'errors': [],           # Optional
        'warnings': [],         # Optional
        'interests': []}        # Optional, a list of strings for each
                                # metadata field that the extension wants
                                # to see

elif request['type'] == 'action':
    # every field except "content" is optional
    response = {
        "errors": [],
        "warnings": [],
        "imports": [],
        "bottom": '',
        "top": '',
        "content": [
            'Strings will be treated as raw text and wil no the escaped, '\
            'if you want to use other extensions you can do so with an object like this.',
            
            # This will output the request as a formatted code block
            {
                'name': 'code',
                'block': True,
                'arguments': [
                    dumps(request, indent=2), 'json']
            },
            
            # if you want to allow for lambdanote markup you can do so by using the
            # identity extension.
            {
                'name': 'id',
                'block': True,
                'arguments': [
                    'This is will be **bold \\lambdanote text**!'
                ]
            }
        ]}
else:
    print("Error: unknown request type")

print(dumps(response))

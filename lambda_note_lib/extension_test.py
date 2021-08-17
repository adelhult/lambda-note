"""
Upper - an extension that makes all letters uppercase

This is just a small python application used to test
Lambda note's extension interface.
"""

from sys import stdin, stdout, exit
from json import dumps, loads, JSONDecodeError

raw = stdin.read()

try:
    request = loads(raw)
except JSONDecodeError:
    print("Failed to decode the request")
    exit()

def hello(request):
    response = {
        "name": "upper",
        "version": "0",
        "description": "Makes all letters uppercase",
        "errors": [],
        "warnings": [],
        "interests": ["inverse"],
        "block_support": True,
        "inline_support": True }

    if request["output_format"] != "Html":
        response["errors"].append(
            f"The output format {request['output_format']} is not supported")

    return response

def act(request):
    pass


if request["request_type"] == "hello":
    response = hello(request)
elif request["request_type"] == "action":
    response = act(request)
else:
    print("Unknown request type")
    exit()

stdout.write(dumps(response))

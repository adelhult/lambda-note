# λnote
A simple, highly extendable, markup language inspired by markdown and org mode.

Syntax ideas
```
Metadata:
:: author = Eli Adelhult
:: year   = 2021

Headings:
# Heading 1
## Heading 2
etc...

Divider / new page
===

Extension block (with arguments):

--- code, python, style = clean -----

for i in range(10):
    print(i)

--------------------------------------

Another example

--- math
x^2 + 3z \cdot \Lambda
---


Inline extension might look something like this
| math, x^2 + 3z |

/Italic/
*Bold*
=underlined=
+strikethrough+
^superscript^
_subscript_

TODO: lists
```

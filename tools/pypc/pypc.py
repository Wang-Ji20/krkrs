#!/usr/bin/env python3
import os

OUTSIDER = 0
INQUOTE = 1
INESCAPE = 2


def line_remove_semicolon(line: str) -> str:
    return line.rstrip(';')


line_modifier_addons = [
    line_remove_semicolon
]


def apply_line_modifiers(line: str) -> str:
    for modifier in line_modifier_addons:
        line = modifier(line)
    return line


class PyptoPy():
    _output = []
    _indent = 0
    state = OUTSIDER

    def pyptopy(self, codes: list[str]) -> list[str]:
        return '\n'.join(filter(
            lambda l: l.strip() != '\001',
            map(apply_line_modifiers,
                map(
                    self.process_line, codes))))

    def process_line(self, code: str) -> str:
        return '    ' * self._indent + ''.join(map(self.process_char, list(code.strip())))

    def process_slash(self, c: str) -> str:
        if self.state == INQUOTE:
            self.state = INESCAPE
        return c

    def process_escaped(self, c: str) -> str:
        if self.state == INESCAPE:
            self.state = INQUOTE
            return c
        else:
            raise Exception("Unexpected escape character")

    def process_quote(self, c: str) -> str:
        if self.state == INQUOTE:
            self.state = OUTSIDER
        else:
            self.state = INQUOTE
        return c

    def process_lbrace(self, c: str) -> str:
        if self.state != OUTSIDER:
            return c
        self._indent += 1
        return ':'

    def process_rbrace(self, c: str) -> str:
        if self.state != OUTSIDER:
            return c
        self._indent -= 1
        return '\001'

    def process_char(self, c: str) -> str:
        if self.state == INESCAPE:
            return self.process_escaped(c)
        elif c == '\\':
            return self.process_slash(c)
        elif c == '"':
            return self.process_quote(c)
        elif c == '{':
            return self.process_lbrace(c)
        elif c == '}':
            return self.process_rbrace(c)
        else:
            return c


def read_file(path: str) -> str:
    with open(path, 'r') as f:
        return f.read().splitlines()


if __name__ == '__main__':
    import sys
    if len(sys.argv) < 2:
        print("Usage: pypc.py <filename>")
        sys.exit(1)
    filename = sys.argv[1]
    if not os.path.exists(filename):
        print("File not found: {}".format(filename))
        sys.exit(1)
    pyptopy = PyptoPy()
    print(pyptopy.pyptopy(read_file(filename)))

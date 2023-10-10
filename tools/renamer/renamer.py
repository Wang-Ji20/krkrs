#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# This application:
#
# renamer :: [(String, String)] -> [Path] -> [Path] -> IO ()
#             ^^^^^^^^^^^^^^   ^^^^^^^^^^   ^^^^^^^^^^
#             translate table      assets    references to assets(such as scripts)

import argparse

def load_translate_table(path: str):
    translate_table = []
    with open(path, "r") as f:
        translate_table = [line.split() for line in f.readlines()]
        for (original, translated) in zip(translate_table[::2], translate_table[1::2]):
            translate_table.append((original, translated))
    return translate_table

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Rename assets and references to assets")
    parser.add_argument(
        "translate_table", help="path to translate table")
    parser.add_argument(
        "assets", help="path to assets to rename")
    parser.add_argument(
        "references", help="path to references to assets")
    args = parser.parse_args()
    translate_table = load_translate_table(args.translate_table)



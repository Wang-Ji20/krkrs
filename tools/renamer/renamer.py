#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# This application:
#
# renamer :: [(String, String)] -> [Path] -> [Path] -> IO ()
#             ^^^^^^^^^^^^^^   ^^^^^^^^^^   ^^^^^^^^^^
#             translate table      assets    references to assets(such as scripts)

import argparse
import os

def load_translate_table(path: str):
    translate_table = []
    with open(path, "r") as f:
        translate_table = [line.split() for line in f.readlines()]
        for (original, translated) in zip(translate_table[::2], translate_table[1::2]):
            translate_table.append((original, translated))
    return translate_table

def rename_assets(translate_table: [(str, str)], assets: [str]):
    for asset in assets:
        if os.path.isdir(asset):
            rename_assets(translate_table, os.listdir(asset))
        else:
            for (original, translated) in translate_table:
                if original in asset:
                    os.rename(asset, asset.replace(original, translated))
                    break

def rename_references(translate_table: [(str, str)], references: [str]):
    for reference in references:
        if os.path.isdir(reference):
            rename_references(translate_table, os.listdir(reference))
        else:
            for (original, translated) in translate_table:
                if original in reference:
                    with open(reference, "r") as f:
                        lines = f.readlines()
                    with open(reference, "w") as f:
                        for line in lines:
                            f.write(line.replace(original, translated))
                    break

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
    rename_assets(translate_table, args.assets)
    rename_references(translate_table, args.references)

#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# This application:
#
# discover :: [Path] -> [(String, String)]
#            ^^^^^^^^^^   ^^^^^^^^^^
#               assets    empty translate table

# translate :: [(String, String)] -> [(String, String)]
#              ^^^^^^^^^^^^^^          ^^^^^^^^^^^^^^
#              empty translate table   translated table

# serialize :: [(String, String)] -> IO ()
#              ^^^^^^^^^^^^^^
#              translate table
#
#   format: <asset>
#           <reference> | <special tokens>
#
#   <special tokens>:
#       @OriginalName
#       @Omit
#

from time import sleep
import translators

import os
import argparse
import logging

ORIGINAL_NAME = "@OriginalName"


def discover(assets: [str]) -> [(str, str)]:
    translate = []
    for asset in assets:
        if os.path.isdir(asset):
            translate += discover(os.listdir(asset))
        else:
            translate.append((asset, ORIGINAL_NAME))
    return translate


def make_translator(from_lang: str = 'ja', to_lang: str = 'en', errata: str = ""):
    """
    Make a translator
    >>> make_translator()([("茶", "@OriginalName")])
    [('茶', 'tea')]
    """
    def translate(translate_table: [(str, str)]) -> [(str, str)]:
        # because I cannot send too many requests to translators at once,
        aggregated_assets = '\n'.join(
            [asset for (asset, _) in translate_table])
        # ... and I cannot send too big requests to translators at once,
        # FIXME: change a way to segment the text, have bugs here.
        segments = [aggregated_assets[i:i + 1000]
                    for i in range(0, len(aggregated_assets), 1000)]
        segmented_trans = []
        for (idx, segment) in enumerate(segments):
            # ... and I cannot send too quick reqs..
            sleep(20)
            logging.info("Translating segment: %d/%d", idx + 1, len(segments))
            segmented_trans.append(translators.translate_text(
                segment, translator='bing', from_language=from_lang, to_language=to_lang))
        aggregated_trans = process_translated(''.join(segmented_trans), errata)
        # spaces make troubles in `.ks` parsing
        return [(asset, trans) for ((asset, _), trans) in zip(translate_table, aggregated_trans.split('\n'))]
    return translate

# ================================================================================================
# there's some problems with the translated text which we may want to fix
# for example, I don't want Ilia, I want 'Illya'. I don't want an 'o' with a bar, it may be `ou`
# so we need to precess the translated text
# =================================================================================================


def process_translated(translated: str, errata: str = '') -> str:
    errata_processor = [lambda x: x.replace(' ', '')]
    if errata:
        errata_processor.append(lambda x: enforce_rules(x, errata=errata))
    for processor in errata_processor:
        translated = processor(translated)
    return translated

# enforce rules
# rules:
# <patterns> -> <action>


def enforce_rules(translated: str, errata: str) -> str:
    rules = parse_rules(errata)
    for rule in rules:
        translated = rule(translated)
    return translated


def parse_rule(rule: str):
    """
    parse a string rule
    >>> parse_rule("ilia -> Illya")("I walk in a little street with ilia.")
    'I walk in a little street with Illya.'
    """
    [pattern, action] = rule.strip().split(" -> ")

    def enforce_rule(translated: str) -> str:
        return translated.replace(pattern, action)
    return enforce_rule


def parse_rules(errata: str):
    with open(errata, "r") as f:
        rules = [parse_rule(line) for line in f.readlines()]
    return rules


def serialize_translate_table(translate_table: [(str, str)], output: str) -> None:
    with open(output, "w") as f:
        for (asset, translated) in translate_table:
            f.write(asset + "\n" + translated + "\n")

# ===---------------------------------------------------------------------====
# COMMAND LINE INTERFACE
# ===---------------------------------------------------------------------====


def discover_subcommand(args) -> None:
    logging.info("Discovering files...")
    with open(args.output, "w") as f:
        for (asset, translated) in discover(args.assets):
            logging.info("Discovered asset: %s", asset)
            f.write(asset + "\n" + translated + "\n")


def translate_subcommand(args) -> None:
    logging.info("Translating...")
    translate = make_translator(errata=args.errata if args.with_errata else "")
    with open(args.output, "w") as f:
        for (asset, translated) in translate(discover(args.assets)):
            f.write(asset + "\n" + translated + "\n")


def errata_subcommand(args) -> None:
    logging.info("Enforcing rules...")
    translate_table = []
    with open(args.output, "r") as f:
        translate_table = f.readlines()
    translate_table = [enforce_rules(line, args.errata)
                       for line in translate_table]
    with open(args.output, "w") as f:
        f.writelines(translate_table)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    parser = argparse.ArgumentParser()
    parser.add_argument("--discover", "-d", action="store_true",
                        help='This mode will only scan your assets folder, find all asset names that you may want to rename.')
    parser.add_argument("--output", "-o", type=str,
                        default="translate-table.txt", help='where you place your resulting translate table.')
    parser.add_argument("--errata", "-e", type=str, default="",
                        help="errata file for replacing some words on which you think machine translators did a bad job.")
    parser.add_argument("--with-errata", "-E",
                        action="store_true", default=False)
    parser.add_argument('--only-errata', "-O",
                        action="store_true", default=False)
    parser.add_argument("assets", type=str, nargs="+")
    args = parser.parse_args()
    if args.discover:
        discover_subcommand(args)
    elif args.only_errata:
        errata_subcommand(args)
    else:
        translate_subcommand(args)

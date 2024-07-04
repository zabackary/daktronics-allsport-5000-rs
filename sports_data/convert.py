import csv
import os
import sys
from os import path
from typing import Iterator, TextIO

from xlsx2csv import Xlsx2csv  # pip install xlsx2csv


def process_sport(reader: Iterator[list[str]], out: TextIO):
    next(reader)  # skip the header row

    missing = False

    class Continue(Exception):
        pass

    # write the prefix of the file
    out.write(
        "//! This sport was generated semi-automatically and may contain errors.\n")
    out.write("super::sport_builder!(\n")
    out.write(f"    {sport_struct_name},\n")
    out.write(f"    \"{sport}\",\n")
    out.write(f"    {'false' if missing else 'true'},\n")
    out.tell()

    for row in reader:
        try:
            if len(row) != 8:
                print("skipping sport since len of columns is not 8",
                      file=sys.stderr)
                break
            row_out = "("
            row_missing = False
            deprecated = False
            for i, cell in enumerate(row):
                if i == 3:
                    row_out += "\"{}\"".format(cell.replace("\"", "\\\""))
                elif i == 7:
                    cell_bang = cell.rsplit("!", 1)
                    cell = cell_bang[0]
                    if len(cell_bang) > 1:
                        match cell_bang[1]:
                            case "exclude":
                                raise Continue()  # skip this row
                            case "deprecated":
                                deprecated = True
                    row_out += "\"{}\"".format(cell.replace("\"", "\\\""))
                elif cell == "":
                    row_out += "/* missing */"
                    row_missing = True
                else:
                    row_out += cell

                if i < 7:
                    row_out += ", "
            if deprecated:
                row_out += ", deprecate"
            row_out += "),"
            if row_missing:
                row_out = f" // {row_out}"
            else:
                row_out = f"    {row_out}"
            out.write(f"{row_out}\n")
        except Continue:
            continue

    # write the suffix to the file
    out.seek(out.tell() - 3)  # chop off the last comma
    out.write("\n);\n")


if __name__ == "__main__":
    root = path.dirname(__file__)
    csv_out = path.join(root, "csv")
    rust_out = path.join(root, "rs")
    Xlsx2csv(
        path.join(root, "sports_data.xlsx"),
        outputencoding="utf-8",
        skip_empty_lines=True,
        skip_trailing_columns=True
    ).convert(
        csv_out,
        sheetid=0
    )
    os.makedirs(rust_out, exist_ok=True)
    for sport in os.listdir(csv_out):
        sport = sport.removesuffix(".csv")
        sport_struct_name = sport.replace(" ", "")
        with open(path.join(rust_out, f"{sport}.rs"), "w+") as out:
            with open(path.join(csv_out, f"{sport}.csv"), "r") as input:
                reader = csv.reader(input)
                process_sport(reader, out)

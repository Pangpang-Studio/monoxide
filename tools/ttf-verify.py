"""
This script is used to verify the correctness of TTF files.

It uses the FontTools library to parse the TTF file and check for any errors.
"""

from io import BytesIO
from fontTools.ttLib import TTFont
import argparse
from pathlib import Path

parser = argparse.ArgumentParser(description="Verify the correctness of a TTF file.")
parser.add_argument("file", type=str, help="The TTF file to verify.")


def main():
    args = parser.parse_args()
    file = Path(args.file)

    if not file.exists():
        print(f"File {file} does not exist.")
        exit(1)

    phase = "read"
    try:
        font = TTFont(file)
        print("Read the TTF file successfully.")

        # Dump info using xml. The process read all data in the file,
        # so it will fail if the file is corrupted.
        phase = "ttf->xml"
        out_xml = file.with_suffix(".ttx")
        font.saveXML(out_xml)

        print(f"Dumped XML to {out_xml}.")

        # Convert the XML back to TTF for an additional round of error checking.
        phase = "xml->ttf"
        font = TTFont()
        font.importXML(out_xml)
        font.save(BytesIO())
        print("Converted XML back to TTF.")

        print(f"File {file} is a valid TTF file.")
    except Exception as e:
        import traceback

        print(f"Error verifying file {file} ({phase}): {e}")
        traceback.print_exc()

        print("The font file generated is malformed.")
        exit(1)


if __name__ == "__main__":
    main()

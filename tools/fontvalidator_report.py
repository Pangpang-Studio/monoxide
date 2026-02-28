#!/usr/bin/env python3
import argparse
import os
import sys
import xml.etree.ElementTree as ET
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from typing import DefaultDict, Dict, List, Tuple


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize FontValidator XML reports."
    )
    parser.add_argument(
        "report",
        help="Path to FontValidator XML report (e.g., target/validate/out.ttf.report.xml)",
    )
    return parser.parse_args()


def truncate(value: str, limit: int = 200) -> str:
    if len(value) <= limit:
        return value
    return value[: limit - 3] + "..."


@dataclass
class GroupInfo:
    count: int = 0
    details: List[str] = field(default_factory=list)


def main() -> int:
    args = parse_args()
    if not os.path.exists(args.report):
        print(f"Report not found: {args.report}", file=sys.stderr)
        return 2

    try:
        root = ET.parse(args.report).getroot()
    except ET.ParseError as exc:
        print(f"Failed to parse XML: {exc}", file=sys.stderr)
        return 2

    counts: Counter[str] = Counter()
    groups: Dict[str, DefaultDict[Tuple[str, str], GroupInfo]] = {
        "E": defaultdict(GroupInfo),
        "W": defaultdict(GroupInfo),
    }

    for report in root.iter("Report"):
        error_type = report.attrib.get("ErrorType", "?")
        counts[error_type] += 1
        if error_type in groups:
            key = (report.attrib.get("ErrorCode", ""), report.attrib.get("Message", ""))
            entry = groups[error_type][key]
            entry.count += 1
            detail = report.attrib.get("Details")
            if detail and len(entry.details) < 3:
                entry.details.append(detail)

    print(f"FontValidator report: {args.report}")
    summary = ", ".join(
        f"{key}={counts.get(key, 0)}" for key in ["E", "W", "P", "I", "?"]
        if counts.get(key, 0)
    )
    print(f"Counts: {summary if summary else 'no Report entries found'}")

    for label, code in [("Errors", "E"), ("Warnings", "W")]:
        items = groups[code]
        total_groups = len(items)
        if total_groups == 0:
            print(f"{label}: none")
            continue

        print(f"{label} ({total_groups} groups):")
        sorted_items = sorted(
            items.items(),
            key=lambda kv: (-kv[1].count, kv[0][0], kv[0][1]),
        )
        for (error_code, message), info in sorted_items:
            detail = info.details[0] if info.details else ""
            detail_text = f" (sample: {truncate(detail)})" if detail else ""
            print(f"- {error_code} x{info.count}: {message}{detail_text}")

    return 0


if __name__ == "__main__":
    sys.exit(main())

import argparse
import gzip
from typing import List


def process_depth_file(file_path: str, thresholds: List[int]) -> List[float]:
    open_func = gzip.open if file_path.endswith(".gz") else open

    total_base = 0
    thresholds_counts = [0] * len(thresholds)

    with open_func(file_path, "rt") as f:
        for line in f:
            if line.startswith("#"):
                continue
            parts = line.strip().split("\t")
            try:
                depth = int(parts[2])
            except ValueError:
                continue

            total_base += 1
            for i, threshold in enumerate(thresholds):
                if depth >= threshold:
                    thresholds_counts[i] += 1

    if total_base == 0:
        return [0.0] * len(thresholds)

    return [round(count / total_base * 100, 2) for count in thresholds_counts]


def main():
    parser = argparse.ArgumentParser(
        description="Calculate coverage percentages for different depth thresholds"
    )
    parser.add_argument(
        "--depth_file", type=str, required=True, help="Path to the depth file"
    )
    parser.add_argument(
        "--thresholds",
        type=str,
        required=True,
        default="1,2,5,10,20",
        help="Comma separated list of depth thresholds, default: 1,2,5,10,20",
    )
    args = parser.parse_args()

    thresholds = list(map(int, args.thresholds.split(",")))

    try:
        coverage_percentages = process_depth_file(args.depth_file, thresholds)
    except Exception as e:
        print(f"Error processing depth file: {str(e)}")
        return

    print("\t".join([f"{t}X" for t in thresholds]))
    print("\t".join([f"{p:.2f}" for p in coverage_percentages]))


if __name__ == "__main__":
    main()

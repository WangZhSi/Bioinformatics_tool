import json
import argparse


# envs
ASCP = "ascp"
ASPERA_KEY = "key"

#### info ####
__author__ = "wangzhsi"
__mail__ = "wang_zsi@outlook.com"
__date__ = "20240909"
__version__ = "1.0"


def filter_data(data, filters):
    if not filters:
        return data
    filter_data = []
    for record in data:
        match = all(record.get(key) == value for key, value in filters.items())
        if match:
            filter_data.append(record)
    return filter_data


def generate_command(data, download_method, output_dir):
    commands = []
    for record in data:
        if download_method == "ftp":
            fastq_url = record["fastq_ftp"]
            urls = fastq_url.split(";")
            for url in urls:
                commands.append(f"wget -P {output_dir}/ {url}")
        elif download_method == "ascp":
            fastq_url = record["fastq_aspera"]
            urls = fastq_url.split(";")
            for url in urls:
                commands.append(
                    f"{ASCP} -QT -l 300m -P33001 -i {ASPERA_KEY} era-fasp@{url} {output_dir}/"
                )
    for command in commands:
        print(command)


def main():
    function = '''This program is used to parse ENA metadata and generate download commands
    \nExample:
        python3 parse_ena_json.py -i metadata.json -m ftp -o output_dir ; 
    If filter one key:
        python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f sample_accession=SAMN12345678 ; 
    If filter multiple keys:
        python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f sample_accession=SAMN12345678 experiment_accession=ERX12345678 ; 
    If space in the value:
        python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f scientific_name="Bos indicus"'''

    parser = argparse.ArgumentParser(
        description=function,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )

    parser.add_argument(
        "-i", "--input", required=True, help="Input JSON file containing metadata"
    )
    parser.add_argument(
        "-m",
        "--method",
        required=True,
        choices=["ftp", "ascp"],
        help="Download method: 'ftp' or 'ascp'",
        default="ftp",
    )
    parser.add_argument(
        "-o",
        "--output",
        required=True,
        help="Output directory for downloaded files",
        default="./",
    )
    parser.add_argument(
        "-f",
        "--filter",
        nargs="*",
        help="Optional filters in the format 'key=value'.",
    )

    args = parser.parse_args()

    with open(args.input, "r") as f:
        data = json.load(f)

    filters = {}
    if args.filter:
        for f in args.filter:
            key, value = f.split("=", 1)
            filters[key] = value.strip('"')

    filtered_data = filter_data(data, filters)

    generate_command(filtered_data, args.method, args.output)


if __name__ == "__main__":
    main()

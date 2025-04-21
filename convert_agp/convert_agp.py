import re
import argparse

from collections import defaultdict, OrderedDict


def _safe_int(value) -> int | None:
    try:
        return int(value)
    except ValueError:
        print(f"Warning: Unable to convert {value} to int. Returning None.")
        return None


def _convert_strand(strand: str) -> str:
    """
    convert strand
    0/1 to +/-
    or
    +/- to 0/1
    """
    conversions = {"0": "+", "1": "-", "+": "0", "-": "1"}
    return conversions.get(strand, ".")


def get_size_dict(input_file: str) -> dict:
    """
    get contig size dictionary from input file, ctg\tsize
    """
    size_dict = {}
    with open(input_file) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            lines = line.strip().split("\t")
            if len(lines) < 2:
                continue
            ctg = lines[0]
            size = _safe_int(lines[1])
            if size is None:
                continue
            size_dict[ctg] = int(size)
    return size_dict


def get_id_relation_dict(input_file: str) -> dict:
    """
    get id relation dictionary from input file, id\tnew_id
    """
    id_relation_dict = {}
    with open(input_file) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            lines = line.strip().split("\t")
            if len(lines) < 2:
                continue
            old_id = lines[0]
            new_id = lines[1]
            id_relation_dict[old_id] = new_id
    return id_relation_dict


def parse_string(string: str) -> list:
    """
    parse string
    """
    if "," in string:
        return string.split(",")
    else:
        return [string]


class AGPParser:
    def __init__(self, agp_file):
        self.agp_file = agp_file
        self.contigs = []
        self.agp_format = None
        self.contig_size_dict = {}

        self._process()

    def _read_line(self):
        valid_lines = []
        with open(self.agp_file) as f:
            for line in f:
                if not line or line.startswith("#"):
                    continue
                lines = line.strip().split("\t")
                if len(lines) == 4 or len(lines) == 9:
                    valid_lines.append(lines)
        return valid_lines

    def _determine_format(self, lines):
        col_counts = set(len(parts) for parts in lines)
        if 9 in col_counts:
            return 9
        elif 4 in col_counts:
            return 4
        else:
            raise ValueError("Invalid AGP format")

    def _process_4col(self, lines):
        for parts in lines:
            if len(parts) != 4:
                continue
            chrom, contig, strand, pos = parts
            if strand not in ("0", "1"):
                raise ValueError("Invalid strand in AGP file")
            try:
                self.contigs.append((chrom, contig, strand, int(pos)))
            except ValueError:
                raise ValueError("Invalid position in AGP file")

    def _process_9col(self, lines):
        chrom_registry = defaultdict(dict)
        temp = []

        for parts in lines:
            if len(parts) != 9 or parts[4] in ["N", "U"]:
                continue
            chrom = parts[0]
            contig = parts[5]
            strand = _convert_strand(parts[8])  # convert strand from +/- to 0/1
            if strand not in ("0", "1"):
                raise ValueError("Invalid strand in AGP file")

            try:
                start = int(parts[6])
                end = int(parts[7])
                self.contig_size_dict[contig] = end - start + 1
            except ValueError:
                raise ValueError("Invalid position in AGP file")

            if contig not in chrom_registry[chrom]:
                chrom_registry[chrom][contig] = len(chrom_registry[chrom]) + 1

            temp.append((chrom, contig, strand, chrom_registry[chrom][contig]))

        self.contigs = sorted(
            temp, key=lambda x: (x[0], x[3])
        )  # sort by chrom and contig order

    def _process(self):
        lines = self._read_line()
        if not lines:
            raise ValueError("AGP file is empty or invalid")

        self.agp_format = self._determine_format(lines)

        if self.agp_format == 4:
            self._process_4col(lines)
        elif self.agp_format == 9:
            self._process_9col(lines)
            self.contig_size_dict = dict(self.contig_size_dict)
        else:
            raise ValueError("Invalid AGP format")

    def get_result(self):
        # contigs: list of tuples [(chrom, contig, strand, pos)]
        # agp_format: 4 or 9
        # contig_size_dict: dictionary of contig size, {contig: size}
        return self.contigs, self.agp_format, self.contig_size_dict


def select_chrom_id(contigs: list, select_list: list) -> list:
    return [
        contig for contig in contigs if any(key in contig[0] for key in select_list)
    ]


def filter_chrom_id(contigs, filter_list) -> list:
    filtered = []

    for chrom, contig, strand, pos in contigs:
        new_chrom = chrom
        for key in filter_list:
            new_chrom = new_chrom.replace(key, "")

        filtered.append((new_chrom, contig, strand, pos))

    return filtered


def change_chrom_id(contigs, id_relation_dict) -> list:
    changed = []

    for chrom, contig, strand, pos in contigs:
        new_chrom = id_relation_dict.get(chrom, chrom)
        changed.append((new_chrom, contig, strand, pos))

    return changed


def reorder_pos(contigs: list) -> list:
    new_contigs = []
    current_chrom = None
    current_pos = 1
    for chrom, contig, strand, pos in contigs:
        if chrom != current_chrom:
            current_chrom = chrom
            current_pos = 1
        new_contigs.append((chrom, contig, strand, current_pos))
        current_pos += 1
    return new_contigs


def group_contigs_by_chrom(contigs: list):
    """
    return:
    chrom_order: list of chrom ids
    chrom_groups: dict of chrom groups
    {chrom: [(contig, strand, pos), ...]}
    """
    chrom_order = []
    chrom_groups = OrderedDict()
    for chrom, contig, strand, pos in contigs:
        if chrom not in chrom_groups:
            chrom_groups[chrom] = []
            chrom_order.append(chrom)
        chrom_groups[chrom].append((contig, strand, pos))
    return chrom_order, chrom_groups


def insert_gap(contigs: list) -> list:
    """
    insert gap between contigs
    return: [(chrom, contig, strand, pos), (chrom, "gap", strand, pos)]
    """
    chrom_order, chrom_groups = group_contigs_by_chrom(contigs)

    processed = []

    for chrom in chrom_order:
        group = chrom_groups[chrom]
        sorted_group = sorted(group, key=lambda x: x[2])

        with_gaps = []
        for i, (contig_name, strand, pos) in enumerate(sorted_group):
            with_gaps.append((chrom, contig_name, strand, pos))

            if i < len(sorted_group) - 1:
                with_gaps.append((chrom, "GAP", ".", 0))

        processed.extend(with_gaps)

    return processed


def revers_chrom(contigs: list, target_chr: list) -> list:
    chrom_order, chrom_groups = group_contigs_by_chrom(contigs)

    processed = []

    for chrom in chrom_order:
        group = chrom_groups[chrom]

        if chrom in target_chr:
            reversed_contigs = list(reversed(group))
            new_group = []
            for contig, strand, pos in reversed_contigs:
                new_strand = 1 - _safe_int(strand)  # 0 -> 1, 1 -> 0
                new_group.append((chrom, contig, new_strand, pos))
            processed.extend(new_group)
        else:
            for contig, strand, pos in group:
                processed.append((chrom, contig, strand, pos))
    return processed


def _nature_key(s: str):
    return [
        int(text) if text.isdigit() else text.lower() for text in re.split(r"(\d+)", s)
    ]


def nature_order(contigs: list) -> list:
    """
    sort contigs using natural order
    """
    processed = []

    chrom_order, chrom_groups = group_contigs_by_chrom(contigs)
    chrom_order_sort = sorted(chrom_order, key=_nature_key)

    for chrom in chrom_order_sort:
        for contig, strand, pos in chrom_groups[chrom]:
            processed.append((chrom, contig, strand, pos))
    return processed


def get_chrom_size_dict(contigs: list, size_dict: dict, chrom_prefix) -> dict:
    chrom_size_dict = {}  # {origin_id: total_size}
    change_relation_dict = {}  # {origin_id: new_id}
    ranked_chrom = {}  # {origin_id: rank} e.g. {"chr1": 1, "chr2": 3, "chr3": 2}
    total_size = 0
    chrom_order, chrom_groups = group_contigs_by_chrom(contigs)

    for chrom in chrom_order:
        for contig, _, _ in chrom_groups[chrom]:
            total_size += size_dict.get(contig, 0)
        chrom_size_dict[chrom] = total_size
        total_size = 0

    ranked_chrom = {
        k: i + 1
        for i, (k, _) in enumerate(sorted(chrom_size_dict.items(), key=lambda x: -x[1]))
    }

    for chrom, rank in ranked_chrom.items():
        change_relation_dict[chrom] = chrom_prefix + str(rank)
        print(f"{chrom} -> {chrom_prefix + str(rank)}")

    return change_relation_dict


def format_9col_agp(contigs: list, size_dict: dict, gap_size: int) -> list:
    """
    input: list of tuples [(chrom, contig, strand, order)]
    output:
    [(chrom, start, end, order, type, contig, contig_start, contig_size, strand)]
    """
    if gap_size < 0:  # 防呆
        raise ValueError("Gap size must be positive")

    GAP_TYPE = "U"
    CONTIG_TYPE = "W"
    GAP_METHOD = "proximity_ligation"

    current_chrom = None
    current_start = 1
    processed = []

    for chrom, contig, strand, order in contigs:
        if chrom != current_chrom:
            current_chrom = chrom
            current_start = 1

        start = current_start

        if contig == "GAP":
            if gap_size == 0:
                continue
            end = start + gap_size - 1
            processed.append(
                (chrom, start, end, order, GAP_TYPE, "GAP", 1, gap_size, GAP_METHOD)
            )
            current_start = end + 1
        else:
            size = size_dict.get(contig)
            if not size:
                raise ValueError(f"Contig size not found for {contig}")
            end = start + size - 1
            converted_stand = _convert_strand(strand)
            processed.append(
                (
                    chrom,
                    start,
                    end,
                    order,
                    CONTIG_TYPE,
                    contig,
                    1,
                    size,
                    converted_stand,
                )
            )
            current_start = end + 1

    return processed


def main():
    parser = argparse.ArgumentParser(description="Convert AGP files")

    basic_group = parser.add_argument_group(
        "Basic parameters", "Basic convert settings"
    )
    basic_group.add_argument(
        "-i", "--input", type=str, required=True, help="Input AGP file"
    )
    basic_group.add_argument(
        "-o", "--output", type=str, required=True, help="Output AGP file"
    )
    basic_group.add_argument(
        "-F",
        "--output_format",
        type=int,
        required=True,
        choices=[4, 9],
        help="Output AGP format (4 or 9)",
    )
    basic_group.add_argument(
        "-s",
        "--size",
        type=str,
        required=False,
        help="Contig size file (tab-separated: ctg\tsize). Required when converting agp4 to agp9",
    )
    basic_group.add_argument(
        "-g",
        "--gap_size",
        type=int,
        required=False,
        default=100,
        help="Gap size for AGP4 to AGP9 conversion, default=100bp",
    )

    change_group = parser.add_argument_group(
        "Changing parameters",
        "using for change/filter chrom id, reverse whole chroms\nProcess order: select -> filter -> id2id -> reverse",
    )
    change_group.add_argument(
        "--select",
        type=str,
        required=False,
        help="Select chromosome contain string(comma-separated, e.g. 'chr,RagTag')",
    )
    change_group.add_argument(
        "--filter",
        type=str,
        required=False,
        help="Filter string (comma-separated, e.g. '_RagTag,_1.0,NX_')",
    )
    change_group.add_argument(
        "--id2id",
        type=str,
        required=False,
        help="ID relation file (id\tnew_id) to change chrom IDs",
    )
    change_group.add_argument(
        "--reverse",
        type=str,
        required=False,
        help="Chromosomes to reverse (comma-separated, e.g. 'chr1,chr2')",
    )

    order_group = parser.add_argument_group(
        "Output order parameters", "Reorder output file by rules"
    )
    order_group.add_argument(
        "--nature",
        action="store_true",
        default=False,
        help="Reorder output in nature order: 1,2,3...10",
    )
    order_group.add_argument(
        "--sizeOrder",
        action="store_true",
        default=False,
        help="Rename chromosome by size.",
    )
    order_group.add_argument(
        "--prefix",
        type=str,
        default="chr",
        required=False,
        help="Prefix of rename chromosome by size, default = 'chr'",
    )

    args = parser.parse_args()

    parse = AGPParser(args.input)
    contigs, agp_format, size_dict = parse.get_result()

    # choose size
    final_size_dict = {}
    if args.size:
        print("Process: parsing input size file as final size dict.")
        final_size_dict = get_size_dict(args.size)
    elif size_dict:
        print("Process: parsing size dict from input agp.")
        final_size_dict = size_dict

    # convert format
    if args.select:
        print("Process: select chromosome by keywords.")
        select_lst = parse_string(args.select)
        contigs = select_chrom_id(contigs, select_lst)
    if args.filter:
        print("Process: filter out keywords.")
        filter_lst = parse_string(args.filter)
        contigs = filter_chrom_id(contigs, filter_lst)
    if args.id2id:
        print("Process: replace chromosome ID.")
        id_relation_dict = get_id_relation_dict(args.id2id)
        contigs = change_chrom_id(contigs, id_relation_dict)
    if args.reverse:
        print("Process: rever whole chromosome.")
        reverse_list = parse_string(args.reverse)
        contigs = revers_chrom(contigs, reverse_list)

    contigs = reorder_pos(contigs)  # must reorder before insert gap

    if args.output_format == 9 and args.gap_size > 0:
        print("Process: insert gap lines.")
        contigs = insert_gap(contigs)

    # final reorder
    contigs = reorder_pos(contigs)
    if args.nature:
        print("Process: reorder chromosome order.")
        contigs = nature_order(contigs)

    if args.sizeOrder:
        if not final_size_dict:
            print("Can't find contig size, skip reorder by size.")
        else:
            print("Process: rename chromosome by total size")
            change_relation_dict = get_chrom_size_dict(
                contigs, final_size_dict, args.prefix
            )
            contigs = change_chrom_id(contigs, change_relation_dict)
            contigs = nature_order(contigs)

    # output
    if args.output_format == 4:
        processed = contigs
    elif args.output_format == 9:
        if not final_size_dict:
            raise ValueError("Contig size file is required for AGP9 format")
        processed = format_9col_agp(contigs, final_size_dict, args.gap_size)
    with open(args.output, "w") as f:
        for info in processed:
            f.write("\t".join(map(str, info)) + "\n")
    print(f"AGP file converted and saved to {args.output}")


if __name__ == "__main__":
    main()

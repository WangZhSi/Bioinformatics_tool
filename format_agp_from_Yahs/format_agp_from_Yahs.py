##### import #####
import os
import sys
import argparse

#### info ####
__author__ = "zhongsiwang"
__date__ = "20231009"
__update__ = "20240311"
__version__ = "1.0"


#### main ####
class Agp:
    """
    初始化ContigProcessor类.

    :param size_path: str, 表示 size 文件的路径.
    :param yahs_path: str, 表示 yahs 文件的路径.
    :param agp4_path: str, 表示 agp4 文件的路径.
    :param split_log_path: str, 表示 split_log 文件的写入路径.
    :param split_bed_path: str, 表示 split_bed 文件的写入路径.
    :param min_contig: int, 保留的最小 contig 长度.
    :param min_scaffold: int, 保留的最小 scaffold 长度.
    """

    def __init__(
        self, size, yahs, agp4, split_log, split_bed, min_contig, min_scaffold
    ):
        self.open_files(size, yahs, agp4, split_log, split_bed)
        self.contig_dic = {}
        self._info = []
        self.split_contigs = []
        self.short_contigs = []
        self.min_contig = min_contig
        self.min_scaffold = min_scaffold
        self.chrxorder = 1

    def __del__(self):
        """
        析构函数, 确保打开的文件被正确关闭
        """
        for file in [self.size, self.yahs, self.split_log, self.split_bed, self.agp4]:
            if hasattr(file):
                file.close()

    def open_files(self, size, yahs, agp4, split_log, split_bed):
        """
        打开文件

        :raises FileNotFoundError: 如果文件不存在.
        :raises PermissionError: 如果没有文件的访问权限.
        """
        try:
            self.size = open(size, "r", encoding="utf-8")
            self.yahs = open(yahs, "r", encoding="utf-8")
            self.split_log = open(split_log, "w", encoding="utf-8")
            self.split_bed = open(split_bed, "w", encoding="utf-8")
            self.agp4 = open(agp4, "w", encoding="utf-8")
        except FileNotFoundError as fnf_error:
            print(f"FileNotFoundError: {fnf_error}")
            sys.exit(1)
        except PermissionError as p_error:
            print(f"PermissionError: {p_error}")
            sys.exit(1)

    def get_short_contigs(self):
        """
        获取长度小于指定值的contigs
        """
        for line in self.size:
            if line.startswith("#") or not line:
                continue
            lines = line.strip().split("\t")
            if int(lines[1]) < self.min_contig:
                self.short_contigs.append(lines[0])

    def parse(self):
        """
        依据 Scaffold ID 拆分 yahs 的 agp 文件
        """
        node = ""
        scaffold = ""
        for line in self.yahs:
            if not line or "#" in line:
                continue
            lines = line.strip().split("\t")
            if scaffold != lines[0]:
                yield node  # 第一个node返回为空, 在self.run()中判断去除
                node = ""
            node = node + line
            scaffold = lines[0]
        yield node

    def judge_contig(self, contig):  # 判断contig是否切割
        """
        判断 contig 是否切割;
        对给定的 contig 进行计数, 如果 contig 不是已有 contig, 则初始化计数为1;
        如果已出现过, 说明 contig 存在切割, 则计数加1.
        :param contig: 要判断的contig, 必须是str类型.
        """
        if contig not in self.contig_dic.keys():
            self.contig_dic[contig] = 1
        else:
            self.contig_dic[contig] += 1

    def parser_node(self, node, chr_num):
        """
        解析node
        提取所需要的信息
        """
        node = node.split("\n")
        order = 1  # 每条chr中contig序号
        for o in node:
            o = o.strip()
            if not o:
                continue
            _o = o.split("\t")
            if _o[4] == "N":
                continue  # 跳过插入的200bp N
            contig = _o[5]
            if contig in self.short_contigs:
                continue  # 过滤较短的contig
            self.judge_contig(contig)
            contig_new = contig + f"_{self.contig_dic[contig]}"  # contig -> contig_2
            start = int(_o[6])
            end = int(_o[7])
            strand = _o[8]
            self._info.append(
                dict(
                    zip(
                        [
                            "chr",
                            "contig_origin",
                            "contig_new",
                            "start",
                            "end",
                            "strand",
                            "number",
                            "order",
                        ],
                        [
                            chr_num,
                            contig,
                            contig_new,
                            start,
                            end,
                            strand,
                            self.contig_dic[contig],
                            order,
                        ],
                    )
                )
            )  # 实际上可以增加更多信息
            order += 1

    def get_split_contigs(self):
        """
        获取切割contig
        判断字典 self.contig_dic 中的值是否大于1
        """
        split_contigs = []
        for key, value in self.contig_dic.items():
            if value > 1:
                split_contigs.append(key)
        split_contigs = list(set(split_contigs))
        return split_contigs

    def get_split_log_info(self, dict_tmp):
        """
        生成切割日志
        不防呆, 输入文件必须为指定格式, 否则无法正常工作
        """
        contig_origin = dict_tmp["contig_origin"]
        contig_new = dict_tmp["contig_new"]
        start = int(dict_tmp["start"])
        end = int(dict_tmp["end"])
        lenth = end - start + 1
        info = f"{contig_origin}\t{contig_new}\t{start}\t{end}\t{lenth}\n"
        return info

    def get_split_bed_info(self, dict_tmp):
        """
        生成切割bed文件
        用于 seqkit 切割 fa
        """
        contig_origin = dict_tmp["contig_origin"]
        start = int(dict_tmp["start"]) - 1
        end = int(dict_tmp["end"])
        num = int(dict_tmp["number"])
        info = f"{contig_origin}\t{start}\t{end}\t{num}\n"
        return info

    def write_split_info(self):
        """
        提取切割了的 contig 并输出 log 和 bed
        """
        r = list(filter(lambda x: x["contig_origin"] in self.split_contigs, self._info))
        for i in r:
            log_info = self.get_split_log_info(i)
            self.split_log.write(log_info)
            bed_info = self.get_split_bed_info(i)
            self.split_bed.write(bed_info)

    def get_agp4(self):
        """
        输出四列 agp 文件
        chr\tcontig\tstrand\torder
        """
        strand_dict = {"+": 0, "-": 1}

        for i in self._info:
            # chr
            chro = i["chr"]
            # contig, 需要区分是否切割
            if i["contig_origin"] in self.split_contigs:
                ctg = i["contig_new"]
            else:
                ctg = i["contig_origin"]
            # strand
            strand = strand_dict[i["strand"]]
            # order, 需要判断是否为 chrx
            if chro == "chrX":
                order = self.chrxorder
                self.chrxorder += 1
            else:
                order = i["order"]

            info = f"{chro}\t{ctg}\t{strand}\t{order}\n"

            self.agp4.write(info)

    def run(self):
        """
        主程序
        """
        num = 1
        self.get_short_contigs()
        for node in self.parse():
            node = node.strip()
            if not node:
                continue  # 空 node 的检查
            if len(node.split("\n")) == 1:
                tmp = node.split("\t")
                if int(tmp[2]) > self.min_scaffold:
                    chrnum = "chr" + str(num)
                else:
                    chrnum = "chrX"
            else:
                chrnum = "chr" + str(num)
            self.parser_node(node, chrnum)
            num += 1
        self.split_contigs = self.get_split_contigs()
        self.write_split_info()
        self.get_agp4()


def main():
    function = (
        "this program is used to get split_log, split_bed, agp4 from agp9 of yahs"
    )
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=f"author:\t{__author__}\ndate:\t{__date__}\nversion:\t{__version__}\nfunction:\t{function}",
    )
    parser.add_argument("-i", help="agp9 of yahs", type=str, required=True)
    parser.add_argument("-o", help="outdir", type=str, required=True, default = './')
    parser.add_argument(
        "-s", help="genome size file, two line", type=str, required=True
    )
    parser.add_argument(
        "-c",
        help="filter contigs less than this, default = 200000",
        type=int,
        required=False,
        default=200000,
    )
    parser.add_argument(
        "-a",
        help="remain scaffold longer than this, default = 1000000",
        type=int,
        required=False,
        default=1000000,
    )
    args = parser.parse_args()

    splitlog = os.path.join(args.o, "Genome_split.log")
    splitbed = os.path.join(args.o, "Genome_split.contig.bed")
    agp4 = os.path.join(args.o, "chr.order.agp")

    agp = Agp(args.s, args.i, agp4, splitlog, splitbed, args.c, args.a)
    agp.run()


if __name__ == "__main__":
    main()

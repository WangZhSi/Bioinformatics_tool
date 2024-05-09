#!/annoroad/data1/software/install/Miniconda/Anaconda3-2021.05/envs/python3.7/bin/python3
##### import #####
import re
import argparse
#### info ####
__author__ = 'wangzhsi'
__mail__ = 'wang_zsi@outlook.com'
__date__ = '20240104'
__version__ = '1.0'
#### main ####


class Type:
    gene = 'gene'
    mrna = 'mrna'
    cds = 'cds'
    lnc_rna = 'lnc_rna'
    exon = 'exon'
    region = 'region'
    pseudogene = 'pseudogene'
    transcript = 'transcript'
    idPattern = 'ID=([^;]+);'
    parentPattern = 'Parent=([^;]+)'


def safeLower(s):
    if isinstance(s, str):
        return s.lower()
    return s


def getIdList(infile) -> list:
    if not infile:
        return None
    ids = set()
    with open(infile, 'r') as f:
        for line in f:
            if not line:  # 如果遇到空行，则跳过此行
                continue
            id_ = line.strip().split('\t', 1)[0]  # 使用了分割时指定的列数，避免了不必要的数组创建
            ids.add(id_)  # 直接添加到集合中，可以避免后续的set转换
    return list(ids)  # 由于函数的返回类型为列表，因此最后需要转换为列表后返回


def _extract_parent(id, node_type, splited_gff8):
    if node_type == Type.gene:
        return id
    else:
        return re.findall(Type.parentPattern, splited_gff8)


class GffNode:

    def setNode(self, node: str):
        self.node = node
        self._info = []
        self._summary = {'gene': '', 'mrna': []}
        return node

    def parserNode(self, node):
        """解析节点字符串并提取相关信息"""
        node_lines = node.split('\n')
        for line in node_lines:
            stripped_line = line.strip()
            if not stripped_line:
                continue
            tab_separated_values = line.split('\t')
            id = re.findall(Type.idPattern, tab_separated_values[8])[0]
            node_type = safeLower(tab_separated_values[2])

            parent = _extract_parent(id, node_type, tab_separated_values[8])

            if node_type == Type.gene:
                self._summary['gene'] = id
                # parent = list(parent)        类型为gene时, 传入的id为字符串格式, 不是列表格式
            elif node_type == Type.mrna:
                self._summary['mrna'].append(id)
            else:
                node_type == 'level3'  # level3节点的处理

            self._info.append({
                'parent': parent,
                'type': node_type,
                'id': id,
                'line': line + '\n'
            })

    def get(self, node):
        if not self._info:
            self.parserNode(node)
        summary = self._summary
        info = self._info
        return summary, info


class Gff:

    def __init__(self, ingff, outgff, targetlist, targettype, functype, keeplist='') -> None:
        self.ingff = open(ingff, 'r')
        self.outgff = open(outgff, 'w')
        self.targetlist = getIdList(targetlist)
        self.functype = functype
        self.targettype = targettype
        self.keeplist = getIdList(keeplist)
        self.gffnode = GffNode()
        pass

    def parse(self):    # 依据第三列gene将gff文件分割处理
        node = ''
        for line in self.ingff:
            if not line or '#' in line:
                continue
            if '\t{0}\t'.format(Type.gene) in line and node:
                yield self.gffnode.setNode(node)
                node = ''
            node = node + line
        yield self.gffnode.setNode(node)

    def search(self):
        result = ''
        for node in self.parse():
            if not node:
                continue
            summary, info = self.gffnode.get(node)
            if self.targettype == 'gene':
                if checkGene(summary, self.targetlist, self.functype, self.keeplist):
                    result = result + node
            if self.targettype == 'mrna':
                result = result + \
                    checkmRNA(node, summary, info, self.targetlist,
                              self.functype, self.keeplist)
        self.outgff.write(result)


def checkGene(summary: dict, target: list, functype: str, keeplist) -> bool:
    if keeplist:
        if summary['gene'] in target or any(x in keeplist for x in summary['mrna']):
            return True
    elif functype == 'fish':
        return summary['gene'] in target
    elif functype == 'del':
        if summary['gene'] not in target:
            return True
    else:
        return False


def mergeTarget(target: list, keeplist, functype: str) -> list:
    if functype == 'fish':
        updated_list = list(set(target) | set(keeplist))
    elif functype == 'del':
        updated_list = list(set(target) - set(keeplist))
    else:
        # 由于传参时加了choice, 实际不会触发
        raise ValueError(
            "Invalid operation type. Supported types are 'fish' and 'del'")

    return updated_list


def checkmRNA(node, summary: dict, info: list, target: list, functype: str, keeplist) -> list:
    if keeplist:
        newTarget = set(mergeTarget(target, keeplist, functype))
    else:
        newTarget = target

    # 取交集mRNA id
    common = set(newTarget) & set(summary['mrna'])

    # 如果没有交集
    if not common:
        if functype == 'fish':
            return ''
        else:
            return node

    # 如果交集等于全部mRNA id
    elif common == set(summary['mrna']):
        if functype == 'fish':
            return node
        else:
            return ''

    # 有一部分交集，基因包含可变剪切且需要筛选
    elif functype == 'fish':
        return _checkInfoFish(info, list(common))
    else:
        return _checkInfoDel(info, list(common))


def _checkInfoFish(info, common: list):
    out = ''
    for i in info:
        if i['type'] == Type.gene:
            out += i['line']
        elif i['id'] in common or i['parent'][0] in common:
            out += i['line']
    return out


def _checkInfoDel(info, common: list):
    out = ''
    for i in info:
        if i['type'] == Type.gene:
            out += i['line']
        elif i['id'] in common or i['parent'][0] in common:
            continue
        else:
            out += i['line']
    return out


def main():
    function = 'this program is used to fish target gff by list'
    author, mail, date, version = __author__, __mail__, __date__, __version__

    parser = argparse.ArgumentParser(
        description=function,
        epilog=f'author: {author}\nmail: {mail}\ndate: {date}\nversion: {version}\nfunction: {function}',
        formatter_class=argparse.RawDescriptionHelpFormatter
    )

    # 添加参数
    parser.add_argument('-ig', help='input gff file', type=str, required=True)
    parser.add_argument('-og', help='output gff file', type=str, required=True)
    parser.add_argument(
        '-l', help='target list file, single line', type=str, required=True)
    parser.add_argument('-t', help='target list id type, mrna or gene',
                        type=str, required=True, choices=['mrna', 'gene'])
    parser.add_argument('-f', help='function select, del or fish',
                        type=str, required=False, default='fish', choices=['del', 'fish'])
    parser.add_argument('-k', help='a list of tr id, will keep parent gene',
                        default='', type=str, required=False)

    # 解析命令行参数
    args = parser.parse_args()

    # 创建 Gff 实例
    gff = Gff(args.ig, args.og, args.l, args.t, args.f, args.k)

    # 执行 search
    gff.search()


if __name__ == '__main__':
    main()

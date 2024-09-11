获取多个表的交集内容

使用方法：perl this.pl -i *.lst ... -o output.txt [--header] [--help]
参数说明：
    -i, --input    输入文件，支持通配符（必须）。
    -o, --output   输出文件路径（必须）。
    --header       文件是否包含表头，默认无表头。
    -h, --help     打印此帮助信息。
示例：
    perl this.pl -i *.lst -o result.txt --header
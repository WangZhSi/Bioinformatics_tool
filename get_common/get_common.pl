#!/usr/bin/perl
use strict;
use warnings;
use Getopt::Long;

# 打印帮助信息
sub print_usage {
    print << "END_USAGE";
Usage: perl this.pl -i '*.lst' ... -o output.txt [--header] [--help]
Parameters:
    -i, --input    输入文件，支持通配符（必须）。
    -o, --output   输出文件路径（必须）。
    --header       文件是否包含表头，默认无表头。
    -h, --help     打印此帮助信息。
Example:
    perl this.pl -i "*.lst" -o result.txt --header
END_USAGE
    exit;
}

# 处理命令行参数
my @input_patterns;
my $output_file;
my $header = 0;   # 默认为没有表头
my $help = 0;     # 帮助选项

GetOptions(
    'i=s{1,}' => \@input_patterns, # 接收一个或多个输入文件模式
    'o=s'     => \$output_file,     # 输出文件路径
    'header!' => \$header,          # 是否有表头，默认为否
    'h|help'  => \$help             # 帮助选项
) or print_usage();


if ($help || !@input_patterns || !$output_file) {
    print_usage();
}

# 通过glob解析通配符，生成文件列表
my @input_files;
for my $pattern (@input_patterns) {
    push @input_files, glob($pattern);
}

# 检查输入文件是否提供
if (!@input_files) {
    die "请提供至少一个输入文件，使用 -i 参数并检查通配符是否匹配到文件。\n";
}

# 检查输出文件是否提供
if (!$output_file) {
    die "请提供输出文件路径，使用 -o 参数。\n";
}

# 按文件大小排序，从小文件开始处理
@input_files = sort { -s $a <=> -s $b } @input_files;


my %common;
open(my $fh, '<', $input_files[0]) or die "无法打开文件 $input_files[0]: $!";

# 如果有表头，跳过第一行
<$fh> if $header;

while (<$fh>) {
    chomp;
    my @columns = split /\t/;
    $common{$columns[0]} = 1;  # 只取第一列
}
close($fh);

# 遍历其余文件，并逐行过滤
for my $file (@input_files[1..$#input_files]) {
    open(my $fh, '<', $file) or die "无法打开文件 $file: $!";
    
    # 如果有表头，跳过第一行
    <$fh> if $header;

    # 记录当前文件中出现的元素
    my %current;
    while (<$fh>) {
        chomp;
        my @columns = split /\t/;
        $current{$columns[0]} = 1;
    }
    close($fh);
    
    # 直接过滤交集哈希表中的项
    foreach my $key (keys %common) {
        delete $common{$key} unless exists $current{$key};
    }

    # 如果交集为空，提前退出
    last unless keys %common;
}

# 输出交集内容到指定文件
open(my $out_fh, '>', $output_file) or die "无法创建输出文件 $output_file: $!";
foreach my $key (keys %common) {
    print $out_fh "$key\n";
}
close($out_fh);

print "交集结果已写入文件：$output_file\n";

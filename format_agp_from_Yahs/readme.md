基于 [yahs](https://github.com/c-zhou/yahs) 流程输出文件 (9 列 agp), 生成 4 列 agp 文件；若 contig 发生切割，则生成切割日志 (log) 及 bed 文件；其中 bed 文件可以接入 seqkit 切割 fa 文件；

**接口说明：**
1. -i, 输入文件，yahs 输出的 9 列 agp 文件；
2. -o, 输出目录，在目录中输出文件，程序不包含创建目录功能；默认为当前目录；
3. -s, contig size 文件，两列 tsv, 第一列为 contig id, 第二列为 contig 大小；
4. -c, 保留的最小 contig 长度，默认 200000bp;
5. -a, 保留的最小 scaffold 长度，默认 1000000bp;

**逻辑说明：**
1. 默认多条 contig 构成一个 chromosome(supper scaffold);
2. 若单条 contig 构成一个 chromosome(supper scaffold), 则比较此 contig 大小与-a 参数大小；即，仅认为大于指定值的 contig 可以单条构成 chromosome;
3. 若单条 contig 小于指定值 (-c 参数), 认为此 contig 过短，干扰组装效果，去除；
4. 在以上筛选条件下，会存在 \[大于最小 contig 长度\] 且 \[不能单条构成 chromosome\] 的 contig, 统一收归到 chr0 中；认为这部分 conig 可能存在挂载信号，但软件直出的挂载结果可能有问题，暂时保留；
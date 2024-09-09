解析 ENA 数据库 json 文件，并生成下载命令：
1. 默认下载 fq 数据，如需下载其他如 bam 数据需修改脚本
2. 默认读取 ftp 地址，若勾选 ascp 地址，可在-m 参数选择 ascp
3. 使用 ascp 下载时，需在脚本中配置软件路径和 key 路径
4. 下载指令 print 到标准输出，可重定向到文件

参数说明：
1. -i: ena json 文件路径
2. -m: 下载模式，默认 ftp, 可选 ftp/ascp
3. -o: 下载文件输出路径，默认当前路径 (./)
4. -f: 过滤条件，筛选 json 中符合条件的条目，下详

数据过滤说明：
1. 依据单个键过滤：
    python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f sample_accession=SAMN12345678 ; 
2. 依据多个键过滤：
        python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f sample_accession=SAMN12345678 experiment_accession=ERX12345678 ; 
3. 若值中存在空格：
        python3 parse_ena_json.py -i metadata.json -m ascp -o output_dir -f scientific_name="Bos indicus"
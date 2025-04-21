Convert AGP files
```bash
options:
  -h, --help            show this help message and exit

Basic parameters:
  Basic convert settings

  -i INPUT, --input INPUT
                        Input AGP file
  -o OUTPUT, --output OUTPUT
                        Output AGP file
  -F {4,9}, --output_format {4,9}
                        Output AGP format (4 or 9)
  -s SIZE, --size SIZE  Contig size file (tab-separated: ctg size). Required when converting agp4 to agp9
  -g GAP_SIZE, --gap_size GAP_SIZE
                        Gap size for AGP4 to AGP9 conversion, default=100bp

Changing parameters:
  using for change/filter chrom id, reverse whole chroms Process order: select -> filter -> id2id -> reverse

  --select SELECT       Select chromosome contain string(comma-separated, e.g. 'chr,RagTag')
  --filter FILTER       Filter string (comma-separated, e.g. '_RagTag,_1.0,NX_')
  --id2id ID2ID         ID relation file (id new_id) to change chrom IDs
  --reverse REVERSE     Chromosomes to reverse (comma-separated, e.g. 'chr1,chr2')

Output order parameters:
  Reorder output file by rules

  --nature              Reorder output in nature order: 1,2,3...10
  --sizeOrder           Rename chromosome by size.
  --prefix PREFIX       Prefix of rename chromosome by size, default = 'chr'
```
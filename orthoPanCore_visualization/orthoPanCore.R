rm(list = ls())

library(optparse)

option_list <- list(
  make_option(c("-i", "--input"), type = "character", default = NULL,
              help = "Orthogroups.GeneCount.tsv file from orthofinder",
              dest = "input"),
  make_option(c("-o", "--outdir"), type = "character", default = NULL,
              help = "Output dir, will save ${outdir}/pan-core.pdf/.png, summary.txt",
              dest = "outdir"),
  make_option(c("-s", "--simulation"), type = "integer", default = 100L,
              help = "simulation number, default is 100",
              dest = "simulation"),
  make_option(c("--core"), type = "double", default = 1.0,
              help = "Core genome threshold (e.g. 1.0 for 100% samples)"),
  make_option(c("--softcore"), type = "double", default = 0.9,
              help = "Softcore genome threshold (e.g. 0.9 for 90% samples)"), 
  make_option(c("--private"), type = "double", default = 0.01,
              help = "Private genome threshold (minimum 1 sample)")
)

options(error=traceback)

parser <- OptionParser(usage = "%prog -i Orthogroups.GeneCount.tsv -o out.dir", option_list=option_list)
opt = parse_args(parser)

if (is.null(opt$input) || is.null(opt$outdir)) {
  print_help(parser)
  quit(status = 1, save = "no")
}

library(ggplot2)
library(tidyr)
library(dplyr)
library(tidyverse)
library(ggsci)

plot_format = theme(
  plot.background = element_blank(),
  panel.grid =  element_blank(),
  panel.border = element_rect(
    colour = "black",
    linewidth = 0.5,
    fill = NA
  ),
  axis.line = element_blank(),
  axis.text = element_text(
    color = "black",
    size = 12
  ),
  axis.ticks = element_line(
    color = "black",
    size = 0.5
  ),
  axis.title = element_text(
    color = "black",
    size = 12
  ),
  plot.title = element_text(
    color = "black",
    size = 12
  ),
  legend.background = element_blank(),
  legend.key = element_blank(),
  legend.title = element_text(
    color="black",size=12
  )
)

data <- read.delim(opt$input) %>%
  select(-Total) %>% 
  column_to_rownames("Orthogroup")

binary_dat <- as.matrix(data > 0)
n_samples <- ncol(binary_dat)
n_genes <- nrow(binary_dat)
sim <- opt$simulation

results <- vector("list", sim)

for (i in 1:sim) {
  if (i %% 10 == 0) message("Simulation ", i, "/", sim)
  
  col_order <- sample(n_samples)
  cumulative <- integer(n_genes)
  
  # Current result
  sim_result <- tibble(
    samples = as.character(i),
    sampleNum = 1:n_samples,
    Pan = integer(n_samples),
    Core = integer(n_samples)
  )
  
  for (j in 1:n_samples) {
    cumulative <- cumulative + binary_dat[, col_order[j]]
    
    # Calculate pan/core
    sim_result$Core[j] <- sum(cumulative == j)
    sim_result$Pan[j] <- sum(cumulative > 0)
  }
  
  results[[i]] <- sim_result
}

df <- bind_rows(results)

df %>% 
  select(-samples) %>% 
  pivot_longer(!sampleNum) %>% 
  mutate(sampleNum=factor(sampleNum,levels = 1:n_samples)) -> longer.df

# Plot pan/core gene families
p1 <- ggplot(data = longer.df, aes(x = sampleNum, y = value)) +
  geom_boxplot(
    aes(fill = name, color = name),
    outlier.shape = NA,
    width = 0.8,
    alpha = 0.6,
    linewidth = 0.3,
    fatten = 1.2
  ) +
  # pan-core color here
  scale_fill_manual(
    values = c("Core" = "#d4101aCC",
               "Pan" = "#3082b3CC"),
    labels = c("Core Genome", "Pan Genome")
  ) +
  scale_color_manual(
    values = c("Core" = "#8b0000",
               "Pan" = "#1a4f6b"),
    guide = "none"
  ) +
  
  theme_bw() +
  theme(
    panel.grid = element_blank(),
    legend.position = c(0.8, 0.5),
    legend.title = element_blank(),
    legend.background = element_blank(),  
    legend.key = element_blank()  
  ) +
  labs(x = "Sample number", y = "Family number") +
  scale_x_discrete(
    breaks = unique(c(1, seq(10, n_samples, by = 10), n_samples))
  ) +
  plot_format

ggsave(paste(opt$outdir, "PanCoreGeneFamilies.pdf", sep = "/"), p1, width = 8, height = 6)
ggsave(paste(opt$outdir, "PanCoreGeneFamilies.png", sep = "/"), p1, width = 8, height = 6, dpi = 600)

# Calculate gene family number
core_threshold <- round(n_samples * opt$core)
softcore_threshold <- round(n_samples * opt$softcore)
private_threshold <- pmax(round(n_samples * opt$private), 1)  # ensure 1 sample at least

freq.df <- binary_dat %>%
  rowSums() %>%
  as.data.frame() %>%
  rownames_to_column("Orthogroup") %>%
  rename(Total = ".") %>%
  mutate(
    Class = case_when(
      Total == core_threshold ~ "Core",
      Total >= softcore_threshold & Total < core_threshold ~ "SoftCore",
      Total > private_threshold & Total < softcore_threshold ~ "Dispensable",
      Total == private_threshold ~ "Private",
      TRUE ~ NA_character_
    ) %>% factor(levels = c("Core", "SoftCore", "Dispensable", "Private"))
  )

write.table(freq.df, file = paste(opt$outdir, "GeneFamiliesClassfication.txt", sep = '/'), quote = FALSE, row.names = FALSE, sep = '\t')

class_summary <- freq.df %>% 
  group_by(Class) %>% 
  summarise(
    Gene_Count = n(),  
    .groups = "drop"
  ) %>% 
  mutate(
    Sample_Threshold = case_when(
      Class == "Core" ~ str_glue("== {core_threshold}"),
      Class == "SoftCore" ~ str_glue(">= {softcore_threshold}"),
      Class == "Dispensable" ~ str_glue("{private_threshold + 1}-{softcore_threshold - 1}"),
      Class == "Private" ~ str_glue("== {private_threshold}"),
      TRUE ~ "Unclassified"
    ),
    Percentage = sprintf("%.2f%%", 100 * Gene_Count / sum(Gene_Count))  
  ) %>% 
  select(Class, Sample_Threshold, Gene_Count, Percentage)

write.table(class_summary, file = paste(opt$outdir, "ClassficationSummary.txt", sep = '/'), quote = FALSE, row.names = FALSE, sep = '\t')

# Plot gene family number
p2<-ggplot(data=freq.df,aes(x=Total))+
  geom_bar(aes(fill=Class),width=0.5)+
  theme_bw()+
  theme(panel.grid = element_blank())+
  scale_y_continuous(expand = expansion(mult = c(0,0.1)))+
  labs(x="Frequeny",y="Family number")+
  scale_fill_npg()+    # bar plot color here
  scale_x_continuous(
    breaks = unique(c(1, seq(10, n_samples, by = 10), n_samples))
  ) +
  plot_format

ggsave(paste(opt$outdir, "GeneFamiliesFrequency.pdf", sep = "/"), p2, width = 8, height = 6)
ggsave(paste(opt$outdir, "GeneFamiliesFrequency.png", sep = "/"), p2, width = 8, height = 6, dpi = 600)

# Plot classfication donut
class <- class_summary %>%
  mutate(Percentage = as.numeric(gsub("%", "", Percentage))) %>%
  arrange(desc(Class)) %>%
  mutate(ymax = cumsum(Percentage), 
         ymin = lag(ymax, default = 0), 
         label_pos = (ymax + ymin) / 2, 
         label_text = paste0(Percentage, "%")) 

p3 <- ggplot(class, aes(ymax = ymax, ymin = ymin, xmax = 4, xmin = 2, fill = Class)) +
  geom_rect() + 
  geom_text(aes(x = 3, y = label_pos, label = label_text), size = 5, color = "white") + 
  coord_polar(theta = "y") + 
  xlim(c(0.1, 4)) + 
  theme_void() +
  scale_fill_npg() + 
  labs(title = "Gene Family Distribution", fill = "Class") +
  theme(legend.position = "right")

ggsave(paste(opt$outdir, "ClassficationDonut.pdf", sep = "/"), p3, width = 8, height = 6)
ggsave(paste(opt$outdir, "ClassficationDonut.png", sep = "/"), p3, width = 8, height = 6, dpi = 600)

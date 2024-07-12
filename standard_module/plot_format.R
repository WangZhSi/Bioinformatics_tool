# ggplot2 图片格式化
# 参考: https://mp.weixin.qq.com/s/EhbWn-q7WPVFjBv99LuNiw

plot_format = theme(
  plot.background = element_blank(),
  panel.grid =  element_blank(),
  panel.border = element_rect(
    colour = "black",
    linewidth = 0.5,
    fill = NA
  ),
  axis.line = element_blank(),
  axis.ticks = element_line(
    color = "black",
    size = 0.5
  ),
  plot.title = element_text(
    color = "black",
    size = 7
  ),
  legend.background = element_blank(),
  legend.key = element_blank(),
  legend.text = element_text(
    color = "black",
    size = 7
  ),
  legend.title = element_text(
    color="black",size=7
  )
)

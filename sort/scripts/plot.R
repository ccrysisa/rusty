# read data
t <- read.table('values.dat', header=TRUE)

# import ggplot2
library(ggplot2)

# to plot comparisons
plot_comparisons <- ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + scale_y_log10() + scale_x_log10() + labs(title = "Comparisons")

# to plot runtime
plot_runtime <- ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + scale_y_log10() + scale_x_log10() + labs(title = "Runtime")

ggsave("images/comparisons.png", plot = plot_comparisons, device = "png")
ggsave("images/runtime.png", plot = plot_runtime, device = "png")
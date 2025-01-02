pkg_path <- "./r-phylo2vec"

fn = devtools::build(pkg_path, binary = TRUE, args = c('--preclean'))

devtools::install_local(fn, force = TRUE)

if (file.exists(fn)) {
  file.remove(fn)
}

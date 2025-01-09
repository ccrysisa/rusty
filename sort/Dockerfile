FROM r-base:latest
COPY ./scripts /usr/local/src/myscripts
COPY ./values.dat /usr/local/src/myscripts
WORKDIR /usr/local/src/myscripts
RUN mkdir images
RUN Rscript install_packages.R
RUN Rscript plot.R
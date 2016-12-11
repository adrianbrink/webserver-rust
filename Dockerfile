FROM adrianbrink/rust:stable

EXPOSE 6767
VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]
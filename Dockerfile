FROM ubuntu:latest
RUN apt update
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN git clone git@github.com:MichaelThoresen/MrRustboto.git



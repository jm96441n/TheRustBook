FROM ubuntu:18.04

LABEL maintainer="john@johnmaguiredeveloper.com"

ENV REFRESHED_AT 2020_07_03

RUN mkdir -p /RustBook
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

FROM scratch

ADD accounting /
ADD config.yml /

CMD ["/accounting", "bot"]

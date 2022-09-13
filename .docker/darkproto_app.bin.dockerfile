FROM public.ecr.aws/amazonlinux/amazonlinux:2 as enclave_app
WORKDIR /app
COPY darkproto-app /app

ENV RUST_LOG="darkproto=debug"
CMD ["/app/darkproto-app"]

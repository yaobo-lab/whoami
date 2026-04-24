# docker build -t whoami:1.0.4 .

FROM crpi-p35inr9lrgirg3t0.cn-hangzhou.personal.cr.aliyuncs.com/yaobo-box/rust-alpine:1.1 AS builder
WORKDIR /app
COPY . ./
RUN cargo build --release


FROM crpi-p35inr9lrgirg3t0.cn-hangzhou.personal.cr.aliyuncs.com/yaobo-box/alpine:3.22.1

COPY --from=builder /app/target/release/app /bin/app

WORKDIR /app
CMD ["/bin/app"]
FROM gitea.familleboyer.net/traxys/viz-build:1.68 as builder

COPY . .
RUN cargo xtask build --release

FROM caddy:2.5.2

COPY --from=builder target/release/html/ /usr/share/caddy

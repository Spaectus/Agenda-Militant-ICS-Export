####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=agenda_militant_ics_export
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /agenda_militant_ics_export

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /agenda_militant_ics_export

# Copy our build
COPY --from=builder /agenda_militant_ics_export/target/release/agenda_militant_ics_export ./

# Use an unprivileged user.
USER agenda_militant_ics_export:agenda_militant_ics_export

CMD ["/agenda_militant_ics_export/agenda_militant_ics_export"]
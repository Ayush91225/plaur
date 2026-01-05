FROM public.ecr.aws/lambda/provided:al2-x86_64

# Install Rust
RUN yum install -y gcc
COPY --from=rust:1.75 /usr/local/cargo /usr/local/cargo
COPY --from=rust:1.75 /usr/local/rustup /usr/local/rustup
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Set working directory
WORKDIR /var/task

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --bin plaur-auth

# Copy the binary to the Lambda runtime directory
RUN cp target/release/plaur-auth ${LAMBDA_RUNTIME_DIR}/bootstrap

# Set executable permissions
RUN chmod +x ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD ["bootstrap"]
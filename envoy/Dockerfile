FROM envoyproxy/envoy-alpine:v1.18.3
COPY envoy.yaml /etc/envoy
RUN chmod go+r /etc/envoy/envoy.yaml
CMD ["/usr/local/bin/envoy", "-c", "/etc/envoy/envoy.yaml", "--service-cluster", "backend-proxy"]
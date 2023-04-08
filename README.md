# empty-backend
empty backend


protoc -I./proto/proto \
  -I./proto/third_party --include_imports \
  --include_source_info \
  --descriptor_set_out=./proto/proto/blog/blog.pb \
  --openapiv2_out=../empty-frontend/src/api/openapi \
  ./proto/proto/blog/blog.proto

sudo docker run -it --rm --network="host" \
  -v "$(pwd)/proto/proto:/proto:ro" \
  -v "$(pwd)/envoy.yaml:/etc/envoy/envoy.yaml:ro" \
  envoyproxy/envoy:v1.25-latest
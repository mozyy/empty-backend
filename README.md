# empty-backend
empty backend


protoc -I./proto/proto \
  -I./proto/third_party --include_imports \
  --include_source_info \
  --descriptor_set_out=./proto/proto/blog/blog.pb \
  --openapiv2_out=../empty-frontend/src/openapi/docs \
  ./proto/proto/blog/blog.proto

sudo docker run --name envoy -it --rm --network="host" \
  -v "$(pwd)/proto/proto:/proto:ro" \
  -v "$(pwd)/envoy.yaml:/etc/envoy/envoy.yaml:ro" \
  envoyproxy/envoy:v1.25-latest

docker run --name envoy -it --rm \
  -v "vsc-remote-containers-empty:/workspaces:ro" \
  envoyproxy/envoy:v1.25-latest -c /workspaces/empty-backend/envoy.yaml


curl -L -o /usr/local/bin/protoc-gen-openapiv2 https://github.com/grpc-ecosystem/grpc-gateway/releases/download/v2.15.2/protoc-gen-openapiv2-v2.15.2-linux-x86_64
chmod +x /usr/local/bin/protoc-gen-openapiv2
# empty-backend
empty backend


## openapi
https://github.com/grpc-ecosystem/grpc-gateway/blob/main/examples/internal/proto/examplepb/a_bit_of_everything.proto#L19C99


protoc -I./proto/proto \
  -I./proto/third_party --include_imports \
  --include_source_info \
  --descriptor_set_out=./proto/proto/blog/blog.pb \
  --openapiv2_out=../empty-frontend/src/openapi/docs \
  ./proto/proto/blog/blog.proto \
  ./proto/proto/lottery/lottery.proto


protoc \
  -I./lottery/proto \
  -I./proto/third_party \
  --include_imports \
  --include_source_info \
  --descriptor_set_out=./target/lottery.pb \
  --openapiv2_out=../empty-frontend/src/openapi/docs/lottery/ \
  --openapiv2_out=../lottery-tool/src/openapi/docs/lottery/ \
  ./proto/proto/lottery/lottery.proto \
  ./proto/proto/lottery/record.proto \
  ./proto/proto/oauth/oauth.proto \
  ./proto/proto/wx/wx.proto
  ./proto/proto/wx/user.proto \
  ./proto/proto/blog/blog.proto \

docker run --name envoy -it --rm \
  -v "vsc-remote-containers-empty:/workspaces:ro" \
  -p 51051:51051 \
  envoyproxy/envoy:v1.25-latest -c /workspaces/empty-backend/envoy.yaml


curl -L -o /usr/local/bin/protoc-gen-openapiv2 https://github.com/grpc-ecosystem/grpc-gateway/releases/download/v2.16.0/protoc-gen-openapiv2-v2.16.0-linux-x86_64
chmod +x /usr/local/bin/protoc-gen-openapiv2


 PROTOC_VERSION=23.2 && \
 PROTOC_ZIP=protoc-${PROTOC_VERSION}-linux-x86_64.zip && \

sudo curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${PROTOC_ZIP} \
    && sudo unzip $PROTOC_ZIP -d /home/vscode/.local \
    && sudo rm -f $PROTOC_ZIP


protoc -I./proto/proto \
  -I../proto/third_party \
  --openapiv2_out=./proto \
  ./proto/proto/record.proto
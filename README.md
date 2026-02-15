<img width="2546" height="1611" alt="image" src="https://github.com/user-attachments/assets/2987aecf-a682-4049-a22b-e3f5de3e148f" />


This project leverages a Rust backend with Axum for high-performance WebSocket and gRPC communication. The data layer is powered by Protobuf for efficient binary serialization. 

## envs

|Variable|Description|
|--------|-----------|
|DEVICE_ADDRESS|Device mac address. Turn band into heart rate broadcast mode getting this.|
|MODIFY_UUID|Heart Rate Measurement Service. Generally `00002a37-0000-1000-8000-00805f9b34fb`. [reference](https://bitbucket.org/bluetooth-SIG/public/src/59af7d1e972a17acbe4a210af158a2740b8a70e8/assigned_numbers/uuids/characteristic_uuids.yaml#lines-167)|
|WRITE_UUID|Useless since not do write in this case. Generally `0xfe07`.|

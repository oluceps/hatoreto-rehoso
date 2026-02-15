<img width="2546" height="1611" alt="image" src="https://github.com/user-attachments/assets/2987aecf-a682-4049-a22b-e3f5de3e148f" />


## envs

|Variable|Description|
|--------|-----------|
|DEVICE_ADDRESS|Device mac address. Turn band into heart rate broadcast mode getting this.|
|MODIFY_UUID|Heart Rate Measurement Service. Generally `00002a37-0000-1000-8000-00805f9b34fb`. [reference](https://bitbucket.org/bluetooth-SIG/public/src/59af7d1e972a17acbe4a210af158a2740b8a70e8/assigned_numbers/uuids/characteristic_uuids.yaml#lines-167)|
|WRITE_UUID|Useless since not do write in this case. Generally `0xfe07`.|

## build

```console
  nix build
```

## run

1. edit `.env` and fill `DEVICE_ADDRESS=<MAC>`
2. `python main.py`
3. `cd hosoer && bun run dev`

## web

```
  cd hosoer
  bun run dev
```


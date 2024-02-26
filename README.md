## envs

DEVICE_ADDRESS:
>   Device mac address. Turn band into heart rate broadcast mode getting this.

MODIFY_UUID:
>  Heart Rate Measurement Service. Generally `00002a37-0000-1000-8000-00805f9b34fb`. [reference](https://bitbucket.org/bluetooth-SIG/public/src/59af7d1e972a17acbe4a210af158a2740b8a70e8/assigned_numbers/uuids/characteristic_uuids.yaml#lines-167)

WRITE_UUID:
>  Useless in this repo, preserve for fun. Generally `0xfe07`.

## build

```console
  nix build
```

## run

1. edit `.env` and fill `DEVICE_ADDRESS=<MAC>`
2. `python main.py`
3. `cd hosoer && bun run dev`

## API

```
  HTTP GET -> url
  response: { "rate": number }
```

## web

```
  cd hosoer
  bun run dev
```

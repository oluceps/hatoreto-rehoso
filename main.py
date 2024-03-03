import asyncio
from bleak import BleakClient, BleakScanner
import logging
import os
import websockets
import json

global rate
rate=0

def load_env():
    global DEVICE_ADDRESS,NOTIFY_UUID,WRITE_UUID
    DEVICE_ADDRESS = os.getenv('DEVICE_ADDRESS') # required
    NOTIFY_UUID = os.getenv('NOTIFY_UUID', default="00002a37-0000-1000-8000-00805f9b34fb")
    WRITE_UUID = os.getenv('WRITE_UUID', default="0000fe01-0000-1000-8000-00805f9b34fb")

async def main():
    logging.info("Start Scan")

    load_env()

    device = await BleakScanner.find_device_by_address(DEVICE_ADDRESS)

    if device is None:
      logging.error("'%s' Not Found", DEVICE_ADDRESS)
      return

    logging.info("Connecting...")

    async with BleakClient(device) as client:
        logging.info("Connected")

        def ntfy_handler(sender, data):
            global rate
            rate = int(data[1])
            print(f"\rHeart rate [ {rate} ]", end='\n')

        await client.start_notify(NOTIFY_UUID,  ntfy_handler)

        while True:
            await asyncio.sleep(10.0)

async def websocket_handler(websocket, path):
    while True:
        await websocket.send(json.dumps({'rate': rate}))
        await asyncio.sleep(1)  # send data every second

def start():
    start_server = websockets.serve(websocket_handler, '0.0.0.0', 7000)

    asyncio.get_event_loop().run_until_complete(asyncio.gather(start_server, main()))
    asyncio.get_event_loop().run_forever()

if __name__ == "__main__":
    start()

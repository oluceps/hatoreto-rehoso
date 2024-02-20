import asyncio
from bleak import BleakClient, BleakScanner
from bleak.backends.characteristic import BleakGATTCharacteristic
import logging
import os

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
            rate = int(data[1])
            print(f"\rHeart rate [ {rate} ]", end='')

        await client.start_notify(NOTIFY_UUID,  ntfy_handler)

        while True:
            await asyncio.sleep(10.0)

def start():
    asyncio.run(main())

if __name__ == "__main__":
    start()

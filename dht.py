import Adafruit_DHT
import json

DHT_SENSOR = Adafruit_DHT.DHT22
DHT_PIN_ONE = 2
DHT_PIN_TWO = 3

def get_atmosphere():
    humidity_one, temperature_one = Adafruit_DHT.read(DHT_SENSOR, DHT_PIN_ONE)
    if humidity_one is not None and temperature_one is not None:
        humidity_two, temperature_two = Adafruit_DHT.read(DHT_SENSOR, DHT_PIN_TWO)
        if temperature_two is not None and humidity_two is not None:
            out = {"t1": temperature_one, "h1": humidity_one, "t2": temperature_two, "h2": humidity_two}
            print(json.dumps(out))
        else:
            out = {"error": "Failed to retrieve data from SECOND sensor"}
            print(json.dumps(out))
    else:
        out = {"error": "Failed to retrieve data from FIRST sensor"}
        print(json.dumps(out))
        
def main():
    get_atmosphere()

if __name__ == "__main__":
    main()
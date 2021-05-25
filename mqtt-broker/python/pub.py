# import paho.mqtt.client as mqtt
# # from random import randrange, uniform
# import time

# mqttBroker = "mqtt.eclipseprojects.io"
# client = mqtt.Client("Temperature_Inside")  # device name
# client.connect(mqttBroker)

# while True:
#     # randNumber = uniform(20.0, 21.0)
#     client.publish("TEMPERATURE", 3)
#     print("Just published " + str(2) + " to Topic TEMPERATURE")
#     time.sleep(1)

import paho.mqtt.client as mqtt
# from random import randrange, uniform
import time

mqttBroker = "mqtt.eclipseprojects.io"
client = mqtt.Client("Temperature_Inside")  # device name
client.connect(mqttBroker)



while True:
    client.publish("TEMPRATURE", input('Message : '))
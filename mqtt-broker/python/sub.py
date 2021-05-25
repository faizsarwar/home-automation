# import paho.mqtt.client as mqtt
# import time

# def on_message(client, userdata, message):
#     print("Received message: ", str(message.payload.decode("utf-8")))

# mqttBroker = "mqtt.eclipseprojects.io"
# client = mqtt.Client("Smartphone")
# client.connect(mqttBroker)

# client.loop_forever()
# client.subscribe("TEMPERATURE")
# client.on_message = on_message
# time.sleep(30)

# second code 
import paho.mqtt.client as mqtt

mqttBroker = "mqtt.eclipseprojects.io"
client = mqtt.Client("Smartphone")
client.connect(mqttBroker)



def on_connect(client, userdata, flags, rc):
    print("Connected to a broker!")
    client.subscribe("TEMPRATURE")

def on_message(client, userdata, message):
    print(message.payload.decode())

while True:
    client.on_connect = on_connect
    client.on_message = on_message
    client.loop_forever()

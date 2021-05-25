#include <ESP8266WiFi.h>

#include <PubSubClient.h>

WiFiClient espClient;
PubSubClient client(espClient);

void reconnectmqttserver() {
while (!client.connected()) {
Serial.print("Attempting MQTT connection...");
String clientId = "ESP8266Client-";
 clientId += String(random(0xffff), HEX);
if (client.connect(clientId.c_str())) {
Serial.println("connected");
  client.subscribe("TOPIC");
} else {
Serial.print("failed, rc=");
Serial.print(client.state());
Serial.println(" try again in 5 seconds");
delay(5000);
}
}
}

void callback(char* topic, byte* payload, unsigned int length) {
  String MQTT_DATA = "";
  for (int i=0;i<length;i++) {
   MQTT_DATA += (char)payload[i];}
  Serial.println("You have a new data on this topic");
  Serial.print(":");
  Serial.println(topic);
  Serial.println(MQTT_DATA);
  Serial.println("");

}

void setup()
{
  Serial.begin(9600);
  WiFi.disconnect();
  delay(3000);
  Serial.println("START");
   WiFi.begin("Paradise","foreverconnected");
  while ((!(WiFi.status() == WL_CONNECTED))){
    delay(300);
    Serial.print("..");

  }
  Serial.println("Connected");
  Serial.println("Your IP is");
  Serial.println((WiFi.localIP().toString()));
  client.setServer("6.tcp.ngrok.io",15223);
  client.setCallback(callback);

}

// jb bhi message recieve huga callback function chalegaa

void loop()
{

    if (!client.connected()) {
    reconnectmqttserver();
    }
    client.loop();

}

#include <ESP8266WiFi.h>

#include <PubSubClient.h>

int  i;
IPAddress staticIP5_100(192,168,2,100);
IPAddress gateway5_100(192,168,2,1);
IPAddress subnet5_100(255,255,255,0);

WiFiClient espClient;
PubSubClient client(espClient);

void reconnectmqttserver() {
while (!client.connected()) {
Serial.print("Attempting MQTT connection...");
String clientId = "ESP8266Client-";
 clientId += String(random(0xffff), HEX);
if (client.connect(clientId.c_str())) {
Serial.println("connected");
} else {
Serial.print("failed, rc=");
Serial.print(client.state());
Serial.println(" try again in 5 seconds");
delay(5000);
}
}
}

char msgmqtt[50];
void callback(char* topic, byte* payload, unsigned int length) {
  String MQTT_DATA = "";
  for (int i=0;i<length;i++) {
   MQTT_DATA += (char)payload[i];}

}

void setup()
{
  i = 0;
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
  WiFi.config(staticIP5_100, gateway5_100, subnet5_100);
  Serial.println("Your IP is");
  Serial.println((WiFi.localIP().toString()));
  client.setServer("2.tcp.ngrok.io",13468);   // yhn hm server ki ip aur port dengay ngrok wali  (tcp url mai // say pehlay wala hata dena)
  client.setCallback(callback);

}


void loop()
{

    if (!client.connected()) {
    reconnectmqttserver();
    }
    client.loop();
    snprintf (msgmqtt, 50, "%d ",i);
    client.publish("TOPIC", msgmqtt);
    i = i + 1;
    delay(5000);

}

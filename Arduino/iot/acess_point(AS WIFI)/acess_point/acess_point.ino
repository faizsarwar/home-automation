#include <ESP8266WiFi.h>

void setup()
{
  Serial.begin(9600);
  WiFi.softAP("faiz","12345678");
  Serial.println((WiFi.softAPIP()));

}


void loop()
{

    Serial.println("connected users");
    Serial.println((WiFi.softAPgetStationNum()));
    delay(5000);

}

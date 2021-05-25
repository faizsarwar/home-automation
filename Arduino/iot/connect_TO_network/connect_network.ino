#include <ESP8266WiFi.h>

void setup()
{
  Serial.begin(9600);
  delay(1000);
  WiFi.disconnect();
   WiFi.begin("Paradise","foreverconnected");

}


void loop()
{
    while ((!(WiFi.status() == WL_CONNECTED))){  // If of connected
      delay(300);
      Serial.println(".....");
    }
    Serial.println("I AM Connected");

}

#include <ESP8266WiFi.h>

IPAddress staticIP61_105(192,168,2,105);
IPAddress gateway61_105(192,168,2,1);
IPAddress subnet61_105(255,255,255,0);

void setup()
{
  Serial.begin(9600);
  delay(1000);
   WiFi.begin("Paradise","foreverconnected");
  while ((!(WiFi.status() == WL_CONNECTED))){
    delay(300);

  }
  WiFi.config(staticIP61_105, gateway61_105, subnet61_105);
  WiFi.hostname("esp8266") ;
  delay(3000);
  Serial.println("connected");
  Serial.println("your ip is ");
  Serial.println((WiFi.localIP().toString()));
  Serial.println("your gateway is ");
  Serial.println((WiFi.gatewayIP().toString().c_str()));
  Serial.println("Your hostname is");
  Serial.println((WiFi.hostname()));

}


void loop()
{


}

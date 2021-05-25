#include <ESP8266WiFi.h>

String  Request;
WiFiServer server(80);

void setup()
{
  Request = "";

Serial.begin(9600);
pinMode(4, OUTPUT);
  delay(1000);
  Serial.println("start");
  WiFi.softAP("faiz","12345678");
  Serial.println("my Ip is :");
  Serial.println((WiFi.softAPIP()));
  server.begin();

}


void loop()
{

    WiFiClient client = server.available();
    if (!client) { return; }
    while(!client.available()){  delay(1); }
    Request = (client.readStringUntil('\r'));
    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println("");
    client.println("<!DOCTYPE HTML>");
    client.println("<html>");
    client.println((millis()/1000));
    client.println("</html>");

    delay(1);
    Request.remove(0, 5);
    Request.remove(Request.length()-9,9);
    if (Request == "on") {
      Serial.println("on");
      digitalWrite(4,HIGH);

    }
    if (Request == "OFF") {
      Serial.println("OFF");
      digitalWrite(4,LOW);

    }
    client.flush();

}

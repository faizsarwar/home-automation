#include <ESP8266WiFi.h>

String  client_request;
WiFiServer server(80);

void setup()
{
  client_request = "";

Serial.begin(9600);
  WiFi.disconnect();
  delay(1000);
   WiFi.begin("Paradise","foreverconnected");
  Serial.println("start");
  while ((!(WiFi.status() == WL_CONNECTED))){
    delay(1000);
    Serial.println(".......");

  }
  Serial.println("I AM connected My Ip is this");
  Serial.println((WiFi.localIP().toString()));
  server.begin();

}


void loop()
{
    WiFiClient client = server.available();
    if (!client) { return; }
    while(!client.available()){  delay(1); }
    client_request = (client.readStringUntil('\r'));
    client_request.remove(0, 5);
    client_request.remove(client_request.length()-9,9);
    Serial.println(client_request);
    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println("");
    client.println("<!DOCTYPE HTML>");
    client.println("<html>");
    client.println("welcome");
    client.println("</html>");

    delay(1);

}

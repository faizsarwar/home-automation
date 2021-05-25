#include <ESP8266WiFi.h>

#include <Ticker.h>

String  client_request;
WiFiServer server(80);

Ticker secondTick;
#define debug 1
volatile int watchdogCount = 0;
void ISRwatchdog() {
watchdogCount++;
if ( watchdogCount == 5 ) {
   (debug) && Serial.println("The watch dog is running");
   ESP.reset();
  }
}


void reconnect(){
    delay(1000);
   WiFi.begin("Paradise","foreverconnected");
   Serial.println("RECONNECTING");
   while ((!(WiFi.status() == WL_CONNECTED))){
    delay(1000);
    Serial.println(".......");
   }
  }


void setup()
{
  client_request = "";

Serial.begin(9600);
secondTick.attach(1, ISRwatchdog);

pinMode(4, OUTPUT);
  WiFi.disconnect();
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
    if ((!(WiFi.status() == WL_CONNECTED))){
    while((!(WiFi.status() == WL_CONNECTED))){
      Serial.println("Connectio dropped");
      reconnect();
      }
    }
    else{
    watchdogCount = 0;
    WiFiClient client = server.available();
    if (!client) { return; }
    while(!client.available()){  delay(1); }
    client_request = (client.readStringUntil('\r'));
    client_request.remove(0, 5);
    client_request.remove(client_request.length()-9,9);
    Serial.println(client_request);
    if (client_request == "on") {
      digitalWrite(4,LOW);
    Serial.println("on hugya");
      

    }
    if (client_request == "off") {
      digitalWrite(4,HIGH);
      Serial.println("off hugya");

    }
    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println("");
    client.println("<!DOCTYPE HTML>");
    client.println("<html>");
    client.println("welcome");
    client.println("</html>");

    delay(1);
    }
}

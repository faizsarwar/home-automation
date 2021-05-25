// connect D0 with RST pin to use deep sleep but not while uploading
void setup()
{
  pinMode(4, OUTPUT);

}


void loop()
{

  for (int count = 0; count < 3; count++) {
    digitalWrite(4,HIGH);
    delay(500);
    digitalWrite(4,LOW);
    delay(500);
  }
  ESP.deepSleep(10e6);

}

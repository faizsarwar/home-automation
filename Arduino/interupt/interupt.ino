void interrupt_2(){
  delay(1000);
  Serial.println("inerupt is added");
}

void setup()
{
  Serial.begin(9600);
attachInterrupt(5,interrupt_2,CHANGE);

}


void loop()
{

}



String  Mystringvariable;
void setup()
{
  Mystringvariable = "";

Serial.begin(9600);
  delay(3000);
  Mystringvariable = "welcome";
  Serial.println("the content of my variable ");
  Serial.println(Mystringvariable);
  Serial.println("char at position 0");
  Serial.println(Mystringvariable.charAt(0));
  Serial.println("char at position 1");
  Serial.println(Mystringvariable.charAt(1));

}


void loop()
{


}

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
  Serial.println(Mystringvariable.charAt(1));     //gives the char at index 
  Serial.println("position of c character is");
  Serial.println(Mystringvariable.indexOf('c'));  //gives the index of string
  if (Mystringvariable.compareTo("hello") == 0) {   //compareTo fuction returns 0 if matched
    Serial.println("these two strings are same");
  } else {
    Serial.println("these two strings are different");
  }
}


void loop()
{

}

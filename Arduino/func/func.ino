void setup() {
  // put your setup code here, to run once:
pinMode(16,OUTPUT);
//pin 2 built in led ki pin hai
}

void blink(){
  digitalWrite(16,HIGH);
  delay(1000);
  digitalWrite(16,LOW);
  delay(1000);
 
}

void loop() {
  // put your main code here, to run repeatedly:
blink();
}

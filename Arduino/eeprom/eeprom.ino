#include <EEPROM.h>

int i;
void setup()
{
  Serial.begin(9600);
  EEPROM.begin(512);
  Serial.println("start writing");
  EEPROM.write(0, 10);  //8 bits store krwasktay hain(255 values)
  EEPROM.write(1, 20);  //number 1 pr 20 store krwaya hai
  EEPROM.write(2, 30);
  Serial.println("finihed writing");

}


void loop()
{

    for (i = 0; i <= 2; i++) {
      Serial.println((EEPROM.read(i)));
      delay(1000);
    }

}

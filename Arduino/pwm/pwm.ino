int i;
void setup()
{

}


void loop()
{
    for (i = 0; i <= 1023; i+=3) {
      analogWrite(4, i);;            //pwm uses analogWrite function
      delay(1000);
    }
}

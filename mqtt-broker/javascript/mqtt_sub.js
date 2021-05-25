var mqtt = require('mqtt');
var client  = mqtt.connect('mqtt://4.tcp.ngrok.io:13622');   //  write mqtt server ip here( tcp ka url dengay aur port bhi uski dengy  tcp k url mai // say pehlay wala string hata dengay)


client.on('connect', function () {
    client.subscribe('TOPIC');    /// write your subscribe topic here        
})
client.on('message', function (topic, message) {
context = message.toString();
console.log(context)
})
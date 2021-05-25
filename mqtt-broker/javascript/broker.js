var mosca = require('mosca');
var settings = {
		port:1883  //ngrok ka tcp server is port pr run kru
}

var server = new mosca.Server(settings);

server.on('ready', function(){
console.log("ready");
});


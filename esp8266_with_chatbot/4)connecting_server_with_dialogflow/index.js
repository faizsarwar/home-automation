const {WebhookClient}=require("dialogflow-fulfillment");
const { request, response } = require("express");
const express=require("express");
// const request1 = require('request');
const app=express();

var mosca = require('mosca');
var settings = {
		port:1883  //ngrok ka tcp server is port pr run kru
}

var mqtt = require('mqtt');
var client  = mqtt.connect('mqtt://4.tcp.ngrok.io:13622');   //  write mqtt server ip here on esp8266 running


var server = new mosca.Server(settings);

server.on('ready', function(){
console.log("ready");
});





// dialogflow app pr post ki request bhejegaa

app.get("/",(req,res)=>{
    res.send("Server Is UP");
})


app.post("/webhook",express.json(),(request,response)=>{          //fulfillment mai bhi url mai /webhook lagana huga 
    const agent=new WebhookClient({request:request,response:response});
    
    function fallback(agent){
        agent.add("your bot does not understand this");
    }

    function welcome(agent){
        // agent.add("Hello Faiz welcome to your home assistant !!!");
        // agent.add("HI Faiz How can I help you");
        // agent.add("HI Faiz whats the order for me");
        agent.add("Hi Faiz this is your personal home assistant how can I help you");
        // agent.add("Hi Faiz this is your personal home assistant");
    }

    function userDetails(agent){
        // let user_name= agent.parameters["person"].name;       // is ka mtlb person ka peremeter fetch huga consoe ki trha yhn pr  // object hai isko convert krna parayga (.name  laga kr)
        let user_city=agent.parameters["geo-city"];
        let user_name=agent.parameters["person"];  
        // jb tk ye doo details nhi ayngi agay hi barayga
        agent.add('welcome to piaic Backend bot to '+user_name+"  from "+user_city);
     
    }


    function Home1(agent){
        let state =agent.parameters["state"]
        let switchnumber=agent.parameters["number"]
        agent.add("switch "+switchnumber+" is succesfully turned  "+state)
        if (state=="Off"){
            client.publish('TOPIC', '0');  // first parameter topic hai aur dosra hamara message hai
        }
        else if (state=="on"){
            client.publish('TOPIC', '1');  // first parameter topic hai aur dosra hamara message hai
        }
        console.log("turned "+ state)

    }

    function calculation(agent){
        let number1=agent.parameters["number"];
        let number2=agent.parameters["number_01"];
        let sum=number1+number2;
        agent.add("the sum is "+sum);
    }

    let intentMap= new Map();
    intentMap.set("Default Fallback Intent",fallback);    //ju name intent ka dailog flow ai huga whi dena hai ,ju function call krwana hai wo
    intentMap.set("Default Welcome Intent",welcome);
    intentMap.set("userDetails",userDetails);//
    intentMap.set("calculation",calculation);
    intentMap.set("Home1",Home1);
    agent.handleRequest(intentMap)
})

app.listen(4000,()=>{
    console.log("server is up on 4000")
})
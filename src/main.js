const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  const ip = document.querySelector("#ip");
  const username = document.querySelector("#username");
  const password = document.querySelector("#password");
  const shopId = document.querySelector("#shop_id");
  const marketingInterval = document.querySelector("#marketing_interval");

  const sync_button = document.querySelector("#sync");
  const play = document.querySelector("#play");
  const pause = document.querySelector("#pause");
  

  invoke("load").then((json_str) => {
    var result = JSON.parse(json_str);
    ip.value = result['ip'];
    username.value = result['username'];
    password.value = result['password'];
    shopId.value = result['shop_id'];
    marketingInterval.value = result['marketing_interval'];
  }).catch((error) => alert(error));

  
  sync_button.addEventListener("click", () => {
    invoke("sync", {
      ip: ip.value,
      username: username.value,
      password: password.value,
      shopId: shopId.value,
      marketingInterval: marketingInterval.value
    }).then((message) => alert(message)).catch((error) => alert(error));
  });

  play.addEventListener("click", () =>{
    invoke("play_mp3", {filePath: '../music/aboba4.mp3'}).catch((error) => alert(error));
  })
});

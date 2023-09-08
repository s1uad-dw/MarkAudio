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

  const player = document.querySelector(".player")
  const play = document.querySelector("#play");
  const pause = document.querySelector("#pause");
  
  invoke("check_sources").catch((error) => alert(error));
});
